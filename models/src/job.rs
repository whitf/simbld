use std::time::SystemTime;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, ToString)]
pub enum JobStatus {
	Blocked,
	Cancelled,
	Failed,
	Pending,
	Running,
	Success,
	Queued,
}

#[derive(Debug)]
pub struct Job {
	pub build:					Uuid,
	pub created_time:			SystemTime,
	pub config_file:			String,
	pub elapsed_time:			f64,
	pub id:						Uuid,
	pub last_failure:			f64,
	pub last_success:			f64,
	pub name:					String,
	pub number:					u32,
	pub start_time:				f64,
	pub status:					JobStatus,
	pub suspended:				bool,
	pub worker:					Option<Uuid>,
}

impl Job {
	pub fn new(build: Uuid, name: String, config_file: String, number: u32) -> Self {

		let created_time: SystemTime = SystemTime::now();

		let elapsed_time: f64 = 0.0;
		let id = Uuid::new_v4();
		let last_failure: f64 = -1.0;
		let last_success: f64 = -1.0;

		let start_time: f64 = -1.0; 
		let status = JobStatus::Pending;
		let suspended: bool = true;
		let worker: Option<Uuid> = None;

		Job {
			build,
			created_time,
			config_file,
			elapsed_time,			
			id,
			last_failure,
			last_success,
			name,
			number,
			start_time,
			status,
			suspended,
			worker,
		}

	}
}
