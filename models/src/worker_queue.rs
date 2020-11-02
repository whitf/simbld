use std::collections::HashMap;
use uuid::Uuid;

use crate::job::{Job, JobStatus};
use crate::worker::{Worker};

// Work: a "list" of jobs for a worker.
pub struct Work {
	pub jobs:						Vec<Job>,
}

impl Work {
	pub fn new() -> Self {
		let jobs: Vec<Job> = Vec::new();

		Work {
			jobs,
		}
	}

	pub fn get(&mut self, job_id: Uuid) -> Option<&Job> {
		let mut k = 0;

		for job in &self.jobs {
			if job_id == job.id {
				return Some(job);
			}

			k = k + 1;
		}

		return None;
	}

	pub fn push(&mut self, job: Job) {
		self.jobs.push(job);
	}
}

// Hold all jobs for a specific worker.
pub struct WorkerQueue {
	pub online:							Vec<Uuid>,
	pub jobqueue:						HashMap<Uuid, Work>,
	pub round_robin_queue:				Vec<Uuid>,
	pub workforce:						HashMap<Uuid, Worker>,

}

impl WorkerQueue {
	pub fn new() -> Self {

		let online: Vec<Uuid> = Vec::new();
		let jobqueue: HashMap<Uuid, Work> = HashMap::new();
		let round_robin_queue = Vec::new();
		let workforce: HashMap<Uuid, Worker> = HashMap::new();

		WorkerQueue {
			online,
			jobqueue,
			round_robin_queue,
			workforce,
		}
	}

	// Assign job to worker_id.
	pub fn assign(&mut self, worker_id: Uuid, mut job: Job) {
		job.worker = Some(worker_id);
		if self.jobqueue.contains_key(&worker_id) {
			let work = self.jobqueue.get_mut(&worker_id).unwrap();
			work.push(job);
		} else {
			let mut work = Work::new();
			work.push(job);
			self.jobqueue.insert(worker_id, work);
		}
	}

	pub fn assign_next(&mut self, job: Job) {
		let worker_id = self.round_robin_queue.remove(0);
		self.round_robin_queue.push(worker_id);
		self.assign(worker_id, job);
	}

	pub fn online(&mut self, worker_id: Uuid) {
		self.online.push(worker_id);
		self.round_robin_queue.push(worker_id);
	}

	pub fn offline(&mut self, worker_id: Uuid) {
		let mut k = 0;
		let mut key = 0;
		for v in &self.online {
			if worker_id == *v {
				key = k;
				println!("(offline) found key, removing it from the online: Vec<Uuid>");
				break;
			}

			k = k + 1;
		}

		self.online.remove(key);

		k = 0;
		key = 0;
		for v in &self.round_robin_queue {
			if worker_id == *v {
				key = k;
				println!("(offline) found key, removing it from the round_robin_queue: Vec<Uuid>");
				break;
			}

			k = k + 1;
		}

		self.round_robin_queue.remove(key);

		if let Some(w) = self.workforce.get_mut(&worker_id) {
			w.online = false;
		}
	}

	pub fn register(&mut self, worker: Worker) {
		let worker_id = worker.id;

		if worker.online {
			self.online(worker_id);
		}

		self.workforce.insert(worker_id, worker);
	}

	pub fn remove(&mut self, worker_id: Uuid) {
		self.offline(worker_id);

		self.workforce.remove(&worker_id);

		// When we remove a worker, any jobs assigned to it (in the jobqueue) need to be re distributed.

		if let Some(work) = self.jobqueue.remove(&worker_id) {
			for mut job in work.jobs {
				job.worker = None;
				job.status = JobStatus::New;
				self.push(job);
			}
		}
	}

	pub fn push(&mut self, job: Job) {
		if let Some(worker_id) = job.worker {
			self.assign(worker_id, job);
		} else {
			self.assign_next(job);
		}
	}

	pub fn cancel(&mut self, _job_id: Uuid) {}

	pub fn pause(&mut self, _job_id: Uuid) {}

	pub fn start(&mut self, _job_id: Uuid) {}

	// set a worker "offline", but do not cancel the jobs.
	pub fn suspend(&mut self, _worker_id: Uuid) {}

	// Set a worker offline, canceling all jobs on that worker.
	pub fn force_halt(&mut self, _worker_id: Uuid) {}
}
