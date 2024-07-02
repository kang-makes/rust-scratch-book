use std::fmt;
use std::path::Display;

use regex::Regex;
use serde::de::{self, Deserializer, Visitor};
use serde::{ser, Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
pub struct Chart {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub name: String,
    pub version: Version,
    #[serde(rename = "kubeVersion")]
    #[serde(default)]
    pub kube_version: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub home: String,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub maintainers: Vec<Maintainer>,
    #[serde(default)]
    pub icon: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String, // The version of the app that this contains (optional). Needn't be SemVer.
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub annotations: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub bugfix: i32,
    pub slug: String,
}

impl Version {
    fn assemble_version(self) -> &str {
        let without_slug = format!("{}.{}.{}", v.major, v.minor, v.bugfix).as_str();
        let with_slug = format!("{}.{}.{}-{}", v.major, v.minor, v.bugfix, v.slug).as_str();
        if v.slug == "" {
            without_slug
        } else {
            with_slug
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Dependency {
    pub name: String,
    pub version: Version,
    #[serde(default)]
    pub repository: String,
    #[serde(default)]
    pub condition: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(rename = "import-values")]
    #[serde(default)]
    pub import_values: Vec<String>,
    #[serde(default)]
    pub alias: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Maintainer {
    pub name: String,
    #[serde(default)]
    pub email: String,
    pub url: String,
}

impl<'de> serde::de::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Version, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VersionVisitor;

        impl<'de> Visitor<'de> for VersionVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`secs` or `nanos`")
            }

            fn visit_str<E>(self, value: &str) -> Result<Version, E>
            where
                E: de::Error,
            {
                let re =
                    Regex::new(r"^(?P<major>\d*)\.(?P<minor>\d*)\.(?P<bugfix>\d*)-(?P<slug>.*)")
                        .map_err(|e| de::Error::custom(format!("error compiling regex: {e}")))?;

                match re.captures(value) {
                    Some(capture) => Ok(Version {
                        major: capture["major"].parse::<i32>().unwrap(),
                        minor: capture["minor"].parse::<i32>().unwrap(),
                        bugfix: capture["bugfix"].parse::<i32>().unwrap(),
                        slug: capture["slug"].to_string(),
                    }),
                    None => Err(de::Error::custom(format!(
                        "version {value} does not match semver regex"
                    ))),
                }
            }
        }

        deserializer.deserialize_identifier(VersionVisitor)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.assemble_version())
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.assemble_version())
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Repository {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub generated: String,
    pub entries: Vec<RepositoryEntry>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct RepositoryEntry {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub created: String,
    pub description: String,
    #[serde(default)]
    pub digest: String,
    pub version: Version,
    #[serde(rename = "kubeVersion")]
    #[serde(default)]
    pub kube_version: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub home: String,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub maintainers: Vec<Maintainer>,
    #[serde(default)]
    pub icon: String,
    #[serde(rename = "appVersion")]
    #[serde(default)]
    pub app_version: String, // The version of the app that this contains (optional). Needn't be SemVer.
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub annotations: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::{Chart, Repository, Version};

    // #[test]
    // fn test_deserialize_chart_yaml() {
    //     let yaml: Chart = serde_yaml::from_str(
    //         r#"
    //     apiVersion: v2
    //     name: test-chart-1
    //     description: A Helm chart for Kubernetes
    //     type: application
    //     version: 0.1.0
    //     appVersion: "1.16.0"
    //     "#,
    //     )
    //     .unwrap();
    //
    //     let expected_output = Chart {
    //         api_version: String::from("v2"),
    //         name: String::from("test-chart-1"),
    //         description: String::from("A Helm chart for Kubernetes"),
    //         type_: String::from("application"),
    //         version: Version {
    //             major: 0,
    //             minor: 1,
    //             bugfix: 0,
    //         },
    //         app_version: String::from("1.16.0"),
    //         ..Default::default()
    //     };
    //
    //     assert_eq!(yaml, expected_output);
    // }

    #[test]
    fn test_deserialize_chart_yaml_with_slug() {
        let yaml: Chart = serde_yaml::from_str(
            r#"
        apiVersion: v2
        name: test-chart-1
        description: A Helm chart for Kubernetes
        type: application
        version: 0.1.0-slug
        appVersion: "1.16.0"
        "#,
        )
        .unwrap();

        let expected_output = Chart {
            api_version: String::from("v2"),
            name: String::from("test-chart-1"),
            description: String::from("A Helm chart for Kubernetes"),
            type_: String::from("application"),
            version: Version {
                major: 0,
                minor: 1,
                bugfix: 0,
                slug: "slug".to_string(),
            },
            app_version: String::from("1.16.0"),
            ..Default::default()
        };

        assert_eq!(yaml, expected_output);
    }

    #[test]
    fn test_deserializa_repo_index() {
        let yaml: Repository = serde_yaml::from_str(
            r#"
        apiVersion: v1
        entries:
            common-library:
                -   apiVersion: v2
                    created: "2023-03-06T16:54:27.78965245Z"
                    description: Provides helpers to provide consistency on all the charts
                    digest: 1df82a701109e29771912be9964eec8e954d49b2afa319d3ef01f5dff5164ecc
                    keywords:
                        - newrelic
                        - chart-library
                    name: common-library
                    type: library
                    urls:
                        - https://github.com/newrelic/helm-charts/releases/download/common-library-1.1.1/common-library-1.1.1.tgz
                    version: 1.1.1
        generated: "2023-12-20T14:26:26.392635056Z"
        "#,
        ).unwrap();

        let expected_output = Repository {
            api_version: String::from("v2"),
            name: String::from("test-chart-1"),
            description: String::from("A Helm chart for Kubernetes"),
            type_: String::from("application"),
            version: Version {
                major: 0,
                minor: 1,
                bugfix: 0,
                slug: "slug".to_string(),
            },
            app_version: String::from("1.16.0"),
            ..Default::default()
        };

        assert_eq!(yaml, expected_output);
    }
}
