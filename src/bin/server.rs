use clap::{load_yaml, App};
use config::Config;
use std::env;
use std::error::Error;
use took::{begin_log, configurate, db_conn_string};

pub trait TookServerRunner {
    fn run(&self, config: &Config, db_url: &str) -> Result<(), Box<dyn Error>>;
}
#[cfg(feature = "actix")]
mod actix_runner {
    use super::TookServerRunner;
    pub struct Server;

    impl Server for TookServerRunner {
        fn run(&self, config: &Config, db_url: &str) -> Result<(), Box<dyn Error>> {
            took_actix::run(config, db_url)
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("server.yml");
    let app = App::from_yaml(yaml)
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"));
    let config = configurate(app).unwrap();
    let db_url = db_conn_string(&config);

    begin_log(&config);

    #[cfg(feature = "actix")]
    actix_runner::Server.run(&config, &db_url)?;

    Ok(())
}
