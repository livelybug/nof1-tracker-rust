use anyhow::Result;
use async_trait::async_trait;
use core_service::{AgentSignal, Position};

#[async_trait]
pub trait Nof1Client: Send + Sync {
    async fn get_account_totals(&self, marker: Option<u64>) -> Result<AgentSignal>;
    async fn get_available_agents(&self) -> Result<Vec<String>>;
    async fn get_agent_data(&self, agent: &str) -> Result<AgentSignal>;
}
