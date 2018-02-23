use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCmd {
    InitSession(String, usize, usize), // Appname, PID, UID
    CloseSession(u32), // token
    CreateWindow(u32, String, u32, u32), // token, title, width, height
    SetWindowTitle(u32, String), // token, title
    DeleteWindow(u32, u32), // token, windowID
    SendNotification(u32, String), // token, text.
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HostReq {
    CloseSession(u32), // token
    DrawWindow(u32), // windowID
    LayoutWindow(u32), // windowID
}


impl ClientCmd {
    pub fn to_mp(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        buf
    }

    pub fn from_mp(buf: &Vec<u8>) -> Self {
        let mut de = Deserializer::new(&buf[..]);
        let res: Self = Deserialize::deserialize(&mut de).unwrap();
        res
    }
}


impl HostReq {
    pub fn to_mp(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        buf
    }

    pub fn from_mp(buf: &Vec<u8>) -> Self {
        let mut de = Deserializer::new(&buf[..]);
        let res: Self = Deserialize::deserialize(&mut de).unwrap();
        res
    }
}