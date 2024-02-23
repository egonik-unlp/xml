use crate::models::{
    Ingrediente, IngredienteProducto, NewIngredienteProducto, SinonimoIngrediente,
};
use crate::PgConnection;
use diesel::prelude::*;

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

pub fn get_ingredientes_for_producto() -> Option<Vec<Ingrediente>> {
    todo!()
}
