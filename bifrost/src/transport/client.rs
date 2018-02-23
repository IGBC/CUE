use super::common::ClientCmd;
use std::path::Path;
use std::os::unix::net::UnixStream;
use rmp_serde::decode::{from_read, Error};
use serde::de::DeserializeOwned;
use std::io::Write;
use std::net::Shutdown;

#[derive(Debug)]
struct ClientSession {
    socket: UnixStream,
    token: u32,
}

impl ClientSession {
    pub fn init_session<P: AsRef<Path>>(app_name: &str, socket_path: P) -> Self {
        let mut socket = UnixStream::connect(socket_path).unwrap();
        let cmd = ClientCmd::InitSession(app_name.to_owned(), 0, 1000);
        let token = Self::send::<u32>(&mut socket, &cmd).unwrap();
        ClientSession {
            socket,
            token,
        }
    }

    pub fn close_session(mut self) { // Consumes session.
        let cmd = ClientCmd::CloseSession(self.token);
        Self::send::<()>(&mut self.socket, &cmd).unwrap();
        self.socket.shutdown(Shutdown::Both).unwrap();
    }
    
    pub fn create_window(mut self, title: &str, width: u32, height: u32) {
        let cmd = ClientCmd::CreateWindow(self.token, title.to_owned(), width, height);
        Self::send::<u32>(&mut self.socket, &cmd).unwrap();
        // TODO: Create a window Contex And return it.
    
        // Implement these in the window: 
        //CreateWindow(u32, String, u32, u32), // token, title, width, height
        //SetWindowTitle(u32, String), // token, title
        //DeleteWindow(u32, u32), // token, windowID
    

    }
    
    pub fn send_notification()
    SendNotification(u32, String), // token, text.

    fn send<T>(socket: &mut UnixStream, cmd: &ClientCmd) -> Result<T, Error> 
    where
    T: DeserializeOwned,
    {
        let payload = cmd.to_mp();
        socket.write_all(&payload[..]).unwrap();
        from_read(socket)
    }
}
