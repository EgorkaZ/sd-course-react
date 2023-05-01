#![feature(impl_trait_in_assoc_type)]
#![feature(result_option_inspect)]
#![feature(ptr_from_ref)]

use currency_converter::CurrencyConverter;
use model::{Product, ProductId, User, UserId, Currency};
use storage::AsyncIterator;

pub mod model;
pub mod requests;
pub mod storage;
pub mod currency_converter;

pub struct Manager {
    user_storage: storage::Storage<UserId, User>,
    product_storage: storage::Storage<ProductId, Product>,
    converter: CurrencyConverter,
}

impl Manager {
    pub fn with_storages(
        user_storage: storage::Storage<UserId, User>,
        product_storage: storage::Storage<ProductId, Product>,
    ) -> Self {
        Self {
            user_storage,
            product_storage,
            converter: CurrencyConverter::new(Currency::Rub, vec![1., 80.45, 88.65])
        }
    }

    pub async fn add_user(&self, user: User) -> bool {
        println!("Manager::add_user({user:?})");
        self.user_storage.store(user.id(), user).await
    }

    pub async fn get_user(&self, id: UserId) -> Option<User> {
        println!("Manager::get_user({id})");
        self.user_storage.load(id).await
    }

    pub async fn add_product(&self, prod: Product) -> bool {
        println!("Manager::add_product({prod:?})");
        self.product_storage.store(prod.id(), prod).await
    }

    pub async fn get_products(&self, user_id: UserId) -> Option<impl AsyncIterator<Item = Product> + '_> {
        println!("Manager::get_products({user_id})");
        let currency = self.get_user(user_id).await?.currency;

        Some(self.product_storage.load_all()
            .await?
            .map(move |mut prod: Product| {
                prod.price = self.converter.convert(prod.price, currency);
                prod
            }))
    }
}
