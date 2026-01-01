use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::{ContentHash, RelativePath};
use ignore::WalkBuilder;
use tokio::task::JoinSet;

use crate::builder::{ServiceContext, ServiceRegistry};
use crate::error::map_domain_error;

#[derive(Debug, Clone)]
pub struct DocumentSeed {
    pub path: RelativePath,
    pub title: String,
    pub content_hash: ContentHash,
    pub lang: Option<String>,
    pub ext: Option<String>,
}

pub struct ScanFolder;

impl ScanFolder {
    pub fn register(_ctx: &ServiceContext, registry: &mut ServiceRegistry) -> AppResult<()> {
        registry.register(Arc::new(ScanFolder));
        Ok(())
    }

    pub async fn execute(
        &self,
        root_path: String,
        extensions: Vec<String>,
    ) -> AppResult<Vec<DocumentSeed>> {
        let root = Path::new(&root_path);
        if !root.exists() || !root.is_dir() {
            return Err(AppError::new(ErrorCode::NotFound, "folder path not found"));
        }

        let normalized_exts = normalize_extensions(extensions);
        let root = root.to_path_buf();
        let files = tokio::task::spawn_blocking({
            let root = root.clone();
            let normalized_exts = normalized_exts.clone();
            move || collect_files(&root, &normalized_exts)
        })
        .await
        .map_err(|err| AppError::with_details(ErrorCode::Internal, "scan task failed", err.to_string()))??;

        let semaphore = Arc::new(tokio::sync::Semaphore::new(8));
        let mut join_set = JoinSet::new();
        for path in files {
            let permit = semaphore.clone().acquire_owned().await.map_err(|_| {
                AppError::new(ErrorCode::Internal, "scan semaphore closed")
            })?;
            let root = root.clone();
            join_set.spawn(async move {
                let _permit = permit;
                build_seed(&root, path).await
            });
        }

        let mut seeds = Vec::new();
        while let Some(result) = join_set.join_next().await {
            let seed = result.map_err(|err| {
                AppError::with_details(ErrorCode::Internal, "scan task failed", err.to_string())
            })??;
            seeds.push(seed);
        }

        Ok(seeds)
    }
}

fn normalize_extensions(extensions: Vec<String>) -> HashSet<String> {
    extensions
        .into_iter()
        .filter_map(|value| {
            let trimmed = value.trim().trim_start_matches('.');
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_lowercase())
            }
        })
        .collect()
}

fn collect_files(root: &Path, extensions: &HashSet<String>) -> AppResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root)
        .hidden(true)
        .ignore(true)
        .git_ignore(true)
        .git_exclude(true)
        .git_global(true)
        .build();
    for entry in walker {
        let entry = entry.map_err(|err| {
            AppError::with_details(ErrorCode::Io, "walk error", err.to_string())
        })?;
        if !entry.file_type().is_some_and(|ty| ty.is_file()) {
            continue;
        }
        let path = entry.path();
        let matches = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| extensions.contains(&ext.to_lowercase()))
            .unwrap_or(false);
        if matches {
            files.push(path.to_path_buf());
        }
    }
    Ok(files)
}

async fn build_seed(root: &Path, path: PathBuf) -> AppResult<DocumentSeed> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| AppError::new(ErrorCode::InvalidState, "invalid folder path"))?;
    let relative_str = relative.to_string_lossy().to_string();
    let relative_path = RelativePath::new(relative_str).map_err(map_domain_error)?;
    let content = tokio::fs::read_to_string(&path).await?;
    let content_hash = hash_content(&content)?;
    let title = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|value| value.to_string())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| relative_path.as_str().to_string());
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|value| value.to_lowercase());
    Ok(DocumentSeed {
        path: relative_path,
        title,
        content_hash,
        lang: None,
        ext,
    })
}

fn hash_content(content: &str) -> AppResult<ContentHash> {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let value = format!("{:016x}", hasher.finish());
    ContentHash::new(value).map_err(map_domain_error)
}
