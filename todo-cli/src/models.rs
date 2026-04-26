use serde:: {Serialize, Deserialize, Debug};

pub struct Task { 
    pub id: u32,
    pub description: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize. Debug)]
pub struct TaskList { 
    pub tasks: Vec<Task>,
}