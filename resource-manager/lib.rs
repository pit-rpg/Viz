extern crate serde;
extern crate serde_json;
extern crate tar;

use std::io::prelude::*;
// use std::fs::File;
use tar::Archive;

use self::serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

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
	pub name: PathBuf,
	pub data_type: String,
	pub path: PathBuf,
	// pub data: Option<Vec<u8>>,
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
	data_loaded: HashMap<String, Vec<u8>>,
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
		self.data_loaded.clear();
	}

	pub fn add_package_data(&mut self, data: &[u8], name: &str) -> Result<(), String> {
		let mut archive = Archive::new(data);

		let data_loaded = &mut self.data_loaded;
		let package_list = self
			.package_list
			.as_mut()
			.ok_or("package_list is None")?;

		let package = package_list
			.packages
			.iter_mut()
			.find(|package| package.name == name)
			.ok_or(format!("package '{}' not exists", name))?;

		for file in archive.entries().or(Err("tar error"))? {
			let mut file = file.or(Err("tar file error"))?;

			let file_path = {
				PathBuf::from(&*file.path().or(Err("tar file path error"))?)
			};

			let res = package
				.resources
				.iter_mut()
				.find(|res| res.name == file_path)
				.ok_or(format!("wrong file path {:?}", file_path))?;

			let mut data = Vec::new();
			file.read(&mut data).unwrap();

			data_loaded.insert(name.into(), data);
		}

		if package.resources.iter().find(|res| data_loaded.get(&*res.name.to_string_lossy()).is_none()).is_some() {
			return Err("tar file path error")?;
		}

		Ok(())
	}

	pub fn get_resource(&self, name: &str) -> Result<&Resource, String> {
		// let package_list = self.package_list.as_ref().ok_or("package_list is None")?;
		unimplemented!()
	}

	pub fn remove_package(&mut self, name: &str) {
		unimplemented!()
	}

	pub fn load_package(&mut self, name: &str) {
		unimplemented!()
	}

	pub fn load_all_resourcess(&mut self, name: &str) {
		// let package_list = self.package_list.as_mut().ok_or("package_list is None")?;
		unimplemented!()
	}
}
