use std::time::{Duration, Instant};

pub const SESSION_TIMEOUT_MINS: u64 = 20; //TODO: Should this be here really or be in config or somewhere more easily editable

pub struct AdminSession {
    pub logged_in: bool,
    pub last_active: Instant,
    pub username: Option<String>,
    pub is_admin: bool,
}

impl Default for AdminSession {
    fn default() -> Self {
        Self {
            logged_in: false,
            is_admin: false,
            last_active: Instant::now(),
            username: None,
        }
    }
}

/// Represents the state of an admin session in the CLI.
impl AdminSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid(&self) -> bool {
        self.logged_in
            && self.last_active.elapsed() < Duration::from_secs(SESSION_TIMEOUT_MINS * 60)
    }

    pub fn update_is_admin(&mut self, is_admin: bool) {
        self.is_admin = is_admin;
    }

    pub fn login(&mut self, username: String, is_admin: bool) {
        self.logged_in = true;
        self.username = Some(username);
        self.is_admin = is_admin;
        self.last_active = Instant::now();
    }

    pub fn logout(&mut self) {
        self.logged_in = false;
        self.username = None;
    }
}
