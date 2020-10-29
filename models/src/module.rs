use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, ToString)]
pub enum ModuleName {
	Api,
	Bifrost,
	Communication,
	DbDirector,
	DbWorker,
	Director,
	Freyr,
	Heimdallr,
	Loki,
	Mimir,
	Sif,
	Web,
	Worker,
}
