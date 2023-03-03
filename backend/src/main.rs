use actix_cors::Cors;
use actix_web::{http::header, web::Data, App, HttpServer};
use coa::{
    cancel_coa::cancel_coa, get_coa::get_coa, get_types::get_types, post_coa::post_coa,
    update_coa::update_coa,
};
use config::{get_connection, Database, JwtKey};
use leave::{
    cancel::cancel_leave, get_credits::get_credits, get_leaves::get_leaves,
    get_pending::get_pending, post_leave::post_leave, update_leave::update_leave,
};
use users::{get_info::get_info, login::emp_login};

mod coa;
mod config;
mod id_builder;
mod jwt;
mod leave;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = get_connection().await;

    let key = std::env::var("JWT_KEY").expect("NO TOKEN KEY PROVIDED");
    let main_url = std::env::var("MAIN_URL").expect("NO MAIN URL PROVIDED");
    let leave_url = std::env::var("LEAVE_URL").expect("NO LEAVE URL PROVIDED");
    let coa_url = std::env::var("COA_URL").expect("NO COA URL PROVIDED");
    let port = std::env::var("PORT").expect("NO COA URL PROVIDED");
    let host = std::env::var("HOST").expect("NO COA URL PROVIDED");

    println!("RUNNING ON {host}:{port}");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(Data::new(Database(db.clone())))
            .app_data(Data::new(JwtKey(key.to_string())))
            .service(emp_login)
            .service(get_info)
            .service(get_credits)
            .service(get_leaves)
            .service(post_leave)
            .service(cancel_leave)
            .service(update_leave)
            .service(get_pending)
            .service(get_coa)
            .service(get_types)
            .service(post_coa)
            .service(update_coa)
            .service(cancel_coa)
    })
    .bind((host, port.parse().unwrap()))?
    .run()
    .await
}
