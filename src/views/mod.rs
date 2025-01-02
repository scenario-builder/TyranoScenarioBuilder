mod login;
mod project;

use rocket::{Build, Rocket};

// 気合routing いつかdynamic routingに変える
pub fn routes(rocket: Rocket<Build>) -> Rocket<Build> {
  rocket
    .mount("/login",   login::routes())
    .mount("/project", project::routes())
}