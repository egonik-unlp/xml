use self::models::*;
use diesel::prelude::*;
use xml::{schema::rubro::name, *};



fn main() -> Result<(), Box<dyn std::error::Error>>{
	use self::schema::rubro::dsl::*;
	let conn = &mut establish_connection();
	let nrubro = "Peterson";
	let scrubro = 0f32;
	create_rubro(conn, "Peter", 0.021);
	create_rubro(conn, "Peter", 0.0221);

	let results = rubro
		.filter(name.eq(nrubro))
		.select(Rubro::as_select())
		.load(conn)
		.optional()?;
	match results {
		Some(dato) => println!("Existe {dato:?}"),
		None => {create_rubro(conn, nrubro, scrubro);()},
	}
	// println!("{:?}", results);

	Ok(())
}