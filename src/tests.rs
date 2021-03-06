use crate::element::Element;

#[test]
fn basic_roundtrip() {
    let xml = r#"<asdf asdf="hi">  <qwer/> <wer/> </asdf>"#;
    let elt: Element = xml.parse().unwrap();
    let new_xml = elt.to_string();

    assert_eq!(xml, new_xml);
}

#[test]
fn nuke_nospace() {
    let xml = "<asdf></asdf>";
    let elt: Element = xml.parse().unwrap();
    let new_xml = elt.to_string();

    assert_eq!(new_xml, "<asdf/>");
}
