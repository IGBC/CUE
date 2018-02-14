use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};

///! This struct contains the contextual data for the Socket
///! libBiFrost connects too. This is a service socket, as such
///! spawns a new thread to handle each new connection.
struct BiFrostParentEndpoint {
    fname: &str,
    socket: UnixListener,
    clients: usize,
}

let mut globalEndpoint: Option<ARC<Mutex<BiFrostParentEndpoint>>>=None;

impl BiFrostParentEndpoint {
    fn init() {
        if globalEndpoint == Some(_) {
            warn!("attempt to create duplicate BiFrostEndpoint blocked. You have a bug.");
            return
        }
        let fname = "/tmp/bifrost";
        info!("creating socket at {}", fname);
        let socket = UnixListener::bind(fname).unwrap();
        globalEndpoint = Some(Arc::new(Mutex::new(BiFrostParentEndpoint{
            fname,
            socket,
            clients = 0,
        })))
        thread::spawn(move || Self::accept_loop(Arc::clone(&globalEndpoint)));
    }

    fn accept_loop(data: Arc<Mutex<BiFrostParentEndpoint>>) {
        info!("API socket listener started");
        loop {
            // Do forever
            let mut t = match data.lock() {
                Ok(cont) => cont,
                Err(e) => {
                    // lock() is a blocking call. this is a fatal error
                    error!("API accept loop thread recieved fatal error: {}", e);
                    return;
                }
            };
            // accept connections and process them, spawning a new thread for each one
            for stream in t.socket.incoming() {
                match stream {
                    Ok(stream) => {
                        /* connection succeeded */
                        thread::spawn(|| Self::client_handler(Arc::clone&data, stream));
                        t.clients += 1;
                    }
                    Err(err) => {
                        error!("Socket connection failed with error: {}", err);
                    }
                }
            }
            drop(t); // mutex is cleared here
        }
    }

    fn client_handler(data: Arc<Mutex<BiFrostParentEndpoint>>, stream: UnixStream) {
        info!("Client connected from {}", stream.peer_addr());
        
    }
}
