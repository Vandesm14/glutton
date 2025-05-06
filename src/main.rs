use clap::Parser;
use glutton::TaskConfig;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
  query: Option<String>,

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

  if let Some(query) = args.query {
    // Find the task by name
    if let Some(task) = config.find_task(query) {
      println!("Found task: {:#?}", task);
      println!();

      // Print the dependencies
      let deps = config.get_deps(task);
      if !deps.is_empty() {
        println!("Dependencies:");
        for dep in deps {
          println!("  {}", dep);
        }
      } else {
        println!("No dependencies found.");
      }
    } else {
      println!("Task not found");
    }
  } else {
    // Print the available tasks
    if !config.tasks.is_empty() {
      println!("Available tasks:");
      for name in config.get_all_tasks() {
        println!("  {}", name);
      }
    } else {
      println!("No tasks found in the config file.");
    }
  }
}
