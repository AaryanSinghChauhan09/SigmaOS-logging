//! SigmaOS Logging — structured, leveled logging framework.
//!
//! Principles: Single Responsibility, Open-Closed, Inversion of Control,
//! Interface Segregation, Composition over Inheritance, Zero-Copy where possible.

use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Log severity levels (ordered by severity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info  = 2,
    Warn  = 3,
    Error = 4,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info  => "INFO ",
            Level::Warn  => "WARN ",
            Level::Error => "ERROR",
        };
        write!(f, "{s}")
    }
}

/// A single log record — immutable once created.
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub level: Level,
    pub target: String,
    pub message: String,
    pub timestamp_ms: u64,
}

impl LogRecord {
    pub fn new(level: Level, target: impl Into<String>, message: impl Into<String>) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        Self { level, target: target.into(), message: message.into(), timestamp_ms }
    }
}

impl fmt::Display for LogRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} {} — {}", self.timestamp_ms, self.level, self.target, self.message)
    }
}

/// Sink trait — open for extension, closed for modification.
pub trait Sink: Send + Sync {
    fn emit(&self, record: &LogRecord);
    fn flush(&self) {}
}

/// In-memory sink for testing (Arrange-Act-Assert friendly).
#[derive(Debug, Default)]
pub struct MemorySink {
    records: Mutex<Vec<LogRecord>>,
}

impl MemorySink {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn records(&self) -> Vec<LogRecord> {
        self.records.lock().unwrap().clone()
    }

    pub fn count(&self) -> usize {
        self.records.lock().unwrap().len()
    }
}

impl Sink for MemorySink {
    fn emit(&self, record: &LogRecord) {
        self.records.lock().unwrap().push(record.clone());
    }
}

/// Stderr sink for production use.
pub struct StderrSink;

impl Sink for StderrSink {
    fn emit(&self, record: &LogRecord) {
        eprintln!("{record}");
    }
}

/// The logger — composed of sinks (Composition over Inheritance).
pub struct Logger {
    min_level: Level,
    sinks: Vec<Arc<dyn Sink>>,
}

impl Logger {
    pub fn new(min_level: Level) -> Self {
        Self { min_level, sinks: Vec::new() }
    }

    /// Add a sink (open for extension).
    pub fn add_sink(&mut self, sink: Arc<dyn Sink>) {
        self.sinks.push(sink);
    }

    // ── Queries ──────────────────────────────────────────────────────────────
    pub fn min_level(&self) -> Level { self.min_level }
    pub fn sink_count(&self) -> usize { self.sinks.len() }

    // ── Commands ─────────────────────────────────────────────────────────────

    pub fn log(&self, level: Level, target: &str, message: &str) {
        if level < self.min_level {
            return; // Efficient: skip below threshold
        }
        let record = LogRecord::new(level, target, message);
        for sink in &self.sinks {
            sink.emit(&record);
        }
    }

    pub fn trace(&self, target: &str, msg: &str) { self.log(Level::Trace, target, msg); }
    pub fn debug(&self, target: &str, msg: &str) { self.log(Level::Debug, target, msg); }
    pub fn info(&self, target: &str, msg: &str)  { self.log(Level::Info,  target, msg); }
    pub fn warn(&self, target: &str, msg: &str)  { self.log(Level::Warn,  target, msg); }
    pub fn error(&self, target: &str, msg: &str) { self.log(Level::Error, target, msg); }

    pub fn set_level(&mut self, level: Level) {
        self.min_level = level;
    }

    pub fn flush(&self) {
        for sink in &self.sinks {
            sink.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logs_at_or_above_min_level() {
        // Arrange
        let sink = MemorySink::new();
        let mut logger = Logger::new(Level::Info);
        logger.add_sink(sink.clone());

        // Act
        logger.debug("test", "should be filtered");
        logger.info("test", "should appear");
        logger.error("test", "also should appear");

        // Assert
        assert_eq!(sink.count(), 2);
    }

    #[test]
    fn test_filters_below_min_level() {
        // Arrange
        let sink = MemorySink::new();
        let mut logger = Logger::new(Level::Error);
        logger.add_sink(sink.clone());

        // Act
        logger.trace("t", "trace");
        logger.debug("t", "debug");
        logger.info("t", "info");
        logger.warn("t", "warn");

        // Assert
        assert_eq!(sink.count(), 0);
    }

    #[test]
    fn test_multiple_sinks_receive_same_record() {
        // Arrange
        let sink1 = MemorySink::new();
        let sink2 = MemorySink::new();
        let mut logger = Logger::new(Level::Trace);
        logger.add_sink(sink1.clone());
        logger.add_sink(sink2.clone());

        // Act
        logger.info("test", "broadcast");

        // Assert
        assert_eq!(sink1.count(), 1);
        assert_eq!(sink2.count(), 1);
    }

    #[test]
    fn test_level_ordering() {
        // Arrange + Assert
        assert!(Level::Trace < Level::Debug);
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info  < Level::Warn);
        assert!(Level::Warn  < Level::Error);
    }

    #[test]
    fn test_set_level_updates_filter() {
        // Arrange
        let sink = MemorySink::new();
        let mut logger = Logger::new(Level::Error);
        logger.add_sink(sink.clone());

        // Act
        logger.info("t", "before level change - filtered");
        logger.set_level(Level::Info);
        logger.info("t", "after level change - passes");

        // Assert
        assert_eq!(sink.count(), 1);
    }
}
