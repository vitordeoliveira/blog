use std::fs;

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    blog: Vec<Blog>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Blog {
    paths: Vec<String>,
    key: Option<String>,
}

impl Config {
    pub fn from_file(blog_config_path: &str) -> Self {
        let content = fs::read_to_string(blog_config_path).unwrap();
        toml::from_str(&content).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
        path::{Path, PathBuf},
    };

    use super::{Blog, Config};

    pub struct TempTomlFile {
        path: PathBuf,
    }

    impl TempTomlFile {
        pub fn new(content: &str) -> Self {
            let path =
                std::env::temp_dir().join(format!("temp_file_{}.toml", uuid::Uuid::new_v4()));
            let mut file = File::create(&path).expect("Failed to create temporary file");
            file.write_all(content.as_bytes())
                .expect("Failed to write to temporary file");
            TempTomlFile { path }
        }

        pub fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempTomlFile {
        fn drop(&mut self) {
            // Delete the file when the struct is dropped
            if self.path.exists() {
                fs::remove_file(&self.path).expect("Failed to delete temporary file");
            }
        }
    }

    #[test]
    fn test_with_temp_toml_file() {
        let toml_content = r#"
            [[blog]]
            paths = ["fake/path/1"]
            key = "00000000-fake-key1-0000-000000000000"

            [[blog]]
            paths = ["fake/path/2"]
            key = "00000000-fake-key2-0000-000000000000"
        "#;

        let temp_file = TempTomlFile::new(toml_content);

        let content = fs::read_to_string(temp_file.path()).expect("Failed to read temporary file");
        assert_eq!(content, toml_content);
    }

    #[test]
    fn should_convert_toml_to_struct() {
        let toml_content = r#"
            [[blog]]
            paths = ["fake/path/1"]
            key = "00000000-fake-key1-0000-000000000000"

            [[blog]]
            paths = ["fake/path/2"]
            key = "00000000-fake-key2-0000-000000000000"
        "#;

        let temp_file = TempTomlFile::new(toml_content);

        let blog = Config::from_file(&temp_file.path().display().to_string());

        assert_eq!(
            blog,
            Config {
                blog: vec![
                    Blog {
                        paths: vec!["fake/path/1".to_string()],
                        key: Some("00000000-fake-key1-0000-000000000000".to_string())
                    },
                    Blog {
                        paths: vec!["fake/path/2".to_string()],
                        key: Some("00000000-fake-key2-0000-000000000000".to_string())
                    }
                ]
            }
        )
    }
}
