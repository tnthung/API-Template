use once_cell::sync::Lazy;


#[get("/")]
fn test() -> &'static str {
  "Hello, world!"
}


#[get("/<n>", rank = 2)]
fn test_n(n: u32) -> String {
  format!("Hello, {}!", n)
}


#[get("/<n>", rank = 3)]
fn test_n_str(n: &str) -> String {
  format!("Hello, {}!", n)
}


pub static ROUTES: Lazy<Vec<rocket::Route>> = Lazy::new(|| {
  routes![
    test,
    test_n,
    test_n_str
  ]
});
