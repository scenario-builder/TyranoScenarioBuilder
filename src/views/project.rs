use rocket::get;
use rocket_dyn_templates::Template;

use tera::Context;

pub fn routes() -> Vec<rocket::Route> {
    routes![scenarios, scenario_edit]
}

#[get("/<project_id>/scenarios")]
fn scenarios(project_id: u8) -> Template {
    let ctx = Context::new();
    Template::render("scenario/project", ctx.into_json())
}

#[get("/<project_id>/scenario/<scenario_id>/edit")]
fn scenario_edit(project_id: u8, scenario_id: u8) -> Template {
    let ctx = Context::new();
    Template::render("scenario/edit", ctx.into_json())
}