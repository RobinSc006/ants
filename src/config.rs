use std::collections::HashMap;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct ConfigParameter {
    pub text: String,
    pub vals: Vec<f64>,
}
#[derive(Clone)]
pub struct Config {
    pub content: String,
    pub parameters: HashMap<String, ConfigParameter>,
}

impl Config {
    pub fn load(filename: &str) -> Self {
        let mut content: String = String::new();
        let config_file = fs::read_to_string(Config::get_conf_path().join(filename));

        match config_file {
            Ok(_) => {
                content = config_file.unwrap();
            }
            Err(_) => {
                log::error!("config file not found");
            }
        }

        let mut conf = Config {
            content: content,
            parameters: HashMap::new(),
        };

        conf.index();

        return conf;
    }

    fn index(&mut self) {
        for line in self.content.lines() {
            if line != "" {
                let param_body: Vec<&str> = line.split(" ").collect();

                let param_name = param_body[0];
                let param_conf = param_body[1];

                let mut param_values: Vec<f64> = Vec::new();

                for param_val_txt in param_conf
                    .replace(" ", "")
                    .split(",")
                    .collect::<Vec<&str>>()
                {
                    let mut param = 0.0;
                    let parsed_param = param_val_txt.parse::<f64>();

                    match parsed_param {
                        Ok(_) => {
                            param = parsed_param.unwrap();
                        }
                        Err(_) => {
                            log::error!("failed to parse '{}' as config parameter", &line);
                        }
                    }

                    param_values.push(param);
                }

                self.parameters.insert(
                    param_name.to_string(),
                    ConfigParameter {
                        text: param_conf.to_string(),
                        vals: param_values,
                    },
                );
            }
        }
    }

    pub fn get_parameter(&self, name: &str) -> ConfigParameter {
        if self.parameters.contains_key(name) {
            return self.parameters[name].clone();
        } else {
            log::error!("parameter '{}' not found in config file", name);
            std::process::exit(-1);
        }
    }

    pub fn get_conf_path() -> PathBuf {
        return find_folder::Search::KidsThenParents(5, 5)
            .for_folder("data")
            .expect("data folder not found");
    }
}
