use crate::model::file_model::{FileDto, FileEntity};
use crate::model::Ex;
use crate::mapper::file_mapper;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use crate::common;
use crate::util::string_util;
use chrono::{NaiveDateTime, Local};
use actix_web::web;
use std::io::Write;

pub async fn find_file_by_file_id(file_id: &str) -> Result<Option<FileDto>, Ex> {
    let entity = file_mapper::find_file_entity_by_file_id(file_id).await?;
    match entity {
        Some(e) => Ok(Some(FileDto::from_file_entity(&e))),
        _ => Ok(None),
    }
}

pub async fn file_upload(mut payload: Multipart) -> Result<FileDto, Ex> {
    let mut field = payload.try_next().await.unwrap().unwrap();
    let content_type = field.content_disposition().unwrap();
    let filename = content_type.get_filename().unwrap();
    let (name, suffix) = string_util::split_file_name(filename);
    let mut entity = FileEntity {
        id: 0,
        file_id: common::snow_flake::balanced_id(),
        name,
        size: 0,
        suffix,
        url: None,
        bucket: "".to_string(),
        upload_time: Local::now().naive_local(),
        completed: 1,
        completed_time: Local::now().naive_local(),
        deleted: 0,
        source: 0,
    };
    let filepath = format!("./data/{}", entity.file_id.as_str());

    let mut f = web::block(|| std::fs::File::create(filepath))
        .await
        .unwrap();


    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data = chunk?;
        entity.size += data.len() as u64;
        f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
    }
    file_mapper::save_file_entity(&entity).await?;
    Ok(FileDto::from_file_entity(&entity))
}