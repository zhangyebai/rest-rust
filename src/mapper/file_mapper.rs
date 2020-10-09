use crate::model::file_model::FileEntity;
use crate::model::Ex;
use crate::config::db_config::REGION_RB;
use rbatis::crud::CRUD;
use rbatis_core::Error;

pub async fn find_file_entity_by_file_id(file_id: &str) -> Result<Option<FileEntity>, Ex>{
    let entity = (*REGION_RB).fetch_by_wrapper::<Option<FileEntity>>("", &(*REGION_RB).new_wrapper().eq("file_id", &file_id.to_string())).await?;
    Ok(entity)
}

pub async fn save_file_entity(entity: &FileEntity) -> Result<u64, Ex>{
    let trans_id = entity.file_id.clone();
    (*REGION_RB).begin(trans_id.as_str()).await?;
    let mut r = (*REGION_RB).save("", entity).await;
    match r{
        Ok(count) => {(*REGION_RB).commit(trans_id.as_str()).await?; Ok(count)},
        Err(e) => {(*REGION_RB).rollback(trans_id.as_str()).await?; Err(e)?}
    }
}