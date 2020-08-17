pub enum Text {
    Settings,
    Affinities,
    CodeNow,
    Schedule,
    Security,
    Messages,
    AffinitiesTableDescription,
    NoMatchingUsersFound,
    ExtendYourLanguageSelection,
}

impl ToString for Text {
    fn to_string(&self) -> String {
        match self {
            Text::Settings => "Settings".into(),
            Text::Affinities => "Affinities".into(),
            Text::CodeNow => "Code Now".into(),
            Text::Schedule => "Schedule".into(),
            Text::Security => "Security".into(),
            Text::Messages => "Messages".into(),
            Text::AffinitiesTableDescription => "In the table below, you can see a list of developers who love the same languages as you. Just click the chat icon to start chatting and organize your next pair-programming session.".into(),
            Text::NoMatchingUsersFound => "Sorry, no matching users found. You can try to ".into(),
            Text::ExtendYourLanguageSelection => "extend your language selection.".into(),
        }
    }
}
