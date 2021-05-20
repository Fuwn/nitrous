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

    crate::cli::Cli::execute().await;
  }

  fn initialize() { let _ = create_dir("nitrous"); }

  pub fn generate(amount: usize) {
    Self::initialize();

    let mut codes = File::create("nitrous/codes.txt").unwrap();

    for _ in 0..amount {
      let code = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>();

      writeln!(codes, "{}", code).unwrap();
      info!("{}", code,)
    }
  }

  pub async fn check(codes_file_name: &str) {
    Self::initialize();

    let _ = create_dir("nitrous/check/");
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
        info!("{}", code);
      } else {
        writeln!(invalid, "{}", code).unwrap();
        error!("{}", code);
      }
    }
  }
}
