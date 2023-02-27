use super::profile::controller as profile_controller;

// this file is needed to avoid public access to the modules
pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket.mount("/api/v1/profile", profile_controller::routes())
    })
}
