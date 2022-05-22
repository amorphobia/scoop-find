#![feature(scoped_threads)]
#![feature(let_chains)]

mod errors;
mod known_buckets;
mod utils;

use anyhow::Result;
use errors::ScoopFindError;
use std::{
    env,
    sync::{Arc, Mutex},
    thread,
};
use utils::{find_manifests, find_remote, get_buckets, github_ratelimit_reached, print_result};

fn main() -> Result<()> {
    let buckets = get_buckets()?;
    let b = &buckets;

    let arg = env::args().nth(1).unwrap_or_default();
    let query = &arg;

    let results = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| -> Result<()> {
        for bucket in b {
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
            if results.is_empty() {
                if !github_ratelimit_reached()? {
                    let remote_res = Arc::new(Mutex::new(Vec::new()));

                    thread::scope(|s| -> Result<()> {
                        for (&bucket_name, &bucket_uri) in &known_buckets::BUCKETS {
                            if b.iter()
                                .any(|p| p.file_name().to_str().unwrap_or_default() == bucket_name)
                            {
                                continue;
                            }

                            let remote_res = remote_res.clone();
                            s.spawn(move || -> Result<()> {
                                let remote_apps = find_remote(bucket_uri, query)?;

                                if remote_apps.is_empty() {
                                    return Ok(());
                                }

                                match remote_res.lock() {
                                    Ok(mut res) => res.push((bucket_name.to_string(), remote_apps)),
                                    Err(_) => Err(ScoopFindError::PoinsonedMutex)?,
                                }

                                Ok(())
                            });
                        }

                        Ok(())
                    })?;

                    match remote_res.lock() {
                        Ok(mut res) => {
                            if res.is_empty() {
                                println!("No matches found.");
                            } else {
                                println!("Results from other known buckets...");
                                println!("(add them using 'scoop bucket add <name>')");
                                println!();

                                res.sort_by(|a, b| a.0.cmp(&b.0));

                                for res in &*res {
                                    println!(
                                        "'{}' bucket (install using 'scoop install {}/<app>'):",
                                        res.0, res.0
                                    );

                                    for app in &res.1 {
                                        println!("    {}", app);
                                    }
                                    println!();
                                }
                            }
                        }
                        Err(_) => Err(ScoopFindError::PoinsonedMutex)?,
                    };
                } else {
                    println!("No matches found.");
                }
            } else {
                results.sort_by(|a, b| a.0.cmp(&b.0));
                for result in &*results {
                    print_result(result);
                }
            }
        }
        Err(_) => Err(ScoopFindError::PoinsonedMutex)?,
    }

    Ok(())
}
