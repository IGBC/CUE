use std::env;

#[derive(PartialEq)]
pub enum AppStatus {
    Standalone,
    CUEApp(String), // Argument is socket Dir
}

pub fn app_status() -> AppStatus {
    let stat = match env::var("CUE") {
        Ok(val) => AppStatus::CUEApp(val),
        Err(_) => AppStatus::Standalone,
    };
    stat
}

pub fn is_standalone() -> bool {
    return app_status() == AppStatus::Standalone;
}