use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiquidationError {
    #[error("Exchange error: {0}")]
    ExchangeError(String),
    #[error("Insufficient balance for {0}")]
    InsufficientBalance(String),
}
