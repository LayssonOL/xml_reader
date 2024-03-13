pub mod reader;

fn main() {
    println!("Hello, world!");

    let xml = r#"<tag1 att1 = "test">
                <tag2><!--Test comment-->Test</tag2>
                <tag2>Test 2</tag2>
             </tag1>"#;

    reader::read_xml(&xml);
}
