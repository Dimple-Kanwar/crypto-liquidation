use std::collections::HashMap;
use rust_decimal::Decimal;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiquidationError {
    #[error("Exchange error: {0}")]
    ExchangeError(String),
    #[error("Insufficient balance for {0}")]
    InsufficientBalance(String),
}

#[async_trait]
pub trait CryptoExchange {
    async fn get_balance(&self, asset: &str) -> Result<Decimal, LiquidationError>;
    async fn place_market_sell_order(&self, asset: &str, amount: Decimal) -> Result<bool, LiquidationError>;
}

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

// Example implementation of CryptoExchange for testing
struct MockExchange {
    balances: HashMap<String, Decimal>,
}

#[async_trait]
impl CryptoExchange for MockExchange {
    async fn get_balance(&self, asset: &str) -> Result<Decimal, LiquidationError> {
        self.balances.get(asset)
            .cloned()
            .ok_or_else(|| LiquidationError::ExchangeError(format!("Asset not found: {}", asset)))
    }

    async fn place_market_sell_order(&self, asset: &str, amount: Decimal) -> Result<bool, LiquidationError> {
        // Simplified implementation for demonstration
        Ok(true)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mock_exchange = MockExchange {
        balances: [
            ("BTC".to_string(), Decimal::new(5000, 4)),  // 0.5 BTC
            ("ETH".to_string(), Decimal::new(10000, 3)), // 10 ETH
            ("XRP".to_string(), Decimal::new(1000, 0)),  // 1000 XRP
        ].iter().cloned().collect(),
    };

    let liquidation_service = LiquidationService::new(mock_exchange);

    let assets_to_liquidate: HashMap<String, Decimal> = [
        ("BTC".to_string(), Decimal::new(3000, 4)), // Liquidate 0.3 BTC
        ("ETH".to_string(), Decimal::new(5000, 3)), // Liquidate 5 ETH
        ("XRP".to_string(), Decimal::new(1000, 0)), // Liquidate 1000 XRP
    ].iter().cloned().collect();

    let results = liquidation_service.liquidate_assets(assets_to_liquidate, "USDT").await;

    println!("Liquidation Results:");
    for (asset, result) in results {
        match result {
            Ok(amount) => println!("{}: {}", asset, amount),
            Err(e) => println!("{}: Error - {}", asset, e),
        }
    }

    Ok(())
}