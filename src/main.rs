use std::error::Error;
use std::fs;
use std::fs::File;

extern crate structopt;
extern crate handlebars;
extern crate serde_yaml;

use structopt::StructOpt;
use std::path::PathBuf;
use serde_yaml::Value;
use handlebars::Handlebars;

#[derive(Debug, StructOpt)]
#[structopt(name = "hbs-cli",
            about = "Simple handlebars CLI",
            rename_all = "kebab-case")]
/// Simple CLI for generating text from handlebars templates, feed with data
/// from file (YAML parser used is for it).
struct Opt {
    #[structopt(parse(from_os_str), name="PROPS_FILE")]
    /// Properties file can be ether YAML or JSON (as JSON is YAMLs subset)
    propsfile: PathBuf,

    #[structopt(parse(from_os_str), name="TEMPLATE_FILE")]
    /// Template Handlebars
    template: PathBuf,

    #[structopt(short,long)]
    /// Not implemented parameter for output file name if not provided output
    /// redirected to stdout.
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    eprintln!("{:?}", opt);
    let reg = Handlebars::new();
    let propsfile = File::open(opt.propsfile)?;
    let data: Value = serde_yaml::from_reader(propsfile)?;
    let template = fs::read_to_string(opt.template)?;

    let text = reg.render_template(&template, &data)?;
    match opt.output {
        Some(path) => fs::write(path, text)?,
        None => println!("{}", text),
    };

    Ok(())
}
