#![feature(plugin)]
#![plugin(rocket_codegen)]

// Crate imports:

extern crate rocket;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate r2d2;
#[macro_use] extern crate log;

extern crate chrono;

// Module imports:

pub mod models;
pub mod utils;

// Code imports:

use std::option;
use std::result::Result;
use std::result::Result::*;
use std::vec::Vec;

use rocket::{Rocket, Route};
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

// Library code:

const DEFAULT_SERVICE_BASE_PATH: &'static str = "/oidcrs";

pub fn init_oidc_service(rocket: Rocket) -> Rocket {
    rocket.init_oidc()
}

pub fn init_oidc_service_at(rocket: Rocket, base: &str) -> Rocket {
    rocket.init_oidc_at(base)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Operations to start and manage an OpenID Connect service
pub trait OidcService {
    /// Initialise the OIDC service.
    /// Read the configuration, open a connection to persisted storage, and mount the service's endpoints.
    fn init_oidc(self) -> Self;

    /// Initialise the OIDC service.
    /// Read the configuration, open a connection to persisted storage, and mount the service's endpoints.
    fn init_oidc_at(self, base: &str) -> Self;
}

impl OidcService for Rocket {
    fn init_oidc(self) -> Self {
        self.init_oidc_at(DEFAULT_SERVICE_BASE_PATH)
    }

    fn init_oidc_at(self, base: &str) -> Rocket {
        self.mount(base, get_routes()).init_state()
    }
}

trait OidcServiceInternal {
    /// Initialise any shared state that the service needs for operation.
    fn init_state(self) -> Self;

    /// Mount the route endpoints that the service requires for operation.
    fn mount_routes(self) -> Self;
}

impl OidcServiceInternal for Rocket {
    fn init_state(self) -> Self {
        // Attach a fairing which adds a database pool connection to the managed state:
        self.attach(rocket::fairing::AdHoc::on_attach(|rocket| {
            let db_url_result = {                
                let config = rocket.config();
                // Written this weird way to satisfy borrowck later on that 'rocket' is not borrowed because of the call to config()
                match config.get_str("database_url") {
                    Err(ce) => Result::Err(ce),
                    Ok(s) => Ok(s.to_owned())
                }
            };

            match db_url_result {
                Err(err) => {
                    error!("Could not read database URL from configuration: {}", err);
                    Err(rocket)
                },
                Ok(url) => {
                    // Init database connection pool
                    //#[cfg(feature = "sqlite")]
                    let pool = get_pool_sqlite(&url);
                    info!("Initialised database pool at {}", &url);

                    // Add it to the managed state
                    Ok(rocket.manage(pool))
                }
            }
        }))
    }

    fn mount_routes(self) -> Self {
        self.mount("/", get_routes())
    }
}

/// Get the route endpoints that the service requires for operation
fn get_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.append(&mut routes![index]);

    routes
}

/// Get a pool connection manager to a sqlite database.
#[cfg(feature = "sqlite")]
fn get_pool_sqlite(db_url: &str) -> r2d2::Pool<ConnectionManager<SqliteConnection>> {
    let config = r2d2::Config::default();
    //let db_url = env!("OIDC_RS_DB_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}

#[cfg(feature = "dev")]
fn emit_config() {
    println!("[DEV]");
}
