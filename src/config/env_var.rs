// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use regex::Regex;
use serde::{Deserialize, Deserializer};
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Deserialize, PartialEq)]
struct WrappedString(String);

pub fn str<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    match self::is_env_var(&value) {
        true => Ok(self::get_env_var(&value)),
        false => Ok(value),
    }
}

pub fn opt_str<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<WrappedString>::deserialize(deserializer).map(|option: Option<WrappedString>| {
        option.map(|wrapped: WrappedString| {
            let value = wrapped.0;

            match self::is_env_var(&value) {
                true => self::get_env_var(&value),
                false => value,
            }
        })
    })
}

pub fn socket_addr<'de, D>(deserializer: D) -> Result<SocketAddr, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    match self::is_env_var(&value) {
        true => Ok(self::get_env_var(&value).parse().unwrap()),
        false => Ok(value.parse().unwrap()),
    }
}

pub fn path_buf<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    match self::is_env_var(&value) {
        true => Ok(PathBuf::from(self::get_env_var(&value))),
        false => Ok(PathBuf::from(value)),
    }
}

fn is_env_var(value: &str) -> bool {
    Regex::new(r"^\$\{env\.\w+\}$")
        .expect("env_var: regex is invalid")
        .is_match(value)
}

fn get_env_var(wrapped_key: &str) -> String {
    let key: String = String::from(wrapped_key)
        .drain(6..(wrapped_key.len() - 1))
        .collect();

    std::env::var(key.clone()).expect(&format!("env_var: variable '{}' is not set", key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_environment_variable_patterns() {
        assert_eq!(self::is_env_var("${env.XXX}"), true);
        assert_eq!(self::is_env_var("${env.XXX"), false);
        assert_eq!(self::is_env_var("${env.XXX}a"), false);
        assert_eq!(self::is_env_var("a${env.XXX}"), false);
        assert_eq!(self::is_env_var("{env.XXX}"), false);
        assert_eq!(self::is_env_var("$env.XXX}"), false);
        assert_eq!(self::is_env_var("${envXXX}"), false);
        assert_eq!(self::is_env_var("${.XXX}"), false);
        assert_eq!(self::is_env_var("${XXX}"), false);
    }

    #[test]
    fn it_gets_environment_variable() {
        std::env::set_var("TEST", "test");

        assert_eq!(self::get_env_var("${env.TEST}"), "test");

        std::env::remove_var("TEST");
    }
}
