use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Test {
    pub input: String,
    pub output: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Task {
    pub name: String,
    pub group: String,
    pub url: String,
    pub tests: Vec<Test>,
}