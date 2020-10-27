use clap::{App, Arg};
use std::env;
use std::path::PathBuf;
use std::thread;

use simbld_models::message::Message;
use simbld_mimir::mimir::Mimir;

pub mod bifrost;
pub mod config;
pub mod db;					// sqlite3 local database to keep track of job related stuffs.
pub mod loki;
pub mod sif;

#[allow(unreachable_code)]
fn main() {

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	println!();
	println!("Starting simbld worker v.{} ...", VERSION);

	let matches = App::new("simbld - worker")
		.version(VERSION)
		.about("A simple build system.")
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.takes_value(true)
			.help("Specify the path to a config file. Default: (dev) PWD/data/etc/simbld/simbld_worker_conf.toml"))
		.get_matches();

	let pwd = env::current_dir().unwrap();
	let config_default = PathBuf::from(pwd.to_str().unwrap().to_owned() + "/data/etc/simbld/simbld_worker_conf.toml");
	let config_file = matches.value_of("config").unwrap_or(config_default.to_str().unwrap());

	// @TODO: hand most stuff off to the logger here....

	println!("- config path = {}", config_file);
	// @TODO fix this.
	let mut config = config::Config::new(config_file.to_string());
	config.save();

	let mut db = db::Db::new();
	if !db.verify() {
		println!("Error with database configuration...");
	}

	let _mimir = Mimir::new();
	let _bf = bifrost::Bifrost::new();


	let loki_handle = thread::spawn(move || {
		let mut loki_process = loki::Loki::new();
		loki_process.run();
	});

	let sif_handle = thread::spawn(move || {
		let mut sif_process = sif::Sif::new();
		sif_process.run();
	});

	loki_handle.join().unwrap();
	sif_handle.join().unwrap();

	println!();
	println!("Exit simbld worker.");
	println!();
}
