//! Config

use std::collections::{btree_map::Entry, BTreeMap};

/// Config value.
#[derive(Debug, Clone)]
enum ConfigVal {
    /// String.
    String(String),

    /// Integer.
    Integer(i64),

    /// Float.
    Float(f64),
}

impl ConfigVal {
    /// To string.
    pub fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Integer(i) => format!("{i}"),
            Self::Float(f) => format!("{f}"),
        }
    }

    /// To integer.
    pub fn to_integer(&self) -> i64 {
        match self {
            Self::String(s) => match s.parse() {
                Ok(i) => i,
                _ => 0,
            },
            Self::Integer(i) => *i,
            Self::Float(f) => *f as i64,
        }
    }

    /// To float.
    pub fn to_float(&self) -> f64 {
        match self {
            Self::String(s) => match s.parse() {
                Ok(f) => f,
                _ => 0.0,
            },
            Self::Integer(i) => *i as f64,
            Self::Float(f) => *f,
        }
    }
}

/// Kitsune2 generic configuration struct.
#[derive(Debug)]
pub struct Config(BTreeMap<String, ConfigVal>);

impl Config {
    /// Mix in defaults. Will not overwrite keys that already exist.
    pub fn set_defaults(&mut self, oth: &Config) {
        for (k, v) in oth.0.iter() {
            if let Entry::Vacant(e) = self.0.entry(k.clone()) {
                e.insert(v.clone());
            }
        }
    }

    /// Set string.
    pub fn set_string(&mut self, key: String, val: String) {
        self.0.insert(key, ConfigVal::String(val));
    }

    /// Get string.
    pub fn get_string(&self, key: &str) -> String {
        match self.0.get(key) {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }
    }

    /// Set integer.
    pub fn set_integer(&mut self, key: String, val: i64) {
        self.0.insert(key, ConfigVal::Integer(val));
    }

    /// Get integer.
    pub fn get_integer(&self, key: &str) -> i64 {
        match self.0.get(key) {
            Some(v) => v.to_integer(),
            None => 0,
        }
    }

    /// Set float.
    pub fn set_float(&mut self, key: String, val: f64) {
        self.0.insert(key, ConfigVal::Float(val));
    }

    /// Get float.
    pub fn get_float(&self, key: &str) -> f64 {
        match self.0.get(key) {
            Some(v) => v.to_float(),
            None => 0.0,
        }
    }
}
