use actix_web::{HttpResponse, web};
use crate::model::{Ex, R};
use serde::{Serialize, Deserialize};
use crate::service::file_service;
use std::io::Write;
use actix_multipart::Multipart;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileQuery{

    #[serde(rename="fileId")]
    pub file_id: String,
}


pub async fn find_file_by_file_id(query: web::Query<FileQuery>) -> Result<HttpResponse, Ex>{
    let file = file_service::find_file_by_file_id(query.file_id.as_str()).await?;
    Ok(HttpResponse::Ok().json(R::ok(file)))
}

pub async fn file_upload(mut payload: Multipart) -> Result<HttpResponse, Ex>{
    let file = file_service::file_upload(payload).await?;
    Ok(HttpResponse::Ok().json(R::ok(Some(file))))
}