use tree_sitter::Language;

fn main() {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(unsafe { tree_sitter_markdown() })
        .unwrap();
    let tree = parser.parse(b"<x>\n</x>", None).expect("parse succedes");
    let mut node = tree.root_node();
    println!("root: {node:?}");
    node = node
        .descendant_for_byte_range(4, 4)
        .expect("desendant exists");
    println!("{node:?}");
    node = node.prev_sibling().unwrap();
    println!("{node:?}");
    let node = node.prev_sibling();
    println!("done: {node:?}");
}

extern "C" {
    fn tree_sitter_markdown() -> Language;
}
