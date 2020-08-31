#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use libflategy;
use rand::prelude::*;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> Option<rocket::response::NamedFile> {
    rocket::response::NamedFile::open("static/example.html").ok()
}

fn test_world() -> libflategy::core::World {
    let mut rng = rand::thread_rng();
    let map = libflategy::core::Map::generate_2p(rng.gen());
    let mut world = libflategy::core::World::create(map);
    world.spawn(libflategy::core::PlayerID(0));
    world.spawn(libflategy::core::PlayerID(0));
    world.spawn(libflategy::core::PlayerID(0));
    world.spawn(libflategy::core::PlayerID(1));
    return world;
}

#[get("/test_map")]
fn test_map() -> Json<libflategy::render::Map> {
    let world = test_world();
    Json(libflategy::render::render_map(&world.map))
}

#[get("/test_units")]
fn test_units() -> Json<libflategy::render::Units> {
    let world = test_world();
    Json(libflategy::render::render_units(&world.units))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, test_map, test_units])
        .launch();
}
