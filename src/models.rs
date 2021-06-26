use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Descriptor {
    pub project: descriptor::Project,
    #[serde(default)]
    pub script: IndexMap<String, descriptor::Scripts>,
    #[serde(default)]
    pub target: IndexMap<String, descriptor::Target>,
    #[serde(default)]
    pub build: IndexMap<String, descriptor::Build>,
    #[serde(default)]
    pub env: IndexMap<String, String>,
    #[serde(default)]
    pub feature_group: IndexMap<String, descriptor::FeatureGroup>,
    #[serde(default)]
    pub feature: IndexMap<String, descriptor::Feature>,
    #[serde(default)]
    #[serde(rename = "crate")]
    pub crates: IndexMap<String, descriptor::Crate>,
}

pub mod descriptor {
    use indexmap::IndexMap;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Build {
        #[serde(default)]
        pub description: Option<String>,
        #[serde(default)]
        pub features: Vec<String>,
        #[serde(default)]
        pub libs: Option<Vec<build::Container>>,
        #[serde(default)]
        pub bins: Option<Vec<build::Container>>,
        #[serde(default)]
        pub schemes: Vec<String>,
    }

    pub mod build {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Container {
            Simple(String),
            Complex(Rule),
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Rule {
            pub name: String,
            #[serde(default)]
            pub features: Vec<String>,
        }

        impl From<Container> for Rule {
            fn from(c: Container) -> Self {
                match c {
                    Container::Simple(name) => Rule {
                        name,
                        features: vec![],
                    },
                    Container::Complex(rule) => rule,
                }
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Target {
        #[serde(default)]
        env: IndexMap<String, String>,
        #[serde(default)]
        feature: IndexMap<String, Feature>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FeatureGroup {
        #[serde(default)]
        pub required: bool,
        #[serde(default)]
        pub multiple: bool,
        #[serde(default)]
        pub description: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Feature {
        #[serde(default)]
        pub group: Option<String>,
        #[serde(default)]
        pub description: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Project {
        #[serde(default)]
        pub name: Option<String>,
        pub version: Option<semver::Version>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Scripts {
        #[serde(default)]
        pub prebuild: Option<scripts::Container>,
        #[serde(default)]
        pub postbuild: Option<scripts::Container>,
        #[serde(default)]
        pub preclean: Option<scripts::Container>,
        #[serde(default)]
        pub postclean: Option<scripts::Container>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Crate {
        #[serde(default)]
        pub description: Option<String>,
        #[serde(default)]
        pub target: IndexMap<String, Target>,
        #[serde(default)]
        pub env: IndexMap<String, String>,
    }

    pub mod scripts {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Script {
            pub path: String,
            #[serde(default)]
            pub runner: Option<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Container {
            Simple(String),
            Complex(Script),
        }

        impl From<Container> for Script {
            fn from(c: Container) -> Self {
                match c {
                    Container::Simple(path) => Script { path, runner: None },
                    Container::Complex(v) => v,
                }
            }
        }
    }
}
