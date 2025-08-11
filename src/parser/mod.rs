
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
    
    // let ast = elem("body".to_string(),
    //             {
    //                 let mut attrs = HashMap::new();
    //                 attrs
    //             }, Children::Nodes(
    //                 vec![
    //                     match pair.as_rule() {
    //                         Rule::heading => heading(2, "".to_string()),
    //                         _ => todo!()
    //                     }
    //             ]
    //             ));
    Ok(dom)
}

pub fn parse_nodes(pair: Pair<Rule>) -> Option<Node> {
    match pair.as_rule() {
        Rule::document => {
                // let children =  pair.into_inner().skip(2).next();

                //  let test: Vec<Pair<'_, Rule>> = pair.into_inner().next().unwrap().into_inner().skip(2).collect();

                let body =  parse_nodes(pair.into_inner().next().unwrap()).unwrap();
                
                // let children: Vec<_> = pair.into_inner().map(|pair| {
                //     pair.into_inner().skip(2).next()
                // }).collect();

                // print!("--------- DEBUG----------- {:#?}", children);
                Some(elem(
                    "body".to_string(),
                    {
                            let mut attrs = HashMap::new();
                            attrs
                        },
                        Children::Node(Box::new(body))

                ))
        },
        Rule::element => {
                let mut inner = pair.into_inner();
                let tag_name = inner.next().unwrap().as_str().to_string();

                fn parse_attrs(pairs: Option<Pair<Rule>>) -> AttrMap {
                      let mut attrs: AttrMap = HashMap::new();

                      pairs.map(|pair| {
                        let mut inner = pair.into_inner();

                        let name = inner.next().unwrap().as_str();
                        let value = inner.next().unwrap().as_str();

                        attrs.insert(name.to_string(), value.to_string());
                    });

                    attrs
                }
                let attrs =  parse_attrs(inner.clone().next());
                
                let children: Vec<Node> = inner.skip(1).next().unwrap().into_inner().map(|child| {
                    parse_nodes(child).unwrap()
                }).collect();

                println!("----------------> ELEMENT DATA is: \n tag_name {:#?};\n attrs {:#?};\n children {:#?}", tag_name, attrs, children);

                Some(elem(tag_name, attrs, Children::Nodes(children)))
        },
        Rule::children => Some(parse_nodes(pair.into_inner().next()?).unwrap()),
        Rule::heading => {
            let mut inner = pair.into_inner();

            let heading_level: u8 = inner.next().unwrap().as_str().parse().unwrap();
            let content = inner.next().unwrap().as_str().to_string();

            Some(heading(heading_level, content))
        },
        Rule::comment => {
            Some(comment(pair.as_str().to_string()))
        },
        Rule::description => {
            parse_nodes(pair.into_inner().next().unwrap())
        },
        Rule::text_content => {
            Some(text(pair.as_str().to_string()))
        },
        _ => {
            println!("-------- None of the PAIR------ {:#?}", pair);
            None
        }
    }
}