use diesel::prelude::*;
use xml::{models::{Ingrediente, SinonimoIngrediente}, schema::{ingredientes, sinonimo_ingredientes}, *};
use diesel::result::Error;
/// Prueba si andaw el inner_join
fn main() -> Result<(), Error> {
	let conn = &mut establish_connection();
	let join = sinonimo_ingredientes::table
		.inner_join(ingredientes::table)
		.filter(ingredientes::actual_name.eq("Borohidruro"))
		.select((SinonimoIngrediente::as_select(), Ingrediente::as_select()))
		.load::<(SinonimoIngrediente, Ingrediente)>(conn)?;
	println!("{:?}", join);
	let ing = &join.get(0).ok_or(Error::NotFound)?.1;

	println!("=>{:?}",ing);
	let fetch_ingredientes = SinonimoIngrediente::belonging_to(ing).select(SinonimoIngrediente::as_select()).load(conn)?;
	println!("\n\n{:?}", fetch_ingredientes);
	Ok(())
}