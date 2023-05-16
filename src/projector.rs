use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

struct Projector {
    pub config: Config,
    pub data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::new(),
    };
}

impl Projector {
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut paths = vec![];
        let mut out = HashMap::new();

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }

        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter())
            }
        }
        return out;
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path());
        let mut out = None;
        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }
            curr = p.parent();
        }
        return out;
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data
            .projector
            .entry(self.config.pwd.clone())
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.projector.get_mut(&self.config.pwd).map(|x| {
            x.remove(key);
        });
    }

    pub fn from_config(config: Config) -> Self {
        if std::fs::metadata(&config.config).is_ok() {
            let contents = std::fs::read_to_string(&config.config)
                .unwrap_or(String::from("{\"projector\": {}}"));
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_data());

            return Projector { config, data };
        }
        return Projector {
            config,
            data: default_data(),
        };
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use collection_macros::hashmap;

    use crate::config::{Config, Operation};

    use super::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        return hashmap! {

         PathBuf::from("/") => hashmap! {
            "baba".into() => "baz1".into(),
            "femto".into() => "is_supreme_soy".into(),
          },
          PathBuf::from("/baba") => hashmap! {
            "baba".into() => "baz2".into(),
          },
          PathBuf::from("/baba/baz") => hashmap! {
            "baba".into() => "baz3".into(),
          },
        };
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            config: Config {
                config: PathBuf::from(""),
                pwd: pwd,
                operation: Operation::Print(None),
            },
            data: Data {
                projector: get_data(),
            },
        };
    }
    #[test]
    fn get_value() {
        let proj = get_projector(PathBuf::from("/baba/baz"));
        assert_eq!(proj.get_value("baba"), Some(&String::from("baz3")));
        assert_eq!(
            proj.get_value("femto"),
            Some(&String::from("is_supreme_soy"))
        );
    }
    #[test]
    fn set_value() {
        let mut proj = get_projector(PathBuf::from("/baba/baz"));
        proj.set_value("baba".into(), "baz4".into());
        proj.set_value("femto".into(), "is_not_soy".into());

        assert_eq!(proj.get_value("baba"), Some(&String::from("baz4")));
        assert_eq!(proj.get_value("femto"), Some(&String::from("is_not_soy")));
    }

    #[test]
    fn remove_value() {
        let mut proj = get_projector(PathBuf::from("/baba/baz"));
        proj.remove_value("baba");
        proj.remove_value("femto");

        assert_eq!(proj.get_value("baba"), Some(&String::from("baz2")));
        assert_eq!(
            proj.get_value("femto"),
            Some(&String::from("is_supreme_soy"))
        );
    }
}
