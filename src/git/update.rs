use git2::{FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;

use crate::error::Result;

pub fn update_repository(path: &Path) -> Result<()> {
    let repo = Repository::open(path)?;

    let mut remote = repo.find_remote("origin")?;

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

    remote.fetch(&[] as &[&str], Some(&mut fetch_options), None)?;
    println!();

    let head = repo.head()?;
    let branch_name = head
        .shorthand()
        .ok_or_else(|| git2::Error::from_str("Could not determine current branch"))?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;

    if analysis.is_up_to_date() {
        return Ok(());
    } else if analysis.is_fast_forward() {
        let refname = format!("refs/heads/{}", branch_name);
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(fetch_commit.id(), "Fast-forward")?;
        repo.set_head(&refname)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    } else {
        return Err(git2::Error::from_str(
            "Fast-forward only merge not possible. Manual merge required.",
        )
        .into());
    }

    Ok(())
}
