use serde::Serialize;
use stytch_macro::post;

#[post()]
#[derive(Serialize)]
struct PostNoArgs<'a> {
    id: &'a str,
}

fn main() {}
