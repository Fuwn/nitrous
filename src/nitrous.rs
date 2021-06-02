// Copyleft (ɔ) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use std::{
  fs::{create_dir, File},
  io::{BufRead, BufReader, Write},
};

use rand::{seq::IteratorRandom, Rng};

use crate::cli::ProxyType;

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

  #[allow(clippy::let_underscore_drop)]
  fn initialize() { let _ = create_dir("nitrous"); }

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

  pub async fn check(
    codes_file_name: &str,
    debug: bool,
    proxy_type: crate::cli::ProxyType,
    proxy_file: &str,
  ) {
    Self::initialize();

    #[allow(clippy::let_underscore_drop)]
    let _ = create_dir("nitrous/check/");
    let codes = File::open(codes_file_name).unwrap();
    let mut invalid = File::create("nitrous/check/invalid.txt").unwrap();
    let mut valid = File::create("nitrous/check/valid.txt").unwrap();

    for code in std::io::BufReader::new(codes).lines() {
      let proxy_addr = if matches!(&proxy_type, ProxyType::Tor) {
        "127.0.0.1:9050".to_string()
      } else {
        BufReader::new(
          File::open(proxy_file).unwrap_or_else(|e| panic!("unable to open file: {}", e)),
        )
        .lines()
        .map(|l| l.expect("couldn't read line"))
        .choose(&mut rand::thread_rng())
        .expect("file had no lines")
      };

      let code = code.unwrap();
      let status = reqwest::Client::builder()
        .proxy(reqwest::Proxy::all(format!("{}://{}", {
          match proxy_type {
            ProxyType::Http => "http",
            ProxyType::Socks4 => "socks4h",
            ProxyType::Socks5 | ProxyType::Tor => "socks5h",
          }
        }, proxy_addr)).unwrap())
        .build()
        .unwrap()
        .get(
          format!("https://discordapp.com/api/v6/entitlements/gift-codes/{}?with_application=false&\
          with_subscription_plan=true", code),
        )
        .send()
        .await
        .unwrap()
        .status()
        .as_u16();

      if status == 200 {
        writeln!(valid, "{}", code).unwrap();
        if debug {
          info!("{}: {}", proxy_addr, code);
        }
      } else {
        writeln!(invalid, "{}", code).unwrap();
        if debug {
          error!("{}: {}", proxy_addr, code);
        }
      }
    }
  }
}
