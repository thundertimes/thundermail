//! Worker Module - Background Task Loop
//!
//! This module provides background synchronization and AI processing.

#![forbid(unsafe_code)]

use super::{Account, Email, Session};
use crate::error::{Result, ThundermailError};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

/// Worker for background sync and AI tasks
pub struct Worker {
    /// Session for IMAP/SMTP
    session: Arc<RwLock<Option<Session>>>,
    /// Whether the worker is running
    is_running: bool,
    /// Sync interval in seconds
    sync_interval: u64,
}

impl Worker {
    /// Create a new worker
    pub fn new(sync_interval: u64) -> Self {
        Self {
            session: Arc::new(RwLock::new(None)),
            is_running: false,
            sync_interval,
        }
    }

    /// Start the background worker
    pub async fn start(&mut self, account: Account) -> Result<()> {
        if self.is_running {
            return Ok(());
        }

        self.is_running = true;
        let session = Session::new(account);
        *self.session.write().await = Some(session);

        // Start background tasks
        let session_clone = Arc::clone(&self.session);
        tokio::spawn(async move {
            let mut sync_timer = interval(Duration::from_secs(3600)); // 1 hour default
            
            loop {
                sync_timer.tick().await;
                
                let session = session_clone.read().await;
                if let Some(ref session) = *session {
                    if session.is_connected() {
                        // Sync emails
                        tracing::info!("Syncing emails...");
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop the background worker
    pub async fn stop(&mut self) {
        self.is_running = false;
        *self.session.write().await = None;
    }

    /// Force a sync
    pub async fn sync(&self) -> Result<()> {
        let session = self.session.read().await;
        if let Some(ref session) = *session {
            if session.is_connected() {
                // Perform sync
                tracing::info!("Manual sync triggered");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_creation() {
        let worker = Worker::new(300);
        assert!(!worker.is_running);
    }
}
