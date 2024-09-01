// In a separate file, e.g., emojis.rs
pub static EMOJIS: &[&str] = &["🧨", "🖊️", "👁️", "🕹️", "🖥️", "🏃"];

fn main() {
    // Now you can use EMOJIS here
    println!("First emoji: {}", EMOJIS[0]);
}
