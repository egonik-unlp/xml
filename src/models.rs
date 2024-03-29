use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone, PartialEq, Identifiable)]
#[diesel(table_name = crate::schema::ingredientes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ingrediente {
    pub id: i32,
    pub categoria: Cat,
    pub actual_name: Option<String>,
    pub info_para_reporte: Option<String>,
    pub cita: Option<String>,
    pub cancer_risk: Option<f32>,
    pub development_risk: Option<f32>,
    pub allergies_risk: Option<f32>,
    pub endocryne_risk: Option<f32>,
    pub prohibited_risk: Option<f32>,
    pub env_risk: Option<f32>,
    pub total_risk: Option<f32>,
}

#[derive(Insertable, Default, AsChangeset)]
#[diesel(table_name = crate::schema::ingredientes)]
pub struct NewIngrediente<'a> {
    pub categoria: Cat,
    pub actual_name: Option<&'a str>,
    pub info_para_reporte: Option<&'a str>,
    pub cita: Option<&'a str>,
    pub cancer_risk: Option<f32>,
    pub development_risk: Option<f32>,
    pub allergies_risk: Option<f32>,
    pub endocryne_risk: Option<f32>,
    pub prohibited_risk: Option<f32>,
    pub env_risk: Option<f32>,
    pub total_risk: Option<f32>,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::Cat"]
pub enum Cat {
    #[db_rename = "BPC"]
    BPC,
    Foods,
    Pets,
}
impl Default for Cat {
    fn default() -> Self {
        Cat::BPC
    }
}

#[derive(Queryable, Selectable, Debug, Clone, PartialEq)]
#[diesel(table_name = crate::schema::rubros)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rubro {
    pub id: i32,
    pub name: Option<String>,
    pub score: Option<f32>,
}

#[derive(Insertable, Default, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::rubros)]
pub struct NewRubro<'a> {
    pub name: Option<&'a str>,
    pub score: Option<f32>,
}

impl<'a> NewRubro<'a> {
    pub fn new(name: &str, score: f32) -> NewRubro {
        NewRubro {
            name: Some(name),
            score: Some(score),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Associations, PartialEq)]
#[diesel(belongs_to(Ingrediente, foreign_key = ingrediente_id))]
#[diesel(table_name = crate::schema::sinonimo_ingredientes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SinonimoIngrediente {
    pub id: i32,
    pub ingrediente_id: i32,
    pub sinonimo: Option<String>,
    pub existe: Option<bool>,
}

#[derive(Insertable, AsChangeset, Debug, Default)]
#[diesel(table_name = crate::schema::sinonimo_ingredientes)]
pub struct NewSinonimoIngrediente<'a> {
    pub ingrediente_id: i32,
    pub sinonimo: Option<&'a str>,
    pub existe: Option<bool>,
}

#[derive(Queryable, Selectable, Debug, Clone, PartialEq)]
#[diesel(table_name = crate::schema::aditivos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Aditivo {
    pub id: i32,
    pub categoria: Option<Cat>,
    pub name: Option<String>,
    pub toxicity: Option<f32>,
    pub sinonimos: Option<String>,
    pub info_to_report: Option<String>,
    pub observaciones: Option<String>,
}

#[derive(Insertable, Default, AsChangeset)]
#[diesel(table_name = crate::schema::aditivos)]
pub struct NewAditivo<'a> {
    pub categoria: Option<Cat>,
    pub name: Option<&'a str>,
    pub toxicity: Option<f32>,
    pub sinonimos: Option<&'a str>,
    pub info_to_report: Option<&'a str>,
    pub observaciones: Option<&'a str>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Associations, PartialEq)]
#[diesel(belongs_to(Rubro))]
#[diesel(table_name = crate::schema::productos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Producto {
    pub id: i32,
    pub codigo: Option<String>,
    pub categoria: Cat,
    pub descripcion: Option<String>,
    pub presentacion: Option<String>,
    pub rubro_id: i32,
    pub observaciones: Option<String>,
    pub numero_ingredientes: Option<i32>,
    pub score: Option<f32>,
}

#[derive(Insertable, Default, AsChangeset)]
#[diesel(table_name = crate::schema::productos)]
pub struct NewProducto<'a> {
    pub codigo: Option<&'a str>,
    pub categoria: Cat,
    pub descripcion: Option<&'a str>,
    pub presentacion: Option<&'a str>,
    pub rubro_id: i32,
    pub observaciones: Option<&'a str>,
    pub numero_ingredientes: Option<i32>,
    pub score: Option<f32>,
}

#[derive(Queryable, Identifiable, Selectable, Debug, Clone, Associations, PartialEq)]
#[diesel(belongs_to(Producto))]
#[diesel(belongs_to(Ingrediente))]
#[diesel(belongs_to(Aditivo))]
#[diesel(primary_key(ingrediente_id, producto_id, aditivo_id))]
#[diesel(table_name = crate::schema::ingredientes_productos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IngredienteProducto {
    pub id: i32,
    pub producto_id: i32,
    pub ingrediente_id: Option<i32>,
    pub aditivo_id: Option<i32>,
}
#[derive(Debug, Insertable, Default, AsChangeset)]
#[diesel(table_name = crate::schema::ingredientes_productos)]

pub struct NewIngredienteProducto {
    pub producto_id: i32,
    pub ingrediente_id: Option<i32>,
    pub aditivo_id: Option<i32>,
}
