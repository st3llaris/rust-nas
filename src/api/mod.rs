use actix_web::web;

mod files;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/api").configure(files::config));
}
