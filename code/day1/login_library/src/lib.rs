use std::path::Path;
use serde::{Deserialize, Serialize};

fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: hash_password(password),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        self.password == hash_password(password)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginManager {
    pub users: Vec<User>,
}

impl LoginManager {
    pub fn new() -> Self {
        let path = Path::new("users.json");
        if path.exists() {
            let data = std::fs::read_to_string(path).expect("Failed to read users.json");
            let users: Vec<User> = serde_json::from_str(&data).expect("Failed to parse users.json");
            Self { users }
        } else {
            Self { users: vec![] }
        }
    }

    pub fn save(&self) {
        let data = serde_json::to_string_pretty(&self.users).expect("Failed to serialize users");
        std::fs::write("users.json", data).expect("Failed to write users.json");
    }

    pub fn add_user(&mut self, username: &str, password: &str) {
        let user = User::new(username, password);
        self.users.push(user);
    }

    pub fn verify_user(&self, username: &str, password: &str) -> Option<&User> {
        self.users.iter().find(|user| user.username == username && user.verify_password(password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_twice() {
        let password = "hunter2";
        let hash1 = hash_password(password);
        let hash2 = hash_password(password);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hashing_different_passwords() {
        let password1 = "hunter2";
        let password2 = "hunter3";
        let hash1 = hash_password(password1);
        let hash2 = hash_password(password2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_user_creation() {
        let user = User::new("alice", "password123");
        assert_eq!(user.username, "alice");
        assert_ne!(user.password, "password123");
    }

    #[test]
    fn test_verify_password() {
        let user = User::new("bob", "secret");
        assert!(user.verify_password("secret"));
        assert!(!user.verify_password("wrong_password"));
    }

    #[test]
    fn test_login_manager_add_and_verify() {
        let mut manager = LoginManager::new();
        manager.add_user("charlie", "mypassword");
        assert!(manager.verify_user("charlie", "mypassword").is_some());
        assert!(manager.verify_user("charlie", "wrongpassword").is_none());
        assert!(manager.verify_user("unknown", "mypassword").is_none());
    }
}