use std::path::Path;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use tokio::{
    runtime::Handle,
    select,
    sync::mpsc::{channel, Receiver},
};
use tracing::{debug, error};

use crate::DB;

pub fn watcher() -> Result<(RecommendedWatcher, Receiver<Result<Event>>)> {
    let (tx, rx) = channel(1);

    let handle = Handle::current();

    let watcher = RecommendedWatcher::new(
        move |res| {
            handle.block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub async fn watch(path: impl AsRef<Path>) -> Result<()> {
    let (mut watcher, mut rx) = watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    loop {
        select! {
            _ = tokio::signal::ctrl_c() => return Ok(()),

            res = rx.recv() => {
                match res {
                    None => return Ok(()),
                    Some(Err(e)) => error!("watch error: {:?}", e),
                    Some(Ok(event)) => {
                        debug!("files changed: {:?}", event.paths);
                        DB.refresh().await;
                    }
                }
            }
        }
    }
}
