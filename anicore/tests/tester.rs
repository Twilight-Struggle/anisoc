use tracing_subscriber;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::EnvFilter;

use anicore::game::Game;
use anicore::randai;

fn init_subscriber() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("Debug"));
    let format = tracing_subscriber::fmt::format().pretty();
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .with_env_filter(env_filter)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .event_format(format)
        .init();
}

#[test]
fn integ_test() {
    init_subscriber();
    let agent1 = randai::Randai {};
    let agent2 = randai::Randai {};
    Game::game(agent1, agent2);
}
