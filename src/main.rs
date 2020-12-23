use clap::{Arg, SubCommand};
use compress_tools::*;
use serde_derive::Deserialize;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
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
        let config: Config = toml::from_str(&std::fs::read_to_string(config_path).unwrap()).unwrap();
        let pkg = matches.value_of("install");

        let name = if pkg == serde::export::Some("Discord") {
            config.discord.package_name
        } else if pkg == serde::export::Some("Firefox") {
            config.firefox.package_name
        } else if pkg == serde::export::Some("Github CLI") {
            config.ghcli.package_name
        } else if pkg == serde::export::Some("Pale Moon") {
            config.palemoon.package_name
        } else {
            "Invalid Package!".to_string()
        };

        let url = if name == "Discord" {
            config.discord.package_url
        } else if name == "Firefox" {
            println!("This package supports 32-bit and 64-bit, please choose a version. (x32 or x64)");
            let mut arch = String::new();
            std::io::stdin()
                .read_line(&mut arch)
                .expect("Failed to read user input.");
            if arch.trim() == "x32" {
                config.firefox.package_url_32
            } else if arch.trim() == "x64" {
                config.firefox.package_url_64
            } else {
                panic!("You need to choose x32 or x64!");
            }
        } else if name == "Github CLI" {
            println!("This package supports 32-bit and 64-bit, please choose a version. (x32 or x64)");
            let mut arch = String::new();
            std::io::stdin()
                .read_line(&mut arch)
                .expect("Failed to read user input.");
            if arch.trim() == "x32" {
                config.ghcli.package_url_32
            } else if arch.trim() == "x64" {
                config.ghcli.package_url_64
            } else {
                panic!("You need to choose x32 or x64!");
            }
        } else if name == "Pale Moon" {
            config.palemoon.package_url
        } else {
            "Invalid Package!".to_string()
        };

        if url == "Invalid Package!" {
            panic!("Invalid package!");
        } else {
            //let path = format!("{}/.ppkg/downloads/", home); // Main path will be $HOME/.ppkg, also create a downloads directory
            //fs::create_dir(path).unwrap(); // TODO: Make this work somehow, for now users must make $HOME/.ppkg/downloads manually

            // Download the file
            let mut response = reqwest::blocking::get(&url).expect("Failed to download the file");

            // Determine the file name
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("tmp.bin");

            println!("File to download: '{}'", fname);

            // Create the download path
            let path = format!("{}/.ppkg/downloads/", home);
            let path: &str = &path[..];

            // Set the file name to the full path
            let fname = [path, fname].concat();
            let fname: &str = &fname[..];
            println!("Will be located at: '{:?}'", fname);

            let mut dest = File::create(fname).expect("Failed to create file"); // Create the file
            copy(&mut response, &mut dest).unwrap(); // Stick the contents in the file

            // Extract the archive
            let extraction_path = format!("{}/.ppkg/opt/{}", home, name);
            println!("Extracting the tarball to {}.", extraction_path);
            let mut source = File::open(fname).expect("Could not open archive");
            let dest = Path::new(&extraction_path);
            uncompress_archive(&mut source, &dest, Ownership::Ignore).expect("Could not extract archive");
            println!("Finished extracting!")
        }
    }
    if let Some(matches) = matches.subcommand_matches("list") {
        if matches.is_present("available") {
            let config: Config = toml::from_str(&std::fs::read_to_string(config_path).unwrap()).unwrap();
            println!(
                "Name: {}\nDescription: {}\nVersion: {}\nURL: {}\n",
                config.discord.package_name,
                config.discord.package_description,
                config.discord.package_version,
                config.discord.package_url,
            );
            println!(
                "Name: {}\nDescription: {}\nVersion: {}\nURL (x32): {}\nURL (x64): {}\n",
                config.firefox.package_name,
                config.firefox.package_description,
                config.firefox.package_version,
                config.firefox.package_url_32,
                config.firefox.package_url_64,
            );
            println!(
                "Name: {}\nDescription: {}\nVersion: {}\nURL (x32): {}\nURL (x64): {}\n",
                config.ghcli.package_name,
                config.ghcli.package_description,
                config.ghcli.package_version,
                config.ghcli.package_url_32,
                config.ghcli.package_url_64,
            );
            println!(
                "Name: {}\nDescription: {}\nVersion: {}\nURL: {}",
                config.palemoon.package_name,
                config.palemoon.package_description,
                config.palemoon.package_version,
                config.palemoon.package_url,
            );
        } else if matches.is_present("installed") {
            let config: Config = toml::from_str(&std::fs::read_to_string(config_path).unwrap()).unwrap();
            let install_path_discord = format!("{}/.ppkg/opt/{}", home, config.discord.package_name);
            let install_path_firefox = format!("{}/.ppkg/opt/{}", home, config.firefox.package_name);
            let install_path_ghcli = format!("{}/.ppkg/opt/{}", home, config.ghcli.package_name);
            let install_path_palemoon = format!("{}/.ppkg/opt/{}", home, config.palemoon.package_name);
            println!("Packages installed:");
            if fs::metadata(install_path_discord).is_ok() {
                println!("- Discord");
            }
            if fs::metadata(install_path_firefox).is_ok() {
                println!("- Firefox");
            }
            if fs::metadata(install_path_ghcli).is_ok() {
                println!("- Github CLI");
            }
            if fs::metadata(install_path_palemoon).is_ok() {
                println!("- Pale Moon");
            }
        } else {
            println!("Invalid option!");
        }
    }
}
