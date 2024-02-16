use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::ingrediente)]
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

#[derive(Insertable, Default)]
#[diesel(table_name = crate::schema::ingrediente)]
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

#[derive(diesel_derive_enum::DbEnum, Debug, Clone)]
#[ExistingTypePath = "crate::schema::sql_types::Cat"]
pub enum Cat {
    #[db_rename="BPC"]
    BPC,
    Foods,
    Pets,
}
impl Default for Cat {
    fn default() -> Self {
        Cat::BPC
    }
}


#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::rubro)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rubro {
    pub id : i32,
    pub name : Option<String>,
    pub score : Option<f32>
}


#[derive(Insertable, Default)]
#[diesel(table_name = crate::schema::rubro)]
pub struct NewRubro<'a> {
   pub name : Option<&'a str>,
   pub score : Option<f32>
}

impl <'a> NewRubro<'a> {
    pub fn new(name:  &str, score : f32) -> NewRubro {
        NewRubro { name: Some(name), score: Some(score) }
    }
}

