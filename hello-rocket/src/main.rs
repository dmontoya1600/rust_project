#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;


mod other{

    #[get("/")]
    pub fn index() -> &'static str {
        "Hello, world!"
    }

}

#[get("/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}", name)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello, other::index]).launch();
}
