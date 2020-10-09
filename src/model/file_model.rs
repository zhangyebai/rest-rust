use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use rbatis::crud::CRUDEnable;
use crate::util::{time_util, string_util};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntity{
    pub id: u64,
    pub file_id: String,
    pub name: String,
    pub size: u64,
    pub suffix: Option<String>,
    pub url: Option<String>,
    pub bucket: String,
    pub upload_time: NaiveDateTime,
    pub completed: u8,
    pub completed_time: NaiveDateTime,
    pub deleted: u8,
    pub source: u32,
}

impl CRUDEnable for FileEntity{
    type IdType = u64;

    fn table_name() -> String{
        "file".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileDto{
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub size: u64,
    #[serde(rename="uploadTime")]
    pub upload_time: String,
}

impl FileDto{
    pub fn from_file_entity(file: & FileEntity) -> FileDto{
        FileDto{
            id: file.file_id.clone(),
            name: string_util::mix_file_name(&file.name, file.suffix.clone()),
            url: file.url.clone(),
            size: file.size,
            upload_time: time_util::date_time(file.upload_time),
        }
    }
}