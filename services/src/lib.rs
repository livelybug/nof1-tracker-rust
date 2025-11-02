use anyhow::Result;
use async_trait::async_trait;
use core_service::{AgentSignal, Position};
use serde_json::Value;

#[async_trait]
pub trait Nof1Client: Send + Sync {
    async fn get_account_totals(&self, marker: Option<u64>) -> Result<AgentSignal>;
    async fn get_available_agents(&self) -> Result<Vec<String>>;
    async fn get_agent_data(&self, agent: &str) -> Result<AgentSignal>;
}

#[derive(Debug, Clone)]
pub struct OpenOrder {
    pub symbol: String,
    pub order_id: String,
    pub order_type: String,
    pub price: Option<f64>,
    pub qty: f64,
    pub side: String,
    pub raw: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ExchangeInfo {
    pub symbol: String,
    pub step_size: f64,
    pub tick_size: f64,
    pub min_qty: f64,
}

#[async_trait]
pub trait BinanceClient: Send + Sync {
    async fn get_positions(&self) -> Result<Vec<Position>>;
    async fn get_open_orders(&self) -> Result<Vec<OpenOrder>>;
    async fn place_limit_order(
        &self,
        symbol: &str,
        side: &str,
        price: f64,
        qty: f64,
        reduce_only: bool,
    ) -> Result<String>;
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<()>;
    async fn set_margin_type(&self, symbol: &str, margin_type: &str) -> Result<()>;
    async fn set_leverage(&self, symbol: &str, leverage: u32) -> Result<()>;
    async fn get_exchange_info(&self, symbol: &str) -> Result<ExchangeInfo>;
}
