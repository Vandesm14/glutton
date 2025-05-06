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
  pub fn get_all_tasks(&self) -> Vec<String> {
    let mut all_tasks = vec![];

    // Recursively find all tasks
    for (name, task) in &self.tasks {
      if !task.shell.is_empty() {
        all_tasks.push(name.to_owned());
      }
      all_tasks.extend(Self::get_subtasks_with_prefix(task, name));
    }

    all_tasks
  }

  fn get_subtasks_with_prefix(task: &Task, prefix: &str) -> Vec<String> {
    let mut subtasks = vec![];

    // Recursively find subtasks with prefix
    for (name, subtask) in &task.subtasks {
      let full_name = format!("{}.{}", prefix, name);
      subtasks.push(full_name.clone());
      subtasks.extend(Self::get_subtasks_with_prefix(subtask, &full_name));
    }

    subtasks
  }

  pub fn find_task(&self, path: impl AsRef<str>) -> Option<&Task> {
    let path = path.as_ref();
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

  pub fn get_deps(&self, task: &Task) -> Vec<String> {
    let mut deps = vec![];

    // Recursively find dependencies
    for dep in &task.deps {
      if let Some(subtask) = self.find_task(dep) {
        deps.extend(self.get_deps(subtask));
        deps.push(dep.to_owned());
      } else {
        deps.push(dep.to_owned());
      }
    }

    deps
  }
}
