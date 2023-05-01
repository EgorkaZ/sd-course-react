use mongodb::options::{ServerAddress, UpdateOptions};

use super::AsyncIterator;

pub struct Mongo {
    db_name: String,
    db: mongodb::Database,
}

impl Mongo {
    pub(crate) fn create_storage(db_name: String) -> Result<Mongo, mongodb::error::Error> {
        let options = mongodb::options::ClientOptions::builder()
            .app_name(Some("react hw".to_string()))
            .hosts(vec![ServerAddress::Tcp {
                host: "localhost".into(),
                port: Some(27017),
            }])
            .build();

        let client = mongodb::Client::with_options(options)?;
        let db = client.database("react");
        Ok(Mongo { db_name, db })
    }

    pub(crate) async fn store<Value>(
        &self,
        key: u64,
        val: Value,
    ) -> Result<(), mongodb::error::Error>
    where
        Value: serde::Serialize,
    {
        println!("Mongo::store");
        let query = mongodb::bson::doc! {
            "id": key as i64
        };
        let collection = self.db.collection::<Value>(&self.db_name);
        println!("Grabbed collection \"{}\"", self.db_name);

        let update = mongodb::bson::doc! {
            "$set": mongodb::bson::to_bson(&val)?
        };

        collection
            .update_one(query, update, UpdateOptions::builder().upsert(true).build())
            .await
            .inspect_err(|err| eprintln!("Mongo couldn't insert id {key}: {err}"))
            .inspect(|res| println!("Insert res: {res:?}"))
            .map(|_| ())
    }

    pub(crate) async fn load<Value>(&self, key: u64) -> Option<Value>
    where
        for<'a> Value: serde::Deserialize<'a> + Unpin + Send + Sync,
    {
        let collection = self.db.collection::<Value>(&self.db_name);
        let query = mongodb::bson::doc! {
            "id": key as i64
        };
        collection
            .find_one(query, None)
            .await
            .inspect_err(|err| eprintln!("Mongo couldn't load key {key}: {err}"))
            .unwrap_or(None)
    }

    pub(crate) async fn load_all<Value>(
        &self,
    ) -> Result<mongodb::Cursor<Value>, mongodb::error::Error>
    where
        for<'a> Value: serde::Deserialize<'a> + Unpin + Send + Sync,
    {
        let collection = self.db.collection::<Value>(&self.db_name);
        collection
            .find(mongodb::bson::doc! {}, None)
            .await
            .inspect_err(|err| eprintln!("Mongo couldn't load values: {err}"))
    }
}

#[async_trait_fn::async_trait]
impl<T> AsyncIterator for mongodb::Cursor<T>
where
    for<'a> T: serde::Deserialize<'a> + Send,
{
    type Item = T;

    async fn next(&mut self) -> Option<T> {
        if self.advance().await.unwrap_or(false) {
            self.deserialize_current().map(Some).unwrap_or(None)
        } else {
            None
        }
    }
}
