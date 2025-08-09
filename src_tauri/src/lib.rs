mod error;
mod entity;
mod handle;
mod response;

mod id;

use crate::error::{ApiError, ApiResult};
use dirs::{data_dir, data_local_dir};
use rusqlite::Connection;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use std::path::{Path, PathBuf};
use tauri::State;
use crate::handle::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化
    init().unwrap();
    id::init().unwrap();
    let db = tauri::async_runtime::block_on(establish_connection()).unwrap();
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![greet,list,create,update,delete])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn init() -> ApiResult<()> {
    if let Some(path) = data_local_dir() {
        let path = path.join("idea_box");
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        let db_path = path.join("idea.db");
        let _conn = connect_or_create_database(&db_path)?;
        println!("成功连接到数据库: {:?}", db_path);
    } else {
        eprintln!("无法获取用户数据目录");
    }

    Ok(())
}

fn connect_or_create_database(path: &PathBuf) -> Result<Connection, rusqlite::Error> {
    if path.exists() {
        // 如果数据库文件存在，直接连接
        println!("数据库文件存在，正在连接...");
        Connection::open(path)
    } else {
        // 如果数据库文件不存在，创建并连接
        println!("数据库文件不存在，正在创建并连接...");
        let conn = Connection::open(path)?;
        // 这里可以添加初始化数据库的 SQL 语句，例如创建表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS test_table (
                id INTEGER PRIMARY KEY,
                color TEXT NULL,
                content TEXT NULL,
                createTime TEXT NULL,
                updateTime TEXT NULL,
                status INTEGER NULL,
                isDelete INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(conn)
    }
}

async fn establish_connection() -> anyhow::Result<DatabaseConnection> {
    if let Some(path) = data_local_dir() {
        let db_path = path.join("idea_box").join("idea.db");
        if let Some(path_str) = db_path.to_str() {
            let path = String::from("sqlite:///") + path_str;
            let conn = Database::connect(path)
                .await
                .expect("connection established: 连接出错");
            println!("已创建连接");
            return Ok(conn);
        }
    }
    Err(anyhow::anyhow!("数据库连接失败"))
}
