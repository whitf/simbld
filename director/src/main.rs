use clap::{App, Arg};
use std::env;
use std::path::{Path, PathBuf};
use std::thread;

pub mod api;
pub mod communication;
pub mod config;
pub mod db;
pub mod freyr;
pub mod heimdallr;
pub mod mimir;
pub mod web;


// Parse config and launch sub processes.
#[allow(unreachable_code)]
fn main() {

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	println!();
	println!("Starting simbld director v.{} ...", VERSION);

	let matches = App::new("simbld - Director")
		.version(VERSION)
		.about("A simple build system.")
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.takes_value(true)
			.help("Specify the path to a config file. Default: (dev) PWD/data/etc/simbld/simbld_conf.toml"))
		.get_matches();

	let pwd = env::current_dir().unwrap();
	let config_default = PathBuf::from(pwd.to_str().unwrap().to_owned() + "/data/etc/simbld/simbld_conf.toml");

	let config_file = matches.value_of("config").unwrap_or(config_default.to_str().unwrap());

	// @TODO - Most of the println! stuff should be handled by the logger (to std out or std err).
	println!("- config path = {}", config_file);
	let mut config = config::Config::new(config_file);
	config.save();

	let mut db = db::Db::new();
	db.verify();

	let mut mimir = mimir::Mimir::new();

	println!("+ Starting api module...");

	let api_handle = thread::spawn(move || {
		let mut api_process = api::Api::new();
		api_process.run();
	});

	println!("+ Starting communication module...");

	let comm_handle = thread::spawn(move || {
		let mut comm_process = communication::Communication::new(config.comm_address, config.comm_port);
		comm_process.run();
	});

	println!("+ Starting freyr module...");

	let freyr_handle = thread::spawn(move || {
		let mut freyr_process = freyr::Freyr::new();
		freyr_process.run();
	});

	println!("+ Starting heimdallr module...");

	let heimdallr_handle = thread::spawn(move || {
		let mut heimdallr_process = heimdallr::Heimdallr::new();
		heimdallr_process.run();
	});

	println!("+ Starting web portal module...");

	let web_handle = thread::spawn(move || {
		let mut web_process = web::Web::new();
		web_process.run();
	});

	println!("simbld director v.{} is up and running.", VERSION);

	api_handle.join().unwrap();
	comm_handle.join().unwrap();
	freyr_handle.join().unwrap();
	heimdallr_handle.join().unwrap();
	web_handle.join().unwrap();

	println!();
	println!("Exit simbld director.");
	println!();
}
