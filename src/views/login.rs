use rocket::serde::Serialize;
use rocket_dyn_templates::{Template, };
use rocket::{get, post};
use rocket::request::FlashMessage;
use rocket::response::{Redirect, Flash};
use rocket::http::{Status, Cookie};
use rocket::form::{Form};

use tera::{Context};

pub fn routes() -> Vec<rocket::Route> {
  routes![login, login_post]
}

#[derive(Serialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct LoginForm {
  username: String,
  password: String,
  flash: Option<(String, String)>,
}

#[get("/")]
pub fn login(flash: Option<FlashMessage<'_>>) -> Template {
  let flash = flash.map(FlashMessage::into_inner);
  let context = LoginForm {
    username: "".into(),
    password: "".into(),
    flash: flash
  };
  Template::render("user/login", &context)
}

#[post("/login", data = "<form>")]
pub fn login_post(
  form: Form<LoginForm>,
  ) -> Result<Redirect, Flash<Redirect>> {
  let form = form.into_inner();
  if form.username == "admin" && form.password == "darallium" {
    Ok(Redirect::to("/"))
  } else {
    Err(Flash::error(Redirect::to("/login"), "Invalid username or password"))
  }
}