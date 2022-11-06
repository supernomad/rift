// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::OsString;

use clap::{error::ErrorKind, Parser};

pub fn config<T>(args: Vec<OsString>) -> Result<T, (exitcode::ExitCode, String)>
where
    T: Parser,
{
    T::try_parse_from(args).map_err(|err| {
        if err.kind() == ErrorKind::DisplayHelp || err.kind() == ErrorKind::DisplayVersion {
            (exitcode::USAGE, err.render().to_string())
        } else {
            (exitcode::CONFIG, err.render().to_string())
        }
    })
}

#[cfg(test)]
mod tests {
    use clap::crate_version;
    use rstest::rstest;

    use super::*;

    #[derive(Debug, Parser, PartialEq)]
    #[command(
        author = "Christian Saide <me@csaide.dev>",
        about = "Run an instance of primsd.",
        version = crate_version!()
    )]
    struct TestConfig {
        #[structopt(long = "field", short = 'f')]
        field: bool,
        #[structopt(long = "number", short = 'n', default_value = "0")]
        number: usize,
    }

    #[rstest]
    #[case(Vec::default(), Ok(TestConfig{field: false, number: 0}))]
    #[case(vec![OsString::from("test"), OsString::from("-f")], Ok(TestConfig{field: true, number: 0}))]
    #[case(vec![OsString::from("test"), OsString::from("-h")], Err((exitcode::USAGE, String::default())))]
    #[case(vec![OsString::from("test"), OsString::from("-V")], Err((exitcode::USAGE, String::default())))]
    #[case(vec![OsString::from("test"), OsString::from("-n"), OsString::from("nope")], Err((exitcode::CONFIG, String::default())))]
    fn test_config(
        #[case] args: Vec<OsString>,
        #[case] expected: Result<TestConfig, (exitcode::ExitCode, String)>,
    ) {
        let result = config::<TestConfig>(args);
        if expected.is_ok() {
            assert!(result.is_ok());
            let result = result.unwrap();
            let expected = expected.unwrap();
            assert_eq!(result, expected);
            assert_eq!(format!("{:?}", result), format!("{:?}", expected))
        } else {
            assert!(result.is_err());
            let result = result.unwrap_err();
            let expected = expected.unwrap_err();
            assert_eq!(result.0, expected.0);
        }
    }
}
