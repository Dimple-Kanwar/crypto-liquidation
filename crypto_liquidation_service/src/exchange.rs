use async_trait::async_trait;
use rust_decimal::Decimal;
use crate::error::LiquidationError;

#[async_trait]
pub trait CryptoExchange {
    async fn get_balance(&self, asset: &str) -> Result<Decimal, LiquidationError>;
    async fn place_market_sell_order(&self, asset: &str, amount: Decimal) -> Result<bool, LiquidationError>;
}