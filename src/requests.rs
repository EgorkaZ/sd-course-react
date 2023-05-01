use crate::model::{Currency, Product, User, UserId};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub enum UserRequest {
    GetAll,
    Register(User),
    Get(UserId),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ProductRequest {
    GetAll(Currency),
    AddProduct(Product),
}

pub mod users {
    use std::str::FromStr;

    use crate::model::{User, UserId};

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
    pub struct GetAll;

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    pub struct Register {
        id: String,
        name: String,
        currency: String,
    }

    impl TryFrom<Register> for User {
        type Error = <u64 as FromStr>::Err;

        fn try_from(value: Register) -> Result<Self, Self::Error> {
            let id = value.id.parse()?;
            let currency = (&value.currency[..]).into();
            Ok(Self {
                id,
                name: value.name,
                currency: currency,
            })
        }
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
    pub struct Get {
        pub id: UserId,
    }
}

pub mod product {
    use std::str::FromStr;

    use crate::model::{Product, Currency, UserId};

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
    pub struct GetAll {
        pub user_id: String,
    }

    impl TryFrom<GetAll> for UserId {
        type Error = <UserId as FromStr>::Err;

        fn try_from(value: GetAll) -> Result<Self, Self::Error> {
            value.user_id.parse()
        }
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
    pub struct AddProduct {
        pub id: String,
        pub name: String,
        pub price: String,
        pub currency: String,
    }

    impl TryFrom<AddProduct> for Product {
        type Error = Box<dyn std::error::Error>;

        fn try_from(value: AddProduct) -> Result<Self, Self::Error> {
            Ok(Product {
                id: value.id.parse()?,
                name: value.name,
                price: (value.price.parse::<f64>()?, Currency::from(&value.currency[..])).into(),
            })
        }
    }
}
