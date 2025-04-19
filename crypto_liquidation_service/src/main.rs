use std::collections::HashMap;
use rust_decimal::Decimal;
use async_trait::async_trait;
use crypto_liquidation_service::{CryptoExchange, LiquidationService, LiquidationError};

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
