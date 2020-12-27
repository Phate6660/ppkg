mod commands;
use commands::{install, list};
use clap::{Arg, SubCommand};
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    discord: Package,
    firefox: PackageMultiArch,
    ghcli: PackageMultiArch,
    palemoon: Package,
}

#[derive(Deserialize)]
struct Package {
    package_name: String,
    package_description: String,
    package_version: String,
    package_url: String,
}

#[derive(Deserialize)]
struct PackageMultiArch {
    package_name: String,
    package_description: String,
    package_version: String,
    package_url_32: String,
    package_url_64: String,
}

fn main() {
    let home = std::env::var("HOME").unwrap();
    let home: &str = &home[..];
    let config_path = format!("{}/.ppkg/config.toml", home);
    let config_path: &str = &config_path[..];
    let matches = clap::App::new("ppkg")
        .version("0.1.0")
        .author("Phate6660 <https://Phate6660.codeberg.page>")
        .about("\nPhate's Package Manager, a binary package manager written in Rust.")
        .arg(Arg::with_name("install")
            .short("i")
            .long("install")
            .help("Install a package.")
            .value_name("PKG")
            .takes_value(true))
        .subcommand(SubCommand::with_name("list")
            .about("List packages.")
            .arg(Arg::with_name("available")
                    .short("a")
                    .long("available")
                    .help("List packages available in the repo."))
            .arg(Arg::with_name("installed")
                    .short("i")
                    .long("installed")
                    .help("List Packages currently installed.")))
        .get_matches();
    if matches.is_present("install") {
        install(config_path, home, &matches);
    }
    if let Some(matches) = matches.subcommand_matches("list") {
        list(config_path, home, &matches);
    }
}
