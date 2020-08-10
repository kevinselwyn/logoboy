use clap::{App, AppSettings, Arg, SubCommand};
use logoboy::*;
use std::{fs, process};

fn main() {
    let app = App::new(env!["CARGO_PKG_NAME"])
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::DeriveDisplayOrder,
        ])
        .version(env!["CARGO_PKG_VERSION"])
        .author(env!["CARGO_PKG_AUTHORS"])
        .about("ROM utility for ripping/replacing the Nintendo Game Boy logo scroll")
        .subcommand(
            SubCommand::with_name("get")
                .settings(&[
                    AppSettings::ArgRequiredElseHelp,
                    AppSettings::DeriveDisplayOrder,
                ])
                .about("Get logo")
                .args(&[
                    Arg::with_name("rom")
                        .long("rom")
                        .short("r")
                        .required(true)
                        .takes_value(true)
                        .help("Input ROM"),
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .required(true)
                        .takes_value(true)
                        .help("Output PNG"),
                ]),
        )
        .subcommand(
            SubCommand::with_name("set")
                .settings(&[
                    AppSettings::ArgRequiredElseHelp,
                    AppSettings::DeriveDisplayOrder,
                ])
                .about("Set logo")
                .args(&[
                    Arg::with_name("rom")
                        .long("rom")
                        .short("r")
                        .required(true)
                        .takes_value(true)
                        .help("Input ROM"),
                    Arg::with_name("png")
                        .long("png")
                        .short("p")
                        .required(true)
                        .takes_value(true)
                        .help("Input PNG"),
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .required(true)
                        .takes_value(true)
                        .help("Output ROM"),
                ]),
        )
        .get_matches();

    if let Some(subcommand) = app.subcommand_matches("get") {
        let rompath = subcommand.value_of("rom").unwrap();
        let rom = match fs::read(rompath) {
            Ok(rom) => rom,
            Err(e) => {
                println!("{}: {}", rompath, e.to_string());

                process::exit(1)
            }
        };

        let mut logoboy = match Logoboy::new(rom) {
            Ok(logoboy) => logoboy,
            Err(e) => {
                println!("{}", e.to_string());

                process::exit(1)
            }
        };

        let output = match logoboy.get_logo() {
            Ok(output) => output,
            Err(e) => {
                println!("{}", e.to_string());

                process::exit(1)
            }
        };

        let outputpath = subcommand.value_of("output").unwrap();

        match fs::write(outputpath, output) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e.to_string());

                process::exit(1);
            }
        };
    }

    if let Some(subcommand) = app.subcommand_matches("set") {
        let rompath = subcommand.value_of("rom").unwrap();
        let rom = match fs::read(rompath) {
            Ok(rom) => rom,
            Err(e) => {
                println!("{}: {}", rompath, e.to_string());

                process::exit(1)
            }
        };

        let pngpath = subcommand.value_of("png").unwrap();
        let png = match fs::read(pngpath) {
            Ok(png) => png,
            Err(e) => {
                println!("{}: {}", pngpath, e.to_string());

                process::exit(1)
            }
        };

        let mut logoboy = match Logoboy::new(rom) {
            Ok(logoboy) => logoboy,
            Err(e) => {
                println!("{}", e.to_string());

                process::exit(1)
            }
        };

        match logoboy.set_logo(png) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e.to_string());

                process::exit(1);
            }
        }

        let output = match logoboy.get_rom() {
            Ok(output) => output,
            Err(e) => {
                println!("{}", e.to_string());

                process::exit(1)
            }
        };

        let outputpath = subcommand.value_of("output").unwrap();

        match fs::write(outputpath, output) {
            Ok(oufile) => oufile,
            Err(e) => {
                println!("{}", e.to_string());

                process::exit(1)
            }
        };
    }
}
