// Add these into our namespace
use serde_json::Value;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
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

    let map = load_keys().unwrap();

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

/*
❯ RUST_BACKTRACE=1 ./target/debug/kv set --key brian --value hello
Opt {
    subcommand: Set,
    key: "brian",
    value: "hello",
}
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 9, kind: Other, message: "Bad file descriptor" }', src/libcore/result.rs:1165:5
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:473
  11: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:380
  12: rust_begin_unwind
             at src/libstd/panicking.rs:307
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  14: core::result::unwrap_failed
             at src/libcore/result.rs:1165
  15: core::result::Result<T,E>::unwrap
             at /rustc/4560ea788cb760f0a34127156c78e2552949f734/src/libcore/result.rs:933
  16: kv::main
             at src/main.rs:47
  17: std::rt::lang_start::{{closure}}
             at /rustc/4560ea788cb760f0a34127156c78e2552949f734/src/libstd/rt.rs:64
  18: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:49
  19: std::panicking::try::do_call
             at src/libstd/panicking.rs:292
  20: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  21: std::panicking::try
             at src/libstd/panicking.rs:271
  22: std::panic::catch_unwind
             at src/libstd/panic.rs:394
  23: std::rt::lang_start_internal
             at src/libstd/rt.rs:48
  24: std::rt::lang_start
             at /rustc/4560ea788cb760f0a34127156c78e2552949f734/src/libstd/rt.rs:64
  25: <kv::Subcommand as core::fmt::Debug>::fmt
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
*/
