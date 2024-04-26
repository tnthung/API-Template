use crate::Logger;

use super::LogLevel;
use super::Loggable;

use std::sync::{Arc, Mutex};

use chrono::prelude::*;


type LogContext = Arc<Mutex<Vec<String>>>;


#[derive(Debug)]
pub enum LogSessionError {
  SessionDied,
}


pub struct LogSession {
  died: bool,
  name: String,
  root: String,
  msgs: LogContext,
  sire: Option<LogContext>,
  time: DateTime<Local>,
}


impl LogSession {
  pub(crate) fn new(name: &str, logger: &str) -> LogSession {
    let msgs = Arc::new(Mutex::new(Vec::new()));
    let sire = None;

    LogSession {
      died: false,
      name: name.to_string(),
      root: logger.to_string(),
      time: Local::now(),
      msgs,
      sire,
    }
  }

  pub fn session(&self, name: &str) -> Result<LogSession, LogSessionError> {
    if self.died { return Err(LogSessionError::SessionDied); }

    let msgs = Arc::new(Mutex::new(Vec::new()));
    let sire = Some(self.msgs.clone());

    Ok(LogSession {
      died: false,
      name: name.to_string(),
      root: self.root.clone(),
      time: Local::now(),
      msgs,
      sire,
    })
  }

  pub(self) fn dump(&mut self) {
    if self.died { return; }
    self.died = true;

    let mut rslt = Vec::new();

    rslt.push(format!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    rslt.push(format!("┃ Session: {}, Elapsed: {}us", self.name, (Local::now() - self.time).num_microseconds().unwrap()));
    rslt.push(format!("┃"));

    for msg in self.msgs.lock().unwrap().iter() {
      for line in msg.lines() {
        let is_border  = line.starts_with("┏") || line.starts_with("┗");
        let is_content = line.starts_with("┃");

        let space = if is_border || is_content { "" } else { " " };
        let line  = if is_border { &line[..line.len()-3] } else { line };

        rslt.push(format!("┃{space}{line}"));
      }
    }

    rslt.push(format!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    if let Some(sire) = &self.sire {
      sire.lock().unwrap().append(&mut rslt);
      return;
    }

    Logger::new(&self.root).write_file(&(rslt.join("\n") + "\n"));
  }
}


impl Loggable for LogSession {
  fn log(&self, level: LogLevel, message: &str) {
    if self.died { return; }

    let logger = Logger::get_logger(&self.root);
    if level < logger.0 { return; }

    let time = Local::now().to_rfc3339_opts(SecondsFormat::Micros, true);
    let root = &self.root;
    let name = &self.name;

    println!("{time} {level:#} {root}:{name} - {message}");
    let message = format!("{time} {level} {root}:{name} - {message}");
    self.msgs.lock().unwrap().push(message);
  }
}


impl Drop for LogSession {
  fn drop(&mut self) {
    self.dump();
  }
}
