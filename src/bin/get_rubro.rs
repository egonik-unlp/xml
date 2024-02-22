use self::models::*;
use diesel::prelude::*;
use xml::*;

fn main() {
    use self::schema::rubros::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<Rubro> = rubros.load(connection).expect("ERROR CARGANDO DE DB");

    println!("Displaying {} posts", results.len());
    for cada_rubro in results {
        println!("{cada_rubro:?}");
    }
}
