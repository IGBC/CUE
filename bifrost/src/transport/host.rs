use super::common::{ClientCmd, SocketManager, MsgPack};
use std::thread;
use std::sync::{Arc, Mutex};
use std::os::unix::net::{UnixStream, UnixListener};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::collections::HashMap;

///! This struct contains the contextual data for the socket
///! libBiFrost connects to. This is a service socket, as such
///! spawns a new thread to handle each new connection.
pub struct BiFrostParentEndpoint {
    //fname: &str,
    socket: UnixListener,
    clients: HashMap<u32 , HostSession>,
    
}

struct HostSession {
    // Socket manager.
    manager: SocketManager,
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
            socket,
            clients: HashMap::new(),
        }));



        //thread::spawn(move || Self::accept_loop(Arc::clone(&endpoint)));
        endpoint
    }

    fn accept_connections(&mut self) {
        // accept connections and process them.
        for stream in self.socket.incoming() {
            match stream {
                Ok(stream) => {
                    /* connection succeeded */
                    let token = 1; //TODO: UUID tokens.

                    // Create channel pair for communications, 
                    let (send_tx, send_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
                    let (recv_tx, recv_rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

                    // Build socket manager.
                    let mut manager = SocketManager {
                        is_host: false,
                        socket: stream,
                        tx: recv_tx,
                        rx: send_rx,
                        incoming_callback: Self::client_handler,
                    };
                    
                    // Create session and save into client's map.
                    let mut session = HostSession {
                        manager,
                        token,
                        sm_tx: send_tx,
                        sm_rx: recv_rx,
                    };
                    self.clients.insert(token, session);
                },
                Err(err) => {
                    /* No new connections */
                },
            }
        }
    }

    /// Polls all current sessions for activity and runs any
    /// pending transactions.
    fn poll_clients(&mut self) {
        // iterate sessions
        for (_, session) in self.clients.iter_mut() {
            // for each session, poll
            session.manager.poll();
        }
    }

    /// Callback from SocketManager - Recieves ClientCmd
    /// This is running on the thread calling manager.poll()
    /// So It better not block
    fn client_handler(i: Vec<u8>) -> Vec<u8> {
        let cmd = ClientCmd::from_mp(i);
        match cmd {
            InitSession(app_name, pid, uid) { 
                /* we have to find the token and return it. 
                 * This is impossible */
                },
            CloseSession(token) {
                /* find all resources used by this token and remove them */
            },
            CreateWindow(token, title, w, h) {
                /* This has to talk to rendering... somehow */
            },
            SetWindowTitle(token, title) {
                /* this is missing the f***ing window ID */
            },
            DeleteWindow(token, window_id) {}, // at this point F*** IT!
            SendNotification(token, text) {
                /* Well technically we have to build a notification system first */
            },
        }
        
        return i;
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
            
            drop(t); // mutex is cleared here
        }
    }
}
