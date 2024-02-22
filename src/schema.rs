// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "cat"))]
    pub struct Cat;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Cat;

    aditivos (id) {
        id -> Int4,
        categoria -> Nullable<Cat>,
        name -> Nullable<Varchar>,
        toxicity -> Nullable<Float4>,
        sinonimos -> Nullable<Varchar>,
        info_to_report -> Nullable<Varchar>,
        observaciones -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Cat;

    ingredientes (id) {
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
    ingredientes_productos (id) {
        id -> Int4,
        producto_id -> Int4,
        ingrediente_id -> Nullable<Int4>,
        aditivo_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Cat;

    productos (id) {
        id -> Int4,
        codigo -> Nullable<Varchar>,
        categoria -> Cat,
        descripcion -> Nullable<Varchar>,
        presentacion -> Nullable<Varchar>,
        rubro_id -> Int4,
        observaciones -> Nullable<Varchar>,
        numero_ingredientes -> Nullable<Int4>,
        score -> Nullable<Float4>,
    }
}

diesel::table! {
    rubros (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        score -> Nullable<Float4>,
    }
}

diesel::table! {
    sinonimo_ingredientes (id) {
        id -> Int4,
        ingrediente_id -> Int4,
        sinonimo -> Nullable<Varchar>,
        existe -> Nullable<Bool>,
    }
}

diesel::joinable!(ingredientes_productos -> aditivos (aditivo_id));
diesel::joinable!(ingredientes_productos -> ingredientes (ingrediente_id));
diesel::joinable!(ingredientes_productos -> productos (producto_id));
diesel::joinable!(productos -> rubros (rubro_id));
diesel::joinable!(sinonimo_ingredientes -> ingredientes (ingrediente_id));

diesel::allow_tables_to_appear_in_same_query!(
    aditivos,
    ingredientes,
    ingredientes_productos,
    productos,
    rubros,
    sinonimo_ingredientes,
);
