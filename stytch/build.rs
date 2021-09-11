use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let user_agent = format!(
        "stytch-rust v{} ({})",
        env!("CARGO_PKG_VERSION"),
        env::var("PROFILE").unwrap(),
    );
    fs::write(Path::new(&out_dir).join("user-agent"), user_agent).unwrap();
}
