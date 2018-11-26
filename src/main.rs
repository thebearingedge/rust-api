#[macro_use]
extern crate serde_derive;

mod database;
mod handlers;
mod models;

use crate::database::Database;
use crate::handlers::*;
use crate::models::*;

use iron::{prelude::Chain, Iron};
use logger::Logger;
use router::Router;
use uuid::Uuid;

fn main() {
    env_logger::init();

    let (logger_before, logger_after) = Logger::new(None);
    let mut db = Database::new();

    db.add_post(Post::new(
        "The First Post",
        "This is the first post in our API",
        "Tim",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    ));

    let handlers = Handlers::new(db);
    let json_content_middleware = JsonAfterMiddleware;

    let mut router = Router::new();

    router.get("/posts", handlers.find, "posts_find");
    router.post("/posts", handlers.create, "posts_create");
    router.get("/posts/:post_id", handlers.find_by_id, "posts_find_by_id");

    let mut chain = Chain::new(router);

    chain.link_before(logger_before);
    chain.link_after(json_content_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:3000").unwrap();

    println!("Hello, world!");
}
