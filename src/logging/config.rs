// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use clap::Args;

use super::Level;

#[derive(Args, Debug, Clone, PartialEq, Eq, PartialOrd)]
/// Rift logging configuration struct.
pub struct Config {
    #[arg(short = 'l', long = "log-level", env = "RIFT_LOG_LEVEL", help = "Set which logging level to utilize", long_help = None, default_value = "info")]
    pub level: Level,
    #[arg(short = 'j', long = "log-json", env = "RIFT_LOG_JSON", help = "Whether or not to log in JSON format.", long_help = None)]
    pub json: bool,
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use clap::Parser;

    use super::*;

    #[derive(Parser, Debug, Clone, PartialEq, PartialOrd)]
    struct TestCommand {
        #[command(flatten)]
        logging: Config,
    }

    #[test]
    fn test_config() {
        let test_cmd =
            TestCommand::try_parse_from(vec![OsString::from("test"), OsString::from("-j")])
                .expect("Shouldn't have errored.");
        assert!(test_cmd.logging.json);
        assert_eq!(test_cmd.logging.level, Level::Info);

        let cloned = test_cmd.clone();
        assert_eq!(cloned, test_cmd);
        assert!(cloned >= test_cmd);
        assert!(cloned <= test_cmd);
        assert!(cloned == test_cmd);
        assert_eq!(format!("{:?}", cloned), format!("{:?}", test_cmd))
    }
}
