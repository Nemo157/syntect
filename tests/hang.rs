use syntect::{
    html::{ClassStyle, ClassedHTMLGenerator},
    util::LinesWithEndings,
};

#[test]
fn hang() {
    let code = r#"
        {
            // this is an inline comment

            "foo": "bar" // another inline comment

            /*
               This is a block comment
               that continues on another line
            */
        }
    "#;

    let mut builder = syntect::parsing::SyntaxSetBuilder::new();
    builder
        .add_from_folder("testdata/Packages/JavaScript", true)
        .unwrap();
    let syntaxes = builder.build();
    let syntax = syntaxes.find_syntax_by_token("javascript").unwrap();

    let mut gen = ClassedHTMLGenerator::new_with_class_style(
        syntax,
        &syntaxes,
        ClassStyle::SpacedPrefixed { prefix: "syntax-" },
    );
    for line in LinesWithEndings::from(code) {
        gen.parse_html_for_line_which_includes_newline(line)
            .unwrap();
    }

    gen.finalize();
}
