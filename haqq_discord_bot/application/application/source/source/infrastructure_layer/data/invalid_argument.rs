pub struct InvalidArgument {
    pub context: Option<String>,
}

impl InvalidArgument {
    pub fn new() -> Self {
        return Self {
            context: None,
        };
    }
}
