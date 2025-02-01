use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::health::build_health_check_routers;
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

// ログ初期化など他の関数をmain関数に挟むため今のうちにサーバ起動分だけ分離
async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?; //AppConfig生成
    let pool = connect_database_with(&app_config.database); //DB接続(コネクションプールの取り出し)
    let registry = AppRegistry::new(pool); // AppRegistryを生成

    // build_health_check_routers関数の呼び出し、AppRegistryをRouterに登録
    let app = Router::new()
        .merge(build_health_check_routers())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}

//     let database_cfg = DatabaseConfig {
//         host: "localhost".into(),
//         port: 5432,
//         username: "app".into(),
//         password: "passwd".into(),
//         database: "app".into(),
//     };
//     let conn_pool = connect_database_with(database_cfg);
//
//     let app = Router::new()
//         .route("/health", get(health_check))
//         .route("/health/db", get(health_check_db))
//         // ルータのStateにプールを登録、各ハンドラで利用できるように
//         .with_state(conn_pool);
//     let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
//     let listener = TcpListener::bind(addr).await?;
//     println!("Listening on {}", addr);
//     Ok(axum::serve(listener, app).await?)
// }
//
// #[tokio::test]
// async fn health_check_works() {
//     let status_code = health_check().await;
//     assert_eq!(status_code, StatusCode::OK);
// }
//
// async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
//     let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
//     match connection_result {
//         Ok(_) => StatusCode::OK,
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
//     }
// }
//
// #[sqlx::test]
// async fn health_check_db_works(pool: sqlx::PgPool) {
//     let status_code = health_check_db(State(pool)).await;
//     assert_eq!(status_code, StatusCode::OK);
// }
