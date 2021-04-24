use crate::element::Element;

#[derive(Debug)]
pub enum Node {
    Text(String),
    Element(Element),
}
