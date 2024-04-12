use chrono::Duration;
use lazy_static::lazy_static;
use tracing::info;

/*
    Used for multiple reads,
    no need for single use as long as it's valid during setup
*/
lazy_static! {
    pub static ref RUN_MODE: String = std::env::var("RUN_MODE").unwrap_or_else(|_| "local".to_string());

    // JWT
    pub static ref JWT_ISS: String = std::env::var("JWT_ISS").expect("JWT_ISS not setup");
    pub static ref JWT_AUD: String = std::env::var("JWT_AUD").expect("JWT_AUD not setup");
    pub static ref JWT_EXP: Duration =
        Duration::try_hours(std::env::var("JWT_EXP_IN_HOURS")
            .expect("JWT_EXP_IN_HOURS not set up")
            .parse()
            .expect("JWT_EXP_IN_HOURS must be a valid integer"))
        .expect("Error converting to duration");
}

pub fn init() {
    info!("RUN_MODE: {:?}", *RUN_MODE);
    info!("JWT_ISS: {:?}", *JWT_ISS);
    info!("JWT_AUD: {:?}", *JWT_AUD);
    info!("JWT_EXP: {:?}", *JWT_EXP);
}