extern crate clap;
extern crate colored;
extern crate serde_json;


use clap::{App, Arg, SubCommand};
#[macro_use]
use serde::{Serialize, Deserialize};
use colored::*;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

// use std::env;
// use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct LoadedResource {
    name: String,
    data_type: String,
    path: PathBuf,
    content_type: String,
    bin: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Resource {
    name: String,
    data_type: String,
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Package {
    name: String,
    priority: i32,
    resources: Vec<Resource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PackageList {
    packages: Vec<Package>,
}


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
            SubCommand::with_name("build").about("build resources").arg(
                Arg::with_name("list")
                    .short("l")
                    .multiple(true)
                    .required(true)
                    .takes_value(true)
                    .help("print debug information verbosely"),
            ),
        )
        .get_matches();


    println!("{:?}", matches);
    println!("===================");

    // if matches.subcommand.is_none() {
    //     return;
    // }

    match matches.subcommand {
        None => {
            println!("{}", matches.usage.unwrap());
            return;
        }
        Some(command) => match &command.name[..] {
            "build" => {
                let vals = &command.matches.args.get("list").unwrap().vals;
                build(vals);
            }
            _ => unimplemented!(),
        }, // Some(command) => println!(" ==> {:?}", command)
    }

}

fn build(files: &Vec<std::ffi::OsString>) {
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
        let context = std::fs::read_to_string(path_buf).unwrap();
        println!("{}", context);

        let mut data: PackageList = serde_json::from_str(&context).unwrap();
        println!("{:?}", data);

        data.packages.drain(..).for_each(|package| {
            if packages.get(&package.name).is_some() {
                println!(
                    "{}",
                    format!("package: '{}' override", package.name).bright_yellow()
                );
            }
            packages.insert(package.name.clone(), package);
        });
    });

    let mut packages: Vec<Package> = packages.drain().map(|item| item.1).collect();
    packages.sort_by_key(|item| item.priority);

    let mut resources = HashMap::new();
    packages.iter().for_each(|package| {
        package.resources.iter().for_each(|resource| {
            let exists = resources.get(&resource.name).is_some();
            if exists {
                println!(
                    "{}",
                    format!("resource: '{}' override", resource.name).bright_yellow()
                );
            }
            resources.insert(resource.name.clone(), resource.clone());
        });
    });

    packages.iter_mut().for_each(|package| {
        package.resources = package.resources.iter().map(|resource| {
            if resources.get(&resource.name).is_some() {
                resources.remove(&resource.name)
            } else {
                None
            }
        }).filter(|item| item.is_some()).map(|item| item.unwrap() ).collect();
    });

    println!("============================");

    // println!("{:?}", packages.len());
}
