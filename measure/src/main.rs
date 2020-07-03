use std::cmp::max;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use toml::Value;

const CARGO_TOML: &[u8] = include_bytes!("../../Cargo.toml");

fn main() {
    // Find where the binaries were written to during compile
    let bindir = Path::new(env!("OUT_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // Get a list of all the members in the workspace
    let unpack = |member| match member {
        Value::String(member) => member,
        _ => panic!("Invalid member!"),
    };
    let members: Vec<String> = match toml::from_slice(CARGO_TOML).unwrap() {
        Value::Table(mut config) => match config.remove("workspace").unwrap() {
            Value::Table(mut workspace) => match workspace.remove("members").unwrap() {
                Value::Array(mut members) => members.drain(..).map(unpack).collect(),

                _ => panic!("Invalid members!"),
            },

            _ => panic!("Invalid workspace!"),
        },

        _ => panic!("Invalid config!"),
    };

    // Convert all members (except this one) to their binary paths
    let binaries: Vec<PathBuf> = members
        .iter()
        .filter(|x| x.as_str() != "measure")
        .map(|x| bindir.join(x))
        .collect();

    // Strip all the binaries
    std::process::Command::new("strip")
        .args(&binaries)
        .output()
        .expect("failed to strip binaries");

    // Get the sizes of all the binaries
    let sizes: Vec<(&OsStr, u64)> = binaries
        .iter()
        .map(|x| (x, x.metadata().unwrap().len()))
        .map(|(p, s)| (p.as_path().file_name().unwrap(), s))
        .collect();

    // Get the size of the largest binary
    let largest = sizes.iter().fold(0, |i, (_, s)| max(i, *s));

    // Make a template for the progress bar output
    let nwidth = binaries
        .iter()
        .map(|x| x.file_name().unwrap().len())
        .fold(0, max);
    let template = format!("{{msg:>{}}} {{wide_bar}} {{bytes:<8}}", nwidth);

    // Put all of our binaries in progress bars
    let mp = MultiProgress::new();
    for (member, size) in sizes {
        let pb = ProgressBar::new(largest);
        pb.set_style(ProgressStyle::default_bar().template(&template));
        pb.set_position(size);
        pb.set_message(member.to_str().unwrap());
        pb.finish_at_current_pos();
        mp.add(pb);
    }
}
