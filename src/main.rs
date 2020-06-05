#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;
use std::io;
use std::path::{Path, PathBuf};

pub mod home_status;
pub mod schema;
pub mod db;

use rocket::response::NamedFile;
use diesel::result::Error;
use std::env;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use home_status::HomeStatus;
use db::DbConn;
use dotenv::dotenv;

#[get("/homestatus")]
fn homestatus_all(connection: DbConn) -> Result<Json<Vec<HomeStatus>>, Status> {
    HomeStatus::all(&connection)
        .map(|home_status| Json(home_status))
        .map_err(|error| error_status(error))
}

#[post("/homestatus", format = "application/json", data = "<home_status>")]
fn homestatus_post(home_status: Json<HomeStatus>, connection: DbConn) -> Result<status::Created<Json<HomeStatus>>, Status> {
    HomeStatus::insert(home_status.into_inner(), &connection)
        .map(|home_status| home_status_created(home_status))
        .map_err(|error| error_status(error))
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/",
               routes![homestatus_all,
                       homestatus_post,
                       index,
                       files]
        ).launch();
}


// handle errors
fn error_status(_error: Error) -> Status {
    return Status::NotAcceptable;
}

fn home_status_created(home_status: HomeStatus) -> status::Created<Json<HomeStatus>> {
    status::Created(
        format!("{host}:{port}/homestatus/{id}", host = host(), port = port(), id = home_status.id).to_string(),
        Some(Json(home_status)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}