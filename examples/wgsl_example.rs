extern crate tree_sitter_wgsl;
extern crate tree_sitter_viewer;

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {

    // initial code to display
    let code = r#"@fragment
fn main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.4, 0.4, 0.8, 1.0);
}"#;

    // specify the parser's language and the initial code.
    let result = tree_sitter_viewer::run(tree_sitter_wgsl::language(), code );

    result.await
}
