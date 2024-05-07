#[macro_use] extern crate rocket;


mod config;
mod routes;
mod models;
mod states;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  let mut rocket = rocket::build();

  // manage states
  rocket = rocket;

  // attach fairings
  rocket = rocket;

  // mount routes
  rocket = rocket;

  if *config::RUN_MODE == "DEV" {
    rocket = rocket.mount("/test", routes::test::ROUTES.clone());
  }

  // launch
  rocket.launch().await?;
  Ok(())
}
