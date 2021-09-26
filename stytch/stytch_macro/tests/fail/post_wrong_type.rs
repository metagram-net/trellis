use serde::Serialize;
use stytch_macro::post;

#[post(1)]
#[derive(Serialize)]
struct PostNoArgs<'a> {
    id: &'a str,
}

fn main() {}
