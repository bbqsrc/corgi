use std::path::{Path, PathBuf};

use gumdrop::Options;

use crate::models::Descriptor;

#[derive(Debug, Options)]
enum Command {
    Build(BuildArgs),
    Info(InfoArgs),
}

#[derive(Debug, Options)]
struct BuildArgs {
    help: bool,
    scheme: String,
}

#[derive(Debug, Options)]
struct InfoArgs {
    help: bool,
}

#[derive(Debug, Options)]
struct Args {
    help: bool,
    descriptor: Option<PathBuf>,
    #[options(command)]
    command: Option<Command>,
}

fn load_cargo_metadata(
    corgi_path: Option<&Path>,
) -> Result<cargo_metadata::Metadata, cargo_metadata::Error> {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    let path = corgi_path
        .and_then(|x| x.parent())
        .unwrap_or_else(|| Path::new("."))
        .join("Cargo.toml");
    cmd.manifest_path(path);
    cmd.exec()
}

fn load_descriptor(path: Option<&Path>) -> Result<Descriptor, std::io::Error> {
    let f = std::fs::read_to_string(path.unwrap_or_else(|| Path::new("./Corgi.toml")))?;
    let model = toml::from_str(&f)?;
    Ok(model)
}

pub fn run() -> anyhow::Result<()> {
    let args = Args::parse_args_default_or_exit();

    if args.command.is_none() {
        println!("{}", Args::usage());
        std::process::exit(1);
    }

    match args.command.unwrap() {
        Command::Build(BuildArgs { scheme, .. }) => {
            let descriptor = load_descriptor(args.descriptor.as_ref().map(|x| &**x))?;
            let cargo_meta = load_cargo_metadata(args.descriptor.as_ref().map(|x| &**x))?;
            crate::action::build::build_scheme(&scheme, &descriptor, &cargo_meta);
        }
        Command::Info(_) => {
            let descriptor = load_descriptor(args.descriptor.as_ref().map(|x| &**x))?;
            let cargo_meta = load_cargo_metadata(args.descriptor.as_ref().map(|x| &**x))?;
            // println!("{:#?}", descriptor);
            crate::action::info::print_info(&descriptor, &cargo_meta);
        }
    }

    // println!("{:#?}", cargo_meta);
    Ok(())
}
