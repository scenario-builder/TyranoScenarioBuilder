mod login;

// 気合routing いつかdynamic routingに変える
pub fn routes() -> Vec<rocket::Route> {
    login::routes()
}