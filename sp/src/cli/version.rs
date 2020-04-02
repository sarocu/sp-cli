use colored::Colorize;

static VERSION: &'static str = "0.1.0";
static AUTHOR: &'static str = "SAROCU";

pub fn version_info() {
    println!("{}", "📈  SUPERPLUS CLI 📊".cyan());
    println!("Version {}", VERSION.cyan());
    println!("Published by {}", AUTHOR.cyan());
    println!("\nCommand uses include: ");
    println!("  Create a new Python based data science project: \n{}", "        sp new project".green());
    println!("  Add data to a Superplus project: \n{}", "       sp new csv [--path]".green());
    println!("  Add visualization boilerplate: \n{}", "     sp new viz".green());
    println!("  Add model boilerplate: \n{}", "     sp new model [--type]".green());

}
