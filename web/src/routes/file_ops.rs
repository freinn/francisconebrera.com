use rocket::Data;
use rocket::response::NamedFile; // ficheros estáticos
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf}; // para static files

#[get("/<file..>", rank = 5)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}
