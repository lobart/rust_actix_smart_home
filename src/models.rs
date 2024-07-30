use crate::schema::{devices, houses, rooms};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub trait Item {
    fn name(&self) -> String;
    fn id(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = devices)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub type_: String,
    pub address: Option<String>,
    pub state: bool,
    pub variable: i32,
    pub room: String,
}

impl Item for Device {
    fn name(&self) -> String {
        String::from(&self.name)
    }
    fn id(&self) -> String {
        String::from(&self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub house: String,
}

impl Item for Room {
    fn name(&self) -> String {
        String::from(&self.name)
    }
    fn id(&self) -> String {
        String::from(&self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = houses)]
pub struct House {
    pub id: String,
    pub name: String,
}

impl Item for House {
    fn name(&self) -> String {
        String::from(&self.name)
    }
    fn id(&self) -> String {
        String::from(&self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDevice {
    pub name: String,
    pub typ: String,
    pub address: String,
    pub room: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoom {
    pub name: String,
    pub house: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHouse {
    pub name: String,
}

impl NewDevice {
    #[cfg(test)] // only needed in tests
    pub fn new(
        name: impl Into<String>,
        typ: impl Into<String>,
        address: impl Into<String>,
        room: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            typ: typ.into(),
            address: address.into(),
            room: room.into(),
        }
    }
}

impl NewRoom {
    #[cfg(test)] // only needed in tests
    pub fn new(name: impl Into<String>, house: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            house: house.into(),
        }
    }
}

impl NewHouse {
    #[cfg(test)] // only needed in tests
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
