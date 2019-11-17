// Add these into our namespace
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt()]
    subcommand: Subcommand,
    #[structopt(short, long)]
    key: String,
    #[structopt(short, long)]
    value: String,
}

#[derive(Debug)]
enum Subcommand {
    Set,
    Get,
}

impl FromStr for Subcommand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "set" => Ok(Subcommand::Set),
            "get" => Ok(Subcommand::Get),
            _ => Err("Subcommand must be either 'get' or 'set'".to_string()),
        }
    }
}
//  kv [subcommand=set|get] [key] {value}
// https://github.com/bketelsen/kv # repo for this
fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);

    match opt.subcommand {
        Subcommand::Get => unimplemented!(),
        Subcommand::Set => set(opt.key, opt.value).unwrap(),
    };
}

fn set(key: String, value: String) -> std::io::Result<()> {
    let mut map = load_keys()?;
    map.insert(key, value);
    write_keys(map)?;
    Ok(())
}

fn write_keys(map: HashMap<String, String>) -> std::io::Result<()> {
    let jstr = serde_json::to_string(&map)?;
    std::fs::write("kv.db", jstr.as_bytes())?;

    Ok(())
}

fn load_keys() -> std::io::Result<HashMap<String, String>> {
    let mut file = match std::fs::File::open("kv.db") {
        Ok(file) => file,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => std::fs::File::create("kv.db")?,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if contents.is_empty() {
        contents.push_str("{}");
    }
    let json: Value = serde_json::from_str(&contents)?;
    match json {
        Value::Object(map) => {
            let mut db = HashMap::new();
            for (k, value) in map {
                match value {
                    Value::String(string) => db.insert(k, string),
                    _ => panic!("Bad Map: CORRUPT DATABASE!!!"),
                };
            }
            Ok(db)
        }
        _ => panic!("Not a Map: CORRUPT DATABASE!!!"),
    }
}
