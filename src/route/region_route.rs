use actix_web::{HttpResponse, web};
use crate::model::{Ex, R};
use crate::config::db_config::MultiMySqlPool;
use crate::model::region_model::{ProvinceEntity, RegionDto};
use log::info;
use rbatis::crud::CRUD;

pub async fn list_simple_province(multi_pool: web::Data<MultiMySqlPool>) -> Result<HttpResponse, Ex> {
    let rb = &multi_pool.region_pool;
    let provinces : Vec<ProvinceEntity> = rb.list_by_wrapper("", rb.new_wrapper().order_by(true, &["id"])).await?;
    /*
    let mut conn = multi_pool.region_pool.get_conn().await?;
    let provinces = conn.exec_map("SELECT id, province_id, province_name FROM province ORDER BY id", (), |(id, province_id, province_name)|{
        ProvinceEntity{
            id,
            province_id,
            province_name,
        }
    }).await?;*/
    let ps: Vec<RegionDto> = provinces.iter().map(|p|{RegionDto::from_province_entity(&p)}).collect();
    Ok(HttpResponse::Ok().json(R::ok(Some(ps))))
}