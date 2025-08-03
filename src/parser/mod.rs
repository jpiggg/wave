
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use pest::error::Error;

use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub enum Children {
    Text(String),
    Node(Box<Node>),
    Nodes(Vec<Node>),
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
    Text,
    OneLineComment,
    MultiLineComment,
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

// @TODO: Text node can contain other inline elements like <b> or <i> as children.
pub fn text(data: String) -> Node {
    Node {
        children: Children::Text(data),
        params: NodeType::Text,
    }
}

pub fn one_line_comment(data: String) -> Node {
    Node {
        children: Children::Text(data),
        params: NodeType::OneLineComment,
    }
}

pub fn multi_line_comment(data: String) -> Node {
    Node {
        children: Children::Text(data),
        params: NodeType::MultiLineComment,
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

pub fn heading(level: u8, content: String) -> Node {
    Node {
        children: Children::Text(content),
        params: NodeType::Heading(HeadingParams { level }),
    }
}

pub fn child_node(node: Node) -> Children {
    Children::Node(Box::new(node))
}

pub fn child_nodes(nodes: Vec<Node>) -> Children {
    Children::Nodes(nodes)
}

pub fn child_text(text: String) -> Children {
    Children::Text(text)
}


#[derive(Parser)]
#[grammar = "grammar/DOM.pest"]
struct DOMParser;

pub fn parse_html(input: &str) -> Result<Pair<'_, Rule>, Error<Rule>> {

    let dom = DOMParser::parse(Rule::document, input)?.next().unwrap();
    
    Ok(dom)
}

pub fn parse_dom_tree(pair: Pair<Rule>) -> Node {
    let node = Node{
        params: NodeType::Text,
        children: Children::Text("Foo bar".to_string()),
    };

    node
}