use epub_to_rn::convert_epub_to_rn_json;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input.epub> <output.json>", &args[0]);
        std::process::exit(1);
    }

    let epub_path = &args[1];
    let output_path = &args[2];

    if let Err(e) = convert_epub_to_rn_json(epub_path, output_path) {
        eprintln!("❌ Conversion failed: {}", e);
        std::process::exit(1);
    }
}
