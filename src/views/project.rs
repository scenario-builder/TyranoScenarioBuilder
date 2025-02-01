use rocket::get;
use rocket_dyn_templates::Template;

use tera::Context;

pub fn routes() -> Vec<rocket::Route> {
    routes![scenarios, scenario_edit]
}

#[get("/scenarios")]
fn scenarios() -> Template {
    let ctx = Context::new();
    Template::render("scenario/project", ctx.into_json())
}

#[get("/scenario/<scenario_id>/edit")]
fn scenario_edit() -> Template {
    let ctx = Context::new();
    Template::render("scenario/edit", ctx.into_json())
}
