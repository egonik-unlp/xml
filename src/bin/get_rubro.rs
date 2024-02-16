use self::models::*;
use diesel::prelude::*;
use xml::*;

fn main() {
    use self::schema::rubro::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<Rubro> = rubro.load(connection).expect("ERROR CARGANDO DE DB");

    println!("Displaying {} posts", results.len());
    for rubro in results {
        println!("{rubro:?}");
    }
}
