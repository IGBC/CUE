use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use rmp_serde::decode::{from_read, Error};
use std::os::unix::net::UnixStream;
use std::io::Write;

/// SocketManager
/// multiplexes bidirectional channel opperating on top of the socket.
/// Uses headers to key the direction of data, and communicate data length.
#[derive(Debug)]
struct SocketManager {
    // True if being used in CUE, False if being used in BiFrost.
    // Used to set the keying on packet headers. 
    is_host: bool,
    // The actual socket. We own this.
    socket: UnixStream,
    // Channel Back to reality.
    tx: Sender<Vec<u8>>,
    rx: Receiver<Vec<u8>>,
}

/// Final Packet Format With Header.
#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    client_to_host: bool, // Direction flag used for sorting inbound data.
    payload_len: u32, // As Stated, not used in the rust implementation, left in for the C peeps.
    payload: Vec<u8>, // The precious cargo! (Our actual data.)
}

/* Truth table of client_to_host flag
 dir | endpoint | c2h 
-----+----------+-----
  in |  client  |  0
 out |  client  |  1  
  in |   host   |  1  
 out |   host   |  0

 Conculsion: out^host
 Send: !is_host
 Recv:  is_host
*/

impl SocketManager {
    fn run(&mut self) {     
        loop {
            // first check for an outbound command from the channel
            match self.rx.try_recv() {                
                // if it exists forward it to the peer process
                Ok(data) => {
                    // Pack data into a packet. 
                    let packet = Packet {
                        client_to_host: !self.is_host,
                        payload_len: data.len(),
                        payload: data,
                    }
                    // Serialise packet. I really don't see how this could panic.
                    let mut buf = Vec::new();
                    packet.serialize(&mut Serializer::new(&mut buf)).unwrap();
                    // Transmit buffer.
                    // TODO: Handle errrors here.
                    self.socket.write_all(&buf[..]).unwrap();
                },
                // no data nothing to do, move on
                Err(TryRecvError::Empty) => (),
                // channel is disconnected then the something is wrong, exit.
                Err(TryRecvError::Disconnected) => return,
            }
            // check for inbound data from the peer.
            match from_read(socket) {
                Ok(packet) => {
                    //TODO: Handle Packet.
                },

                // The Below *SHOULD* ignore timeout errors (which are expected)
                // and panic on all other errors. The proceedure for properly 
                // handling these errors is not yet defined.

                Err(Error::InvalidMarkerRead(Io(e))) => {
                    if !(e.kind() == ErrorKind::TimeOut) {
                        panic!("Recieved Socket Error {}", e.kind());
                    }
                },
                Err(Error::InvalidDataRead(Io(e))) => {
                    if !(e.kind() == ErrorKind::TimeOut) {
                        panic!("Recieved Socket Error {}", e.kind());
                    }
                },
                _ => panic!("Recieved Socket Error"),
            };
        }
    }
}

/// Data Class, Enumerates Commands that BiFrost Can send to CUE
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCmd {
    InitSession(String, usize, usize), // Appname, PID, UID
    CloseSession(u32), // token
    CreateWindow(u32, String, u32, u32), // token, title, width, height
    SetWindowTitle(u32, String), // token, title
    DeleteWindow(u32, u32), // token, windowID
    SendNotification(u32, String), // token, text.
}

/// Data Class, Enumerates Requests that CUE can make to BiFrost
#[derive(Serialize, Deserialize, Debug)]
pub enum HostReq {
    CloseSession(u32), // token
    DrawWindow(u32), // windowID
    LayoutWindow(u32), // windowID
}


impl ClientCmd {
    /// Serialiser
    pub fn to_mp(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        buf
    }

    /// Deserialiser
    pub fn from_mp(buf: &Vec<u8>) -> Self {
        let mut de = Deserializer::new(&buf[..]);
        let res: Self = Deserialize::deserialize(&mut de).unwrap();
        res
    }
}


impl HostReq {
    /// Serialiser
    pub fn to_mp(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        buf
    }

    /// Deserialiser
    pub fn from_mp(buf: &Vec<u8>) -> Self {
        let mut de = Deserializer::new(&buf[..]);
        let res: Self = Deserialize::deserialize(&mut de).unwrap();
        res
    }
}