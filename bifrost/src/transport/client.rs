use super::common::{ClientCmd, SocketManager, MsgPack};
use std::path::Path;
use std::os::unix::net::UnixStream;
use rmp_serde::decode;
use serde::de::DeserializeOwned;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
struct ClientSession {
    // Session token provided by CUE.
    token: u32,
    // Connection to socket channel
    sm_tx: Sender<Vec<u8>>,
    sm_rx: Receiver<Vec<u8>>,
}

impl ClientSession {
    pub fn init_session<P: AsRef<Path>>(app_name: &str, socket_path: P) -> Self {
        // Create socket.
        let socket = UnixStream::connect(socket_path).unwrap();
        // Create channel pair for communications, 
        let (send_tx, send_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let (recv_tx, recv_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

        // Build socket manager and dispatch to a thread.
        let mut manager = SocketManager {
            is_host: false,
            socket,
            tx: recv_tx,
            rx: send_rx,
            incoming_callback: Self::host_event_callback,
        };
        thread::spawn(move || manager.run());

        // Create session.
        let mut session = ClientSession {
            token: 0,
            sm_tx: send_tx,
            sm_rx: recv_rx,
        };

        // Initialise the session with the host:
        let cmd = ClientCmd::InitSession(app_name.to_owned(), 0, 1000); //TODO: PID, UID
        let token = session.send::<u32>(&cmd).unwrap();
        // add token back to session and return it.
        session.token = token;
        return session;        
    }

    pub fn close_session(mut self) { // Consumes session.
        let cmd = ClientCmd::CloseSession(self.token);
        self.send::<()>(&cmd).unwrap();
        //self.socket.shutdown(Shutdown::Both).unwrap();
    }
    
    pub fn create_window(mut self, title: &str, width: u32, height: u32) {
        let cmd = ClientCmd::CreateWindow(self.token, title.to_owned(), width, height);
        self.send::<u32>(&cmd).unwrap();
        // TODO: Create a window Contex And return it.
    
        // Implement these in the window: 
        //CreateWindow(u32, String, u32, u32), // token, title, width, height
        //SetWindowTitle(u32, String), // token, title
        //DeleteWindow(u32, u32), // token, windowID
    

    }
    
    pub fn send_notification() {
        
    }
    //SendNotification(u32, String), // token, text.

    /// Push a command struct to the SocketManager and block for the return
    /// Data.
    fn send<T>(&mut self, cmd: &ClientCmd) -> Result<T, decode::Error> 
    where
    T: DeserializeOwned,
    {
        // Serialise the ClientCmd 
        let tx_payload = cmd.to_mp();
        // push serialised data to SocketManager
        self.sm_tx.send(tx_payload).unwrap(); // TODO: Handle Errors here
        // Block Waiting for the response from the SocketManager
        let resp = self.sm_rx.recv().unwrap(); // TODO: Handle Errors here
        // Deserialise the returned data
        let ret: Result<T, decode::Error> = decode::from_slice(&resp[..]);
        return ret;
    }

    fn host_event_callback(i: Vec<u8>) -> Vec<u8> {
        return i;
    }
}