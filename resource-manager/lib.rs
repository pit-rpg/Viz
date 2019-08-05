extern crate serde;
extern crate serde_json;

use std::path::PathBuf;
use self::serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct LoadedResource {
	pub name: String,
	pub data_type: String,
	pub path: PathBuf,
	pub content_type: String,
	pub bin: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
	pub name: String,
	pub data_type: String,
	pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
	pub name: String,
	pub priority: i32,
	pub resources: Vec<Resource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageList {
	pub packages: Vec<Package>,
}



pub struct ResourceManager {

}