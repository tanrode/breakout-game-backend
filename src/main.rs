use db::config;
use actix_web::{web, App, HttpServer};    
use server::routes;
mod db;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    config::load_env();
    let pool = db::db_connect::create_db_pool().await;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::health_check)
            .service(routes::get_users)
            .service(routes::get_or_add_user)
            .service(routes::get_or_add_user_1)
            .service(routes::get_stats)
            .service(routes::get_leaderboard)
            .service(routes::update_stats)
    }).bind(("127.0.0.1", 8080))?.run();

    println!("Server running at http://127.0.0.1:8080/");
    let _ = server.await;

    Ok(())
}
