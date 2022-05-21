use crate::errors::ScoopFindError;
use anyhow::Result;
use serde_json::{json, Value};
use std::{
    env,
    fs::{DirEntry, File},
    io::BufReader,
    path::{Path, PathBuf},
};

fn scoop_home() -> Result<PathBuf, ScoopFindError> {
    if let Some(scoop_home) = env::var_os("SCOOP")
        && let Some(scoop_home) =
            env::split_paths(&scoop_home)
                .find_map(|dir| if dir.is_dir() { Some(dir) } else { None }) {
        return Ok(scoop_home);
    }

    if let Some(user_home) = env::var_os("USERPROFILE") {
        let mut scoop_home = PathBuf::from(user_home);
        scoop_home.push("scoop");
        if scoop_home.is_dir() {
            return Ok(scoop_home);
        }
    }

    Err(ScoopFindError::ScoopHomeNotFound)
}

pub fn get_buckets() -> Result<Vec<DirEntry>> {
    let mut buckets_path = scoop_home()?;
    buckets_path.push("buckets");
    if !buckets_path.is_dir() {
        Err(ScoopFindError::ScoopBadInstalled)?;
    }

    let dirs = buckets_path
        .read_dir()?
        .filter_map(|entry| {
            if let Ok(entry) = entry && entry.path().is_dir() {
                Some(entry)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(dirs)
}

fn contains_query<'json, 'q>(val: &'json Value, query: &'q str) -> Option<&'json str> {
    if let Value::String(val) = val {
        let path = Path::new(val);
        if let Some(stem) = path.file_stem()
            && let Some(stem) = stem.to_str()
                && stem.to_ascii_lowercase().contains(query)
                    && let Some(name) = path.file_name()
        {
            return name.to_str();
        }
    }

    None
}

pub fn find_manifests(bucket_path: &Path, query: &str) -> Result<Vec<(String, String, String)>> {
    let query = query.to_ascii_lowercase();
    let manifests = bucket_path
        .join("bucket")
        .read_dir()?
        .filter_map(|file| {
            if let Ok(file) = file {
                if let Some(ext) = file.path().extension() {
                    if file.path().is_file() && ext == "json" {
                        return Some(file.path());
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();

    let mut results = Vec::new();

    for manifest in manifests {
        let manifest = manifest.as_path();
        let file = File::open(manifest)?;
        let reader = BufReader::new(file);

        let root: Value = serde_json::from_reader(reader)?;

        let version = root
            .get("version")
            .unwrap_or(&json!(null))
            .as_str()
            .unwrap_or_default();

        let name = manifest
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        if version.is_empty() || name.is_empty() {
            continue;
        }

        if name.to_ascii_lowercase().contains(&query) {
            results.push((name.to_string(), version.to_string(), "".to_string()));
        } else if let Some(val) = root.get("bin") {
            if let Some(bin) = contains_query(val, &query) {
                results.push((name.to_string(), version.to_string(), bin.to_string()));
            } else if let Value::Array(vals) = val {
                'bins: for val in vals {
                    if let Some(bin) = contains_query(val, &query) {
                        results.push((name.to_string(), version.to_string(), bin.to_string()));
                        break 'bins;
                    } else if let Value::Array(vals) = val {
                        let vals = vals.iter().take(2);
                        for val in vals {
                            if let Some(bin) = contains_query(val, &query) {
                                results.push((
                                    name.to_string(),
                                    version.to_string(),
                                    bin.to_string(),
                                ));
                                break 'bins;
                            }
                        }
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| {
        let mut a = a.0.to_ascii_lowercase();
        let mut b = b.0.to_ascii_lowercase();
        a.retain(|c| c != '-');
        b.retain(|c| c != '-');
        a.cmp(&b)
    });

    Ok(results)
}

pub fn print_result(result: &(String, Vec<(String, String, String)>)) {
    if result.1.is_empty() {
        return;
    }

    println!("'{}' bucket:", result.0);

    for app in &result.1 {
        print!("    {} ({})", app.0, app.1);
        if app.2.is_empty() {
            println!();
        } else {
            println!(" --> includes '{}'", app.2);
        }
    }
    println!();
}
