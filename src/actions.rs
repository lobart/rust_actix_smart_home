use crate::models;
use crate::report_generator::generate_report;
use diesel::prelude::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Run query using Diesel to find device by uid and return it.
pub fn find_device_by_id(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::Device>, DbError> {
    use crate::schema::devices::dsl::*;

    let device = devices
        .filter(id.eq(uid.to_string()))
        .first::<models::Device>(conn)
        .optional()?;

    Ok(device)
}

/// Run query using Diesel to find device by uid and return it.
pub fn get_devices_list(conn: &mut SqliteConnection) -> Result<Vec<models::Device>, DbError> {
    use crate::schema::devices::dsl::*;

    let devices_list = devices.load::<models::Device>(conn)?;

    Ok(devices_list)
}

pub fn get_rooms_list(conn: &mut SqliteConnection) -> Result<Vec<models::Room>, DbError> {
    use crate::schema::rooms::dsl::*;

    let rooms_list = rooms.load::<models::Room>(conn)?;

    Ok(rooms_list)
}

/// Run query using Diesel to remove device by uid and return it.
pub fn remove_device_by_id(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::Device>, DbError> {
    use crate::schema::devices::dsl::*;

    let old_count = devices.count().first::<i64>(conn);
    let device = devices
        .filter(id.eq(uid.to_string()))
        .first::<models::Device>(conn)
        .optional()?;

    diesel::delete(devices.filter(id.eq(uid.to_string()))).execute(conn)?;

    if old_count.map(|count| count - 1) == devices.count().first(conn) {
        Ok(device)
    } else {
        Err(DbError::from("Error finding device for remove"))
    }
}

/// Run query using Diesel to remove room by uid and return it.
pub fn remove_room_by_id(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::Room>, DbError> {
    use crate::schema::rooms::dsl::*;

    let old_count = rooms.count().first::<i64>(conn);
    let room = rooms
        .filter(id.eq(uid.to_string()))
        .first::<models::Room>(conn)
        .optional()?;

    diesel::delete(rooms.filter(id.eq(uid.to_string()))).execute(conn)?;

    if old_count.map(|count| count - 1) == rooms.count().first(conn) {
        Ok(room)
    } else {
        Err(DbError::from("Error finding room for remove"))
    }
}

pub fn remove_house_by_id(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::House>, DbError> {
    use crate::schema::houses::dsl::*;

    let old_count = houses.count().first::<i64>(conn);
    let other_house = houses
        .filter(id.eq(uid.to_string()))
        .first::<models::House>(conn)
        .optional()?;

    diesel::delete(houses.filter(id.eq(uid.to_string()))).execute(conn)?;

    if old_count.map(|count| count - 1) == houses.count().first(conn) {
        Ok(other_house)
    } else {
        Err(DbError::from("Error finding house for remove"))
    }
}

/// Run query using Diesel to list rooms by uid of house and return it.
pub fn list_room_by_id(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Vec<models::Room>, DbError> {
    use crate::schema::rooms as rms;

    let rooms: Vec<models::Room> = rms::dsl::rooms
        .filter(rms::dsl::house.eq(uid.to_string()))
        .load::<models::Room>(conn)?;

    Ok(rooms)
}

/// Run query using Diesel to list rooms by uid of house and return it.
pub fn list_houses(conn: &mut SqliteConnection) -> Result<Vec<models::House>, DbError> {
    use crate::schema::houses as hs;

    let houses: Vec<models::House> = hs::dsl::houses.load::<models::House>(conn)?;

    Ok(houses)
}

/// Run query using Diesel to list devices by uid in room and return it.
pub fn list_device_in_room(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Vec<models::Device>, DbError> {
    use crate::schema::devices as dvs;

    let devices: Vec<models::Device> = dvs::dsl::devices
        .filter(dvs::dsl::room.eq(uid.to_string()))
        .load::<models::Device>(conn)?;

    Ok(devices)
}

/// Run query using Diesel to find room by uid and return it.
pub fn find_room_by_id(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::Room>, DbError> {
    use crate::schema::rooms::dsl::*;

    let room = rooms
        .filter(id.eq(uid.to_string()))
        .first::<models::Room>(conn)
        .optional()?;

    Ok(room)
}

/// Run query using Diesel to find room by uid and return it.
pub fn find_house_by_id(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::House>, DbError> {
    use crate::schema::houses::dsl::*;

    let other_house = houses
        .filter(id.eq(uid.to_string()))
        .first::<models::House>(conn)
        .optional()?;

    Ok(other_house)
}

/// Run query using Diesel to find room by uid and return it.
pub fn get_house_report(conn: &mut SqliteConnection, uid: Uuid) -> Result<String, DbError> {
    use crate::schema::devices::dsl as dvs;
    use crate::schema::houses::dsl as hs;
    use crate::schema::rooms::dsl as rms;

    let other_house = hs::houses
        .filter(hs::id.eq(uid.to_string()))
        .first::<models::House>(conn)?;
    let mut report = String::from("");
    report.push_str(&format!(
        "В доме {0} установлены следующие приборы: \n",
        other_house.name
    ));
    let rooms: Vec<_> = rms::rooms
        .filter(rms::house.eq(other_house.id))
        .select(rms::id)
        .load::<String>(conn)?;
    for room in rooms {
        let devices: Vec<models::Device> = dvs::devices
            .filter(dvs::room.eq(room.to_string()))
            .load::<models::Device>(conn)?;
        report.push_str(&generate_report(devices).unwrap());
    }

    Ok(report)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_device(
    conn: &mut SqliteConnection,
    nm: &str,
    tp: &str,
    adrs: &str,
    rm: &str,
) -> Result<models::Device, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::devices::dsl::*;

    let new_device = models::Device {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
        type_: tp.to_owned(),
        address: Some(adrs.to_owned()),
        room: rm.to_owned(),
        state: false,
        variable: 0,
    };

    diesel::insert_into(devices)
        .values(&new_device)
        .execute(conn)?;

    Ok(new_device)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn update_state_device(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<models::Device>, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::devices::dsl::*;

    let device_pre_state = devices
        .filter(id.eq(uid.to_string()))
        .first::<models::Device>(conn)
        .optional()?;

    let _ = diesel::update(devices.find(uid.to_string()))
        .set(state.eq(diesel::dsl::not(state)))
        .execute(conn);

    let device = devices
        .filter(id.eq(uid.to_string()))
        .first::<models::Device>(conn)
        .optional()?;

    if device.as_ref().unwrap().state != device_pre_state.as_ref().unwrap().state {
        Ok(devices
            .filter(id.eq(uid.to_string()))
            .first::<models::Device>(conn)
            .optional()?)
    } else {
        Err(DbError::from("Error"))
    }
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_room(
    conn: &mut SqliteConnection,
    nm: &str,
    hs: &str,
) -> Result<models::Room, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::rooms::dsl::*;

    let new_room = models::Room {
        id: Uuid::new_v4().to_string(),
        name: String::from(nm),
        house: String::from(hs),
    };

    println!("Trying insert room {}", new_room.house.len());

    //diesel::insert_into(rooms).values(&new_room).execute(conn)?;
    diesel::insert_into(rooms).values(&new_room).execute(conn)?;

    Ok(new_room)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_house(conn: &mut SqliteConnection, nm: &str) -> Result<models::House, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::houses::dsl::*;

    let new_house = models::House {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(houses)
        .values(&new_house)
        .execute(conn)?;

    Ok(new_house)
}
