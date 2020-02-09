use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::str::from_utf8;
use std::process::Command;


#[macro_use] extern crate handlebars;

use structopt::StructOpt;

use serde_yaml::Value;

//use handlebars as hbs;
use handlebars::Handlebars;
use handlebars::Helper;
use handlebars::RenderError;
use handlebars::Output;
use handlebars::Context;
use handlebars::RenderContext;
use handlebars::HelperResult;

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

    #[structopt(short,long)]
    /// Make error output verobse
    verbose: bool,

    #[structopt(short = "h", long)]
    /// Register helper
    register_helper: Option<String>,

    #[structopt(short = "E", long)]
    /// Use environment variables as data source
    env: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    if opt.verbose { eprintln!("{:?}", opt); }
    let mut reg = Handlebars::new();

    match opt.register_glob {
        Some(pattern) => {
            if opt.verbose {
                eprintln!( "Registring templates matching to {:?}", pattern );
            }
            register_templates_from_pattern(&mut reg, pattern)?;
        }
        None => if opt.verbose {
            eprintln!("No glob provided for templates registration.");
        }
    };

    let propsfile = File::open(opt.propsfile)?;
    let data: Value = serde_yaml::from_reader(propsfile)?;
    let template = fs::read_to_string(opt.template)?;

    // try helpers:
    handlebars_helper!(hex: |v: i64| format!("0x{:x}", v));
    reg.register_helper("hex", Box::new(hex));

    reg.register_helper("sh-cat",
        Box::new(
            | h: &Helper, _r: &Handlebars, _: &Context, _rc: &mut RenderContext,
              out: &mut dyn Output| -> HelperResult {
            let param = h.param(0).ok_or(RenderError::new("param not found"))?;
            let param = param.value().as_str().unwrap_or("");

            let proc = Command::new("cat")
                        .args(&[ param ])
                        .output()
                        .expect("Failed to execute process `cat`");
            out.write(from_utf8(&proc.stdout).unwrap())?;

            Ok(())
        }));
    // end, TODO registration should be moved to the other function
    //           and applied to each -h value and -H pattern matchings

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
