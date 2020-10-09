use crate::model::region_model::{ProvinceEntity, CityEntity, CountyEntity, TownEntity};
use crate::config::db_config::REGION_RB;
use rbatis::crud::CRUD;
use crate::model::Ex;

pub async fn list_province_entities() -> Result<Vec<ProvinceEntity>, Ex>{
    let rb = &(*REGION_RB);
    let provinces : Vec<ProvinceEntity> = rb.list_by_wrapper("", rb.new_wrapper().gt("id", 0).order_by(true, &["id"])).await?;
    Ok(provinces)
}

pub async fn list_city_entities(province_id: &str) -> Result<Vec<CityEntity>, Ex>{
    let cities: Vec<CityEntity> = (*REGION_RB).list_by_wrapper("", (*REGION_RB).new_wrapper().eq("province_id", province_id).order_by(true, &["id"])).await?;
    Ok(cities)
}

pub async fn list_county_entities_by_city_id(city_id: &str)->Result<Vec<CountyEntity>, Ex>{
    let counties: Vec<CountyEntity> = (*REGION_RB).list_by_wrapper("", (*REGION_RB).new_wrapper().eq("city_id", city_id).order_by(true, &["id"])).await?;
    Ok(counties)
}

pub async fn list_town_entities_by_county_id(county_id: &str)->Result<Vec<TownEntity>, Ex>{
    let towns: Vec<TownEntity> = (*REGION_RB).list_by_wrapper("", (*REGION_RB).new_wrapper().eq("county_id", county_id).order_by(true, &["id"])).await?;
    Ok(towns)
}