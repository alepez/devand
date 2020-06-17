pub fn pure_table_odd(i: usize) -> Option<&'static str> {
    if i % 2 == 1 {
        Some("pure-table-odd")
    } else {
        None
    }
}
