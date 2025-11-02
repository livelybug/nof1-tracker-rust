use anyhow::Result;
use async_trait::async_trait;
use core_service::{FollowState, TradeRecord};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    async fn atomic_write(&self, path: &Path, data: &str) -> Result<()> {
        let tmp = path.with_extension("tmp");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let mut f = fs::File::create(&tmp).await?;
        f.write_all(data.as_bytes()).await?;
        f.flush().await?;
        drop(f);
        fs::rename(&tmp, &path).await?;
        Ok(())
    }
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_follow_state(&self, state: &FollowState) -> Result<()>;
    async fn save_trade_record(&self, record: &TradeRecord) -> Result<()>;
    async fn is_order_processed(&self, entry_oid: i64, symbol: &str) -> Result<bool>;
}

#[async_trait]
impl Storage for FileStorage {
    async fn save_follow_state(&self, state: &FollowState) -> Result<()> {
        let path = self
            .base_path
            .join("follow-state")
            .join(format!("{}-{}.json", state.agent, state.symbol));
        let s = serde_json::to_string_pretty(state)?;
        self.atomic_write(&path, &s).await?;
        Ok(())
    }

    async fn save_trade_record(&self, record: &TradeRecord) -> Result<()> {
        let path = self
            .base_path
            .join("trade-records")
            .join(format!("{}.json", record.trade_id));
        let s = serde_json::to_string_pretty(record)?;
        self.atomic_write(&path, &s).await?;
        Ok(())
    }

    async fn is_order_processed(&self, _entry_oid: i64, _symbol: &str) -> Result<bool> {
        // TODO: implement index/store for processed entry_oids
        Ok(false)
    }
}
