mod level;
mod r#macro;
mod session;

pub use r#macro::*;
pub use level::LogLevel;
pub use session::LogSession;

use std::fs::{File, create_dir_all, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use once_cell::sync::Lazy;

use chrono::prelude::*;


#[derive(Debug, Clone)]
pub struct Logger(String);

#[derive(Debug, Clone)]
struct LoggerInner(LogLevel, u32, Arc<Mutex<File>>);


static LOGGERS: Lazy<Mutex<HashMap<String, LoggerInner>>> =
  Lazy::new(|| Mutex::new(HashMap::new()));


pub trait Loggable {
  fn log(&self, level: LogLevel, message: &str);

  fn debug   (&self, message: &str) { self.log(LogLevel::Debug   , message); }
  fn verbose (&self, message: &str) { self.log(LogLevel::Verbose , message); }
  fn info    (&self, message: &str) { self.log(LogLevel::Info    , message); }
  fn warn    (&self, message: &str) { self.log(LogLevel::Warn    , message); }
  fn critical(&self, message: &str) { self.log(LogLevel::Critical, message); }
  fn error   (&self, message: &str) { self.log(LogLevel::Error   , message); }

  /// **THIS WILL TERMINATE THE PROGRAM**\
  /// Logging a unrecoverable error and cause the program to panic.
  fn fatal(&self, message: &str) {
    self.log(LogLevel::Fatal, message);
    panic!("Fatal error: {}", message);
  }
}


impl Logger {
  pub fn new(name: &str) -> Logger {
    Logger(name.to_string())
  }

  fn update_logger(name: &str) {
    let time  = Local::now();
    let year  = time.year ();
    let month = time.month();
    let day   = time.day  ();
    let hour  = time.hour ();

    let mut loggers = LOGGERS.lock().unwrap();
    let mut old_lvl = LogLevel::Info;

    if let Some(logger) = loggers.get(name) {
      if logger.1 == hour { return; }

      old_lvl = logger.0;
    }

    let path = format!("./logs");
    create_dir_all(&path).unwrap();

    let path = format!("{path}/{year}-{month}-{day}-{hour}.log");
    let file = OpenOptions::new().create(true).append(true).open(&path).unwrap();

    let logger = LoggerInner(old_lvl, hour, Arc::new(Mutex::new(file)));
    loggers.insert(name.to_string(), logger);
  }

  fn get_logger(name: &str) -> LoggerInner {
    Self::update_logger(name);

    let loggers = LOGGERS.lock().unwrap();
    loggers.get(name).unwrap().clone()
  }

  pub fn get_level(&self) -> LogLevel {
    let logger = Self::get_logger(&self.0);
    logger.0
  }

  pub fn set_level(&self, level: LogLevel) {
    Self::update_logger(&self.0);

    let mut loggers = LOGGERS.lock().unwrap();
    let logger = loggers.get_mut(&self.0).unwrap();

    logger.0 = level;
  }

  pub(self) fn write_file(&self, message: &str) {
    let logger = Self::get_logger(&self.0);
    let mut file = logger.2.lock().unwrap();
    file.write_all(message.as_bytes()).unwrap()
  }

  pub fn session(&self, name: &str) -> LogSession {
    LogSession::new(name, &self.0)
  }
}


impl Loggable for Logger {
  fn log(&self, level: LogLevel, message: &str) {
    let logger = Self::get_logger(&self.0);
    if level < logger.0 { return; }

    let mut file = logger.2.lock().unwrap();

    let time = Local::now().to_rfc3339_opts(SecondsFormat::Micros, true);
    let name = &self.0;

    println!("{time} {level:#} {name} - {message}");
    file.write_all(format!("{time} {level} {name} - {message}\n").as_bytes()).unwrap()
  }
}
