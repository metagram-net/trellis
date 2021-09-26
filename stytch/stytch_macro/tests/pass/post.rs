use serde::Serialize;
use stytch_macro::post;

#[post("/top-level")]
#[derive(Serialize)]
struct Post<'a> {
    id: &'a str,
}

fn main() {}
