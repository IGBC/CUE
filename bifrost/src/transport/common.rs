use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rmp_serde::{Deserializer, Serializer};
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use rmp_serde::decode::{from_read, Error};
use std::os::unix::net::UnixStream;
use std::io::Write;
use std::io::ErrorKind;
use std::time::Duration;

/// SocketManager
/// multiplexes bidirectional channel opperating on top of the socket.
/// Uses headers to key the direction of data, and communicate data length.
pub struct SocketManager {
    // True if being used in CUE, False if being used in BiFrost.
    // Used to set the keying on packet headers. 
    pub is_host: bool,
    // The actual socket. We own this.
    pub socket: UnixStream,
    // Channel Back to reality.
    pub tx: Sender<Vec<u8>>,
    pub rx: Receiver<Vec<u8>>,
    // closure fired when a new command packet comes in.
    pub incoming_callback: Box<FnMut(Vec<u8>) -> Vec<u8> + Send + Sync>,
}

/// Final Packet Format With Header.
#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    is_resp: bool, // Response flag used for sorting inbound data.
    payload_len: u32, // As Stated, not used in the rust implementation, left in for the C peeps.
    payload: Vec<u8>, // The precious cargo! (Our actual data.)
}

impl MsgPack for Packet{}

impl SocketManager {
    pub fn run(&mut self) {
        loop {
            self.poll();
        }
    }

    pub fn poll(&mut self) {
        // initially set the socket to nonblocking with a 0.5µs timeout
        // *important* if the socket blocks polling on the channel will stop.
        self.socket.set_read_timeout(Some(Duration::new(0, 500))).expect("Couldn't set read timeout");
        
        // check for an outbound command from the channel
        match self.rx.try_recv() {                
            // if it exists forward it to the peer process
            Ok(data) => self.send(data, false),
            // no data nothing to do, move on
            Err(TryRecvError::Empty) => (),
            // channel is disconnected then the something is wrong, exit.
            Err(TryRecvError::Disconnected) => return,
        }
        // check for inbound data from the peer.
        match from_read::<&UnixStream, Packet>(&self.socket) {
            Ok(packet) => {
                let data = packet.payload;
                let resp = packet.is_resp;
                match resp {
                    // There is no state in this adapter, the function should
                    // be blocking for the data, so just send it off.
                    true => self.tx.send(data).unwrap(),
                    // New Inbound Command: Fire the callback. Lack of command 
                    // Seqencing means we can probably get away with doing it in
                    // this thread, as long as it doesn't run too long.

                    // New thread or no new thread the peer is blocking RN waiting 
                    // for the response so lets skip the thread and just do the callback.
                    false => {
                        // run the callback
                        let resp = (self.incoming_callback)(data);
                        // Bye, Bye. Don't come back!
                        self.send(resp, true);
                    },
                };
            },

            // The Below *SHOULD* ignore timeout errors (which are expected)
            // and panic on all other errors. The proceedure for properly 
            // handling these errors is not yet defined.
            Err(Error::InvalidMarkerRead(e)) => {
                if !(e.kind() == ErrorKind::TimedOut) {
                    panic!("Recieved Socket Error {:?}", e.kind());
                }
            },
            Err(Error::InvalidDataRead(e)) => {
                if !(e.kind() == ErrorKind::TimedOut) {
                    panic!("Recieved Socket Error {:?}", e.kind());
                }
            },
            _ => panic!("Recieved Socket Error"),
        };
    }

    fn send(&mut self, payload: Vec<u8>, is_resp: bool) {
        // Pack data into a packet. 
        let packet = Packet {
            is_resp,
            payload_len: payload.len() as u32,
            payload,

        };
        // Serialise packet, and send
        self.socket.write_all(&packet.to_mp()[..]).unwrap();
    }
}

/// Data Class, Enumerates Commands that BiFrost Can send to CUE
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientCmd {
    InitSession(String, usize, usize), // (Appname, PID, UID) returns u32
    CloseSession(), // () returns ()
    CreateWindow(u32, String, u32, u32), // (token, title, width, height) returns ?
    SetWindowTitle(u32, String), // title returns ()
    DeleteWindow(u32, u32), // token, windowID
    SendNotification(String), // text returns ().
}

impl MsgPack for ClientCmd{}

/// Data Class, Enumerates Requests that CUE can make to BiFrost
#[derive(Serialize, Deserialize, Debug)]
pub enum HostReq {
    CloseSession(), // 
    DrawWindow(u32), // windowID
    LayoutWindow(u32), // windowID
}

impl MsgPack for HostReq{}

/// Trait wrapping the mess needed to run RMP on the structs in here.
pub trait MsgPack: Serialize + DeserializeOwned {
    /// Serialiser
    fn to_mp(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        buf
    }

    /// Deserialiser
    fn from_mp(buf: &Vec<u8>) -> Self {
        let mut de = Deserializer::new(&buf[..]);
        let res: Self = Deserialize::deserialize(&mut de).unwrap();
        res
    }
}