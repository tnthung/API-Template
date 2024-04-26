

#[macro_export]
macro_rules! log_debug {
  ($loggable:expr, $($arg:tt)*) => {{
    use crate::logger::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.debug(message.as_str());
  }};
}


#[macro_export]
macro_rules! log_verbose {
  ($loggable:expr, $($arg:tt)*) => {{
    use crate::logger::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.verbose(message.as_str());
  }};
}


#[macro_export]
macro_rules! log_info {
  ($loggable:expr, $($arg:tt)*) => {{
    use crate::logger::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.info(message.as_str());
  }};
}


#[macro_export]
macro_rules! log_warn {
  ($loggable:expr, $($arg:tt)*) => {{
    use crate::logger::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.warn(message.as_str());
  }};
}


#[macro_export]
macro_rules! log_critical {
  ($loggable:expr, $($arg:tt)*) => {{
    use crate::logger::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.critical(message.as_str());
  }};
}


#[macro_export]
macro_rules! log_error {
  ($loggable:expr, $($arg:tt)*) => {{
    use crate::logger::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.error(message.as_str());
  }};
}


#[macro_export]
macro_rules! log_fatal {
  ($loggable:expr, $($arg:tt)*) => {{
    use crate::logger::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.fatal(message.as_str());
  }};
}


pub use log_debug;
pub use log_verbose;
pub use log_info;
pub use log_warn;
pub use log_critical;
pub use log_error;
pub use log_fatal;
