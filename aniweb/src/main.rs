use aniweb::configuration::get_configuration;
use aniweb::telemetry::init_subscriber;
use aniweb::webmanage::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(std::io::stdout);
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", configuration.host, configuration.port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
