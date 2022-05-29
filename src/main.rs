use flexi_logger::{DeferredNow, Duplicate, Record, style, TS_DASHES_BLANK_COLONS_DOT_BLANK};

mod configs;
mod db;
mod enums;
mod routing;
mod utils;
mod structs;
mod bili_requests;

fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "[{}] {} - {}",
        style(level).paint(now.format(TS_DASHES_BLANK_COLONS_DOT_BLANK)),
        style(level).paint(record.level().to_string()),
        style(level).paint(&record.args().to_string())
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // logger
    flexi_logger::Logger::try_with_env_or_str("info")
        .unwrap()
        .duplicate_to_stderr(Duplicate::Info)
        .format(log_format)
        .start()
        .unwrap();

    // config
    let config = configs::Config::parse("config.toml").unwrap();

    // database
    db::prepare().await;

    // server
    let port = config.server.unwrap().port.unwrap_or(7788);

    routing::run_server(port).await
}
