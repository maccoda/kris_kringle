#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::Json;
use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("client/build/index.html")
}

#[get("/<file..>", rank = 4)]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("client/build").join(file);
    NamedFile::open(path).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, files]).launch();
}
