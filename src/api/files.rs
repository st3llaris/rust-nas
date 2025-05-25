use std::{fs, os::unix::fs::MetadataExt, path::{Path, PathBuf}};
use crate::models::files::File;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

#[get("/files")]
async fn list_files() -> impl Responder {
	let path = Path::new("/home/ale");
	let mut entries = Vec::new();
    match fs::read_dir(path) {
        Ok(read_dir) => {
            for entry in read_dir.flatten() {
                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                entries.push(File {
                    name: entry.file_name().to_string_lossy().into_owned(),
					size: metadata.size(),
                    is_directory: metadata.is_dir(),
                });
            }
            HttpResponse::Ok().json(entries)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(list_files);
}
