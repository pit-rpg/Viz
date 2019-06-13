extern crate clap;

use clap::{App, Arg, SubCommand};
use std::path::PathBuf;
use std::path::Path;


struct Resource {
    name: String,
    data_type: String,
    path: PathBuf,
}

struct Package {
    name: String,
    priority: i32,
    resources: Vec<Resource>,
}

struct PackageList {
    Packages: Vec<Package>,
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
            _ => unimplemented!()
        }
        // Some(command) => println!(" ==> {:?}", command)
    }

}

fn build(files: &Vec<std::ffi::OsString>) {
    let dir = std::env::current_dir().unwrap();

    files.iter().for_each(|item| {
        let file_path = dir.clone().join(item);

        if !file_path.is_file() {
            eprintln!("is not a file {}", file_path.to_string_lossy());
            panic!();
        }

        println!("{:?}", file_path);
    });

}
