use rocket::{routes, ignite, get, response::NamedFile};
use std::io::Result;
use std::path::{Path, PathBuf};

fn get_path() -> String {
    format!("{}/../frontend/build", env!("CARGO_MANIFEST_DIR"))
}

#[get("/")]
fn index_file() -> Result<NamedFile> {
    return NamedFile::open(Path::new(&get_path()).join("index.html"));
}

#[get("/<file..>")]
fn pathed_file(file: PathBuf) -> Result<NamedFile> {
    return NamedFile::open(Path::new(&get_path()).join(file));
}

pub fn start() {
    println!("Starting server...");
    ignite().mount("/", routes![index_file, pathed_file]).launch();
}