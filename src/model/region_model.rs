use serde::{Serialize, Deserialize};
use rbatis::crud::CRUDEnable;

/*CRUDEnable,*/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProvinceEntity {
    pub id: u64,
    pub province_id: String,
    pub province_name: String,
}

impl CRUDEnable for ProvinceEntity {
    type IdType = u64;

    fn table_name() -> String {
        "province".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CityEntity {
    pub id: u64,
    pub city_id: String,
    pub city_name: String,
    pub province_id: String,
}

impl CRUDEnable for CityEntity {
    type IdType = u64;

    fn table_name() -> String {
        "city".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CountyEntity {
    pub id: u64,
    pub county_id: String,
    pub county_name: String,
    pub city_id: String,
}

impl CRUDEnable for CountyEntity {
    type IdType = u64;

    fn table_name() -> String {
        "county".to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TownEntity {
    pub id: u64,
    pub town_id: String,
    pub town_name: String,
    pub county_id: String,
}

impl CRUDEnable for TownEntity {
    type IdType = u64;

    fn table_name() -> String {
        "town".to_string()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RegionDto {
    pub id: String,
    pub name: String,
    pub pid: Option<String>,
    pub children: Vec<RegionDto>,
}

impl RegionDto {
    pub fn from_province_entity(province: &ProvinceEntity) -> RegionDto {
        RegionDto {
            id: province.province_id.clone(),
            name: province.province_name.clone(),
            pid: None,
            children: vec![],
        }
    }

    pub fn from_city_entity(city: &CityEntity) -> RegionDto {
        RegionDto {
            id: city.city_id.clone(),
            name: city.city_name.clone(),
            pid: Some(city.province_id.clone()),
            children: vec![],
        }
    }

    pub fn from_county_entity(county: &CountyEntity) -> RegionDto {
        RegionDto {
            id: county.county_id.clone(),
            name: county.county_name.clone(),
            pid: Some(county.city_id.clone()),
            children: vec![],
        }
    }

    pub fn from_town_entity(town: &TownEntity) -> RegionDto {
        RegionDto {
            id: town.town_id.clone(),
            name: town.town_name.clone(),
            pid: Some(town.county_id.clone()),
            children: vec![],
        }
    }
}