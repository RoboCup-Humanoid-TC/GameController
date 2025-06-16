//! This module defines the logging facilites of the GameController application.

use std::path::Path;

use anyhow::{Context, Result};
use tokio::{fs::File, io::AsyncWriteExt, sync::mpsc, task::JoinSet};

use game_controller_core::log::{Logger, TimestampedLogEntry};

/// This struct defines a log that is backed by a file. The actual writing happens asynchronously
/// in a concurrent task.
pub struct FileLogger {
    /// The channel via which the GameController sends entries to the logger task.
    entry_sender: mpsc::UnboundedSender<TimestampedLogEntry>,
}

impl FileLogger {
    /// This function creates a new log file at a given path. If requested, the file will be synced
    /// to the storage medium after each added entry. The caller must supply a join set in which
    /// the worker will be spawned.
    pub async fn new<P: AsRef<Path>>(
        path: P,
        join_set: &mut JoinSet<Result<()>>,
        sync: bool,
    ) -> Result<Self> {
        let mut file = File::create(path)
            .await
            .context("could not create log file")?;
        let (entry_sender, mut entry_receiver) = mpsc::unbounded_channel();
        join_set.spawn(async move {
            while let Some(entry) = entry_receiver.recv().await {
                file.write_all(serde_yaml::to_string(&vec![&entry])?.as_bytes())
                    .await?;
                file.flush().await?;
                if sync {
                    let _ = file.sync_data().await;
                }
            }
            if sync {
                let _ = file.sync_all().await;
            }
            Ok(())
        });
        Ok(Self { entry_sender })
    }
}

impl Logger for FileLogger {
    fn append(&mut self, entry: TimestampedLogEntry) {
        let _ = self.entry_sender.send(entry);
    }
}
