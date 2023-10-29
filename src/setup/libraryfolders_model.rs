use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename(deserialize = "libraryfolders"))]
pub struct LibraryFolders {
  pub libraries: Vec<Library>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Library {
  // pub path: PathBuf,
  pub path: String,
  // pub label: String,
  // #[serde(rename = "contentid")]
  // pub content_id: i128,
  // #[serde(rename = "totalsize")]
  // pub total_size: u64,
  // pub update_clean_bytes_tally: u64,
  // pub time_last_update_corruption: u64,

  pub apps: HashMap<String, String>,
}