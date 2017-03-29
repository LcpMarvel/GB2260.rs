extern crate phf_codegen;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::{BufReader, Read};
use std::io::prelude::*;
use std::path::Path;

use std::ascii::AsciiExt;

use serde_json::Value;

fn main() {
    let revisions = revisions();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("data.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write_data(&revisions, &mut file);

    write_revisions(&revisions, &mut file);
}

fn revisions() -> Value {
    let path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("data").join("revisions.json");

    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).unwrap();

    serde_json::from_str(&buffer).unwrap()
}

fn write_revisions<W: Write>(revisions: &Value, mut file: &mut BufWriter<W>) {
    if let Value::Array(ref gb_revisions) = revisions["gb"] {
        if gb_revisions.len() > 0 {
            write!(&mut file, "static GB_DATA: phf::Map<&'static str, &'static phf::Map<&'static str, &'static str>> = ");

            let mut builder = phf_codegen::Map::new();

            for revision in gb_revisions.into_iter() {
                let revision_value = revision_value(revision);
                let definition_map_name = definition_map_name("gb", &revision_value);

                builder.entry(revision_value.to_string(), format!("&{}", definition_map_name).as_str());
            }
            builder.build(&mut file).unwrap();
            write!(&mut file, ";\n");
        }
    }

    if let Value::Array(ref stats_revisions) = revisions["stats"] {
        if stats_revisions.len() > 0 {
            write!(&mut file, "static STATS_DATA: phf::Map<&'static str, &'static phf::Map<&'static str, &'static str>> = ");

            let mut builder = phf_codegen::Map::new();

            for revision in stats_revisions.into_iter() {
                let revision_value = revision_value(revision);
                let definition_map_name = definition_map_name("stats", &revision_value);

                builder.entry(revision_value.to_string(), format!("&{}", definition_map_name).as_str());
            }
            builder.build(&mut file).unwrap();
            write!(&mut file, ";\n");
        }
    }
}

fn write_data<W: Write>(revisions: &Value, mut file: &mut BufWriter<W>) {
    if let Value::Array(ref gb_revisions) = revisions["gb"] {
        build_data(gb_revisions, &mut file, "gb");
    }

    if let Value::Array(ref stats_revisions) = revisions["stats"] {
        build_data(stats_revisions, &mut file, "stats");
    }
}

fn revision_value(revision: &Value) -> String {
    str::replace(revision.as_str().unwrap(), "\"", "")
}

fn file_name(revision: &str) -> String {
    format!("{}.tsv", revision)
}

fn definition_map_name(source: &str, revision_value: &str) -> String {
    format!("REVISION_{}_{}", source.to_ascii_uppercase(), revision_value)
}

fn data_dir(namespace: &str) -> &str {
    let dir: &str;
    if namespace == "gb" {
        dir = "";
    } else {
        dir = namespace;
    }

    dir
}

fn build_data<W: Write>(revisions: &[Value], mut file: &mut BufWriter<W>, namespace: &str) {
    for revision in revisions.into_iter() {
        let revision_value = revision_value(revision);
        let definition_map_name = definition_map_name(namespace, &revision_value);

        let path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("data")
                                                                      .join(data_dir(namespace))
                                                                      .join(file_name(&revision_value));

        let revision_file = File::open(&path).unwrap();
        let reader = BufReader::new(revision_file);

        write!(&mut file, "static {}: phf::Map<&'static str, &'static str> = ", definition_map_name).unwrap();
        let mut builder = phf_codegen::Map::new();

        for line in reader.lines() {
            let string_line = line.unwrap();
            let str_line = string_line.as_str();

            if !str_line.starts_with("Source") {
                let vec: Vec<&str> = str_line.split("\t").collect();

                let key = vec[2].to_string();
                let value = format!("\"{}\"", vec[3]);

                builder.entry(key, value.as_str());
            }
        }

        builder.build(&mut file).unwrap();
        write!(&mut file, ";\n").unwrap();
    }
}
