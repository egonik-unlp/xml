use crate::models::{
    Aditivo, Ingrediente, IngredienteProducto, NewIngredienteProducto, Producto, SinonimoIngrediente
};
use crate::schema::{aditivos, ingredientes};
use crate::PgConnection;
use diesel::prelude::*;


impl IngredienteProducto {
    pub fn convert_to_ingrediente(&self, conn: &mut PgConnection) -> Option<Ingrediente> {
        ingredientes::table
            .filter(ingredientes::id.eq(self.id))
            .first(conn)
            .ok()
    }
    pub fn convert_to_aditivo(&self, conn: &mut PgConnection) -> Option<Aditivo> {
        aditivos::table
            .filter(aditivos::id.eq(self.id))
            .first(conn)
            .ok()
    }
}


pub fn create_ingrediente_producto(
    conn: &mut PgConnection,
    producto_id: i32,
    ingrediente_id: Option<i32>,
    aditivo_id: Option<i32>,
) -> IngredienteProducto {
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

pub fn get_ingredientes_producto_for_producto(conn: &mut PgConnection, producto : Producto) -> Option<Vec<IngredienteProducto>> {
    IngredienteProducto::belonging_to(&producto)
    .select(IngredienteProducto::as_select())
    .load(conn)
    .ok()
}
