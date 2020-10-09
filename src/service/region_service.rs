use crate::model::region_model::RegionDto;
use crate::mapper::region_mapper;
use crate::model::Ex;


pub async fn list_provinces()->Result<Vec<RegionDto>, Ex>{
    let entities = region_mapper::list_province_entities().await?;
    let ps: Vec<RegionDto> = entities.iter().map(|p|{RegionDto::from_province_entity(&p)}).collect();
    Ok(ps)
}


pub async fn list_cities_by_province_id(province_id: &str)->Result<Vec<RegionDto>, Ex>{
    let entities = region_mapper::list_city_entities(province_id).await?;
    let cs: Vec<RegionDto> = entities.iter().map(|c|{RegionDto::from_city_entity(c)}).collect();
    Ok(cs)
}

pub async fn list_counties_by_city_id(city_id: &str) -> Result<Vec<RegionDto>, Ex>{
    let entities = region_mapper::list_county_entities_by_city_id(city_id).await?;
    let cs: Vec<RegionDto> = entities.iter().map(|c|{RegionDto::from_county_entity(c)}).collect();
    Ok(cs)
}

pub async fn list_towns_by_county_id(county_id: &str) -> Result<Vec<RegionDto>, Ex>{
    let entities = region_mapper::list_town_entities_by_county_id(county_id).await?;
    let ts: Vec<RegionDto> = entities.iter().map(|t|{RegionDto::from_town_entity(t)}).collect();
    Ok(ts)
}