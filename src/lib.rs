use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  #[serde(default)]
  pub shell: Vec<String>,

  #[serde(default)]
  pub deps: Vec<String>,

  #[serde(flatten)]
  pub subtasks: HashMap<String, Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
  pub tasks: HashMap<String, Task>,
}

impl TaskConfig {
  pub fn find_task(&self, path: String) -> Option<&Task> {
    let parts = path.split('.').collect::<Vec<_>>();
    let len = parts.len();

    let mut current_task = &self.tasks;
    for (i, part) in parts.iter().enumerate() {
      if let Some(task) = current_task.get(part.to_owned()) {
        current_task = &task.subtasks;

        if i == &len - 1 {
          return Some(task);
        }
      } else {
        return None;
      }
    }

    None
  }
}
