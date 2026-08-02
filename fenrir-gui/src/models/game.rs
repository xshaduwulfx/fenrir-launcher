#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub executable: String,
}

impl Game {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        executable: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            executable: executable.into(),
        }
    }
}
