use colorful::{Color, Colorful};
use env_logger::{Builder, Target};
use log::LevelFilter;

pub struct Logger {
    builder: Builder,
}

impl Logger {
    pub fn default() -> Self {
        let mut builder = Builder::from_default_env();
        builder.filter_level(LevelFilter::Trace);
        builder.target(Target::Stdout);
        Self { builder }
    }
    pub fn init(&mut self) -> &Self {
        self.builder.init();
        self
    }

    pub fn print_banner(&self) {
        println!(
            "{}",
            r#" __  __          _____ _____ _____  _____  _____          _   _ _   _ ______ _____  "#
                .gradient(Color::LightBlue)
        );
        println!(
            "{}",
            r#"|  \/  |   /\   / ____|_   _/ ____|/ ____|/ ____|   /\   | \ | | \ | |  ____|  __ \ "#
                .gradient(Color::LightBlue)
        );
        println!(
            "{}",
            r#"| \  / |  /  \ | |  __  | || |    | (___ | |       /  \  |  \| |  \| | |__  | |__) |"#
                .gradient(Color::LightBlue)
        );
        println!(
            "{}",
            r#"| |\/| | / /\ \| | |_ | | || |     \___ \| |      / /\ \ | . ` | . ` |  __| |  _  / "#
                .gradient(Color::LightBlue)
        );
        println!(
            "{}",
            r#"| |  | |/ ____ \ |__| |_| || |____ ____) | |____ / ____ \| |\  | |\  | |____| | \ \ "#
                .gradient(Color::LightBlue)
        );
        println!(
            "{}",
            r#"|_|  |_/_/    \_\_____|_____\_____|_____/ \_____/_/    \_\_| \_|_| \_|______|_|  \_\"#
                .gradient(Color::LightBlue)
        );
    }
}
