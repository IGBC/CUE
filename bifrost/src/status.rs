use std::env;

enum AppStatus {
    standalone,
    cue_app(str), // Argument is socket Dir
}

static mut cached_status: Option<AppStatus> = None;

pub fn app_status() -> &AppStatus {
    if cached_status == None {
        let stat = match env::var("CUE") {
            Ok(val) => AppStatus::cue_app(val),
            Err(_) => AppStatus::standalone,
        };
        cached_status = Option::new(stat);
    }
    return &(cached_status.unwrap());
}

pub fn is_standalone() -> bool {
    return app_status() == AppStatus::standalone;
}