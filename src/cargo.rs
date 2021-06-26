use cargo_metadata::Package;

use crate::models::descriptor::build::Rule;

fn feature_args<S: AsRef<str>>(pkg: &Package, features: &[S]) -> (Vec<String>, String) {
    let rustflags = std::env::var("RUSTFLAGS").ok();

    let mut args = vec![];
    let mut flags = vec![];

    for feature in features {
        let f = feature.as_ref();
        if pkg.features.contains_key(f) {
            args.push(f);
        } else {
            flags.push(format!("--cfg feature=\"{}\"", f));
        }
    }
    let flags = flags.join(" ");
    let args = vec!["--features".to_string(), args.join(",")];

    let flags = rustflags
        .map(|x| format!("{} {}", x, flags))
        .unwrap_or(flags);

    (args, flags)
}

pub fn build(
    metadata: &cargo_metadata::Metadata,
    schema_name: &str,
    rule: &Rule,
    cargo_args: Vec<String>,
) {
    let mut name_chunks = rule.name.split("/").collect::<Vec<_>>();
    if name_chunks.len() != 2 {
        eprintln!("ERROR: No.");
        return;
    }

    let bin_name = name_chunks.pop().unwrap();
    let subcrate = name_chunks.pop().unwrap();

    let id = metadata
        .workspace_members
        .iter()
        .find(|x| x.to_string().split(" ").next().unwrap() == subcrate)
        .unwrap();
    let pkg = metadata.packages.iter().find(|x| &x.id == id).unwrap();
    let target_dir = metadata.target_directory.join("corgi").join(schema_name);
    std::fs::create_dir_all(&target_dir).unwrap();

    let (args, flags) = feature_args(pkg, &rule.features);
    let mut handle = std::process::Command::new("cargo")
        .current_dir(&pkg.manifest_path.parent().unwrap())
        .env("RUSTFLAGS", flags)
        .arg("build")
        .arg("--bin")
        .arg(bin_name)
        .args(args)
        .args(cargo_args)
        .spawn()
        .unwrap();

    handle.wait().unwrap();

    std::fs::remove_file(target_dir.join(bin_name)).unwrap_or(());
    std::fs::hard_link(
        metadata.target_directory.join("debug").join(bin_name),
        target_dir.join(bin_name),
    )
    .unwrap();
}
