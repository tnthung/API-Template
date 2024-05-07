pub(super) use session_log::prelude::*;


#[macro_export]
macro_rules! Var {
  ($key:expr) => {
    std::env::var($key)
  };

  ($name:ident, $key:expr) => {
    pub static $name: Lazy<String> = Lazy::new(|| {
      if let Ok(arg) = std::env::var($key) { return arg; }
      log_fatal!(Logger::new("Env"), "{} is not set", $key);
    });
  };

  ($name:ident, $key:expr, true) => {
    pub static $name: Lazy<Option<String>> = Lazy::new(|| std::env::var($key).ok());
  };

  ($name:ident, $key:expr, $default:expr) => {
    pub static $name: Lazy<String> = Lazy::new(||
      std::env::var($key).unwrap_or_else(|_| {
        log_warning!(Logger::new("Env"), "{} is not set, using default value", $key);
        let tmp: &'static str = $default;
        $default.to_string()
      }));
  };

  ($name:ident, $key:expr, | $arg:ident | -> $type:ty $body:block) => {
    pub static $name: Lazy<$type> = Lazy::new(||
      { let $arg = std::env::var($key); $body });
  };

  ($name:ident, $key:expr, | $arg:ident | $body:block) => {
    pub static $name: Lazy<String> = Lazy::new(||
      { let $arg = std::env::var($key); $body });
  };
}


pub(super) use Var;
