// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "cat"))]
    pub struct Cat;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Cat;

    ingrediente (id) {
        id -> Int4,
        categoria -> Cat,
        actual_name -> Nullable<Varchar>,
        info_para_reporte -> Nullable<Varchar>,
        cita -> Nullable<Varchar>,
        cancer_risk -> Nullable<Float4>,
        development_risk -> Nullable<Float4>,
        allergies_risk -> Nullable<Float4>,
        endocryne_risk -> Nullable<Float4>,
        prohibited_risk -> Nullable<Float4>,
        env_risk -> Nullable<Float4>,
        total_risk -> Nullable<Float4>,
    }
}

diesel::table! {
    rubro (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        score -> Nullable<Float4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ingrediente,
    rubro,
);
