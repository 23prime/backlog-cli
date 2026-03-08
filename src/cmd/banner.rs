pub fn print_banner() {
    use owo_colors::OwoColorize;

    let art = [
        r" ____             _     _             ____ _     ___ ",
        r"| __ )  __ _  ___| | __| | ___   __ _/ ___| |   |_ _|",
        r"|  _ \ / _` |/ __| |/ /| |/ _ \ / _` | |   | |    | | ",
        r"| |_) | (_| | (__|   <| | (_) | (_| | |___| |___ | | ",
        r"|____/ \__,_|\___|_|\_\_|\___/ \__, |\____|_____|___|",
        r"                                |___/",
    ];

    let version = concat!("  v", env!("CARGO_PKG_VERSION"));

    for (i, line) in art.iter().enumerate() {
        if i + 1 == art.len() {
            anstream::println!("{}{}", line.truecolor(76, 188, 154), version.dimmed());
        } else {
            anstream::println!("{}", line.truecolor(76, 188, 154));
        }
    }
}
