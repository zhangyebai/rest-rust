use serde::{Serialize, Deserialize};
use rbatis::crud::CRUDEnable;

#[macro_use]
extern crate rbatis_macro_driver;

#[derive(CRUDEnable, Serialize, Deserialize, Clone, Debug)]
pub struct ProvinceEntity {
    pub id: u64,
    pub province_id: String,
    pub province_name: String,
}

impl CRUDEnable for ProvinceEntity{
    type IdType = u64;

    fn table_name() -> String{
         "province".to_string()
       }
}
/*
impl<'c> sqlx::FromRow<'c, MySqlRow<'c>> for ProvinceEntity {
    fn from_row(row: &MySqlRow<'c>) -> Result<Self, Error> {
        Ok(
            ProvinceEntity {
                id: row.get("id"),
                province_id: row.get("province_id"),
                province_name: row.get("province_name"),
            }
        )
    }
}*/

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionDto{
    pub id: String,
    pub name: String,
    pub children: Vec<RegionDto>,
}

impl RegionDto{
    pub fn from_province_entity(province: &ProvinceEntity) -> RegionDto{
        RegionDto{
            id: province.province_id.clone(),
            name: province.province_name.clone(),
            children: vec![],
        }
    }
}