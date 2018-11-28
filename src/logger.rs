use chrono::prelude::Utc;
use log;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};


pub struct Logger {
    log_file: PathBuf,
    level: log::Level,
}

impl Logger {
    ///
    /// Start a new log file with the time and date at the top.
    ///
    pub fn new<P: AsRef<Path>>(log_file: P, level: log::Level) -> Logger {
        Logger {
            log_file: log_file.as_ref().to_path_buf(),
            level: level,
        }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        <Logger as log::Log>::flush(self);
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    /// Write a message to the log file.
    fn log(&self, record: &log::Record) {
        let file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(&self.log_file);

        if file.is_err() {
            eprintln!(
                "ERROR: Could not open the file {} for appending.",
                self.log_file.display()
            );

            return;
        }

        let mut file = file.unwrap();
        let date = Utc::now();
        writeln!(file, "[{}] {}", date, record.args()).unwrap();
    }

    /// Finish writing to a log. This function is used to place any final
    /// information in a log file before the logger goes out of scope.
    fn flush(&self) {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.log_file);

        if file.is_err() {
            eprintln!(
                "ERROR: Could not open the file {} for appending.",
                self.log_file.display()
            );

            return;
        }

        let mut file = file.unwrap();
        let date = Utc::now();
        writeln!(file, "[{}] END LOG", date).unwrap();
    }
}

pub fn init_with_level(log_file: &str, level: log::Level) -> Result<(), log::SetLoggerError> {
    let logger = Logger::new(log_file, level);
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init(log_file: &str) -> Result<(), log::SetLoggerError> {
    init_with_level(log_file, log::Level::Trace)
}
