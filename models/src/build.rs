


use uuid::Uuid;




pub struct Build {


	pub id:									Uuid,

	pub jobs:								Vec<Uuid>,

	pub name:								String,


}

impl Build {
	pub fn new() -> Self {
		let id = Uuid::new_v4();
		let jobs: Vec<Uuid> = Vec::new();
		let name = String::from("New Build Pipelien");

		Build {
			id,
			jobs,
			name,
		}
	}
}
