use std::{fs::File, io::Write};

use clap::Parser;
use site::Site;
use tesseract::html::HTMLElement;

mod site;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    output_path: String,

    #[arg(short, long, default_value_t=false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let site = Site::default();

    let mut file = get_output_file(&args.output_path);

    if args.verbose {
        println!("{}", site.render());
    }
    file.write_all(site.render().as_bytes()).unwrap();
}

fn get_output_file(path: &str) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(_) => create_file(path)
    }
}

fn create_file(path: &str) -> File {
    match File::create(path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    }
}

