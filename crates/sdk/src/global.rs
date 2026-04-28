use std::sync::Mutex;

struct GlobalConfig {
    log_level: u64,
}

impl GlobalConfig {
    const fn new() -> Self {
        Self { log_level: 3 }
    }
}

static GLOBAL_CONFIG: Mutex<GlobalConfig> = Mutex::new(GlobalConfig::new());

pub fn set_log_level(level: u64) {
    // validate level bounds
    GLOBAL_CONFIG.lock().unwrap().log_level = level
}

pub fn get_log_level() -> u64 {
    GLOBAL_CONFIG.lock().unwrap().log_level
}
