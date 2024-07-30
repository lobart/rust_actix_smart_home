extern crate diesel;

use crate::handlers::*;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use diesel::{connection::SimpleConnection, prelude::*, r2d2};
mod actions;
mod handlers;
mod models;
pub mod report_generator;
mod schema;
/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // initialize DB pool outside of `HttpServer::new` so that it is shared across all workers
    let pool = initialize_db_pool();
    let conn = pool.get();
    let _ = conn.unwrap().batch_execute("PRAGMA foreign_keys = 1;");

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // add DB pool handle to app data; enables use of `web::Data<DbPool>` extractor
            .app_data(web::Data::new(pool.clone()))
            // add request logger middleware
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin())
            // add route handlers
            .service(get_device)
            .service(add_device)
            .service(get_room)
            .service(add_room)
            .service(get_house)
            .service(add_house)
            .service(rem_house)
            .service(rem_room)
            .service(rem_device)
            .service(get_devices_report)
            .service(get_list_houses)
            .service(get_list_rooms)
            .service(get_list_devices)
            .service(change_state_device)
            .service(get_devices_list)
            .service(get_device_var)
            .service(get_rooms_list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);

    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};
    use uuid::Uuid;

    use super::*;

    #[actix_web::test]
    async fn device_routes() {
        dotenvy::dotenv().ok();
        env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info")).ok();

        let pool = initialize_db_pool();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .service(get_device)
                .service(add_device)
                .service(get_devices_report),
        )
        .await;

        // send something that isn't a UUID to `get_user`
        let req = test::TestRequest::get().uri("/device/123").to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = test::read_body(res).await;
        assert!(
            body.starts_with(b"UUID parsing failed"),
            "unexpected body: {body:?}",
        );

        // try to find a non-existent user
        let req = test::TestRequest::get()
            .uri(&format!("/device/{}", Uuid::nil()))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        let body = test::read_body(res).await;
        assert!(
            body.starts_with(b"No device found"),
            "unexpected body: {body:?}",
        );

        // create new user
        let req = test::TestRequest::post()
            .uri("/device")
            .set_json(models::NewDevice::new(
                "Test device",
                "Socket",
                "192.168.0.1",
                "First",
            ))
            .to_request();
        let res: models::Device = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.name, "Test device");

        // get a user
        let req = test::TestRequest::get()
            .uri(&format!("/device/{}", res.id))
            .to_request();
        let res: models::Device = test::call_and_read_body_json(&app, req).await;
        assert_eq!(res.name, "Test device");

        // delete new user from table
        use crate::schema::devices::dsl::*;
        diesel::delete(devices.filter(id.eq(res.id)))
            .execute(&mut pool.get().expect("couldn't get db connection from pool"))
            .expect("couldn't delete test device from table");
    }
}
