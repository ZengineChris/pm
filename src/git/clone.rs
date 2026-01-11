use git2::{FetchOptions, RemoteCallbacks};
use std::path::Path;

use crate::error::Result;

pub fn clone_repository(url: &str, path: &Path, is_worktree: bool) -> Result<Option<String>> {
    if is_worktree {
        let default_branch = detect_default_branch(url)?;
        let worktree_path = path.join(&default_branch);

        std::fs::create_dir_all(&worktree_path)?;

        let mut callbacks = RemoteCallbacks::new();
        callbacks.transfer_progress(|stats| {
            if stats.received_objects() == stats.total_objects() {
                print!(
                    "Resolving deltas {}/{}\r",
                    stats.indexed_deltas(),
                    stats.total_deltas()
                );
            } else if stats.total_objects() > 0 {
                print!(
                    "Receiving objects {}/{}\r",
                    stats.received_objects(),
                    stats.total_objects()
                );
            }
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            true
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        builder.clone(url, &worktree_path)?;
        println!();

        Ok(Some(default_branch))
    } else {
        std::fs::create_dir_all(path)?;

        let mut callbacks = RemoteCallbacks::new();
        callbacks.transfer_progress(|stats| {
            if stats.received_objects() == stats.total_objects() {
                print!(
                    "Resolving deltas {}/{}\r",
                    stats.indexed_deltas(),
                    stats.total_deltas()
                );
            } else if stats.total_objects() > 0 {
                print!(
                    "Receiving objects {}/{}\r",
                    stats.received_objects(),
                    stats.total_objects()
                );
            }
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            true
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        builder.clone(url, path)?;
        println!();

        Ok(None)
    }
}

fn detect_default_branch(url: &str) -> Result<String> {
    let mut remote = git2::Remote::create_detached(url)?;
    let connection = remote.connect_auth(git2::Direction::Fetch, None, None)?;

    let default_branch = connection
        .default_branch()?
        .as_str()
        .ok_or_else(|| {
            git2::Error::from_str("Could not determine default branch")
        })?
        .to_string();

    let branch_name = default_branch
        .strip_prefix("refs/heads/")
        .unwrap_or(&default_branch)
        .to_string();

    Ok(branch_name)
}
