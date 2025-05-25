use serde::Serialize;

#[derive(Serialize)]
pub struct File {
  pub name: String,
  pub size: u64,
  pub is_directory: bool
}
