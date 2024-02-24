use diesel::RunQueryDsl;
use xml::*;
use xml::models::{Cat, Ingrediente, NewIngrediente};
use xml::schema::ingredientes;
use diesel::prelude::*;


fn main() -> Result<(), diesel::result::Error> {
	let conn = &mut establish_connection();
	let mut nuevo = NewIngrediente::default() ;
	nuevo.actual_name = Some("Borohidruro");
	nuevo.allergies_risk = Some(2.1);
	nuevo.cancer_risk = Some(2.1);
	nuevo.categoria = Cat::BPC;
	nuevo.cita = Some("La RAE");
	nuevo.total_risk = Some(12.1);
	let inserted_row = diesel::insert_into(ingredientes::table)
		.values(&nuevo)
		.on_conflict(ingredientes::actual_name)
		.do_update()
		.set(&nuevo)
		.returning(Ingrediente::as_returning())
		.get_result(conn)?;
	println!("{:?}", inserted_row);
	let ingredientes_falopa = vec!["sinonimo1", "sinonimo2", "sinonimo3"];
	let final_result = create_sinonimo_ingrediente(conn, ingredientes_falopa, inserted_row);
	println!("{:?}", final_result);
	Ok(())
}