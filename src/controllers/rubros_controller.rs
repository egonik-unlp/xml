use crate::models::{NewRubro, Rubro};
use crate::schema::rubros;
use crate::PgConnection;
use diesel::prelude::*;

pub fn create_rubro(conn: &mut PgConnection, name: &str, score: f32) -> Rubro {
    let mut nrubro = NewRubro::default();
    nrubro.name = Some(name);
    nrubro.score = Some(score);

    diesel::insert_into(rubros::table)
        .values(&nrubro)
        .on_conflict(rubros::name)
        .do_update()
        .set(&nrubro)
        .returning(Rubro::as_returning())
        .get_result(conn)
        .expect("Error creando rubro {nrubro.name}")
}

pub fn get_rubro_by_name(conn: &mut PgConnection, query_name: &str) -> Option<Rubro> {
    todo!();
}
