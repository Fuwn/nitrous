// Copyleft (ɔ) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use structopt::{
  clap,
  clap::{App, Arg, SubCommand},
};

use crate::nitrous::Nitrous;

pub struct Cli;
impl Cli {
  pub async fn execute() {
    let matches = Self::cli().get_matches();

    let mut debug = false;
    if matches.is_present("debug") {
      debug = true;
    }

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
        matches
          .subcommand_matches("check")
          .unwrap()
          .value_of("file")
          .unwrap(),
        debug,
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
        SubCommand::with_name("generate").alias("gen").arg(
          Arg::with_name("amount")
            .required(true)
            .index(1)
            .takes_value(true),
        ),
        SubCommand::with_name("check").arg(
          Arg::with_name("file")
            .required(true)
            .takes_value(true)
            .index(1),
        ),
      ])
      .arg(Arg::with_name("debug")
        .long("debug")
        .short("d")
        .takes_value(false)
        .multiple(false)
        .global(true)
      )
  }
}
