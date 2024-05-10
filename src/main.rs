use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::Parser;
use tree_sitter_php;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File paths to search for comments
    filepaths: Vec<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_php::language_php()).expect("Error loading PHP grammar");

    for path in args.filepaths {
        let mut file = File::open(&path)?;
        let mut source_code = String::new();
        file.read_to_string(&mut source_code)?;

        let tree = parser.parse(&source_code, None).unwrap();
        let root_node = tree.root_node();

        let mut cursor = root_node.walk();
        'traverse: loop {
            let node = cursor.node();
            if node.kind() == "comment" {
                let comment_text = node.utf8_text(source_code.as_bytes()).unwrap();
                for (i, comment_line) in comment_text.lines().enumerate() {
                    println!("{}:{:05}: {}", path.to_str().unwrap(), node.start_position().row + 1 + i, comment_line);
                }
            }

            if cursor.goto_first_child() {
                continue;
            }
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    break 'traverse;
                }
            }
        }
    }

    Ok(())
}
