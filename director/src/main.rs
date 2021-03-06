use clap::{App, Arg};
use std::env;
use std::path::{PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use simbld_models::job::Job;
use simbld_models::message::Message;
use simbld_models::module::ModuleName;
use simbld_models::log::{Log, LogType};
use simbld_mimir::mimir::Mimir;

// DEBUG stuffs
use generate_worker_data::job_generator::JobGenerator;

pub mod api;
pub mod communication;			// @TODO: rename this as bifrost to match the worker.
pub mod config;
pub mod db;
pub mod freyr;
pub mod heimdallr;
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

	// Logging process.
	let (ltx, lrx) = mpsc::channel::<Log>();
	let log_dir = PathBuf::from(pwd.to_str().unwrap().to_owned() + "/data/var/log/simbld_director");
	let mimir_handle = thread::spawn(move || {
		let mut mimir_process = Mimir::new(lrx, log_dir.to_str().unwrap().to_owned());
		mimir_process.run();
	});

	ltx.send(Log::new(ModuleName::Director, LogType::System, String::from("Starting db_process..."))).unwrap();
	let mut db = db::Db::new();
	db.verify();

	// Set up inter process communication channels.
	let (ftx, frx) = mpsc::channel::<Message>();


	println!("+ Starting api module...");

	let api_handle = thread::spawn(move || {
		let mut api_process = api::Api::new();
		api_process.run();
	});

	println!("+ Starting communication module...");

	let comm_ltx = ltx.clone();
	let comm_ftx = ftx.clone();

	let (comm_jtx, comm_jrx) = mpsc::channel::<Job>();

	let comm_handle = thread::spawn(move || {
		let mut comm_process = communication::Communication::new(config.comm_address, config.comm_port, comm_ltx, comm_ftx, comm_jrx);
		comm_process.run();
	});

	println!("+ Starting freyr module...");

	let freyr_handle = thread::spawn(move || {
		let mut freyr_process = freyr::Freyr::new(frx);
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

	// periodically insert worker test data.
	println!("periodically inserting worker test data...");
	let build_id = Uuid::new_v4();
	let worker_id = Uuid::new_v4();
	let test_comm_jtx = comm_jtx.clone();
	println!("generating test data for worker: {:?}", worker_id);

	let test_handle_1 = thread::spawn(move || {
		let mut job_gen_process = JobGenerator::new(build_id, None, Some(Duration::from_secs(5)), Some(true));
		job_gen_process.run(worker_id, test_comm_jtx);
	});

	api_handle.join().unwrap();
	comm_handle.join().unwrap();
	freyr_handle.join().unwrap();
	heimdallr_handle.join().unwrap();
	mimir_handle.join().unwrap();
	web_handle.join().unwrap();

	test_handle_1.join().unwrap();

	println!();
	println!("Exit simbld director.");
	println!();
}
