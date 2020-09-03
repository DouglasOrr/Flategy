#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use libflategy;
use rand::prelude::*;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

// Server management

struct ActiveGames {
    counter: AtomicU64,
    games: RwLock<HashMap<u64, Arc<libflategy::core::World>>>,
}

impl ActiveGames {
    fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            games: RwLock::new(HashMap::new()),
        }
    }
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

#[derive(Debug, Serialize)]
struct NewGame {
    id: u64,
}

// Handlers

#[get("/")]
fn index() -> Option<rocket::response::NamedFile> {
    rocket::response::NamedFile::open("static/example.html").ok()
}

#[get("/new_game")] // TODO: should be POST
fn new_game(state: rocket::State<ActiveGames>) -> Json<NewGame> {
    let id = state.counter.fetch_add(1, Ordering::Relaxed);
    // TODO - convert to internal server error
    let mut games = state.games.write().expect("another thread has paniced");
    // TODO - evict old games to save memory usage
    games.insert(id, Arc::new(test_world()));
    Json(NewGame { id: id })
}

#[get("/game/<id>/map")]
fn game_map(
    id: u64,
    state: rocket::State<ActiveGames>,
) -> Result<Json<libflategy::render::Map>, status::NotFound<String>> {
    // TODO - convert to internal server error
    let games = state.games.read().expect("another thread has paniced");
    match games.get(&id) {
        Some(game) => Ok(Json(libflategy::render::render_map(&game.map))),
        None => Err(status::NotFound(format!(
            "Error 404\n\ngame id:{} not found",
            id
        ))),
    }
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
        .mount(
            "/",
            routes![index, new_game, game_map, test_map, test_units],
        )
        .mount("/static", StaticFiles::from("/static"))
        .manage(ActiveGames::new())
        .launch();
}
