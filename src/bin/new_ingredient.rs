use xml::{schema::aditivos::categoria, *};
use xml::models::Cat;
fn main() {
    let connection = &mut establish_connection();

    let post = create_ingrdiente(
        connection, "hello", Cat::BPC ,"hello", "hello", 2., 2., 2., 2., 2., 2., 2.,
    );
    println!(
        "\nSaved draft {} with id {}",
        post.actual_name.clone().unwrap(),
        post.id
    );
    println!("{post:?}");
}
