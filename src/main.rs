pub mod zone;
use nostr_relay::App;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    info!("Start relay server");
    let app_data = App::create(
        Some("./config/config.toml"),
        true,
        Some("NOSTR".to_owned()),
        None,
    )?;
    let db = app_data.db.clone();
    app_data
        .add_extension(nostr_extensions::Metrics::new())
        .add_extension(nostr_extensions::Auth::new())
        .add_extension(nostr_extensions::Ratelimiter::new())
        .add_extension(nostr_extensions::Count::new(db))
        .add_extension(nostr_extensions::Search::new())
        .add_extension(zone::Zone::new())
        .web_server()?
        .await?;
    info!("Relay server shutdown");
    Ok(())
}
