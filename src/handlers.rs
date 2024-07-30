use crate::actions;
use crate::models;
use crate::report_generator::{generate_list_id, generate_name_id, generate_report_id};
use actix_web::{error, get, post, web, HttpResponse, Responder};
use diesel::{prelude::*, r2d2};
use uuid::Uuid;
/// Short-hand for the database pool type to use throughout the app.
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

/// Get device report.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/report/{house_uid}")]
pub async fn get_devices_report(
    pool: web::Data<DbPool>,
    house_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let house_uid = house_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let report = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::get_house_report(&mut conn, house_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(match report {
        // house was found; return 200 response with JSON formatted user object
        Ok(report) => HttpResponse::Ok().json(report),

        // House was not found; return 404 response with error message
        Err(e) => HttpResponse::NotFound().body(format!(
            "No report found for house with UID: {house_uid} and error {e}"
        )),
    })
}

/// Get devices in room.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/room/{room_uid}/list")]
async fn get_list_devices(
    pool: web::Data<DbPool>,
    room_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let room_uid = room_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let devices = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::list_device_in_room(&mut conn, room_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(match devices {
        // house was found; return 200 response with JSON formatted user object
        Ok(devices) => {
            let res = generate_list_id(devices);
            match res {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
            }
        }

        // House was not found; return 404 response with error message
        Err(e) => HttpResponse::NotFound().body(format!("No devices found with error {e}")),
    })
}

/// Get devices in room.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/house/{house_uid}/list")]
async fn get_list_rooms(
    pool: web::Data<DbPool>,
    house_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let house_uid = house_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let rooms = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::list_room_by_id(&mut conn, house_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(match rooms {
        // house was found; return 200 response with JSON formatted user object
        Ok(rooms) => {
            let res = generate_name_id(rooms);
            match res {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
            }
        }

        // House was not found; return 404 response with error message
        Err(e) => HttpResponse::NotFound().body(format!("No rooms found with error {e}")),
    })
}

/// Get houses.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/house-list")]
async fn get_list_houses(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let houses = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::list_houses(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(match houses {
        // house was found; return 200 response with JSON formatted user object
        Ok(houses) => {
            let res = generate_list_id(houses);
            match res {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
            }
        }

        // House was not found; return 404 response with error message
        Err(e) => HttpResponse::NotFound().body(format!("No houses found with error {e}")),
    })
}

#[get("/devices-list")] //todo
async fn get_devices_list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let devices = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::get_devices_list(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(match devices {
        // house was found; return 200 response with JSON formatted user object
        Ok(devices) => {
            let res = generate_list_id(devices);
            match res {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
            }
        }

        // House was not found; return 404 response with error message
        Err(e) => HttpResponse::NotFound().body(format!("No devices found with error {e}")),
    })
}

#[get("/rooms-list")] //todo
async fn get_rooms_list(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let devices = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::get_rooms_list(&mut conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(match devices {
        // house was found; return 200 response with JSON formatted user object
        Ok(devices) => {
            let res = generate_report_id(devices);
            match res {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(e) => HttpResponse::BadRequest().json(e.to_string()),
            }
        }

        // House was not found; return 404 response with error message
        Err(e) => HttpResponse::NotFound().body(format!("No rooms found with error {e}")),
    })
}

/// Finds user by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/device/{device_uid}")]
async fn get_device(
    pool: web::Data<DbPool>,
    device_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let device_uid = device_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let device = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::find_device_by_id(&mut conn, device_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match device {
        // user was found; return 200 response with JSON formatted user object
        Some(device) => HttpResponse::Ok().json(device),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {device_uid}")),
    })
}

#[get("/device/{device_uid}/var")]
async fn get_device_var(
    pool: web::Data<DbPool>,
    device_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let device_uid = device_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let device = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::find_device_by_id(&mut conn, device_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match device {
        // user was found; return 200 response with JSON formatted user object
        Some(device) => HttpResponse::Ok().json(device.variable),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {device_uid}")),
    })
}

#[get("/device/{device_uid}/state")]
async fn change_state_device(
    pool: web::Data<DbPool>,
    device_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let device_uid = device_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let device = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::update_state_device(&mut conn, device_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match device {
        // user was found; return 200 response with JSON formatted user object
        Some(device) => HttpResponse::Ok().json(device),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {device_uid}")),
    })
}

#[post("/device/{device_uid}")]
async fn post_device(
    pool: web::Data<DbPool>,
    device_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let device_uid = device_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let device = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::find_device_by_id(&mut conn, device_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match device {
        // user was found; return 200 response with JSON formatted user object
        Some(device) => HttpResponse::Ok().json(device),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {device_uid}")),
    })
}

/// Remove device by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/device/{device_uid}/remove")]
async fn rem_device(
    pool: web::Data<DbPool>,
    device_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let device_uid = device_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let device = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::remove_device_by_id(&mut conn, device_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match device {
        // user was found; return 200 response with JSON formatted user object
        Some(device) => HttpResponse::Ok().json(device),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {device_uid}")),
    })
}

/// Remove room by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/room/{room_uid}/remove")]
async fn rem_room(
    pool: web::Data<DbPool>,
    room_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let room_uid = room_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let room = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::remove_room_by_id(&mut conn, room_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match room {
        // user was found; return 200 response with JSON formatted user object
        Some(room) => HttpResponse::Ok().json(room),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {room_uid}")),
    })
}

/// Remove house by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/house/{house_uid}/remove")]
async fn rem_house(
    pool: web::Data<DbPool>,
    house_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let house_uid = house_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let house = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::remove_house_by_id(&mut conn, house_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match house {
        // user was found; return 200 response with JSON formatted user object
        Some(house) => HttpResponse::Ok().json(house),

        // user was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {house_uid}")),
    })
}

/// Finds room by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/room/{room_uid}")]
async fn get_room(
    pool: web::Data<DbPool>,
    room_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let room_uid = room_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let room = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::find_room_by_id(&mut conn, room_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match room {
        // room was found; return 200 response with JSON formatted user object
        Some(room) => HttpResponse::Ok().json(room),

        // room was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {room_uid}")),
    })
}

/// Finds house by UID.
///
/// Extracts:
/// - the database pool handle from application data
/// - a user UID from the request path
#[get("/house/{house_uid}")]
async fn get_house(
    pool: web::Data<DbPool>,
    house_uid: web::Path<Uuid>,
) -> actix_web::Result<impl Responder> {
    let house_uid = house_uid.into_inner();

    // use web::block to offload blocking Diesel queries without blocking server thread
    let house = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::find_house_by_id(&mut conn, house_uid)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    Ok(match house {
        // house was found; return 200 response with JSON formatted user object
        Some(house) => HttpResponse::Ok().json(house),

        // house was not found; return 404 response with error message
        None => HttpResponse::NotFound().body(format!("No device found with UID: {house_uid}")),
    })
}

/// Creates new device.
///
/// Extracts:
/// - the database pool handle from application data
/// - a JSON form containing new device info from the request body
#[post("/device")]
async fn add_device(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewDevice>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let device = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::insert_new_device(&mut conn, &form.name, &form.typ, &form.address, &form.room)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // deivce was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(device))
}

/// Creates new room.
///
/// Extracts:
/// - the database pool handle from application data
/// - a JSON form containing new device info from the request body
#[post("/room")]
async fn add_room(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewRoom>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let room = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::insert_new_room(&mut conn, &form.name, &form.house)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // room was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(room))
}

/// Creates new house.
///
/// Extracts:
/// - the database pool handle from application data
/// - a JSON form containing new device info from the request body
#[post("/house")]
async fn add_house(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewHouse>,
) -> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let house = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get()?;

        actions::insert_new_house(&mut conn, &form.name)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError)?;

    // house was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(house))
}
