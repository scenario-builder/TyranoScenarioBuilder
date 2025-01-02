#[macro_use] extern crate rocket;
extern crate rocket_dyn_templates;

mod models;
mod views;
mod teras;

use rocket::{Build, Rocket};

use rocket_dyn_templates::Template;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use dotenvy;
use std::env;

#[rocket::main]
async fn main() {
    // 1000パーこの辺のdb周りの処理を別のファイルに切り出した方がいいかもしれない
    let Ok(env) = dotenvy::dotenv() else {
      eprintln!(".envファイルが足りません！");
      std::process::exit(1);
    };
    let Ok(db_url) = env::var("DATABASE_URL") else {
      eprintln!(".envファイルにDATABASE_URLが設定されていません！");
      std::process::exit(1);
    };
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
      match Sqlite::create_database(&db_url).await {
        Ok(_) => (),
        Err(e) => {
          eprintln!("DBの作成に失敗しました: {}", e);
          std::process::exit(1);
        }
      }
    }
    let Ok(db) = SqlitePool::connect(&db_url).await else {
      eprintln!("DBとの接続に失敗しました");
      std::process::exit(1);
    };
    if let Err(_) = sqlx::migrate::Migrator::new(std::path::Path::new("./migrations")).await {
      eprintln!("DBのマイグレーションに失敗しました");
      std::process::exit(1);
    };
    let _ = rocket().await.launch().await.unwrap();
}

async fn rocket() -> Rocket<Build> {
    let r = rocket::build()
        .mount("/static", rocket::fs::FileServer::from("static"))
        .attach(Template::fairing());
        //.attach(Template::custom(|engine| { teras::register_tera_functions(&mut engine.tera) }))
    views::routes(r)
}