use my_parser_kma_makhynia::*;

fn get_content_from_file(name: &str) -> String {
    let path = format!("example/{}", name);
    return std::fs::read_to_string(path).expect("Unable to read file");
}

#[test]
fn svg_open_test() -> anyhow::Result< () > {
    let input = get_content_from_file("svg_open_with_attributes.svg");

    let result = my_parser_kma_makhynia::parse_svg(&input);
    let attributes = vec![
        ("height".to_string(),"140".to_string()),
        ("width".to_string(),"500".to_string())
    ];
    let expect = vec! [
        SvgContent::Svg(attributes),
    ];
    assert!(result.is_ok());
    let actual = result.unwrap();
    assert_eq!(expect, actual);

    Ok( () )
}