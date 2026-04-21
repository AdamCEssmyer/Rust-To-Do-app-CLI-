use serde:: {Serialize, Deserialize, Debug};

pub struct Task { 
    pub id: u32,
    pub description: String,
    pub done: bool,
}

pub struct TaskList { 
    pub tasks: Vec<Task>,
}