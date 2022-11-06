// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io;

use slog::Drain;

use super::{config, filter};

#[cfg(test)]
pub fn noop() -> slog::Logger {
    let drain = slog::Discard;
    slog::Logger::root(drain, o!())
}

/// Return a newly constructed slog::Logger based on the supplied configuration.
/// This also injects the application name and version as base key/value pairs for the
/// returned root logger.
///
/// # Example
/// ```
/// use slog::info;
///
/// let logger = librift::logging::new(
///     &librift::logging::Config {
///         level: librift::logging::Level::Info,
///         json: true,
///     },
///     "example",
///     "0.1.1",
/// );
///
/// info!(logger, "Hello world!"; "woot" => "woot");
/// ```
pub fn new(cfg: &config::Config, bin: &'static str, version: &'static str) -> slog::Logger {
    let drain: Box<dyn Drain<Ok = (), Err = slog::Never> + Send> = if cfg.json {
        Box::new(
            slog_json::Json::new(io::stdout())
                .add_default_keys()
                .build()
                .fuse(),
        )
    } else {
        let decorator = slog_term::TermDecorator::new().build();
        Box::new(
            slog_term::FullFormat::new(decorator)
                .use_utc_timestamp()
                .build()
                .fuse(),
        )
    };

    let drain = filter::LevelFilter {
        drain,
        level: cfg.level.to_slog(),
    }
    .fuse();

    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!("binary" => bin, "version" => version))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::logging::level;

    #[rstest]
    #[case::with_json(true)]
    #[case::with_json(false)]
    fn test_new(#[case] json: bool) {
        let cfg = config::Config {
            json,
            level: level::Level::Debug,
        };
        new(&cfg, "test", "alpha");
    }
}
