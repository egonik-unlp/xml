use crate::models;
use crate::models::{Aditivo, NewAditivo};
use crate::schema::aditivos;
use crate::PgConnection;
use diesel::prelude::*;

pub fn create_aditivo(
    conn: &mut PgConnection,
    categoria: models::Cat,
    name: &str,
    toxicity: f32,
    sinonimos: &str,
    info_to_report: &str,
    observaciones: &str,
) -> Aditivo {
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

pub fn get_aditivo_by_name(conn: &mut PgConnection, query_name: &str) -> Option<Aditivo> {
    use crate::schema::aditivos;
    use aditivos::dsl::*;
    aditivos.filter(name.eq(query_name)).first(conn).ok()
}
