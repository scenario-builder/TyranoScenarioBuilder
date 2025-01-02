use rocket::serde::Serialize;
use rocket_dyn_templates::{Template, };
use rocket::{get, post};
use rocket::request::FlashMessage;
use rocket::response::{Redirect, Flash};
use rocket::http::{Status, Cookie};
use rocket::form::{Form};

use tera::{Context};

pub fn routes() -> Vec<rocket::Route> {
  routes![scenarios]
}


#[get("/scenarios")]
fn scenarios() -> Template {
  let mut ctx = Context::new();
  Template::render("scenario/project", &ctx.into_json())
}