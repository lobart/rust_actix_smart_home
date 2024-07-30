use crate::models::Item;
use std::{collections::HashMap, fmt, string::ParseError};

#[derive(Debug)]
pub enum DoubleError {
    EmptyVec,
    Parse(ParseError),
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError::EmptyVec => {
                write!(f, "Error EmptyValue")
            }
            DoubleError::Parse(ref e) => e.fmt(f),
        }
    }
}

pub fn generate_report<U: Item>(data: Vec<U>) -> Result<String, DoubleError> {
    let _l: usize = match data.len() {
        0 => return Err(DoubleError::EmptyVec),
        n => n,
    };
    let item_type: &str = std::any::type_name::<U>()
        .split("::")
        .collect::<Vec<&str>>()[2];

    let mut report: String = String::from("");
    report.push_str(&format!("Отчет для устройств {0}:", item_type));
    for item in data {
        report.push_str(&format!(" Имя {0}, id {1}", item.name(), item.id()));
    }
    Ok(report)
}

pub fn generate_report_id<U: Item>(data: Vec<U>) -> Result<String, DoubleError> {
    let _l: usize = match data.len() {
        0 => return Err(DoubleError::EmptyVec),
        n => n,
    };
    let item_type: &str = std::any::type_name::<U>()
        .split("::")
        .collect::<Vec<&str>>()[2];

    let mut report: String = String::from("");
    report.push_str(&format!("Список id для устройств {0}:", item_type));
    for item in data {
        report.push_str(&format!(" id {0}", item.id()));
    }
    Ok(report)
}

pub fn generate_list_id<U: Item>(data: Vec<U>) -> Result<Vec<String>, DoubleError> {
    let _l: usize = match data.len() {
        0 => return Err(DoubleError::EmptyVec),
        n => n,
    };

    let mut report: Vec<String> = Vec::<String>::new();
    for item in data {
        report.push(format!("{0} ", item.id()));
    }
    Ok(report)
}

pub fn generate_name_id<U: Item>(data: Vec<U>) -> Result<HashMap<String, String>, DoubleError> {
    let _l: usize = match data.len() {
        0 => return Err(DoubleError::EmptyVec),
        n => n,
    };

    let mut report: HashMap<String, String> = HashMap::<String, String>::new();
    for item in data {
        report.insert(item.name().to_string(), item.id().to_string());
    }
    Ok(report)
}
