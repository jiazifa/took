use std::{env, fmt};

use clap::App;
use config::{Config, ConfigError};

pub fn configurate(app: App) -> Result<Config, ConfigError> {
    let mut config = Config::default();
    config
        .set::<&str>("cgf_file", concat!(env!("CARGO_PKG_NAME"), ".toml"))
        .unwrap();
    config.set_default("Log.file", "_CONSOLE_").unwrap();
    config.set_default("Web.address", "127.0.0.1").unwrap();
    config.set_default("Web.port", 7878).unwrap();

    // merge env vars

    let args = app.get_matches();

    if let Some(c) = args.value_of("config") {
        config.set("cgf_config", c).unwrap();
    }

    if let Ok(cfg_file_string) = config.get_str("cgf_file") {
        let cfg_file = config::File::with_name(&cfg_file_string);
        config.merge(cfg_file)?;
    }

    if let Some(url) = db_conn_string(&config) {
        env::set_var("DATABASE_URL", url);
    }

    Ok(config)
}

pub fn db_conn_string(config: &Config) -> Option<String> {
    let keys = vec![
        "Database.type",
        "Database.username",
        "Database.password",
        "Database.host",
        "Database.port",
        "Database.database",
    ];
    let (string_vec, error_vec) = keys.into_iter().map(|k| config.get_str(k)).fold(
        (Vec::new(), Vec::new()),
        |(mut string_vec, mut error_vec), res| {
            match res {
                Ok(string) => string_vec.push(string),
                Err(error) => {
                    if let ConfigError::NotFound(key) = error {
                        error_vec.push(key);
                    }
                }
            }
            (string_vec, error_vec)
        },
    );
    if !error_vec.is_empty() {
        return None;
    }
    if let Ok(url) = config.get_str("Database.url") {
        return Some(url);
    }
    Some(format!(
        "{scheme}://{username}:{password}@{host}:{port}/{database}",
        scheme = string_vec[0],
        username = string_vec[1],
        password = string_vec[2],
        host = string_vec[3],
        port = string_vec[4],
        database = string_vec[5],
    ))
}

pub fn begin_log(config: &Config) {}
