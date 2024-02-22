use diesel::{insert_into, prelude::*, result};
use dotenvy::Iter;
use xml::{
    establish_connection,
    models::{self, *},
    schema::{
        self,
        ingredientes::{self, id},
        sinonimo_ingredientes,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use xml::schema::ingredientes::dsl::*;
    use xml::schema::sinonimo_ingredientes::dsl::*;
    let conn = &mut establish_connection();
    let mut ni = NewSinonimoIngrediente::default();
    let all_ing = ingredientes
        .filter(actual_name.eq("Henderson"))
        .select(Ingrediente::as_select())
        .load(conn)?;
    ni.ingrediente_id = all_ing.get(0).unwrap().id;
    ni.sinonimo = Some("Hasselbach");
    ni.existe = Some(true);

    println!("{:?}", ni);
    let new_si = insert_into(sinonimo_ingredientes)
        .values(&ni)
        .returning(SinonimoIngrediente::as_returning())
        .get_result(conn)?;
    println!("{:?}", new_si);

    Ok(())
}
