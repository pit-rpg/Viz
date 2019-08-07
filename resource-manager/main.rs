extern crate clap;
extern crate colored;
extern crate tar;
extern crate pathdiff;

mod lib;

use clap::{App, Arg, SubCommand};
use std::ffi::OsString;
#[macro_use]
use colored::*;

use std::collections::HashMap;
use std::fs::File;
// use std::io::prelude::*;
use lib::*;
use std::path::PathBuf;
use tar::Builder;
use pathdiff::diff_paths;
// use tar::Builder;

// use std::env;
// use std::fs;

fn main() {
	let matches = App::new("My Super Program")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Stepan K. <shtefanrpg@gmail.com>")
		.about("resource manager")
		//   .arg(Arg::with_name("config")
		//        .short("c")
		//        .long("config")
		//        .value_name("FILE")
		//        .help("Sets a custom config file")
		//        .takes_value(true))
		//   .arg(Arg::with_name("INPUT")
		//        .help("Sets the input file to use")
		//        .required(true)
		//        .index(1))
		//   .arg(Arg::with_name("v")
		//        .short("v")
		//        .multiple(true)
		//        .help("Sets the level of verbosity"))
		.subcommand(
			SubCommand::with_name("build")
				.about("build resources")
				.arg(
					Arg::with_name("list")
						.short("l")
						.multiple(true)
						.required(true)
						.takes_value(true)
						.help("print debug information verbosely"),
				)
				.arg(
					Arg::with_name("output_dir")
						.short("-o")
						.required(false)
						.takes_value(true)
						.default_value("./")
						.help("Specify output dir"),
				)
				.arg(
					Arg::with_name("single_file")
						.short("-s")
						.required(false)
						.help("force to create one file \"res.tar\""),
				),
		)
		.get_matches();

	println!("{:?}", matches);
	println!("===================");

	match matches.subcommand {
		None => {
			println!("{}", matches.usage.unwrap());
			return;
		}
		Some(command) => match &command.name[..] {
			"build" => {
				println!("************");
				println!("{:?}", command.matches);
				println!("************");

				let vals = &command.matches.args.get("list").unwrap().vals;
				let out_dir = &command.matches.args.get("output_dir").unwrap().vals[0];
				let single_file = command.matches.args.get("single_file").is_some();

				build(vals, out_dir, single_file);
			}
			_ => unimplemented!(),
		}, // Some(command) => println!(" ==> {:?}", command)
	}
}

fn build(files: &Vec<OsString>, out_dir: &OsString, single_file: bool) {
	let dir = std::env::current_dir().unwrap();

	let files: Vec<PathBuf> = files
		.iter()
		.map(|item| {
			let file_path = dir.clone().join(item);

			if !file_path.is_file() {
				eprintln!("is not a file {}", file_path.to_string_lossy());
				panic!();
			}
			file_path
		})
		.collect();

	let mut packages = HashMap::new();
	files.iter().for_each(|path_buf| {
		let mut package_dir = path_buf.clone();
		package_dir.pop();

		let context = std::fs::read_to_string(path_buf).unwrap();
		println!("{}", context);

		let mut data: PackageList = serde_json::from_str(&context).unwrap();
		println!("{:?}", data);

		data.packages.drain(..).for_each(|mut package| {
			if packages.get(&package.name).is_some() {
				println!(
					"{}",
					format!("package: '{}' override", package.name).bright_yellow()
				);
			}

			package.resources.iter_mut().for_each(|item| {
				let item_path = package_dir.clone().join(&item.path);
				if !item_path.is_file() {
					eprintln!("is not a file {}", item_path.to_string_lossy());
					panic!();
				}
				item.path = item_path;
			});

			packages.insert(package.name.clone(), package);
		});
	});

	// sort packages by priority
	let mut packages: Vec<Package> = packages.drain().map(|item| item.1).collect();
	packages.sort_by_key(|item| item.priority);

	{
		// remove duplicated resources
		let mut resources = HashMap::new();
		packages.iter().for_each(|package| {
			package.resources.iter().for_each(|resource| {
				let exists = resources.get(&resource.name).is_some();
				if exists {
					println!(
						"{}",
						format!("resource: '{:?}' override", resource.name).bright_yellow()
					);
				}
				resources.insert(resource.name.clone(), resource.clone());
			});
		});

		packages.iter_mut().for_each(|package| {
			package.resources = package
				.resources
				.iter()
				.map(|resource| {
					if resources.get(&resource.name).is_some() {
						resources.remove(&resource.name)
					} else {
						None
					}
				})
				.filter(|item| item.is_some())
				.map(|item| item.unwrap())
				.collect();
		});
	}

	println!("single_file: {}", single_file);
	if single_file {
		// merge packages if needed
		packages = vec![Package {
			name: "res".to_string(),
			priority: 0,
			resources: packages
				.drain(..)
				.map(|item| item.resources)
				.flatten()
				.collect(),
		}];
	} else {
		// or remove empty packages
		packages = packages
			.drain(..)
			.filter(|package| package.resources.len() != 0)
			.collect();
	}

	// write packages
	let package_file_dir = dir.clone().join(&out_dir);
	packages.iter_mut().for_each(|package| {
		let package_file_path = package_file_dir
			.clone()
			.join(format!("{}.tar", package.name));

		println!("{:?}, {:?}", package_file_dir, package_file_path);
		std::fs::create_dir_all(&package_file_dir).unwrap();
		let package_file = File::create(package_file_path).unwrap();

		let mut tar_file = Builder::new(package_file);

		package.resources.iter_mut().for_each(|item| {
			tar_file
				.append_file(
					item.name.clone(),
					&mut File::open(item.path.clone()).unwrap(),
				)
				.unwrap();

			item.path = diff_paths(&item.path.clone(), &package_file_dir.clone()).unwrap();
		});
	});

	let res_data = serde_json::to_string(&PackageList { packages }).unwrap();
	let res_data_path = package_file_dir.clone().join("res.json");
	std::fs::write(&res_data_path, res_data)
		.expect(&format!("Unable to write file: {:?}", res_data_path));
}
