use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author="Marc Carré", version, about="Tool to convert a Vagrant SSH configuration to an Ansible inventory YAML", long_about = None)]
pub struct Arguments {
    #[clap(flatten)]
    pub verbose: Verbosity<WarnLevel>,

    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Name of the environment to generate.
    #[arg(short, long, default_value_t = String::from("local"))]
    pub environment: String,

    /// Path of the Ansible inventory file to generate.
    #[arg(short, long, default_value = PathBuf::from("local.yaml").into_os_string())]
    pub filepath: PathBuf,
}
