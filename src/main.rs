use std::{borrow::Cow, collections::BTreeSet, path::PathBuf};

use clap::Parser;
use git2::{ErrorClass, ErrorCode, Repository, TreeWalkMode, TreeWalkResult};

/// Finds all blobs within the repo larger than a cutoff (default 1KB).
///
/// Looks at all branches, does not consider "orphaned" objects.
#[derive(Debug, Parser)]
struct Args {
    /// Stop looking for blobs smaller than this in bytes
    #[arg(long, default_value_t = 1024)]
    cutoff: usize,

    /// Path to the git repository
    #[arg(long, default_value = ".")]
    repo: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut selected_blobs = BTreeSet::new();

    let repo = Repository::discover(&args.repo)?;
    repo.odb()?.foreach(|oid| {
        match repo.find_blob(*oid) {
            Err(err) if err.code() == ErrorCode::NotFound && err.class() == ErrorClass::Invalid => {
            }
            Err(err) => {
                dbg!(err.code(), err.class(), err);
            }
            Ok(blob) => {
                let size = blob.size();
                if size >= args.cutoff {
                    selected_blobs.insert((size, *oid));
                }
            }
        }

        true
    })?;

    eprintln!(
        "Found {} blobs above {} bytes, resolving to objects...",
        selected_blobs.len(),
        args.cutoff
    );

    println!(
        "size\t{blob:<40}\t{commit:<40}\tfilename",
        blob = "blob",
        commit = "commit",
    );

    // iterate largest first
    let mut it = selected_blobs.iter();
    while let Some((size, blob)) = it.next_back() {
        let mut walk = repo.revwalk()?;
        walk.push_glob("refs/*")?; // all branches
        for commit_oid in walk {
            let Ok(commit_oid) = commit_oid else { continue };
            let Ok(commit) = repo.find_commit(commit_oid) else {
                continue;
            };
            let Ok(tree) = commit.tree() else { continue };
            let _ = tree.walk(TreeWalkMode::PreOrder, |_, entry| {
                if entry.id() == *blob {
                    println!(
                        "{size}\t{blob}\t{commit}\t{filename}",
                        commit = commit_oid,
                        filename = entry
                            .name()
                            .map(Cow::Borrowed)
                            .unwrap_or_else(|| String::from_utf8_lossy(entry.name_bytes())),
                    );
                    TreeWalkResult::Abort
                } else {
                    TreeWalkResult::Ok
                }
            });
        }
    }

    Ok(())
}
