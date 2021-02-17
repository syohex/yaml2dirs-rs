use std::env;
use std::fs;
use std::path::PathBuf;

use serde_yaml::Value;

fn make_directories(tree: &serde_yaml::Value, path: &mut PathBuf) -> Result<(), String> {
    match tree {
        Value::Mapping(m) => {
            for (k, v) in m {
                let base = match k {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => {
                        if n.is_i64() {
                            n.as_i64().unwrap().to_string()
                        } else if n.is_u64() {
                            n.as_u64().unwrap().to_string()
                        } else if n.is_f64() {
                            n.as_f64().unwrap().to_string()
                        } else {
                            return Err("unsuppoeted number type".to_string());
                        }
                    }
                    _ => {
                        return Err("unsuppoeted key type".to_string());
                    }
                };

                match v {
                    Value::Null => {
                        if let Err(e) = fs::create_dir_all(path.join(&base).as_path()) {
                            return Err(e.to_string());
                        }
                    }
                    _ => {
                        let mut new_path = path.join(base);
                        make_directories(v, &mut new_path)?;
                    }
                }
            }
        }
        _ => {
            return Err(format!("found unsupported type: {:?}", tree));
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: yaml2dirs yaml_file");
        return Ok(());
    }

    let file = &args[1];
    let content = fs::read_to_string(file).unwrap();
    let value: serde_yaml::Value = serde_yaml::from_str(&content).unwrap();
    let mut path = PathBuf::new();
    make_directories(&value, &mut path)?;

    Ok(())
}
