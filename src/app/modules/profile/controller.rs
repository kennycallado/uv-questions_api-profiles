use rocket::{http::Status, serde::json::Json};

use crate::app::providers::constants::ROBOT_TOKEN_EXPIRATION;
use crate::app::providers::guards::robot::RobotClaims;

use crate::app::modules::profile::services::repository as profile_repository;
use crate::config::database::Db;

pub fn routes() -> Vec<rocket::Route> {
    routes![index, token, token_fails]
}

#[get("/")]
fn index() -> &'static str {
    "Hello from profiles"
}

// It doesn't allow expiration upper 5 minutes
#[post("/token", data = "<token>", rank = 1)]
async fn token(db: Db, _robot: RobotClaims, token: Json<String>) -> Result<Json<i32>, Status> {
    let limit_exp = chrono::Utc::now().timestamp() + ROBOT_TOKEN_EXPIRATION;
    if _robot.0.exp > limit_exp {
        return Err(Status::Unauthorized);
    }

    let token = token
        .clone()
        .into_inner()
        .replace("\"", "")
        .replace("{ ", "")
        .replace("}", "");
    let token = token.trim_matches('"').trim();

    let profile = profile_repository::get_profile_by_token(&db, token.to_string()).await;
    match profile {
        Ok(profile) => Ok(Json(profile.user_id)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/token", data = "<_token>", rank = 2)]
async fn token_fails(_token: Json<String>) -> Status {
    Status::Unauthorized
}
