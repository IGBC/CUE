use super::status;
use super::transport::client::ClientSession;
use super::window::Window;

#[derive(Debug)]
pub struct BiFrost {
    app_name: &str,
    session: Option<ClientSession>,
    window_list: Vec<Window>,
}

impl BiFrost {
    pub fn init(app_name: &str) ->  Self {
        session = match status::app_status() {
            status::AppStatus::cue_app(path) => Some(ClientSession::init_session(app_name, path)),
            status::AppStatus::standalone => None,
        };
        window_list = Vec::new();
        BiFrost {
            app_name,
            session,
            window_list,
        }
    }

    pub fn run() {

    }
}