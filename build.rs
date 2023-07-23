fn main() {
    cc::Build::new()
        .include("tree-sitter-markdown/tree-sitter-markdown/src")
        .file("tree-sitter-markdown/tree-sitter-markdown/src/scanner.c")
        .file("tree-sitter-markdown/tree-sitter-markdown/src/parser.c")
        .compile("tree-sitter-markdown")
}
