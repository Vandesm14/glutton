use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
  pub tasks: HashMap<String, Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  #[serde(default)]
  pub shell: Vec<String>,

  #[serde(default)]
  pub deps: Vec<String>,
}
