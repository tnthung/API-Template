

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
  Debug,      // Detailed information, typically of interest only when diagnosing problems.
  Verbose,    // Verbose information.
  Info,       // Informational messages.
  Warn,       // Warning messages that can be ignored.
  Critical,   // Warning messages that should be addressed.
  Error,      // Error messages that will not terminate the program.
  Fatal,      // Fatal error messages that will cause the program to stop.
}


impl std::fmt::Display for LogLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match (*self, f.alternate()) {
      (LogLevel::Debug   , false) => write!(f, "[D]"),
      (LogLevel::Verbose , false) => write!(f, "[V]"),
      (LogLevel::Info    , false) => write!(f, "[I]"),
      (LogLevel::Warn    , false) => write!(f, "[W]"),
      (LogLevel::Critical, false) => write!(f, "[C]"),
      (LogLevel::Error   , false) => write!(f, "[E]"),
      (LogLevel::Fatal   , false) => write!(f, "[F]"),

      (LogLevel::Debug   , true ) => write!(f, "\x1b[90m[D]\x1b[0m"),
      (LogLevel::Verbose , true ) => write!(f, "\x1b[90m[V]\x1b[0m"),
      (LogLevel::Info    , true ) => write!(f, "\x1b[32m[I]\x1b[0m"),
      (LogLevel::Warn    , true ) => write!(f, "\x1b[33m[W]\x1b[0m"),
      (LogLevel::Critical, true ) => write!(f, "\x1b[33m[C]\x1b[0m"),
      (LogLevel::Error   , true ) => write!(f, "\x1b[31m[E]\x1b[0m"),
      (LogLevel::Fatal   , true ) => write!(f, "\x1b[31m[F]\x1b[0m"),
    }
  }
}
