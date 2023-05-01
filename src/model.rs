#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    Rub = 0,
    Eur,
    Usd,
}

impl<'s> From<&'s str> for Currency {
    fn from(value: &'s str) -> Self {
        match value {
            "rub"|"Rub"|"RUB" => Currency::Rub,
            "eur"|"Eur"|"EUR" => Currency::Eur,
            "usd"|"Usd"|"USD" => Currency::Usd,
            _ => Currency::Rub,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct User {
    pub(crate) id: UserId,
    pub name: String,
    pub currency: Currency,
}

impl User {
    pub fn id(&self) -> UserId { self.id }
}

pub type UserId = u64;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Product {
    pub(crate) id: ProductId,
    pub name: String,
    pub price: Price,
}

impl Product {
    pub fn id(&self) -> ProductId { self.id }
}

pub type ProductId = u64;

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Price {
    pub(crate) value: i64,
    pub(crate) currency: Currency,
}

impl From<Price> for f64 {
    fn from(value: Price) -> Self {
        value.value as f64 / 100.
    }
}

impl From<(f64, Currency)> for Price {
    fn from((value, currency): (f64, Currency)) -> Self {
        Price { value: (value * 100.).floor() as i64, currency }
    }
}
