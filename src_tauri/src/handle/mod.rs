use crate::entity::prelude::*;
use crate::entity::test_table;
use crate::entity::test_table::ActiveModel;
use crate::error::{ApiError, ApiResult};
use crate::response::ApiResponse;
use rusqlite::ffi::sqlite3_complete;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel, QueryOrder};
use serde::Deserialize;
use tauri::State;
use crate::id;

#[tauri::command]
pub async fn list(db: State<'_, DatabaseConnection>) -> Result<Vec<test_table::Model>, ()> {
    let conn = db.inner();
    let list = TestTable::find()
        .filter(test_table::Column::IsDelete.eq(0))
        .order_by_desc(test_table::Column::UpdateTime)
        .all(conn)
        .await
        .unwrap();
   Ok(list)
}

#[derive(Deserialize, Debug, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct CardParams {
    pub color: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub status:  Option<i8>,
}

#[tauri::command]
pub async fn create(
    db: State<'_, DatabaseConnection>,
    params: CardParams,
// ) -> ApiResult<ApiResponse<test_table::Model>> {
) -> Result<ApiResponse<test_table::Model>,()> {
    let conn = db.inner();
    let mut active_model = params.into_active_model();
    active_model.is_delete = ActiveValue::Set(0);
    let result = active_model.insert(conn).await.unwrap();
    Ok(ApiResponse::success(Some(result)))
}

#[derive(Deserialize, Debug, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct CardUpdateParams {
    pub id:i32,
    pub color: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub status:  Option<i8>,
}
#[tauri::command]
pub async fn update(
    db: State<'_, DatabaseConnection>,
    params: CardUpdateParams,
) -> Result<ApiResponse<test_table::Model>,()> {
    let conn = db.inner();
    let mut active_model = params.into_active_model();
    let result = active_model.update(conn).await.unwrap();
    Ok(ApiResponse::success(Some(result)))
}

#[tauri::command]
pub async fn delete(
    db: State<'_, DatabaseConnection>,
    id: i32
) -> Result<ApiResponse<test_table::Model>,()> {
    let conn = db.inner();
    let existed_card = TestTable::find_by_id(id)
        .one(conn)
        .await.unwrap()
        .ok_or_else(|| ApiError::Biz(format!("User with id {} not found", id))).unwrap();
    let result = existed_card.delete(conn).await.unwrap();
    Ok(ApiResponse::ok("删除成功",None))
}
