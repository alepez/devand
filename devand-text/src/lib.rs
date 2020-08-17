pub enum Text {
    Settings,
    Affinities,
    CodeNow,
    Schedule,
    Security,
    Messages,
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
        }
    }
}
