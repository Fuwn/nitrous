// Copyleft (ɔ) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use std::str::FromStr;

use structopt::{
  clap,
  clap::{App, Arg, SubCommand},
};

use crate::nitrous::Nitrous;

pub enum ProxyType {
  Http,
  Socks4,
  Socks5,
  Tor,
}
impl FromStr for ProxyType {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "http" => Ok(Self::Http),
      "socks4" => Ok(Self::Socks4),
      "socks5" => Ok(Self::Socks5),
      "tor" => Ok(Self::Tor),
      _ => Err("no match"),
    }
  }
}

pub struct Cli;
impl Cli {
  pub async fn execute() {
    let matches = Self::cli().get_matches();

    let debug = matches.is_present("debug");

    if matches.is_present("generate") {
      Nitrous::generate(
        matches
          .subcommand_matches("generate")
          .unwrap()
          .value_of("amount")
          .unwrap()
          .to_string()
          .parse::<usize>()
          .unwrap(),
        debug,
      );
    } else if matches.is_present("check") {
      Nitrous::check(
        {
          let argument = matches
            .subcommand_matches("check")
            .unwrap()
            .value_of("file");
          if argument.is_some() {
            argument.unwrap()
          } else if std::fs::File::open(".nitrous/codes.txt").is_err() {
            panic!("cannot open nitrous generated codes.txt");
          } else {
            ".nitrous/codes.txt"
          }
        },
        debug,
        ProxyType::from_str(
          matches
            .subcommand_matches("check")
            .unwrap()
            .value_of("proxy_type")
            .unwrap(),
        )
        .unwrap(),
        matches
          .subcommand_matches("check")
          .unwrap()
          .value_of("proxy_list")
          .unwrap_or("null"),
      )
      .await;
    }
  }

  fn cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
      .about(env!("CARGO_PKG_DESCRIPTION"))
      .version(env!("CARGO_PKG_VERSION"))
      .author(env!("CARGO_PKG_AUTHORS"))
      .setting(clap::AppSettings::SubcommandRequiredElseHelp)
      .subcommands(vec![
        SubCommand::with_name("generate")
          .alias("gen")
          .about("Generate a specified number Discord Nitro codes")
          .arg(
            Arg::with_name("amount")
              .required(true)
              .index(1)
              .takes_value(true),
          ),
        SubCommand::with_name("check")
          .about("Check a file of Discord Nitro codes for valid/ invalid codes")
          .long_about(
            "Check a file of Discord Nitro codes for valid/ invalid codes.\n\nIf a codes file is \
             not explicitly specified, the check routine will run on a default file value of \
             `./.nitrous/codes.txt`. If you would like to override this behaviour, specify your \
             file after the subcommand.",
          )
          .arg(
            Arg::with_name("file")
              .required(false)
              .takes_value(true)
              .long("file")
              .short("f"),
          )
          .args(&[
            Arg::with_name("proxy_type")
              .required(true)
              .takes_value(true)
              .index(1)
              .possible_values(&["http", "socks4", "socks5", "tor"]),
            Arg::with_name("proxy_list")
              .required_ifs(&[
                ("proxy_type", "http"),
                ("proxy_type", "socks4"),
                ("proxy_type", "socks5"),
              ])
              .takes_value(true)
              .index(2),
          ]),
      ])
      .arg(
        Arg::with_name("debug")
          .long("debug")
          .short("d")
          .takes_value(false)
          .multiple(false)
          .global(true),
      )
  }
}
