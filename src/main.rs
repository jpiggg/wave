use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::HashMap};

#[derive(Serialize, Deserialize)]
struct Node {   
    // data specific to each node type:
    node_type: NodeType,
    // data common to all nodes:
    children: Vec<Node>,
}
#[derive(Serialize, Deserialize)]
enum NodeType {
    Text(String),
    Description(String),
    Element(ElementData),
    Heading(HeadingData),
}

#[derive(Serialize, Deserialize)]
struct ElementData {
    tag_name: String,
    content: String,
    attrs: AttrMap,
}

#[derive(Serialize, Deserialize)]
struct HeadingData {
    level: u8,
    content: String,
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>, content: String) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name,
            attrs,
            content,
        }),
    }
}

fn desc(content: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Description(content),
    }
}

fn heading(level: u8, content: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Heading(HeadingData { level, content }),
    }
}

fn print_tree() -> Result<()> {
    let tree: Node = elem(
        "div".to_string(),
        HashMap::new(),
        vec![
            heading(1, "Hello, world!".to_string()),
            elem(
                "div".to_string(),
                HashMap::new(),
                vec![
                    elem(
                        "div".to_string(),
                        HashMap::new(),
                        vec![
                            heading(2, "Foo".to_string()),
                            desc("This is very nice item.".to_string()),
                            text("It costs 100$".to_string()),
                        ],
                        "Item 1".to_string(),
                    ),
                    elem(
                        "div".to_string(),
                        HashMap::new(),
                        vec![
                            heading(2, "Bar".to_string()),
                            desc("This is small item.".to_string()),
                            text("It costs 20$".to_string()),
                        ],
                        "Item 2".to_string(),
                    ),
                ],
                "Container".to_string(),
            ),
        ],
        "Root".to_string(),
    );
    let serialized = serde_json::to_string_pretty(&tree)?;
    println!("{}", serialized);
    Ok(())
}

fn main() -> Result<()> {
    print_tree()?;
    Ok(())
}
