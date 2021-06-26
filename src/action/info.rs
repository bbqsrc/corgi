use crate::models::Descriptor;

pub fn print_info(descriptor: &Descriptor, metadata: &cargo_metadata::Metadata) {
    if let Some(name) = descriptor.project.name.as_ref() {
        print!("Project: {}", name);
        if let Some(version) = descriptor.project.version.as_ref() {
            print!(" {}", version);
        }
        println!("\n");
    }

    if !descriptor.feature.is_empty() {
        println!("  Features:");
        for (name, feature) in &descriptor.feature {
            print!("   - {}", name);
            if let Some(group) = feature.group.as_ref() {
                print!(" (group: {})", group);
            }
            if let Some(desc) = feature.description.as_ref() {
                print!(": {}", desc);
            }
            println!();
        }
        println!();
    }

    if !descriptor.feature_group.is_empty() {
        println!("  Feature groups:");
        for (name, feature_group) in &descriptor.feature_group {
            print!("   - {}", name);
            if feature_group.multiple || feature_group.required {
                print!(" (");
                if feature_group.multiple && feature_group.required {
                    print!("one or more");
                } else if feature_group.multiple {
                    print!("zero or more");
                } else if feature_group.required {
                    print!("one only");
                }
                print!(")");
            }

            if let Some(desc) = feature_group.description.as_ref() {
                print!(": {}", desc);
            }
            println!();
        }
        println!();
    }

    println!("  Build schemes:");
    if !descriptor.build.contains_key("all") {
        println!("   - all (default): Builds all binaries and libraries in the workspace");
    }

    if !descriptor.build.is_empty() {
        for (name, scheme) in &descriptor.build {
            print!("   - {}", name);

            if let Some(desc) = &scheme.description {
                print!(": {}", desc);
            }
            println!();
        }
    }
    println!();

    println!("  Crates:");

    metadata
        .packages
        .iter()
        .filter(|x| metadata.workspace_members.contains(&x.id))
        .for_each(|x| {
            let version = descriptor
                .project
                .version
                .as_ref()
                .map(|x| x.to_string())
                .unwrap_or_else(|| x.version.to_string());
            print!("    {} {}", x.name, version);

            if let Some(desc) = &x.description {
                print!(": {}", desc);
            }
            println!();

            if !x.targets.is_empty() {
                if let Some(lib) = x
                    .targets
                    .iter()
                    .find(|x| x.kind.iter().any(|x| x.ends_with("lib")))
                {
                    println!("      * Library: {}", lib.name);
                }

                let bins = x
                    .targets
                    .iter()
                    .filter(|x| x.kind.contains(&"bin".to_string()))
                    .map(|x| x.name.to_string())
                    .collect::<Vec<_>>();
                if !bins.is_empty() {
                    println!("      * Binaries: [{}]", bins.join(", "));
                }

                // for target in &x.targets {
                //     println!("        - {:?}", target);
                // }
                println!();
            }

            // if !x.features.is_empty() {
            //     println!("      Features:");
            //     for (name, feature) in &x.features {
            //         println!("        - {}: [{}]", name, feature.join(", "));
            //     }
            //     println!();
            // }

            // println!("\n      Debug: {:#?}", x.targets);
        });

    // println!("{:?}", metadata.workspace_members.iter().map(|x| x).collect::<Vec<_>>());
}
