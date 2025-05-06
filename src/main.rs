use clap::Parser;
use glutton::TaskConfig;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
  /// Path to config file
  #[arg(short = 'c', long = "config", default_value = "glutton.toml")]
  config: PathBuf,
}

fn main() {
  let args = Args::parse();

  // Read the config file
  let config_content =
    std::fs::read_to_string(&args.config).unwrap_or_else(|_| {
      panic!("Failed to read config file: {:?}", args.config)
    });

  // Parse the config file
  let config: TaskConfig =
    toml::from_str(&config_content).unwrap_or_else(|err| {
      panic!(
        "Failed to parse config file {}: {}",
        args.config.display(),
        err
      )
    });

  // Print the available tasks
  if !config.tasks.is_empty() {
    println!("Available tasks:");
    for task_name in config.tasks.keys() {
      println!("  {}", task_name);
    }
  } else {
    println!("No tasks found in the config file.");
  }
}
