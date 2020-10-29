use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use simbld_models::log;

pub struct Mimir {
	pub access_log:				PathBuf,
	pub err_log:				PathBuf,
	pub lrx:					Receiver<log::Log>,
	pub online:					bool,
	pub sys_log:				PathBuf,
}

impl Mimir {
    pub fn new(lrx: Receiver<log::Log>, log_dir: String) -> Self {
    	let online: bool = false;
    	let access_log = PathBuf::from(log_dir.clone() + "/access_log.log");
    	let err_log = PathBuf::from(log_dir.clone() + "/err_log.log");
    	let sys_log = PathBuf::from(log_dir + "/sys_log.log");

        Mimir {
        	access_log,
        	err_log,
        	lrx,
        	online,
        	sys_log,
        }
    }

    pub fn run(&mut self) {
    	self.online = true;

    	while self.online {
    		let data = self.lrx.recv();
    		let log: log::Log = data.unwrap();
    	   	let log_str = format!("{:?} - {:?} - {:?} - {}\n", log.date_time, log.module, log.log_type, log.message);

	    	match log.log_type {
	    		log::LogType::Access => {
	    			let mut file = OpenOptions::new().create(true).append(true).open(self.access_log.to_str().unwrap().to_owned())
	    				.expect("Can not open access_log.");
	    			file.write_all((log_str).as_bytes()).expect("write failed");
	    		},
	    		log::LogType::Critical |
	    		log::LogType::Error |
	    		log::LogType::Warning => {
	    			let mut file = OpenOptions::new().create(true).append(true).open(self.err_log.to_str().unwrap().to_owned())
	    				.expect("Can not open error log.");
	    			file.write_all((log_str).as_bytes()).expect("write failed");
	    		},
	    		log::LogType::System => {
	    			let mut file = OpenOptions::new().create(true).append(true).open(self.sys_log.to_str().unwrap().to_owned())
	    				.expect("Can not open sys log.");
	    			file.write_all((log_str).as_bytes()).expect("write failed");
	    		},
	    		log::LogType::StdOut => {
	    			println!("{}", log.message);
	    		},
	    		log::LogType::StdErr => {
	    			eprintln!("{}", log.message);
	    		},
	    	}
    	}
    }
}
