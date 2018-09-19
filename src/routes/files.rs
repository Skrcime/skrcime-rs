use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
