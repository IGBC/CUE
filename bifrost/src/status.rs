use std::env;

#[derive(PartialEq)]
enum AppStatus {
    standalone,
    cue_app(String), // Argument is socket Dir
}

static mut cached_status: Option<AppStatus> = None;

pub fn app_status() -> &'static AppStatus {
    if cached_status == None {
        let stat = match env::var("CUE") {
            Ok(val) => AppStatus::cue_app(val),
            Err(_) => AppStatus::standalone,
        };
        cached_status = Some(stat);
    }
    return &(cached_status.unwrap());
}

pub fn is_standalone() -> bool {
    return *app_status() == AppStatus::standalone;
}