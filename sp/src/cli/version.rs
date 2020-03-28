use colored::Colorize;

static VERSION: &'static str = "0.1.0";
static AUTHOR: &'static str = "SAROCU";

pub fn version_info() {
    println!("{}", "📈  SUPERPLUS CLI 📊".cyan());
    println!("Version {}", VERSION.cyan());
    println!("Published by {}", AUTHOR.cyan());
}
