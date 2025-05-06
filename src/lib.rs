use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  #[serde(default)]
  pub cmd: Vec<String>,

  #[serde(default)]
  pub deps: Vec<String>,

  #[serde(flatten)]
  pub subtasks: IndexMap<String, Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
  pub tasks: IndexMap<String, Task>,
}

impl TaskConfig {
  pub fn get_all_tasks(&self) -> Vec<String> {
    let mut all_tasks = vec![];

    // Recursively find all tasks
    for (name, task) in &self.tasks {
      if !task.cmd.is_empty() {
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

  pub fn get_deps(&self, path: impl AsRef<str>) -> Vec<String> {
    let mut deps = vec![];

    let path = path.as_ref();
    if let Some(task) = self.find_task(path) {
      // Recursively find dependencies
      for dep in &task.deps {
        deps.extend(self.get_deps(dep));
        deps.push(dep.to_owned());
      }

      // Recursively find dependencies of subtasks
      for subtask in &task.subtasks {
        let subtask_name = format!("{}.{}", path, subtask.0);
        deps.extend(self.get_deps(&subtask_name));
        deps.push(subtask_name.to_owned());
      }
    }

    // Dedupe
    let mut deduped = vec![];
    for dep in deps {
      if !deduped.contains(&dep) {
        deduped.push(dep);
      }
    }

    deduped
  }
}
