pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{NewRubro, Rubro};
use schema::{rubro, sql_types::Cat};
use crate::{models::Ingrediente, schema::ingrediente::dsl};
use dotenvy::dotenv;
use std::env;

use crate::models::NewIngrediente;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn create_ingrdiente(
    conn: &mut PgConnection,
    actual_name: &str,
    info_para_reporte: &str,
    cita: &str,    
    cancer_risk: f32,
    development_risk: f32,
    allergies_risk: f32,
    endocryne_risk: f32,
    prohibited_risk: f32,
    env_risk: f32,
    total_risk: f32,
) -> Ingrediente {
    use crate::schema::ingrediente;

    let mut new_entry = NewIngrediente::default();
    new_entry.actual_name = Some("Henderson");

    diesel::insert_into(ingrediente::table)
        .values(&new_entry)
        .returning(Ingrediente::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_rubro(conn: &mut PgConnection, name : &str, score : f32 ) -> Rubro {
    let mut nrubro = NewRubro::new(name , score);
    
    diesel::insert_into(rubro::table)
        .values(&nrubro)
        .returning(Rubro::as_returning())
        .get_result(conn)
        .expect("Error creando rubro {nrubro.name}")
}