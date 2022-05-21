#![feature(scoped_threads)]
#![feature(let_chains)]

mod errors;
mod utils;

use anyhow::Result;
use errors::ScoopFindError;
use std::{
    env,
    sync::{Arc, Mutex},
    thread,
};
use utils::{find_manifests, get_buckets, print_result};

fn main() -> Result<()> {
    let buckets = get_buckets()?;

    let arg = env::args().nth(1).unwrap_or_default();
    let query = &arg;

    let results = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| -> Result<()> {
        for bucket in buckets {
            let results = results.clone();
            s.spawn(move || -> Result<()> {
                let manifests = find_manifests(&bucket.path(), query)?;

                if manifests.is_empty() {
                    return Ok(());
                }

                match results.lock() {
                    Ok(mut results) => results.push((
                        bucket
                            .file_name()
                            .to_str()
                            .ok_or(ScoopFindError::ScoopBadInstalled)?
                            .to_string(),
                        manifests,
                    )),
                    // Send is not implemented for MutexGuard
                    Err(_) => Err(ScoopFindError::PoinsonedMutex)?,
                }

                Ok(())
            });
        }

        Ok(())
    })?;

    match results.lock() {
        Ok(mut results) => {
            results.sort_by(|a, b| a.0.cmp(&b.0));
            for result in &*results {
                print_result(result);
            }
        }
        Err(_) => Err(ScoopFindError::PoinsonedMutex)?,
    }

    Ok(())
}
