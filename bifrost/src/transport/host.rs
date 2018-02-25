use super::common::{ClientCmd, SocketManager, MsgPack};
use std::thread;
use std::sync::{Arc, Mutex};
use std::os::unix::net::{UnixStream, UnixListener};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

///! This struct contains the contextual data for the socket
///! libBiFrost connects to. This is a service socket, as such
///! spawns a new thread to handle each new connection.
pub struct BiFrostParentEndpoint {
    //fname: &str,
    socket: UnixListener,
    clients: usize,
}

struct HostSession {
    // Session token created at start.
    token: u32,
    // Connection to socket channel
    sm_tx: Sender<Vec<u8>>,
    sm_rx: Receiver<Vec<u8>>,
}


impl BiFrostParentEndpoint {
    /// Initialise a new service socket Listener,
    /// return the struct, in a state ready for connections
    pub fn init() -> Arc<Mutex<Self>> {
        //TODO: use a more inteligent path for the socket
        let fname = "/tmp/bifrost";
        // Generate a socket at the desired path.
        let socket = UnixListener::bind(fname).unwrap(); //TODO: error handling.
        
        let endpoint = Arc::new(Mutex::new(BiFrostParentEndpoint{
            fname,
            socket,
            clients: 0,
        }));
        thread::spawn(move || Self::accept_loop(Arc::clone(&endpoint)));
        endpoint
    }

    fn accept_loop(data: Arc<Mutex<BiFrostParentEndpoint>>) {
        loop {
            // Do forever
            let mut t = match data.lock() {
                Ok(cont) => cont,
                Err(e) => {
                    // lock() is a blocking call. this is a fatal error
                    return;
                }
            };
            // accept connections and process them, spawning a new thread for each one
            for stream in t.socket.incoming() {
                match stream {
                    Ok(stream) => {
                        /* connection succeeded */
                        thread::spawn(|| Self::client_handler(Arc::clone(&data), stream));
                        t.clients += 1;
                    },
                    Err(err) => {

                    },
                }
            }
            drop(t); // mutex is cleared here
        }
    }

    fn client_handler(data: Arc<Mutex<BiFrostParentEndpoint>>, stream: UnixStream) {

    }
}
