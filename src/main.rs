#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate dotenv;
extern crate rocket_cors;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use diesel::result::Error;
use std::env;

use rocket::http::{Status, Method};
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use dotenv::dotenv;

pub mod home_status;
pub mod schema;
pub mod db;

use home_status::HomeStatus;
use db::DbConn;


/*--------------------------------------------------HomeStatus-Endpoints----------------------------------------------*/

#[get("/homestatus")]
fn home_status_get_all(connection: DbConn) -> Result<Json<Vec<HomeStatus>>, Status> {
    HomeStatus::all(&connection)
        .map(|home_status| Json(home_status))
        .map_err(|error| error_status(error))
}

#[get("/homestatus/<id>")]
fn home_status_get(id: i32, connection: DbConn) -> Result<Json<HomeStatus>, Status> {
    HomeStatus::get(id, &connection)
        .map(|home_status| Json(home_status))
        .map_err(|error| error_status(error))
}

#[post("/homestatus", format = "application/json", data = "<home_status>")]
fn home_status_post(home_status: Json<HomeStatus>, connection: DbConn) -> Result<status::Created<Json<HomeStatus>>, Status> {
    HomeStatus::insert(home_status.into_inner(), &connection)
        .map(|home_status| home_status_created(home_status))
        .map_err(|error| error_status(error))
}

fn home_status_created(home_status: HomeStatus) -> status::Created<Json<HomeStatus>> {
    status::Created(
        format!("{host}:{port}/homestatus/{id}", host = host(), port = port(), id = home_status.id).to_string(),
        Some(Json(home_status)))
}

#[put("/homestatus/<id>", format = "application/json", data = "<home_status>")]
fn home_status_put(id: i32, home_status: Json<HomeStatus>, connection: DbConn) -> Result<Json<HomeStatus>, Status> {
    HomeStatus::update(id, home_status.into_inner(), &connection)
        .map(|home_status| Json(home_status))
        .map_err(|error| error_status(error))
}

#[delete("/homestatus/<id>")]
fn home_status_delete(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    match HomeStatus::get(id, &connection) {
        Ok(_) => HomeStatus::delete(id, &connection)
            .map(|_| status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}

/*--------------------------------------------------------------------------------------------------------------------*/

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>", rank=1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/*-----------------------------------------------------Main-----------------------------------------------------------*/

fn main() -> Result<(), rocket_cors::Error> {
    // get env vars
    dotenv().ok();
    // specific URL 
    // let cors_allowed_url = env::var("CORS_ALLOWED_URL").expect("CORS_ALLOWED_URL must be set");
    // let allowed_origins = AllowedOrigins::some_exact(&[cors_allowed_url.as_str()]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: false,
        ..Default::default()
    }.to_cors()?;
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/",
               routes![
                    index,
                    files,
                    home_status_get_all,
                    home_status_post,
                    home_status_get,
                    home_status_put,
                    home_status_delete])
        .attach(cors)
        .launch();
    Ok(())
}


/*-----------------------------------------------------Helpers--------------------------------------------------------*/

fn error_status(_error: Error) -> Status {
    return Status::NotAcceptable;
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

/*--------------------------------------------------------------------------------------------------------------------*/