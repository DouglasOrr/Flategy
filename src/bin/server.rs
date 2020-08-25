#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Option<rocket::response::NamedFile> {
    rocket::response::NamedFile::open("static/example.html").ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
