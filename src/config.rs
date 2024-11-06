//! Config

/// Config value.
#[derive(Debug, Clone)]
pub enum ConfigVal {
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

/// A Kitsune2 configuration entry.
#[derive(Debug, Clone)]
pub struct Config {
    /// A dot-separated configuration name. E.g. `tx.tx5.min-port`.
    pub name: &'static str,

    /// An end-user oriented description of this config entry,
    /// designed to be written as a comment in a configuration file.
    /// E.g. "Set the lower bound on webrtc port usage".
    pub desc: &'static str,

    /// The current value associated with this configuration entry.
    pub val: ConfigVal,
}

/// A full map of configuration entries.
pub type ConfigMap = std::collections::BTreeMap<&'static str, Config>;

/// Helper functions on config maps.
pub trait ConfigMapExt {
    /// Mixin defaults, only adding entries that don't exist.
    fn mixin_defaults(&mut self, defaults: &[Config]);

    /// Get a config value as a string.
    fn to_string(&self, name: &str) -> String;

    /// Get a config value as an integer.
    fn to_integer(&self, name: &str) -> i64;

    /// Get a config value as a float.
    fn to_float(&self, name: &str) -> f64;
}

impl ConfigMapExt for ConfigMap {
    fn mixin_defaults(&mut self, defaults: &[Config]) {
        use std::collections::btree_map::Entry::*;
        for config in defaults.iter() {
            if let Vacant(e) = self.entry(config.name) {
                e.insert(config.clone());
            }
        }
    }

    fn to_string(&self, name: &str) -> String {
        match self.get(name) {
            Some(c) => c.val.to_string(),
            None => "".to_string(),
        }
    }

    fn to_integer(&self, name: &str) -> i64 {
        match self.get(name) {
            Some(c) => c.val.to_integer(),
            None => 0,
        }
    }

    fn to_float(&self, name: &str) -> f64 {
        match self.get(name) {
            Some(c) => c.val.to_float(),
            None => 0.0,
        }
    }
}
