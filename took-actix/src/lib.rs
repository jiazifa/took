use config::Config;
use std::error::Error;

use actix::System;
use actix_web::{
    middleware::Logger,
    App, HttpServer
};

pub struct AppConfig {
    pub is_debug: bool,
    pub service: took_service::ServiceContext,
}

fn run(config: &Config, db_url: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
