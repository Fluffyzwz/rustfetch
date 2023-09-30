extern crate colored;
use colored::*;
pub fn print_art() {
    let ascii_art = [
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀".magenta(),
        "⠀⠀⠀⠀⠀⠀⠀⢀⠠⢐⣤⢄⠔⡆⠈⠀⠀".magenta(),
        "⠀⠀⠀⠀⢀⡄⠋⠀⠀⠀⠊⠃⢠⠄⢰⠀⡀".magenta(),
        "⠀⠀⠀⣰⡉⠣⠀⠈⠁⣠⠀⢀⠡⠖⢰⠃⠃".magenta(),
        "⠀⣠⣾⣿⣿⣆⠑⣀⡈⠄⢚⡵⠂⠰⠃⠌⠀".magenta(),
        "⢿⣿⣿⣿⣿⣿⣿⣿⣿⣦⠀⠀⢈⡠⠀⠀⠀".magenta(),
        "⠀⠙⠻⠿⠿⠿⢿⢿⡿⠃⠉⠒⠉⣼⠀⠀⠀".magenta(),
        "⠀⠀⠀⠀⠀⠠⠚⠀⠀⢂⠠⠀⣠⣸⠆⠀⠀".white(),
        "⠀⠀⠀⠀⠀⠀⠑⢒⠀⠀⡴⡋⠋⠁⠀⠀⠀".white(),
        "⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠀⠀⠀⠀⠀⠀⠀".white(),
    ];

    for line in &ascii_art {
        println!("{}", line);
    }
}
pub fn print_blocks() {
    let mut output = String::new();
    fn add_colored_block(text: &str, color: Color, width: usize) -> String {
        format!("{:width$}", text.color(color), width = width)
    }
    let colors = vec![
        (Color::Red, ""),
        (Color::Green, ""),
        (Color::Yellow, ""),
        (Color::Blue, ""),
        (Color::Magenta, ""),
        (Color::Cyan, ""),
        (Color::White, ""),
    ];
    let block_width = 5;
    for (color, label) in colors {
        let colored_block = add_colored_block(label, color, block_width);
        output.push_str(&colored_block);
        output.push(' ');
    }
    println!("{}", output);
}
