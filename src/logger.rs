// Verbose logs are written to stderr to avoid interfering with stdout output
// (e.g. --json), so that pipes and redirects work as expected.
pub fn verbose(msg: &str) {
    if is_verbose() {
        use owo_colors::OwoColorize;
        anstream::eprintln!("{}", msg.dimmed());
    }
}

fn is_verbose() -> bool {
    std::env::var("BL_VERBOSE").is_ok_and(|v| v != "0" && v != "false")
}
