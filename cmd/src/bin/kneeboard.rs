use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;

use clap::{Parser, Subcommand};
use common::{create_template_plan, KneeboardError};

use core::planner::create_planning;
use definition::Plan;

#[derive(Parser, Debug)]
pub struct CommandLine {
    #[command(subcommand)]
    command: Commands,
}

const DEFAULT_INPUT: &str = "kneeboard-notes.yaml";
const DEFAULT_OUPUT: &str = "kneeboard-notes.pdf";
const DEFAULT_TEMPLATE: &str = "kneeboard-notes-template.yaml";

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create kneeboard notes from a definition file
    Create {
        /// Definition file name
        #[arg(short, long, default_value=DEFAULT_INPUT)]
        input: String,

        /// Kneeboard notes PDF file name
        #[arg(short, long, default_value=DEFAULT_OUPUT)]
        output: String,
    },
    /// Create a template definition file
    Template {
        /// Template definition file name
        #[arg(short, long, default_value=DEFAULT_TEMPLATE)]
        template: String,
    },
}

fn main() {
    let args = CommandLine::parse();

    match args.command {
        Commands::Create { input, output } => create(&input, &output),
        Commands::Template { template } => create_template(&template),
    }
}

fn create(input: &str, output: &str) {
    let in_path = Path::new(input);
    let in_meta = match in_path.metadata() {
        Ok(meta) => meta,
        Err(err) => {
            print_file_err("Could not read definition", input, err);
            return;
        }
    };
    if !in_meta.is_file() {
        println!("Definition isn't a file: {input}");
        return;
    }

    let plan: Plan = match decode(in_path) {
        Ok(plan) => plan,
        Err(err) => {
            println!("Failed to parse definition {input} - {:?}", err);
            return;
        }
    };

    let doc = create_planning(&plan);

    let mut file = match File::create(output) {
        Ok(file) => file,
        Err(err) => {
            print_file_err("Could not write output file", output, err);
            return;
        }
    };

    if let Err(err) = doc.write(&mut file) {
        println!("Could not write to output file: {output} - {err}");
    }
}

fn decode(path: &Path) -> Result<Plan, KneeboardError> {
    let in_file = File::open(path)?;

    let (is_yaml, is_json) = if let Some(ext) = path.extension() {
        let lower = ext.to_ascii_lowercase();

        let is_yaml = lower == "yml" || lower == "yaml";
        let is_json = lower == "jsn" || lower == "json";

        (is_yaml, is_json)
    } else {
        return Err(KneeboardError::String(
            "Unknown file format (expect file extention yaml or json)".to_owned(),
        ));
    };

    let plan: Plan = if is_yaml {
        serde_yaml::from_reader(in_file)?
    } else if is_json {
        serde_json::from_reader(in_file)?
    } else {
        return Err(KneeboardError::String(
            "Unknown file format (expect file extention yaml or json)".to_owned(),
        ));
    };

    Ok(plan)
}

fn create_template(output: &str) {
    if let Err(err) = create_template_inner(output) {
        println!("{:?}", err);
    }
}

fn create_template_inner(output: &str) -> Result<(), KneeboardError> {
    let plan = create_template_plan();

    let path = Path::new(output);

    let (is_yaml, is_json) = if let Some(ext) = path.extension() {
        let lower = ext.to_ascii_lowercase();

        let is_yaml = lower == "yml" || lower == "yaml";
        let is_json = lower == "jsn" || lower == "json";

        (is_yaml, is_json)
    } else {
        return Err(KneeboardError::String(
            "Unknown file format (expect file extention yaml or json)".to_owned(),
        ));
    };

    let out_file = File::create(path)?;

    if is_json {
        serde_json::to_writer_pretty(out_file, &plan)?;
        println!("Wrote json template to {output}");
    } else if is_yaml {
        serde_yaml::to_writer(out_file, &plan)?;
        println!("Wrote yaml template to {output}");
    } else {
        return Err(KneeboardError::String(
            "Unknown file format (expect file extention yaml or json)".to_owned(),
        ));
    };

    Ok(())
}

fn print_file_err(msg: &str, file: &str, err: Error) {
    match err.kind() {
        ErrorKind::NotFound => println!("{msg} - could not find {file}"),
        ErrorKind::PermissionDenied => println!("{msg} - don't have permissions for {file}"),
        _ => println!("{msg}: {file} - {:?}", err.raw_os_error()),
    };
}
