// https://github.com/notify-rs/notify/tree/main/examples/hot_reload_tide

use std::path::Path;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

pub fn file_watcher() -> Result<()> {
  // Automatically select the best implementation for your platform.
  let mut watcher = notify::recommended_watcher(|res| {
      match res {
         Ok(event) => println!("event: {:?}", event),
         Err(e) => println!("watch error: {:?}", e),
      }
  })?;

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

  Ok(())
}