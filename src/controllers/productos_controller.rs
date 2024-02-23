use crate::models;
use crate::models::{NewProducto, Producto};
use crate::schema::productos;
use crate::PgConnection;
use diesel::prelude::*;

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

pub fn get_producto_by_name(conn: &mut PgConnection, query_descripcion: &str) -> Option<Producto> {
    use crate::schema::productos;
    use productos::dsl::*;
    productos
        .filter(descripcion.eq(query_descripcion))
        .first(conn)
        .ok()
}
