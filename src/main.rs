use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use glob::glob;
use handlebars::Handlebars;
use serde_yaml::Value;

#[derive(Parser, Debug)]
#[command(
    version,
    name = "hbs-cli",
    about = "Simple handlebars CLI",
    long_about = "Simple CLI for generating text from handlebars templates, \
                 feed with data from file (YAML parser used is for it)."
)]
struct Cli {
    #[arg(value_name = "PROPS_FILE")]
    /// Properties file can be ether YAML or JSON (as JSON is YAMLs subset)
    propsfile: PathBuf,

    #[arg(value_name = "TEMPLATE_FILE")]
    /// Template Handlebars
    template: PathBuf,

    #[arg(short, long)]
    /// Not implemented parameter for output file name if not provided output
    /// redirected to stdout.
    output: Option<String>,

    #[arg(short, long)]
    /// Register all templates matching to the glob provided by the option.
    /// Those templates can be used as partials.
    register_glob: Option<String>,

    #[arg(short, long)]
    /// Make error output verbose.
    verbose: bool,
}

// StructOpt    rename_all = "kebab-case"
// structopt(parse(from_os_str), name = "PROPS_FILE")]

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Cli::parse();
    if opt.verbose {
        eprintln!("{:?}", opt);
    }
    let mut reg = Handlebars::new();

    match opt.register_glob {
        Some(pattern) => {
            if opt.verbose {
                eprintln!("Registering templates matching to {:?}", pattern);
            }
            register_templates_from_pattern(&mut reg, pattern)?;
        }
        None => {
            if opt.verbose {
                eprintln!("No glob provided for templates registration.");
            }
        }
    };

    let propsfile = File::open(opt.propsfile)?;
    let data: Value = serde_yaml::from_reader(propsfile)?;
    let template = fs::read_to_string(opt.template)?;

    let text = reg.render_template(&template, &data)?;
    match opt.output {
        Some(path) => fs::write(path, text)?,
        None => print!("{}", text),
    };

    Ok(())
}

fn register_templates_from_pattern(
    hbs: &mut Handlebars,
    pattern: String,
) -> Result<(), Box<dyn Error>> {
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let the_file = Path::new(&path);
                if the_file.is_file() {
                    hbs.register_template_file(
                        the_file.file_stem().and_then(|x| x.to_str()).unwrap(),
                        &path,
                    )?;
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
    Ok(())
}
