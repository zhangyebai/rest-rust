use actix_web::{HttpResponse, web};
use crate::model::{Ex, R};
use serde::{Serialize, Deserialize};
use crate::service::file_service;
use std::io::{Write, Read};
use actix_multipart::Multipart;
use actix_web::http::header::{ContentDisposition, DispositionType, DispositionParam, ExtendedValue, Charset};
use crate::util::string_util;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileQuery {
    #[serde(rename = "fileId")]
    pub file_id: String,
}


pub async fn find_file_by_file_id(query: web::Query<FileQuery>) -> Result<HttpResponse, Ex> {
    let file = file_service::find_file_by_file_id(query.file_id.as_str()).await?;
    Ok(HttpResponse::Ok().json(R::ok(file)))
}

pub async fn file_upload(mut payload: Multipart) -> Result<HttpResponse, Ex> {
    let file = file_service::file_upload(payload).await?;
    Ok(HttpResponse::Ok().json(R::ok(Some(file))))
}

pub async fn file_download(query: web::Query<FileQuery>) -> Result<HttpResponse, Ex> {
    let file = match file_service::find_file_by_file_id(query.file_id.as_str()).await? {
        Some(f) => f,
        _ => return Err(Ex::from(std::io::Error::from(std::io::ErrorKind::NotFound))),
    };

    let cd = ContentDisposition {
        disposition: DispositionType::Attachment,

        parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
            charset: Charset::Iso_8859_1, // The character set for the bytes of the filename

            language_tag: None, // The optional language tag (see `language-tag` crate)

            value: file.name.into_bytes(), // the actual bytes of the filename
        })],
    };

    let filepath = format!("./data/{}", file.id.as_str());
    /*

    let mut f = web::block(|| std::fs::File::open(filepath))
        .await
        .unwrap();
    let mut buffer: [u8; file.size] = [0; file.size];
    ;*/
    let content = web::block(||std::fs::read(filepath)).await.unwrap();
    Ok(
        HttpResponse::Ok()
            .set_header(actix_web::http::header::CONTENT_DISPOSITION, cd)
            .message_body(actix_web::body::Body::from(content))
    )
}