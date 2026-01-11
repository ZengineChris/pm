use git2::Repository;
use std::path::Path;

use crate::error::Result;

#[derive(Debug)]
pub struct RepoStatus {
    pub has_changes: bool,
    pub staged: usize,
    pub modified: usize,
    pub untracked: usize,
}

pub fn get_repository_status(path: &Path) -> Result<RepoStatus> {
    let repo = Repository::open(path)?;
    let statuses = repo.statuses(None)?;

    let mut staged = 0;
    let mut modified = 0;
    let mut untracked = 0;

    for entry in statuses.iter() {
        let status = entry.status();

        if status.is_index_new()
            || status.is_index_modified()
            || status.is_index_deleted()
            || status.is_index_renamed()
            || status.is_index_typechange()
        {
            staged += 1;
        }

        if status.is_wt_modified()
            || status.is_wt_deleted()
            || status.is_wt_renamed()
            || status.is_wt_typechange()
        {
            modified += 1;
        }

        if status.is_wt_new() {
            untracked += 1;
        }
    }

    Ok(RepoStatus {
        has_changes: staged > 0 || modified > 0 || untracked > 0,
        staged,
        modified,
        untracked,
    })
}
