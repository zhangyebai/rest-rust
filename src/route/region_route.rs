use actix_web::{HttpResponse, web, HttpRequest};
use crate::model::{Ex, R};
use crate::config::db_config::{MultiMySqlPool, REGION_RB};
use crate::model::region_model::{RegionDto};
use crate::service::region_service;
use log::info;
use rbatis::crud::CRUD;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionQuery{
    pub pid: String,
}

pub async fn list_simple_provinces() -> Result<HttpResponse, Ex> {
    let ps = region_service::list_provinces().await?;
    Ok(HttpResponse::Ok().json(R::ok(Some(ps))))
}

pub async fn list_simple_cities_by_province_id(query: web::Query<RegionQuery>) -> Result<HttpResponse, Ex>{
    let cs = region_service::list_cities_by_province_id(query.pid.as_str()).await?;
    Ok(HttpResponse::Ok().json(R::ok(Some(cs))))
}

pub async fn list_simple_counties_by_city_id(query: web::Query<RegionQuery>) -> Result<HttpResponse, Ex>{
    let cs = region_service::list_counties_by_city_id(query.pid.as_str()).await?;
    Ok(HttpResponse::Ok().json(R::ok(Some(cs))))
}

pub async fn list_simple_towns_by_county_id(query: web::Query<RegionQuery>)->Result<HttpResponse, Ex>{
    let ts = region_service::list_towns_by_county_id(query.pid.as_str()).await?;
    Ok(HttpResponse::Ok().json(R::ok(Some(ts))))
}