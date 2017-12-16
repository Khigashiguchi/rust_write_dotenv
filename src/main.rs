use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::create("foo.txt").unwrap();
    let x = r#"
        This is a "raw string literal," roughly equivalent to a heredoc.
    "#;

    f.write_all(x.as_bytes());
}
