use crate::models::{Ingrediente, NewSinonimoIngrediente, SinonimoIngrediente};
use crate::PgConnection;
use crate::schema::sinonimo_ingredientes;
use diesel::prelude::*;

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

    pub fn get_sinonimos_for_ingrediente(conn: &mut PgConnection, ingrediente : Ingrediente) -> Option<Vec<SinonimoIngrediente>> {
	todo!()
    } 