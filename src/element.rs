use crate::node::Node;
use quick_xml::Reader;
use quick_xml::{events::*, Writer};
use std::{collections::HashMap, fmt::Display};
use std::{
    io::{Cursor, Write},
    str::FromStr,
};

#[derive(Debug)]
pub struct Element {
    pub children: Vec<Node>,
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_str())?;

        Ok(())
    }
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Element::from(s))
    }
}

impl Element {
    pub fn from(xml: &str) -> Element {
        let mut reader = Reader::from_str(xml);

        //let mut txt = Vec::new();
        let mut buf = Vec::new();

        let elt = loop {
            match reader.read_event(&mut buf) {
                Ok(event) => match event {
                    Event::Start(ref e) => {
                        break construct_elt(e);
                    }
                    Event::End(_) => {}
                    Event::Empty(ref e) => {
                        return construct_elt(e);
                    }
                    Event::Text(_) => {}
                    Event::Comment(_) => {}
                    Event::Eof => panic!("unhandled"),
                    Event::CData(_) => {}
                    Event::Decl(_) => {}
                    Event::PI(_) => {}
                    Event::DocType(_) => {}
                },
                Err(e) => panic!("bad: {:?}", e),
            };
        };

        let mut stack = vec![elt];
        loop {
            match reader.read_event(&mut buf) {
                Ok(event) => match event {
                    Event::Start(ref e) => stack.push(construct_elt(e)),
                    Event::End(ref e) => {
                        let curr_elt = stack.pop().unwrap();
                        if stack.len() == 0 {
                            return curr_elt;
                        }
                        println!("ending {:?} with {:?}", curr_elt.name, stringify(e.name()));
                        if curr_elt.name != stringify(e.name()) {
                            panic!("mismatched names");
                        }
                        stack
                            .last_mut()
                            .unwrap()
                            .children
                            .push(Node::Element(curr_elt));
                    }
                    Event::Empty(ref e) => {
                        stack
                            .last_mut()
                            .unwrap()
                            .children
                            .push(Node::Element(construct_elt(e)));
                    }
                    Event::Text(ref e) => {
                        let text = stringify(e);
                        if text != "" {
                            stack.last_mut().unwrap().children.push(Node::Text(text))
                        }
                    }
                    Event::Comment(_) => {}
                    Event::Eof => break,
                    Event::CData(_) => {}
                    Event::Decl(_) => {}
                    Event::PI(_) => {}
                    Event::DocType(_) => {}
                },
                Err(e) => panic!("bad: {:?}", e),
            };
        }
        stack.pop().unwrap()
    }
    pub fn to_str(&self) -> String {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        write_to_writer(&self, &mut writer);

        String::from_utf8(writer.into_inner().into_inner()).unwrap()
    }
}

fn construct_elt(event: &BytesStart) -> Element {
    Element {
        children: vec![],
        name: stringify(event.name()),
        attrs: event
            .attributes()
            .map(|x| {
                let x = x.unwrap();
                return (stringify(x.key), stringify(&x.value));
            })
            .collect::<HashMap<String, String>>(),
    }
}

pub(crate) fn stringify(thing: &[u8]) -> String {
    String::from_utf8(thing.to_owned()).unwrap()
}

fn write_to_writer<W: Write>(elt: &Element, writer: &mut Writer<W>) {
    let start = create_bytes_start(&elt);
    writer
        .write_event(match elt.children.len() {
            0 => Event::Empty(start),
            _ => Event::Start(start),
        })
        .unwrap();

    if elt.children.len() == 0 {
        return;
    }

    for child in &elt.children {
        match child {
            Node::Text(text) => writer
                .write_event(Event::Text(BytesText::from_plain(text.as_bytes())))
                .unwrap(),
            Node::Element(child) => write_to_writer(&child, writer),
        }
    }

    writer
        .write_event(Event::End(BytesEnd::borrowed(elt.name.as_bytes())))
        .unwrap();
}

fn create_bytes_start<'a>(elt: &'a Element) -> BytesStart<'a> {
    let mut start = BytesStart::borrowed(elt.name.as_bytes(), elt.name.len());
    for (k, v) in elt.attrs.iter() {
        start.push_attribute((k.as_bytes(), v.as_bytes()))
    }

    start
}
