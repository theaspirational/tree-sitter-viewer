extern crate tree_sitter_python; 
extern crate tree_sitter_viewer;

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {

    // initial code to display
    let code = r#"
fills[0].seq()
"#;

    // specify the parser's language and the initial code.
    let result = tree_sitter_viewer::run(tree_sitter_python::language(), code ); 

    result.await
}
