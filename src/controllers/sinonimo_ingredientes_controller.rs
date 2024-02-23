use crate::models::{Ingrediente, NewSinonimoIngrediente, SinonimoIngrediente};
use crate::schema::{ingredientes, sinonimo_ingredientes};
use crate::PgConnection;
use diesel::prelude::*;

pub fn create_sinonimo_ingrediente(
    conn: &mut PgConnection,
    sinonimos: Vec<&str>,
    ingrediente: Ingrediente,
) -> Vec<SinonimoIngrediente> {
    let mut nuevos = vec![];

    let mut nsin = NewSinonimoIngrediente::default();
    for sinonimo_nuevo in sinonimos {
        nsin.ingrediente_id = ingrediente.id;
        nsin.sinonimo = Some(sinonimo_nuevo);
        nsin.existe = Some(true);
        let created_value = diesel::insert_into(sinonimo_ingredientes::table)
            .values(&nsin)
            .returning(SinonimoIngrediente::as_returning())
            .get_result(conn)
            .expect("Error creando sinonimo {nsim.sinonimo}");
        nuevos.push(created_value);
    }
    nuevos
}

pub fn get_sinonimos_for_ingrediente(
    conn: &mut PgConnection,
    ingrediente: Ingrediente,
) -> Option<Vec<SinonimoIngrediente>> {
    SinonimoIngrediente::belonging_to(&ingrediente)
        .select(SinonimoIngrediente::as_select())
        .load(conn)
        .ok()
}

pub fn get_sinonimo_ingrediente_by_name(
    conn: &mut PgConnection,
    name: &str,
) -> Option<SinonimoIngrediente> {
    sinonimo_ingredientes::table
        .filter(sinonimo_ingredientes::sinonimo.eq(name))
        .first(conn)
        .ok()
}
// pub fn create_orphan_sinonimo_ingrediente(conn: &mut PgConnection, name : &str) -> Option<SinonimoIngrediente> {
// 	let mut orphan = NewSinonimoIngrediente::default();
// 	orphan.existe = Some(false);
// 	orphan.sinonimo = Some(name);
// 	diesel::insert_into(sinonimo_ingredientes::table)
// 	.values()
// }
