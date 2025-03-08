pub struct Auth {
    username: String,
    password: String,
}

impl Auth {
    pub fn new(username: String, password: String) -> Self {
        Auth { username, password }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        self.username == username && self.password == password
    }
}
