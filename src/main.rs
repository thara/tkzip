use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::{BufReader, BufWriter};

use std::io::prelude::*;

extern crate zip;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0];
    let prefix = &args[1];
    let dest = &args[2];

    fs::create_dir_all(dest);

    let mut f = File::open(path).ok().expect("File open failed");

    let mut zip = zip::ZipArchive::new(f).ok().expect(
        "zip archive loading failed",
    );

    let dest_dir = Path::new(dest);

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).expect("zip file index out bound");
        let dest_path = {
            let path = file.name();
            if file.name().starts_with(prefix) {
                Some(dest_dir.join(path))
            } else {
                None
            }
        };

        if let Some(path) = dest_path {
            if path.extension().is_some() {
                println!("path: {:?}", path);

                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir);
                }

                if let Ok(f) = fs::File::create(path) {
                    let mut w = BufWriter::new(f);
                    for b in file.bytes() {
                        if let Ok(byte) = b {
                            w.write(&[byte]);
                        }
                    }
                    w.flush();
                }
            }
        }
    }
}
