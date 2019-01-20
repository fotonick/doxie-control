mod discover;
mod doxie;

use clap::{app_from_crate, crate_name, crate_authors, crate_description, crate_version, Arg, SubCommand};
use failure::Error;
use log::info;
use simplelog::{
    Config,
    LevelFilter,
    TermLogger,
};
use std::process::exit;

fn main() -> Result<(), Error> {
    let matches = app_from_crate!()
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("increase verbosity"))
        .subcommand(SubCommand::with_name("list")
            .help("list scans"))
        .subcommand(SubCommand::with_name("download")
            .alias("dl")
            .arg(Arg::with_name("name")
                .required(true)
                .help("name of file to download; expect full path in Doxie storage"))
            .help("download a named scan"))
        .subcommand(SubCommand::with_name("download_all")
            .alias("dl_all")
            .help("download all scans"))
        .get_matches();
    let level_filter = if matches.is_present("verbose") { LevelFilter::Info } else { LevelFilter::Error };
    TermLogger::init(level_filter, Config::default()).unwrap();

    let doxie_base_url = discover::discover_doxie().unwrap_or_else(|| { eprintln!("Couldn't find Doxie"); exit(1); });
    let doxie = doxie::Doxie::from_base_url_string(&doxie_base_url).expect("Should really be able to construct a Doxie struct from the URL");
    info!("Found Doxie at {}", doxie.base_url);
    let scans = doxie.list_scans().expect("couldn't list scans");
    match matches.subcommand() {  // guaranteed to exist by .required(true)
        ("list", Some(_)) => {
            println!("scans = {:?}", scans);
            Ok(())
        }
        ("download", Some(sub_matches)) => {
            let name = sub_matches.value_of("name").unwrap();
            match doxie.download_scan_by_name(name, None) {
                Ok(filename) => { println!("{} → {}", name, filename); Ok(()) }
                Err(e) => { println!("{} → ❌", name); return Err(e); }
            }
        }
        ("download_all", Some(_)) => {
            if scans.len() == 0 {
                println!("nothing to download");
            }
            for name in scans.iter().map(|se| &se.name) {
                match doxie.download_scan_by_name(name, None) {
                    Ok(filename) => { println!("{} → {}", name, filename); }
                    Err(e) => { println!("{} → ❌", name); return Err(e); }
                }
            }
            Ok(())
        }
        _ => { panic!("illegal command") }
    }
}