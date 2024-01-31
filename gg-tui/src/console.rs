pub use console;

#[macro_export]
macro_rules! ep_error {
    ($($t:tt)*) => {{
        eprintln!("{} {}",
              gg_tui::console::console::style(" ERROR ").bg(gg_tui::console::console::Color::Red).black(),
              gg_tui::console::console::style(format!($($t)*)).red(),
        )
    }};
}

#[macro_export]
macro_rules! ep_warning {
    ($($t:tt)*) => {{
        eprintln!("{} {}",
              gg_tui::console::console::style(" WARNING ").bg(gg_tui::console::console::Color::Yellow).black(),
              gg_tui::console::console::style(format!($($t)*)).yellow(),
        )
    }};
}

