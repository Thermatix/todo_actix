pub mod create {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct TodoList {
        pub title: String,
    }
}
