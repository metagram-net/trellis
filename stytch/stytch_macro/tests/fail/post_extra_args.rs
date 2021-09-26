use serde::Serialize;
use stytch_macro::post;

#[post("/top-level", "extra-arg")]
#[derive(Serialize)]
struct PostExtraArgs<'a> {
    id: &'a str,
}

fn main() {}
