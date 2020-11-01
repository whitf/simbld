use rand::Rng;
use std::sync::mpsc::{Sender};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

use simbld_models::job::Job;

pub struct JobGenerator {
	build_id:				Uuid,
	limit:					i32,
	online:					bool,
	rand:					bool,
	tic:					Duration,
}

impl JobGenerator {
	pub fn new(build_id: Uuid, limit: Option<i32>, tic: Option<Duration>, rand: Option<bool>) -> Self {

		let mut limit_default = -1;
		let mut tic_default = Duration::from_secs(15);
		let mut rand_default = false;

		match limit {
			Some(i) => {
				limit_default = i;
			},
			_ => {},
		}

		match tic {
			Some(i) => {
				tic_default = i;
			},
			_ => {},
		}

		match rand {
			Some(i) => {
				rand_default = i;
			},
			_ => {},
		}

		JobGenerator {
			build_id,
			limit: limit_default,
			rand: rand_default,
			tic: tic_default,
			online: false,
		}
	}

	pub fn run(&mut self, worker_id: Uuid, jtx: Sender<Job>) {
		
		self.online = true;

		println!("J START JobGenerator::run()");

		let mut rng = rand::thread_rng();
		let mut count = 0u32;

		while self.online {

			let mut job = Job::new(self.build_id, String::from("test_job_") + &count.to_string(), String::from("config_file"), count);
			job.suspended = false;
			job.worker = Some(worker_id);

			println!("J SEND JobGenerator::run()");

			jtx.send(job).unwrap();

			count = count + 1;

			if self.limit != -1 {
				if (self.limit as u32 - 1) < count {
					self.online = false;
				}
			}

			let mut nap: Duration = self.tic;
			if self.rand {
				nap = nap + Duration::from_secs(rng.gen_range(0, self.tic.as_secs()));
			}

			thread::sleep(nap);
		}

		println!("J END JobGenerator::run()")
	}
}
