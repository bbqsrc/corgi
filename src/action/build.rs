use indexmap::IndexMap;

use crate::models::{
    descriptor::{build::Rule, Build},
    Descriptor,
};

fn recurse_schemes<'a>(
    schemes: IndexMap<&'a str, &'a Build>,
    descriptor: &'a Descriptor,
    scheme: &'a Build,
) -> IndexMap<&'a str, &'a Build> {
    let mut schemes = schemes;
    for id in &scheme.schemes {
        if schemes.contains_key(&**id) {
            continue;
        }
        let sub = &descriptor.build[id];
        schemes.insert(id, sub);
        schemes = recurse_schemes(schemes, descriptor, sub);
    }
    schemes
}

pub fn build_scheme(name: &str, descriptor: &Descriptor, metadata: &cargo_metadata::Metadata) {
    let name = if name == "" { "all" } else { name };

    if name != "all" && !descriptor.build.contains_key(name) {
        eprintln!("No scheme found with name `{}`.", name);
        eprintln!(
            "Schemes: [{}]",
            descriptor
                .build
                .keys()
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        );
        std::process::exit(1);
    }

    println!("Building scheme `{}`...", name);

    let scheme = &descriptor.build[name];
    let mut subschemes = recurse_schemes(Default::default(), descriptor, scheme);
    subschemes.insert(name, scheme);

    for (scheme_id, scheme) in subschemes {
        let bins = scheme
            .bins
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .cloned()
            .map(|x| {
                let mut rule = Rule::from(x);
                rule.features.append(
                    &mut scheme
                        .features
                        .iter()
                        .filter(|x| !rule.features.contains(x))
                        .cloned()
                        .collect::<Vec<_>>(),
                );

                rule
            })
            .collect::<Vec<_>>();

        println!("{:?}", bins);

        // TODO: cargo args should include --release etc
        let cargo_args = vec![];

        for bin in bins {
            println!("{}: {:?}", scheme_id, bin);
            println!("Building `{}`", &bin.name);
            let scheme_id = if scheme_id != name {
                format!("{}/{}", name, scheme_id)
            } else {
                scheme_id.to_string()
            };
            crate::cargo::build(metadata, &scheme_id, &bin, cargo_args.clone());
        }
    }
}
