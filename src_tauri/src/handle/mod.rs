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

#[tauri::command]
pub async fn list(db: State<'_, DatabaseConnection>) -> Result<String, ()> {
    let conn = db.inner();
    let list = TestTable::find()
        .filter(test_table::Column::IsDelete.eq(0))
        .all(conn)
        .await
        .unwrap();
    let result = serde_json::to_string(&list);
    // result.unwrap_or_else(|_| String::new())
    result.map_err(|_| ())
}

#[derive(Deserialize, Debug, DeriveIntoActiveModel)]
pub struct CardParams {
    pub color: String,
    pub content: String,
    pub create_time: String,
    pub update_time: String,
    pub status: String,
    pub is_delete: i32,
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
//
//
// async fn update(
//     db: State<'_, DatabaseConnection>
// ) -> ApiResult<ApiResponse<user::Model>> {
//     let existed_user = User::find_by_id(id)
//         .one(&db)
//         .await?
//         .ok_or_else(|| ApiError::Biz(format!("User with id {} not found", id)))?;
//     let password = params.password.clone();
//     let old_passwd = existed_user.password.clone();
//     let mut existed_active_model = existed_user.into_active_model();
//     let mut active_model = params.into_active_model();
//     existed_active_model.clone_from(&active_model);
//     // active_model.id = ActiveValue::Set(existed_user.id);
//     // unchanged 是设置为不变的意思
//     existed_active_model.id = ActiveValue::Unchanged(id);
//     if password.is_empty() {
//         // 密码为空，设置为旧密码
//         existed_active_model.password = ActiveValue::Unchanged(old_passwd);
//     } else {
//         // 密码非空，转Hash
//         existed_active_model.password = ActiveValue::Set(encode_password(&password)?);
//     }
//     let result = active_model.update(&db).await?;
//
//     Ok(ApiResponse::success(Some(result)))
// }
//
//
// async fn delete(
//     db: State<'_, DatabaseConnection>
// ) -> ApiResult<ApiResponse<()>> {
//     let existed_user = User::find_by_id(id)
//         .one(&db)
//         .await?
//         .ok_or_else(|| ApiError::Biz(format!("User with id {} not found", id)))?;
//     let result = existed_user.delete(&db).await?;
//     Ok(ApiResponse::success(None))
// }
