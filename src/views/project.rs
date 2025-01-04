use rocket::get;
use rocket_dyn_templates::Template;

use tera::Context;

pub fn routes() -> Vec<rocket::Route> {
    routes![scenarios]
}

#[get("/scenarios")]
fn scenarios() -> Template {
    let ctx = Context::new();
    Template::render("scenario/project", ctx.into_json())
}
