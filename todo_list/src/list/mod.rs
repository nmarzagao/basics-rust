struct Task {
    desc: String,
    status: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Self {
            desc: description,
            status: false,
        }
    }
}

pub struct List {
    tasks: Vec<Task>,
}

impl List {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }

    pub fn push(&mut self, task_title: String) {
        self.tasks.push(Task::new(task_title));
    }

    pub fn remove(&mut self, target: String) {
        let mut counter = 0;

        for task in &self.tasks {
            if task.desc == target {
                self.tasks.remove(counter);
                break;
            }
            counter += 1;
        }
    }

    pub fn status_toggle(&mut self, target: String) {
        for task in &mut self.tasks {
            if task.desc == target {
                if task.status == false { task.status = true; }
                else { task.status = false; }
            }
        }
    }

    pub fn print(&self) {
        for task in &self.tasks {
            println!("Task: {}\nStatus: {}\n", task.desc, task.status);
        }
    }
}