use crate::node::Node;
use quick_xml::{events::*, Writer};
use quick_xml::{Error, Reader};
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
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        match self.write_to_writer(&mut writer) {
            Ok(()) => {}
            Err(_) => return Err(std::fmt::Error {}),
        }

        let str = String::from_utf8(writer.into_inner().into_inner()).unwrap();
        f.write_str(&str)?;

        Ok(())
    }
}

impl FromStr for Element {
    type Err = Error;

    fn from_str(xml: &str) -> Result<Self, Self::Err> {
        let mut reader = Reader::from_str(xml);

        let mut buf = Vec::new();

        // construct the root element
        let elt = loop {
            match reader.read_event(&mut buf)? {
                Event::Start(ref e) => {
                    break construct_elt(e);
                }
                Event::End(_) => {}
                Event::Empty(ref e) => return Ok(construct_elt(e)),
                Event::Text(_) => {}
                Event::Comment(_) => {}
                Event::Eof => panic!("unhandled"),
                Event::CData(_) => {}
                Event::Decl(_) => {}
                Event::PI(_) => {}
                Event::DocType(_) => {}
            };
        };

        let mut stack = vec![elt];
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(ref e) => stack.push(construct_elt(e)),
                Event::End(ref e) => {
                    let curr_elt = stack.pop().unwrap();
                    if stack.len() == 0 {
                        return Ok(curr_elt);
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
            };
        }
        Ok(stack.pop().unwrap())
    }
}

impl Element {
    fn write_to_writer<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), Error> {
        let start = self.create_bytes_start();
        writer.write_event(match self.children.len() {
            0 => Event::Empty(start),
            _ => Event::Start(start),
        })?;

        if self.children.len() == 0 {
            return Ok(());
        }

        for child in &self.children {
            match child {
                Node::Text(text) => {
                    writer.write_event(Event::Text(BytesText::from_plain(text.as_bytes())))?
                }
                Node::Element(child) => child.write_to_writer(writer)?,
            }
        }

        writer.write_event(Event::End(BytesEnd::borrowed(self.name.as_bytes())))?;

        Ok(())
    }

    fn create_bytes_start(&self) -> BytesStart {
        let mut start = BytesStart::borrowed(self.name.as_bytes(), self.name.len());
        for (k, v) in self.attrs.iter() {
            start.push_attribute((k.as_bytes(), v.as_bytes()))
        }

        start
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
