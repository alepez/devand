pub fn trimlow(s: String) -> String {
    // We help the user, trimming spaces and converting to lowercase
    let s = s.to_lowercase();
    // Note: this allocates a new string, in place trimming does not exist
    s.trim().to_string()
}
