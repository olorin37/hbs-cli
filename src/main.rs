use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;

extern crate structopt;
extern crate handlebars;
extern crate serde_yaml;

use structopt::StructOpt;
use std::path::PathBuf;
use serde_yaml::Value;
use handlebars::Handlebars;
use glob::glob;

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

    #[structopt(short,long)]
    /// Register all templates matching to the glob provided by the option.
    /// Those templates can be used as partials.
    register_glob: Option<String>,

}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    eprintln!("{:?}", opt);
    let mut reg = Handlebars::new();

    match opt.register_glob {
        Some(pattern) => register_templates_from_pattern(&mut reg, pattern).unwrap(),
        None => eprintln!("No glob provided for templates registration."),
    };

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

fn register_templates_from_pattern(
    hbs: &mut Handlebars,
    pattern: String
) -> Result<(), Box<dyn Error>> {
    eprintln!( "Registring templates matching to {:?}", pattern );
    for entry in glob( &pattern ).expect( "Failed to read glob pattern" ) {
        match entry {
            Ok( path ) => {
                let the_file = Path::new( &path );
                if the_file.is_file() {
                  hbs.register_template_file(
                      the_file.file_stem()
                              .and_then(|x| x.to_str())
                              .unwrap(),
                      &path
                  )?;
                }
            }
            Err(e) => eprintln!("{:?}", e)
        }
    }
    Ok(())
}
