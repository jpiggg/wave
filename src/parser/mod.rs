
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use pest::error::Error;
use serde::de::value;

use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub enum Children {
    Text(String),
    Node(Box<Node>),
    Nodes(Vec<Node>),
    None
}

#[derive(Debug, PartialEq)]
pub struct Node {
    // data specific to each node type:
    params: NodeType,
    // data common to all nodes:
    children: Children,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Comment,
    Description,
    Element(ElementParams),
    Heading(HeadingParams),
}

#[derive(Debug, PartialEq)]
pub struct ElementParams {
    tag_name: String,
    attrs: AttrMap,
}

#[derive(Debug, PartialEq)]
pub struct HeadingParams {
    level: u8
}

fn add_class(attrs: &mut AttrMap, class: &str) {
    if let Some(existing) = attrs.get_mut("class") {
        existing.push_str(&format!(" {}", class));
    } else {
        attrs.insert("class".to_string(), class.to_string());
    }
}

type AttrMap = HashMap<String, String>;

pub fn comment(data: String) -> Node {
    Node {
        children: Children::Text(data),
        params: NodeType::Comment,
    }
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Children) -> Node {
    Node {
        children,
        params: NodeType::Element(ElementParams {
            tag_name,
            attrs,
        }),
    }
}

pub fn desc(content: String) -> Node {
    Node {
        children: Children::Text(content),
        params: NodeType::Element(ElementParams {
            tag_name: "desc".to_string(),
            attrs: HashMap::new(),
        }),
    }
}

pub fn text(content: String) -> Node {
    Node {
        children: Children::Text(content),
        params: NodeType::Element(ElementParams {
            tag_name: "p".to_string(),
            attrs: HashMap::new()
        },
        )
    }
}

pub fn heading(level: u8, content: String) -> Node {
    Node {
        children: Children::Text(content),
        params: NodeType::Heading(HeadingParams { level }),
    }
}


#[derive(Parser)]
#[grammar = "grammar/DOM.pest"]
struct DOMParser;

pub fn parse_html(input: &str) -> Result<Pair<'_, Rule>, Error<Rule>> {

    let dom = DOMParser::parse(Rule::document, input)?.next().unwrap();

    Ok(dom)
}

pub fn parse_nodes(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::document => {
                let body =  parse_nodes(pair.into_inner().next().unwrap());
                let result = elem(
                    "body".to_string(),
                    HashMap::new(),
                        Children::Node(Box::new(body))

                );

                result
        },
        Rule::element => {
                let mut inner = pair.into_inner();
                let tag_name = inner.next().unwrap().as_str().to_string();

                fn parse_attrs(inner_pair: Pair<Rule>) -> AttrMap {
                      let mut attrs: AttrMap = HashMap::new();

                      if inner_pair.as_rule() == Rule::tag_attr {
                            let mut inner_inner = inner_pair.into_inner();
                            let name = inner_inner.next().unwrap().as_str();
                            let value = inner_inner.next().unwrap().as_str();

                            attrs.insert(name.to_string(), value.to_string());
                      }

                    attrs
                }
                
                let attrs =  parse_attrs(inner.clone().next().unwrap());
                
                let children_iter = if attrs.is_empty() {
                    inner.skip(0)
                } else {
                    inner.skip(1)
                };
                
                let children: Vec<Node> = children_iter.into_iter().map(|child| {
                    parse_nodes(child)
                }).collect();

                elem(tag_name, attrs,  Children::Nodes(children))
        },
        Rule::children => {
            parse_nodes(pair.into_inner().next().unwrap())
        },
        Rule::heading => {
            let mut inner = pair.into_inner();

            let heading_level: u8 = inner.next().unwrap().as_str().parse().unwrap();
            let content = inner.next().unwrap().as_str().to_string();

            heading(heading_level, content)
        },
        Rule::comment => {
            comment(pair.as_str().to_string())
        },
        Rule::description => {
            desc(pair.into_inner().next().unwrap().as_str().to_string())
        },
        Rule::text_content => {
            text(pair.as_str().to_string())
        },
        Rule::WHITESPACE
        | Rule::_opening_tag
        | Rule::_closing_tag
        | Rule::_QUOTE
        | Rule::_comment_block
        | Rule::_comment_start
        | Rule::_comment_end
        | Rule::heading_level
        | Rule::_heading_opening
        | Rule::_heading_closing
        | Rule::_left_chevron
        | Rule::_right_chevron
        | Rule::tag_name
        | Rule::tag_attr
        | Rule::attr_name
        | Rule::attr_value
        | Rule::_description_opening
        | Rule::_description_closing
       | Rule::eoi => {
        unreachable!()
       }
    }
}