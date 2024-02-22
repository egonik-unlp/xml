use xml::*;

fn main() {
    let connection = &mut establish_connection();
    let post = create_rubro(connection, "Johnny", 0.20);
    println!("\nSaved draft with id {}", post.id);
    println!("{post:?}");
}
