pub mod models;
pub mod schema;

use crate::models::{Aditivo, NewAditivo};
use crate::models::{IngredienteProducto, NewIngredienteProducto};
use crate::models::{NewProducto, Producto};
use crate::{models::Ingrediente, schema::ingredientes::dsl};
use diesel::prelude::*;
use diesel::{dsl::NotSimilarTo, pg::PgConnection};
use dotenvy::dotenv;
use models::{NewRubro, NewSinonimoIngrediente, Rubro, SinonimoIngrediente};
use schema::{rubros, sinonimo_ingredientes, sql_types::Cat};
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
    use crate::schema::ingredientes;

    let mut new_entry = NewIngrediente::default();
    new_entry.actual_name = Some("Henderson");

    diesel::insert_into(ingredientes::table)
        .values(&new_entry)
        .returning(Ingrediente::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_rubro(conn: &mut PgConnection, name: &str, score: f32) -> Rubro {
    let mut nrubro = NewRubro::new(name, score);

    diesel::insert_into(rubros::table)
        .values(&nrubro)
        .returning(Rubro::as_returning())
        .get_result(conn)
        .expect("Error creando rubro {nrubro.name}")
}

pub fn create_sinonimo_ingrediente(
    conn: &mut PgConnection,
    sinonimo: &str,
    ingrediente_id: i32,
    existe: bool,
) -> SinonimoIngrediente {
    let mut nsin = NewSinonimoIngrediente::default();
    nsin.ingrediente_id = ingrediente_id;
    nsin.sinonimo = Some(sinonimo);
    nsin.existe = Some(existe);
    diesel::insert_into(sinonimo_ingredientes::table)
        .values(&nsin)
        .returning(SinonimoIngrediente::as_returning())
        .get_result(conn)
        .expect("Error creando sinonimo {nsim.sinonimo}")
}

pub fn create_aditivo(
    conn: &mut PgConnection,
    categoria: models::Cat,
    name: &str,
    toxicity: f32,
    sinonimos: &str,
    info_to_report: &str,
    observaciones: &str,
) -> Aditivo {
    use crate::schema::aditivos;
    let mut nad = NewAditivo::default();
    nad.categoria = Some(categoria);
    nad.name = Some(name);
    nad.toxicity = Some(toxicity);
    nad.sinonimos = Some(sinonimos);
    nad.info_to_report = Some(info_to_report);
    nad.observaciones = Some(observaciones);
    diesel::insert_into(aditivos::table)
        .values(&nad)
        .returning(Aditivo::as_returning())
        .get_result(conn)
        .expect("Error agregando aditivo")
}

pub fn create_producto(
    conn: &mut PgConnection,
    codigo: &str,
    categoria: models::Cat,
    descripcion: &str,
    presentacion: &str,
    rubro_id: i32,
    observaciones: &str,
    numero_ingredientes: i32,
    score: f32,
) -> Producto {
    use crate::schema::productos;
    let mut new_product = NewProducto::default();
    new_product.codigo = Some(codigo);
    new_product.categoria = categoria;
    new_product.descripcion = Some(descripcion);
    new_product.presentacion = Some(presentacion);
    new_product.rubro_id = rubro_id;
    new_product.observaciones = Some(observaciones);
    new_product.numero_ingredientes = Some(numero_ingredientes);
    new_product.score = Some(score);
    diesel::insert_into(productos::table)
        .values(&new_product)
        .returning(Producto::as_returning())
        .get_result(conn)
        .expect("Error insertando nuevo producto")
}

pub fn create_ingrediente_producto(conn: &mut PgConnection, producto_id : i32, ingrediente_id: Option<i32>, aditivo_id : Option<i32>) -> IngredienteProducto {
    use crate::schema::ingredientes_productos;
    let mut new_ingrediente_producto = NewIngredienteProducto::default();
    new_ingrediente_producto.producto_id = producto_id;
    new_ingrediente_producto.ingrediente_id = ingrediente_id;
    new_ingrediente_producto.aditivo_id = aditivo_id;
    diesel::insert_into(ingredientes_productos::table)
        .values(&new_ingrediente_producto)
        .returning(IngredienteProducto::as_returning())
        .get_result(conn)
        .expect("Error Agregando IngredienteProducto") 
}