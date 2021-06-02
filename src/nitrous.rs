// Copyleft (ɔ) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use std::{
  fs::{create_dir, File},
  io::{BufRead, Write},
};

use rand::Rng;

pub struct Nitrous;
impl Nitrous {
  pub async fn execute() {
    // Environment
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "nitrous=trace");

    // Logging
    pretty_env_logger::init();
    human_panic::setup_panic!();

    crate::cli::Cli::execute().await;
  }

  fn initialize() { create_dir("nitrous").unwrap(); }

  pub fn generate(amount: usize, debug: bool) {
    Self::initialize();

    let mut codes = File::create("nitrous/codes.txt").unwrap();

    for _ in 0..amount {
      let code = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>();

      writeln!(codes, "{}", code).unwrap();
      if debug {
        info!("{}", code,);
      }
    }
  }

  pub async fn check(codes_file_name: &str, debug: bool) {
    Self::initialize();

    create_dir("nitrous/check/").unwrap();
    let codes = File::open(codes_file_name).unwrap();
    let mut invalid = File::create("nitrous/check/invalid.txt").unwrap();
    let mut valid = File::create("nitrous/check/valid.txt").unwrap();

    for code in std::io::BufReader::new(codes).lines() {
      let code = code.unwrap();
      let status = reqwest::get(format!(
		      "https://discordapp.com/api/v6/entitlements/gift-codes/{}?with_applica\
		      tion=false&with_subscription_plan=true",
		      code
	      ))
      .await
      .unwrap()
      .status()
      .as_u16();

      if status == 200 {
        writeln!(valid, "{}", code).unwrap();
        if debug {
          info!("{}", code);
        }
      } else {
        writeln!(invalid, "{}", code).unwrap();
        if debug {
          error!("{}", code);
        }
      }
    }
  }
}
