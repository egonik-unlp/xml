pub mod controllers;
pub mod models;
pub mod schema;
pub mod utils;

pub use utils::traits::*;

pub use controllers::aditivos_controller::*;
pub use controllers::ingredientes_productos_controller::*;
pub use controllers::ingredients_controller::*;
pub use controllers::productos_controller::*;
pub use controllers::rubros_controller::*;
pub use controllers::sinonimo_ingredientes_controller::*;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
