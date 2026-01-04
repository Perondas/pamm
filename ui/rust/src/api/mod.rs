use flutter_rust_bridge::frb;

pub mod commands;

#[frb(sync)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
