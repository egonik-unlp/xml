use self::models::*;
use diesel::prelude::*;
use xml::*;

fn main() {
    use self::schema::ingredientes::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<Ingrediente> = ingredientes.load(connection).expect("ERROR CARGANDO DE DB");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{:?}", post.categoria);
        println!("-----------\n");
        println!("{}", post.actual_name.unwrap());
    }
}
