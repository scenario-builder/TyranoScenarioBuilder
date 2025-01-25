use rocket::get;
use rocket_dyn_templates::Template;

use tera::Context;

pub fn routes() -> Vec<rocket::Route> {
    routes![blockly]
}

#[get("/test")]
fn blockly() -> Template {
    let ctx = Context::new();
    Template::render("blockly/test", ctx.into_json())
}
