// Copyleft (ɔ) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

#![warn(rust_2018_idioms)]
#![recursion_limit = "128"]

#[macro_use]
extern crate log;

#[cfg(target_family = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(target_family = "unix")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod cli;
mod nitrous;

use crate::nitrous::Nitrous;

#[tokio::main]
async fn main() { Nitrous::execute().await; }
