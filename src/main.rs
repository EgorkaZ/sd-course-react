use std::{sync::Arc};

use axum::{
    routing::get,
    Router, extract,
};
use reactive_hw::{requests::{self}, Manager, storage::{Storage, AsyncIterator}, model::Product};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Arc::new(Manager::with_storages(Storage::create_mongo("users".into())?, Storage::create_mongo("products".into())?));

    let app = Router::new()
        .route("/add-user", get(add_user))
        .route("/get-user", get(get_user))
        .route("/add-product", get(add_product))
        .route("/get-products", get(get_products))
        .with_state(manager);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn add_user(extract::State(manager): extract::State<Arc<Manager>>, extract::Query(user): extract::Query<requests::users::Register>) -> String {
    let add_res = manager.add_user(user.try_into().unwrap()).await;
    if add_res { "Success" } else { "Failed" }.into()
}

async fn get_user(extract::State(manager): extract::State<Arc<Manager>>, extract::Query(user): extract::Query<requests::users::Get>) -> String {
    manager.get_user(user.id).await
        .map(|user| serde_json::to_string(&user).unwrap())
        .unwrap_or_else(|| "Not found".into())
}

async fn add_product(extract::State(manager): extract::State<Arc<Manager>>, extract::Query(prod): extract::Query<requests::product::AddProduct>) -> String {
    let prod = match prod.try_into() {
        Ok(prod) => prod,
        Err(err) => return format!("couldn't parse request: {err}"),
    };
    let success = manager.add_product(prod).await;

    if success { "Success" } else { "Failed" }.into()
}

async fn get_products(extract::State(manager): extract::State<Arc<Manager>>, extract::Query(user): extract::Query<requests::product::GetAll>) -> axum::Json<Result<Vec<Product>, String>> {
    let user_id = match user.try_into() {
        Ok(user) => user,
        Err(err) => return axum::Json(Err(format!("couldn't parse request: {err}"))),
    };
    let mut prods_it = match manager.get_products(user_id).await {
        Some(prods) => prods,
        None => return axum::Json(Err(format!("couldn't get products"))),
    };
    let mut response = vec![];
    while let Some(prod) = prods_it.next().await {
        response.push(prod);
    }
    axum::Json(Ok(response))
}
