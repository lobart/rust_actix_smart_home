use std::collections::HashMap;
use std::ffi::CString;

use reqwest::Error;

pub struct ClientTcp {
    url: String,
}

impl ClientTcp {
    pub async fn new(addr: String) -> Result<Self, Error> {
        let url = addr;
        Ok(Self { url })
    }

    pub async fn get_id_all_devices(&mut self) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/devices-list", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn get_id_all_rooms(&mut self) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/rooms-list", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn get_id_all_houses(&mut self) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/house-list", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn get_device_var(&mut self, device_uid: &str) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/device/{device_uid}/var", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn create_device(
        &mut self,
        name: &str,
        typ: &str,
        address: &str,
        room: &str,
    ) -> Result<String, Error> {
        let mut map = HashMap::new();
        map.insert("name", name);
        map.insert("typ", typ);
        map.insert("address", address);
        map.insert("room", room);

        let client = reqwest::Client::new();
        let res = client
            .post(format!("{0}/device", self.url))
            .json(&map)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }

    pub async fn create_room(&mut self, name: &str, house: &str) -> Result<String, Error> {
        let mut map = HashMap::new();
        map.insert("name", name);
        map.insert("house", house);

        let client = reqwest::Client::new();
        let res = client
            .post(format!("{0}/room", self.url))
            .json(&map)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }

    pub async fn create_house(&mut self, name: &str) -> Result<String, Error> {
        let mut map = HashMap::new();
        map.insert("name", name);

        let client = reqwest::Client::new();
        let res = client
            .post(format!("{0}/house", self.url))
            .json(&map)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }

    pub async fn get_list_of_houses(&mut self) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/house-list", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn get_list_of_rooms(&mut self, house_uid: &str) -> Result<String, Error> {
        let u = format!("{0}/house/{house_uid}/list", self.url);
        let resp = reqwest::get(u).await?.text().await?;
        Ok(resp)
    }

    pub async fn get_list_of_devices(&mut self, room_uid: &str) -> Result<String, Error> {
        println!("{0}/room/{1}/list", self.url, room_uid);
        let resp = reqwest::get(format!("{0}/room/{1}/list", self.url, room_uid))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn get_full_report(&mut self, house_uid: &str) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/report/{house_uid}", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn get_device_description(&mut self, device_uid: &str) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/device/{device_uid}", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn get_device_by_id(&mut self, device_uid: &str) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/device/{device_uid}", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn remove_device_by_id(&mut self, device_uid: &str) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/device/{device_uid}/remove", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn remove_room_by_id(&mut self, room_uid: &str) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/room/{room_uid}/remove", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn remove_house_by_id(&mut self, house_uid: &str) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/house/{house_uid}/remove", self.url))
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    pub async fn change_state(&mut self, dev_id: &str) -> Result<String, Error> {
        let resp = reqwest::get(format!("{0}/device/{1}/state", self.url, dev_id))
            .await?
            .text()
            .await?;
        Ok(resp)
    }
}

#[no_mangle]
pub extern "C" fn get_device_description() -> *mut i8 {
    let dev_id = reqwest::blocking::get("http://127.0.0.1:8080/devices-list")
        .unwrap()
        .text()
        .unwrap();
    let id = dev_id.split(' ').collect::<Vec<&str>>()[0];

    let device_description = reqwest::blocking::get(format!(
        "{0}/device/{1}",
        "http://127.0.0.1:8080",
        id.trim_start_matches('"')
    ))
    .unwrap()
    .text()
    .unwrap();

    CString::new(device_description).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn set_device_state() -> *mut i8 {
    let dev_id = reqwest::blocking::get("http://127.0.0.1:8080/devices-list")
        .unwrap()
        .text()
        .unwrap();
    let id = dev_id.split(' ').collect::<Vec<&str>>()[0];

    reqwest::blocking::get(format!(
        "{0}/device/{1}/state",
        "http://127.0.0.1:8080",
        id.trim_start_matches('"')
    ))
    .unwrap()
    .text()
    .unwrap();

    let device_description = reqwest::blocking::get(format!(
        "{0}/device/{1}",
        "http://127.0.0.1:8080",
        id.trim_start_matches('"')
    ))
    .unwrap()
    .text()
    .unwrap();

    CString::new(device_description).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
