use super::common::{ClientCmd, SocketManager, MsgPack};
use std::thread;
use std::sync::{Arc, Mutex};
use std::os::unix::net::{UnixListener};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::collections::HashMap;

///! This struct contains the contextual data for the socket
///! libBiFrost connects to. This is a service socket, as such
///! spawns a new thread to handle each new connection.
pub struct BiFrostParentEndpoint {
    //fname: &str,
    socket: UnixListener,
    // Each connection is a Socket / Session pair, these are mapped
    // Against the connection's token.
    clients: HashMap<u32 , (SocketManager, HostSession)>,
    
}

enum SessionStatus {
    PreAuth, // Socket opened, awaiting InitSession command.
    Open, // Session open.
    Closed, // Session waiting deletion.
}

type HostSession = Arc<Mutex<HostSessionContainer>>;
struct HostSessionContainer {
    // Session token created at start.
    token: u32,
    // Current status of this session.
    status: SessionStatus,

    // Data provided by the child process.
    pid: Option<usize>, // ID of child process.
    uid: Option<usize>, // User of child process.
    app_name: Option<String>, // appname provided by child.

    // Connection to socket channel
    sm_tx: Sender<Vec<u8>>,
    sm_rx: Receiver<Vec<u8>>,
}

impl MsgPack for u32{}
impl MsgPack for () {}

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

        let loop_copy = Arc::clone(&endpoint);

        thread::spawn(move || Self::accept_loop(loop_copy));
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

                    // Create session and save into client's map.
                    let mut session = HostSessionContainer {
                        token,
                        status: SessionStatus::PreAuth,
                        sm_tx: send_tx,
                        sm_rx: recv_rx,
                        pid: None,
                        uid: None,
                        app_name: None, 
                    };
                    
                    // Wrap session into ArcTex and create two copies,
                    // one for the hashmap and one for the lambda in socket manager.

                    let tex = Mutex::new(session);
                    let arc = Arc::new(tex);
                    
                    let lambda_copy = Arc::clone(&arc);

                    // Build socket manager.
                    let mut manager = SocketManager {
                        is_host: false,
                        socket: stream,
                        tx: recv_tx,
                        rx: send_rx,
                        incoming_callback: Box::new(move |i| Self::client_handler(i, &lambda_copy)),
                    };

                    self.clients.insert(token, (manager, arc));
                },
                Err(_) => {
                    /* No new connections - This is not an error */
                },
            }
        }
    }

    /// Polls all current sessions for activity and runs any
    /// pending transactions.
    fn poll_clients(&mut self) {
        // iterate sessions
        for (_, tuple) in self.clients.iter_mut() {
            // for each session, poll
            tuple.0.poll();
        }
    }

    /// Callback from SocketManager - Recieves ClientCmd
    /// This is running on the thread calling manager.poll()
    /// So It better not block
    fn client_handler(i: Vec<u8>, session: &HostSession) -> Vec<u8> {
        let cmd = ClientCmd::from_mp(&i);

        let mut session = session.lock().unwrap();

        match cmd {
            ClientCmd::InitSession(app_name, pid, uid) => { 
                // Fill in all the information provided by the child.
                session.pid = Some(pid);
                session.uid = Some(uid);
                session.app_name = Some(app_name.clone());
                session.status = SessionStatus::Open;
                // Return Session
                return session.token.to_mp(); 
            },
            ClientCmd::CloseSession() => {
                /* find all resources used by this token and remove them */
                session.status = SessionStatus::Closed; // Something will clean this up later

                return ().to_mp();
            },
            ClientCmd::CreateWindow(token, title, w, h) => {
                /* This has to talk to rendering... somehow */
            },
            ClientCmd::SetWindowTitle(token, title) => {
                /* this is missing the f***ing window ID */
            },
            ClientCmd::DeleteWindow(token, window_id) => {}, // at this point F*** IT!
            ClientCmd::SendNotification(text) => {
                /* Well technically we have to build a notification system first */
                println!("{}", text);
                return ().to_mp();
            },
        }
        
        return i;
    }    

    fn accept_loop(data: Arc<Mutex<BiFrostParentEndpoint>>) {
        loop {
            // Do forever
            let mut t = match data.lock() {
                Ok(cont) => cont,
                Err(_) => {
                    // lock() is a blocking call. this is a fatal error
                    return;
                }
            };
            t.accept_connections();
            t.poll_clients();
            drop(t); // mutex is cleared here
        }
    }
}
