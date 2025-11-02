use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSignal {
    pub agent: String,
    pub marker: Option<u64>,
    pub positions: Vec<Position>,
    pub raw: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_oid: i64,
    pub entry_price: f64,
    pub current_price: f64,
    pub leverage: u32,
    pub margin: f64,
    pub exit_plan: Option<ExitPlan>,
    pub tp_oid: Option<i64>,
    pub sl_oid: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitPlan {
    pub profit_target: Option<f64>,
    pub stop_loss: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowPlan {
    pub action: FollowAction,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub leverage: u32,
    pub reason: String,
    pub agent: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FollowAction {
    Enter,
    Exit,
    Replace,
    Noop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowState {
    pub agent: String,
    pub symbol: String,
    pub entry_oid: i64,
    pub side: Side,
    pub size: f64,
    pub leverage: u32,
    pub margin_type: Option<String>,
    pub binance_order_ids: Vec<String>,
    pub opened_at: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeRecord {
    pub trade_id: String,
    pub agent: String,
    pub symbol: String,
    pub side: Side,
    pub size: f64,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub realized_pnl: Option<f64>,
    pub fees: Option<f64>,
    pub timestamps: Vec<DateTime<Utc>>,
    pub exit_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceToleranceResult {
    pub should_execute: bool,
    pub price_difference_pct: f64,
    pub tolerance_pct: f64,
}

pub type SymbolToleranceMap = HashMap<String, f64>;
