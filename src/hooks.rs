use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hooks {
    pre: Option<String>,

    post: Option<String>,
}

impl Hooks {
    pub fn new(pre: Option<String>, post: Option<String>) -> Self {
        Hooks { pre, post }
    }

    pub fn get_pre(&self) -> Option<String> {
        self.pre.clone()
    }

    pub fn get_post(&self) -> Option<String> {
        self.post.clone()
    }
}
