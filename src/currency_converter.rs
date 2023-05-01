use crate::model::{Currency, Price};

pub struct CurrencyConverter {
    quote: Currency,
    coeffitients: Vec<i64>,
}

impl CurrencyConverter {
    pub fn new(quote_currency: Currency, coeffitients: Vec<f64>) -> Self {
        let coeffitients: Vec<_> = coeffitients.into_iter()
            .map(|coef| (coef * 100.).floor() as i64)
            .collect();

        assert_eq!(coeffitients[quote_currency as usize], 100);
        CurrencyConverter { quote: quote_currency, coeffitients }
    }

    pub fn convert(&self, price: Price, to_currency: Currency) -> Price {
        let price_in_quote = Self::convert_into_quote(price, self.price_of_currency(price.currency));
        Self::convert_from_quote(price_in_quote, self.reverse_price_of_currency(to_currency))
    }

    fn convert_into_quote(price: Price, mut currency_price: Price) -> Price {
        currency_price.value *= price.value;
        currency_price.value /= 100;
        currency_price
    }

    fn convert_from_quote(mut price: Price, mut reversed_price: Price) -> Price {
        price.value *= 100;
        reversed_price.value = price.value / reversed_price.value;
        reversed_price
    }

    fn price_of_currency(&self, currency: Currency) -> Price {
        let coef = self.coeffitients[currency as usize];
        Price { value: coef, currency: self.quote }
    }

    fn reverse_price_of_currency(&self, currency: Currency) -> Price {
        let coef = self.coeffitients[currency as usize];
        Price { value: coef, currency: currency }
    }
}
