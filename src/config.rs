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
        let mut conf = Config {
            content: fs::read_to_string(Config::get_conf_path().join(filename))
                .expect("something went wrong reading the file"),
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
                    param_values.push(param_val_txt.parse().unwrap());
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
        return self.parameters[name].clone();
    }

    pub fn get_conf_path() -> PathBuf {
        return find_folder::Search::KidsThenParents(5, 5)
            .for_folder("data")
            .expect("data folder not found");
    }
}
