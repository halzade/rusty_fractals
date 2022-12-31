use log::{debug, info};

pub fn update() {
    debug!("update()");
    ITERATION_MAX += 150;
    info!("ITERATION_MAX = {}", ITERATION_MAX);
}