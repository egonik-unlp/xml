use self::models::*;
use diesel::{prelude::*, result};
use xml::{schema::rubros::name, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::rubros::dsl::*;
    let conn = &mut establish_connection();
    let nrubro = "Poronga";
    let scrubro = 0f32;
    create_rubro(conn, "Peter", 0.021);
    create_rubro(conn, "Peter", 0.0221);
    // Hola ceci como andas
    let results = rubros
        .filter(name.eq(nrubro))
        .select(Rubro::as_select())
        .load(conn)?;
    if results.len() > 0 {
        println!("Existe {:?}", results)
    } else {
        let nrubro = create_rubro(conn, nrubro, scrubro);
        println!("{:?}", nrubro);
    }
    println!("{:?}", results.len());

    Ok(())
}
