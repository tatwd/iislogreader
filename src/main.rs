use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use regex::Regex;

pub mod w3c_format;

//D:\works\tmp\W3SVC1-20201203

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //println!("args:{:?}", args);
    if args.len() == 1 {
        println!("input a file or dir!");
        return Ok(());
    }

    // read map.conf
    let map_file = File::open("./map.conf")?;
    let buf = BufReader::new(map_file);
    let mut map = HashMap::new();
    for l in buf.lines() {
        let li = l.unwrap();
        let parts: Vec<&str> = li.trim().split_whitespace().collect();
        map.insert(parts[0].to_string(), parts[1].to_string());
    }

    let mut found_fields = false;
    let path = args.get(1).expect("input a valid value!");
    let paths = fs::read_dir(path).unwrap(); // path D:\works\tmp\W3SVC1-20201203

    for path in paths {
        // println!("{}", path.unwrap().path().display())
        // let log_file = path.unwrap().path().display();
        let f = File::open(path.unwrap().path())?;
        let buf_reader = BufReader::new(f);

        for line in buf_reader.lines() {
            let l = line.unwrap();
            if l.starts_with("#Fields:") {
                if found_fields {
                    // write buff to a new file
                    continue;
                }
                let fields = w3c_format::get_fields(&l);
                println!("{}", fields.join(","));
                found_fields = true;
                continue;
            }
            if !l.starts_with("#") {
                let mut values = w3c_format::get_values(&l);
                for v in values.iter_mut() {
                    for kv in &map {
                        let regex = Regex::new(kv.0).unwrap();
                        if regex.is_match(*v) {
                            *v = kv.1;
                            break;
                        }
                    }
                }
                println!("{}", values.join(","));
            }
        }
    }

    Ok(())
}

// fn writelogs(line: &String) -> std::io::Result<()> {
//     let mut w = BufWriter::new(std::io::stdout());
//     w.write(line.as_bytes()).unwrap();
//     w.flush().unwrap();
//     Ok(())
// }
