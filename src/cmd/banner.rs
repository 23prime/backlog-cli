const LOGO: &[&str] = &[
    "██████╗  █████╗  ██████╗██╗  ██╗██╗      ██████╗  ██████╗   ██████╗██╗     ██╗",
    "██╔══██╗██╔══██╗██╔════╝██║ ██╔╝██║     ██╔═══██╗██╔════╝  ██╔════╝██║     ██║",
    "██████╔╝███████║██║     █████╔╝ ██║     ██║   ██║██║  ███╗ ██║     ██║     ██║",
    "██╔══██╗██╔══██║██║     ██╔═██╗ ██║     ██║   ██║██║   ██║ ██║     ██║     ██║",
    "██████╔╝██║  ██║╚██████╗██║  ██╗███████╗╚██████╔╝╚██████╔╝ ╚██████╗███████╗██║",
    "╚═════╝ ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝  ╚═════╝   ╚═════╝╚══════╝╚═╝",
];

pub fn print_banner() {
    use owo_colors::OwoColorize;

    anstream::println!();
    let last = LOGO.len() - 1;
    for (i, line) in LOGO.iter().enumerate() {
        if i == last {
            anstream::println!(
                "{}  v{}",
                line.truecolor(76, 188, 154),
                env!("CARGO_PKG_VERSION").dimmed()
            );
        } else {
            anstream::println!("{}", line.truecolor(76, 188, 154));
        }
    }
    anstream::println!();
}
