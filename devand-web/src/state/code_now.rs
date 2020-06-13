use devand_core::{CodeNowUsers, PublicUserProfile, User, UserId};
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct CodeNowUsersMap(HashMap<UserId, PublicUserProfile>);

impl CodeNowUsersMap {
    pub fn add(&mut self, u: User) {
        let id = u.id;
        let profile = PublicUserProfile::from(u);
        self.0.insert(id, profile);
    }

    pub fn contains(&self, u: &User) -> bool {
        self.0.contains_key(&u.id)
    }
}

impl From<CodeNowUsersMap> for CodeNowUsers {
    fn from(m: CodeNowUsersMap) -> Self {
        CodeNowUsers(m.0.into_iter().map(|(_k, v)| v).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_code_now_users() {
        let mut code_now_users_map = CodeNowUsersMap::default();
        let user = devand_core::mock::user();
        code_now_users_map.add(user);
        let code_now_users = CodeNowUsers::from(code_now_users_map);
        let json = serde_json::to_string(&code_now_users).unwrap();
        let code_now_users_2: CodeNowUsers = serde_json::from_str(&json).unwrap();
        let json_2 = serde_json::to_string(&code_now_users_2).unwrap();
        assert!(json == json_2);
    }
}
