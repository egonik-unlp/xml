use crate::models::{Ingrediente, NewIngrediente};
use crate::PgConnection;
use diesel::prelude::*;

pub fn create_ingrdiente(
    conn: &mut PgConnection,
    actual_name: &str,
    info_para_reporte: &str,
    cita: &str,
    cancer_risk: f32,
    development_risk: f32,
    allergies_risk: f32,
    endocryne_risk: f32,
    prohibited_risk: f32,
    env_risk: f32,
    total_risk: f32,
) -> Ingrediente {
    use crate::schema::ingredientes;

    let mut new_entry = NewIngrediente::default();
    new_entry.actual_name = Some(actual_name);
    new_entry.info_para_reporte = Some(info_para_reporte);
    new_entry.cita = Some(cita);
    new_entry.cancer_risk = Some(cancer_risk);
    new_entry.development_risk = Some(development_risk);
    new_entry.allergies_risk = Some(allergies_risk);
    new_entry.endocryne_risk = Some(endocryne_risk);
    new_entry.prohibited_risk = Some(prohibited_risk);
    new_entry.env_risk = Some(env_risk);
    new_entry.total_risk = Some(total_risk);
    diesel::insert_into(ingredientes::table)
        .values(&new_entry)
        .returning(Ingrediente::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
pub fn get_ingredient_by_name(conn: &mut PgConnection, name: &str) -> Vec<Ingrediente> {
    use crate::schema::ingredientes;
    use ingredientes::dsl::*;
    ingredientes
        .filter(actual_name.eq(name))
        .load(conn)
        .expect("ANDA ETO?")
}
