use wasm_bindgen::JsCast;

// TODO: Unify with server-side definitions
pub const COOKIE_NAME: &'static str = "trellis.csrf_protection";
pub const HEADER_NAME: &'static str = "X-CSRF-Protection";

pub fn token() -> String {
    let window = web_sys::window().expect("global `window`");
    let document = window.document().expect("`document` on `window`");
    let html_document = document.dyn_into::<web_sys::HtmlDocument>().unwrap();
    find_cookie(html_document, COOKIE_NAME).unwrap_or_default()
}

fn find_cookie(doc: web_sys::HtmlDocument, name: &str) -> Option<String> {
    let prefix: &str = &format!("{}=", name);
    let cookies = doc.cookie().unwrap_or_default();
    for cookie in cookies.split("; ") {
        if let Some(value) = cookie.strip_prefix(prefix) {
            return Some(value.to_owned());
        }
    }
    None
}
