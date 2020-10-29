use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, ToString)]
pub enum ModuleName {
	Api,
	Bifrost,
	Communication,
	DbDirector,
	DbWorker,
	Freyr,
	Heimdallr,
	Loki,
	Mimir,
	Sif,
	Web,
}
