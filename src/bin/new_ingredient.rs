use xml::*;

fn main() {
    let connection = &mut establish_connection();

    let post = create_ingrdiente(connection, "hello", "hello", "hello", 2., 2.,2.,2.,2.,2.,2. );
    println!("\nSaved draft {} with id {}",post.actual_name.clone().unwrap(), post.id);
    println!("{post:?}");
}

