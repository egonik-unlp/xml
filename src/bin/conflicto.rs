use xml::models::{NewRubro, Rubro};
// use xml::PgConnection;
use diesel::prelude::*;
use xml::schema::rubros;
use xml::*;

fn main() {
    use crate::schema::rubros;
    use rubros::dsl::*;
    let conn = &mut establish_connection();
    let mut nr = NewRubro::default();
    nr.name = Some("Esnudgfsevo");
    nr.score = Some(2.0);

    let res = diesel::insert_into(rubros::table)
        .values(&nr)
        .returning(Rubro::as_returning())
        .get_result(conn)
        .unwrap();
    println!("{:?}", res);
    let mut nr2 = nr.clone();
    nr2.score = Some(3.0);
    let res2 = diesel::insert_into(rubros::table)
        .values(vec![&nr2])
        .on_conflict(name)
        .do_update()
        .set(&nr2)
        .returning(Rubro::as_returning())
        .get_result(conn)
        .unwrap();
    println!("{:?}", res2);
}
