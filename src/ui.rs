use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

pub fn get_color(value: u8) -> ColoredString {
    if value < 20 {
        "red".red()
    } else if value <= 50 {
        "yellow".yellow()
    } else {
        "green".green()
    }
}

pub fn render_bar(label: &str, value: u8, no_unicode: bool) {
    let warning_icon = if value < 20 {
        if no_unicode { " !" } else { " ⚠️" }
    } else {
        ""
    };

    let bar_style = ProgressStyle::with_template(&format!(
        "{{prefix}} {{bar}} | {{pos}}/{{len}}%{}",
        warning_icon
    ))
    .unwrap()
    .progress_chars("█░");

    let color = if value < 20 {
        colored::Color::Red
    } else if value <= 50 {
        colored::Color::Yellow
    } else {
        colored::Color::Green
    };

    let pb = ProgressBar::new(100);
    pb.set_style(bar_style);
    pb.set_prefix(label.color(color).to_string());
    pb.set_position(value.into());
    pb.finish_and_clear();

    // Print the bar with color in label and bar
    println!(
        "{} {} | {}%{}",
        label.color(color),
        generate_bar(value, 100, color),
        value,
        warning_icon
    );
}

fn generate_bar(value: u8, max: u8, color: colored::Color) -> String {
    let width = 30;
    let filled = (value as usize * width) / (max as usize);
    let mut bar = String::new();
    for i in 0..width {
        if i < filled {
            bar.push('█');
        } else {
            bar.push('░');
        }
    }
    bar.color(color).to_string()
}
