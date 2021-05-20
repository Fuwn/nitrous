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
        {
          let argument = matches
            .subcommand_matches("check")
            .unwrap()
            .value_of("file");
          if argument.is_some() {
            argument.unwrap()
          } else {
            if std::fs::File::open("nitrous/codes.txt").is_err() {
              panic!("cannot open nitrous generated codes.txt");
            } else {
              "nitrous/codes.txt"
            }
          }
        },
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
             `./nitrous/codes.txt`. If you would like to override this behaviour, specify your \
             file after the subcommand.",
          )
          .arg(
            Arg::with_name("file")
              .required(false)
              .takes_value(true)
              .index(1),
          ),
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
