use crate::element::Element;
#[test]
fn basic_roundtrip() {
    let xml = r#"<asdf asdf="hi">  <qwer/> <wer/> </asdf>"#;
    let elt = Element::from(xml);
    let new_xml = elt.to_str();

    assert_eq!(xml, new_xml);
}
