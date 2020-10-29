use clap::{App, Arg};
use std::env;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use simbld_models::message::Message;
use simbld_models::module::ModuleName;
use simbld_models::log::{Log, LogType};
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

	let (ltx, lrx) = mpsc::channel::<Log>();
	let (stx, _srx) = mpsc::channel::<Message>();

	let pwd = env::current_dir().unwrap();
	let log_dir = PathBuf::from(pwd.to_str().unwrap().to_owned() + "/data/var/log/simbld_worker");

	ltx.send(Log::new(ModuleName::Worker, LogType::System, String::from("Starting mimir_process."))).unwrap();
	let mimir_handle = thread::spawn(move || {
		let mut mimir_process = Mimir::new(lrx, log_dir.to_str().unwrap().to_owned());
		mimir_process.run();
	});

	let mut db = db::Db::new();
	if !db.verify() {
		println!("Error with database configuration...");
	}

	ltx.send(Log::new(ModuleName::Worker, LogType::System, String::from("Starting bifrost_process"))).unwrap();
	let bifrost_ltx = ltx.clone();
	let bifrost_stx = stx.clone();
	let (btx, brx) = mpsc::channel::<Message>();
	let bifrost_handle = thread::spawn(move || {
		let mut bifrost_process = bifrost::Bifrost::new(bifrost_ltx, bifrost_stx, brx);
		bifrost_process.run();
	});

	ltx.send(Log::new(ModuleName::Worker, LogType::System, String::from("Starting loki_process."))).unwrap();

	let _loke_ltx = ltx.clone();
	let _lok_btx = btx.clone();
	let loki_handle = thread::spawn(move || {
		let mut loki_process = loki::Loki::new();
		loki_process.run();
	});

	let sif_handle = thread::spawn(move || {
		let mut sif_process = sif::Sif::new();
		sif_process.run();
	});

	bifrost_handle.join().unwrap();
	mimir_handle.join().unwrap();
	loki_handle.join().unwrap();
	sif_handle.join().unwrap();

	println!();
	println!("Exit simbld worker.");
	println!();
}
