extern crate serde;
extern crate serde_json;
extern crate tar;

use std::io::prelude::*;
// use std::fs::File;
use tar::Archive;

use self::serde::{Deserialize, Serialize};
use std::path::PathBuf;
// use self::serde_json::Error;

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
	pub data: Option<Vec<u8>>,
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

#[derive(Debug, Clone)]
pub struct ResourceManager {
	package_list: Option<PackageList>,
}

impl ResourceManager {
	pub fn parse_package_list(data: &str) -> Result<PackageList, serde_json::Error> {
		let package_list: Result<PackageList, serde_json::Error> = serde_json::from_str(data);
		package_list
	}

	pub fn set_package_list(&mut self, package_list: PackageList) {
		self.package_list = Some(package_list);
	}

	pub fn set_package_list_from_str(&mut self, data: &str) -> Result<(), serde_json::Error> {
		let data = ResourceManager::parse_package_list(data)?;
		self.set_package_list(data);
		Ok(())
	}

	pub fn get_package_list(&self) -> &Option<PackageList> {
		&self.package_list
	}

	pub fn clear(&mut self) {
		self.package_list = None;
	}

	pub fn add_package_data(&mut self, data: &[u8], name: &str) -> Result<(), String> {
		let mut archive = Archive::new(data);

		match &mut self.package_list {
			None => Err("package_list is None".to_string()),
			Some(package_list) => {
				let package = package_list
					.packages
					.iter_mut()
					.find(|package| package.name == name)
					.ok_or(format!("package '{}' not exists", name))?;

				for file in archive.entries().or(Err("tar error".to_string()))? {
					let mut file = file.or(Err("tar file error".to_string()))?;

					// // Inspect metadata about the file
					// println!("{:?}", file.header().path().unwrap());
					// println!("{}", file.header().size().unwrap());

					// files implement the Read trait
					// let mut s = String::new();



					// file.read_to_string(&mut s).unwrap();

					// println!("{}", s);
				}

				unimplemented!()
			}
		}
	}
}
