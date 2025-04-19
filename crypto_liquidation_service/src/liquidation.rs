use std::collections::HashMap;
use rust_decimal::Decimal;
use crate::error::LiquidationError;
use crate::exchange::CryptoExchange;

pub struct LiquidationService<T: CryptoExchange> {
    exchange: T,
}

impl<T: CryptoExchange> LiquidationService<T> {
    pub fn new(exchange: T) -> Self {
        Self { exchange }
    }

    pub async fn liquidate_assets(
        &self,
        assets_to_liquidate: HashMap<String, Decimal>,
        target_asset: &str,
    ) -> HashMap<String, Result<Decimal, LiquidationError>> {
        let mut results = HashMap::new();

        for (asset, amount) in assets_to_liquidate {
            if asset == target_asset {
                continue;
            }

            let result = self.liquidate_single_asset(&asset, amount).await;
            results.insert(asset, result);
        }
        results
    }

    async fn liquidate_single_asset(&self, asset: &str, amount: Decimal) -> Result<Decimal, LiquidationError> {
        let balance = self.exchange.get_balance(asset).await?;
        let amount_to_liquidate = amount.min(balance);

        if amount_to_liquidate > Decimal::ZERO {
            match self.exchange.place_market_sell_order(asset, amount_to_liquidate).await {
                Ok(true) => Ok(amount_to_liquidate),
                Ok(false) => Ok(Decimal::ZERO),
                Err(e) => Err(e),
            }
        } else {
            Ok(Decimal::ZERO)
        }
    }
}