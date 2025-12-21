//! Logging initialization based on the JJ_LOG environment variable

use std::io;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

const ENV_VAR_NAME: &str = "JJ_LOG";

/// Initialize tracing with the default configuration.
/// The logging level can be controlled by the JJ_LOG environment variable.
///
/// Examples:
/// - `JJ_LOG=debug` - Enable debug logs
/// - `JJ_LOG=info` - Enable info and above
/// - `JJ_LOG=jj_spr=debug` - Enable debug logs only for the jj_spr crate
/// - `JJ_LOG=trace` - Enable all trace logs including dependencies
pub fn init() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::metadata::LevelFilter::WARN.into())
        .with_env_var(ENV_VAR_NAME)
        .from_env_lossy();

    let fmt_layer = tracing_subscriber::fmt::layer().with_writer(io::stderr);

    tracing_subscriber::registry()
        .with(fmt_layer.with_filter(filter))
        .init();
}
