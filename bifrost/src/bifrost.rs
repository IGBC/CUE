use std::env;
use std::os::unix::net::UnixStream;

#[derive(Debug)]
pub struct BiFrost {
    standalone: bool,
}

impl BiFrost {
    pub fn init() ->  Self {
        match env::var("CUE") {
            Ok(val) => Self::init_cue(val),
            Err(_) => Self::init_standalone(),
        }
    }

    fn init_cue(socketpath: &str) -> Self {
        let standalone = false;
        
        let socket = match UnixStream::connect(socketpath) {
            Ok(sock) => sock,
            Err(e) => {
                panic!("CUE fatal error: Couldn't connect to socket {}: {:?}",socketpath, e);
            }
        };
        
        BiFrost {
            standalone,
        }
    }

    fn init_standalone() -> Self {
        let standalone = true;
        BiFrost {
            standalone,
        }
    }
}