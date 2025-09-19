use clap::CommandFactory;
use std::env;
use std::fs;
use std::path::Path;

#[path = "src/args.rs"]
mod args;
use args::Args;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/args.rs");

    let out_dir =
        env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let man_dir = Path::new(&out_dir).join("man");
    fs::create_dir_all(&man_dir)?;

    let cmd = Args::command();

    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let man_path = man_dir.join(format!("{}.1", cmd.get_name()));
    fs::write(&man_path, buffer)?;

    Ok(())
}
