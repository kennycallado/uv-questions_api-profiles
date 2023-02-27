use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::profiles;

use crate::app::modules::profile::model::Profile;

pub async fn _get_profile_by_id(db: &Db, id: i32) -> Result<Profile, diesel::result::Error> {
    let profile = db
        .run(move |c| {
            profiles::table
                .filter(profiles::id.eq(id))
                .first::<Profile>(c)
        })
        .await;

    profile
}

pub async fn get_profile_by_token(
    db: &Db,
    token: String,
) -> Result<Profile, diesel::result::Error> {
    let profile = db
        .run(move |c| {
            profiles::table
                .filter(profiles::profile_token.eq(token))
                .first::<Profile>(c)
        })
        .await;

    profile
}
