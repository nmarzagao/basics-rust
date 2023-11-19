pub struct Task {
    pub desc: String,
    pub status: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            desc: description,
            status: false,
        }
    }
}