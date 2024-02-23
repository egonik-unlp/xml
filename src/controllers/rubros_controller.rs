use crate::models::{Rubro, NewRubro};
use crate::PgConnection;
use crate::schema::rubros;
use diesel::prelude::*;

pub fn create_rubro(conn: &mut PgConnection, name: &str, score: f32) -> Rubro {
	let mut nrubro = NewRubro::default();
	nrubro.name = Some(name);
	nrubro.score = Some(score);
    
	diesel::insert_into(rubros::table)
	    .values(&nrubro)
	    .returning(Rubro::as_returning())
	    .get_result(conn)
	    .expect("Error creando rubro {nrubro.name}")
    }
    