use devand_core::{CodeNowUsers, PublicUserProfile, User, UserId};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CodeNowUsersMap {
    users: HashMap<UserId, (Instant, PublicUserProfile)>,
    last_clear: Instant,
}

impl Default for CodeNowUsersMap {
    fn default() -> Self {
        Self {
            users: HashMap::new(),
            last_clear: Instant::now(),
        }
    }
}

impl CodeNowUsersMap {
    const TTL: Duration = Duration::from_secs(60);
    const CLEAR_INTERVAL: Duration = Duration::from_secs(30);

    pub fn add(&mut self, u: User) {
        let id = u.id;
        let profile = PublicUserProfile::from(u);
        let now = Instant::now();
        self.users.insert(id, (now, profile));
    }

    pub fn touch(&mut self, id: UserId) -> bool {
        if self.is_time_to_clear() {
            self.clear();
        }

        if let Some(entry) = self.users.get_mut(&id) {
            entry.0 = Instant::now();
            true
        } else {
            false
        }
    }

    pub fn contains(&self, u: &User) -> bool {
        self.users.contains_key(&u.id)
    }

    fn is_time_to_clear(&self) -> bool {
        self.last_clear.elapsed() > Self::CLEAR_INTERVAL
    }

    fn clear(&mut self) {
        self.last_clear = Instant::now();

        let old_entities: Vec<_> = self
            .users
            .iter()
            .filter(|(_, (t, _))| t.elapsed() > Self::TTL)
            .map(|(&id, _)| id)
            .collect();

        dbg!(&old_entities);

        for id in old_entities {
            self.users.remove(&id);
        }
    }
}

impl From<CodeNowUsersMap> for CodeNowUsers {
    fn from(m: CodeNowUsersMap) -> Self {
        CodeNowUsers(m.users.into_iter().map(|(_k, v)| v.1).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_user() {
        let mut m = CodeNowUsersMap::default();
        let user = devand_core::mock::user();
        m.add(user.clone());
        assert!(m.contains(&user));
    }
}
