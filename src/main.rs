use dotenv::dotenv;

mod configuration;
mod errors;
mod persistence;
mod routes;
mod services;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use configuration::{appdatapool::AppDataPool, config::Config};
use mongodb::Database;
use routes::Routes;
use services::Services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let conf: Config = Config::from_env().unwrap();
    let db: Database = crate::configuration::config::get_mongo_config(&conf).await;

    let services = Services::new(&conf);
    let pool = AppDataPool::new(db, services, conf.jwt);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .configure(Routes::configure_routes)
    })
    .bind(format!("{}:{}", conf.server.host, conf.server.port))?
    .run()
    .await
}
