extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/leetcode.c")
        .compile("leetcode");
}