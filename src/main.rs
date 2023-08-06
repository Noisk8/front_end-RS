#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {

        title:"mera vuelta"
            },
    )
}

#[get("/sornero")]
fn sornero() -> &'static str {
    "sornero"
}
#[get("/neas")]
fn neas() -> &'static str {
    "neas"
}

#[post("/neas")]
fn create_neas() -> &'static str {
    "neas"
}

#[put("/neas")]
fn update_neas() -> &'static str {
    "mera vuelta"
}

#[delete("/neas")]
fn delete_neas() -> &'static str {
    "mera vuelta"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, neas, sornero,])
        .mount("/neas", routes![delete_neas, update_neas, create_neas])
        .attach(Template::fairing())
}
