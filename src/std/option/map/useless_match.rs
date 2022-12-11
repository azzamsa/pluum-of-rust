/// Useless use of `match`
pub fn useless_match() {
    let maybe_some_string = Some("Hello".to_string());

    // If you use `match` repeteadly like this:
    // let number_length: Option<usize> = match maybe_some_string {
    //     None => None,
    //     Some(s) => Some(s.len()),
    // };

    // These are better alternative:
    let number: Option<usize> = maybe_some_string.map(|s| s.len());
}
