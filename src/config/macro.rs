pub(super) use session_log::prelude::*;


macro_rules! Val {
  ($key:expr) => {{
    std::env::var($key).ok()
  }};

  ($key:expr, true) => {{
    let arg = std::env::var($key);

    if arg.is_err() {
      log_fatal!(Logger::new("Env"), "{} is not set", $key);
    };

    arg.unwrap()
  }};

  ($key:expr, true, $default:expr) => {{
    let arg = std::env::var($key);

    if arg.is_err() {
      log_warning!(Logger::new("Env"), "{} is not set, using default value", $key);
      $default.to_string()
    }

    else {
      arg.unwrap()
    }
  }};
}


pub(super) use Val;


macro_rules! Var {
  ($name:ident, $key:expr) => {
    pub static $name: Lazy<Option<String>> = Lazy::new(|| r#macro::Val!($key));
  };

  ($name:ident, $key:expr, true) => {
    pub static $name: Lazy<String> = Lazy::new(|| r#macro::Val!($key, true));
  };

  ($name:ident, $key:expr, true, $default:expr) => {
    pub static $name: Lazy<String> = Lazy::new(|| r#macro::Val!($key, true, $default));
  };
}


pub(super) use Var;
