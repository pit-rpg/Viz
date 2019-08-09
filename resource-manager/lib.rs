extern crate serde;
extern crate serde_json;
extern crate tar;

use std::io::prelude::*;
// use std::fs::File;
use tar::Archive;

use self::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{PathBuf, Path};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct LoadedResource {
// 	pub name: String,
// 	pub data_type: String,
// 	pub path: PathBuf,
// 	pub content_type: String,
// 	pub bin: Option<Vec<u8>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Resource {
	pub name: PathBuf,
	pub data_type: String,
	pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
	pub name: String,
	#[serde(default)]
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
	package_list_path: Option<PathBuf>,
	data_loaded: HashMap<String, Vec<u8>>,
}

impl Resource {
	pub fn name_str(&self) -> String {
		(*self.name.to_string_lossy()).to_string()
	}
}

impl ResourceManager {
	pub fn parse_package_list(data: &str) -> Result<PackageList, String> {
		let package_list: Result<PackageList, serde_json::Error> = serde_json::from_str(data);
		package_list.or_else(|e| Err(format!("{:?}", e)))
	}

	pub fn set_package_list(&mut self, package_list: PackageList) {
		self.package_list = Some(package_list);
	}

	pub fn load_package_list(&mut self, path: &PathBuf) -> Result<(), String> {
		let data = std::fs::read_to_string(path)
			.or(Err(format!("incorrect path:{}", path.to_string_lossy())))?;
		self.package_list = Some(Self::parse_package_list(&data)?);
		self.package_list_path = Some(path.clone());

		Ok(())
	}

	pub fn set_package_list_from_str(&mut self, data: &str) -> Result<(), String> {
		let data = ResourceManager::parse_package_list(data)?;
		self.set_package_list(data);
		Ok(())
	}

	pub fn get_package_list(&self) -> Result<&PackageList, String> {
		Ok(self.package_list.as_ref().ok_or("package_list is None")?)
	}

	pub fn clear(&mut self) {
		self.package_list = None;
		self.package_list_path = None;
		self.data_loaded.clear();
	}

	pub fn add_package_data(&mut self, data: &[u8], name: &str) -> Result<(), String> {
		let mut archive = Archive::new(data);

		let data_loaded = &mut self.data_loaded;
		let package_list = self.package_list.as_mut().ok_or("package_list is None")?;

		let package = package_list
			.packages
			.iter_mut()
			.find(|package| package.name == name)
			.ok_or(format!("package '{}' not exists", name))?;

		for file in archive.entries().or(Err("tar error"))? {
			let mut file = file.or(Err("tar file error"))?;

			let file_path = { PathBuf::from(&*file.path().or(Err("tar file path error"))?) };

			let res = package
				.resources
				.iter_mut()
				.find(|res| res.name == file_path)
				.ok_or(format!("wrong file path {:?}", file_path))?;

			let mut data = Vec::new();
			file.read(&mut data).unwrap();

			data_loaded.insert(res.name_str(), data);
		}

		if package
			.resources
			.iter()
			.find(|res| data_loaded.get(&res.name_str()).is_none())
			.is_some()
		{
			return Err("tar file path error")?;
		}

		Ok(())
	}

	pub fn get_resource_data(&self, name: &str) -> Option<&Vec<u8>> {
		self.data_loaded.get(name)
	}

	pub fn remove_package(&mut self, name: &str) -> Result<(), String> {
		let data_loaded = &mut self.data_loaded;
		let package_list = self.package_list.as_mut().ok_or("package_list is None")?;

		let package = package_list
			.packages
			.iter_mut()
			.find(|package| package.name == name)
			.ok_or(format!("package '{}' not exists", name))?;

		package.resources.iter_mut().for_each(|res| {
			data_loaded.remove(&res.name_str());
		});

		package_list.packages = package_list
			.packages
			.drain(..)
			.filter(|package| package.name != name)
			.collect();
		Ok(())
	}

	pub fn load_package_from_disk(&mut self, name: &str) -> Result<(), String> {
		let package_list_path = self
			.package_list_path
			.as_ref()
			.ok_or("package list path is not specified")?;
		let package_list = self.package_list.as_mut().ok_or("package_list is None")?;
		let data_loaded = &mut self.data_loaded;

		let package = package_list
			.packages
			.iter_mut()
			.find(|package| package.name == name)
			.ok_or(format!("package '{}' not exists", name))?;

		for res in package.resources.iter_mut() {
			let mut path = package_list_path.clone();
			path.pop();
			path.join(&res.path);

			let buffer = std::fs::read(&path)
				.or(Err(format!("cant read file: {}", path.to_string_lossy())))?;

			data_loaded.insert(res.name_str(), buffer);
		}

		Ok(())
	}

	pub fn load_all_packages(&mut self) -> Result<(), String> {
		let package_names: Vec<String> = {
			self.get_package_list()?
				.packages
				.iter()
				.map(|package| package.name.clone())
				.collect()
		};

		for package in package_names {
			self.load_package_from_disk(&package)?;
		}

		Ok(())
	}

	pub fn load_all(&mut self, path: &PathBuf) -> Result<(), String> {
		self.load_package_list(path)?;
		self.load_all_packages()
	}

	fn get_package_list_mut(&mut self) -> Result<&mut PackageList, String> {
		Ok(self.package_list.as_mut().ok_or("package_list is None")?)
	}
}
