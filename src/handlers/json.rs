pub mod create {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct TodoList {
        pub title: String,
    }
}

pub mod update {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct Result {
        pub success: bool,
    }
}
