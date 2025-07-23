use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

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
    OneLineComment(String),
    MultiLineComment(String),
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

fn add_class(attrs: &mut AttrMap, class: &str) {
    if let Some(existing) = attrs.get_mut("class") {
        existing.push_str(&format!(" {}", class));
    } else {
        attrs.insert("class".to_string(), class.to_string());
    }
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

fn one_line_comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::OneLineComment(data),
    }
}

fn multi_line_comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::MultiLineComment(data),
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
        node_type: NodeType::Element(ElementData {
            tag_name: "desc".to_string(),
            attrs: HashMap::new(),
            content,
        }),
    }
}

fn heading(level: u8, content: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Heading(HeadingData { level, content }),
    }
}

fn print_tree() -> Result<()> {
    // Example tree structure:
    // <div class="root">
    //     <h1>Hello, world!</h1>
    //     <!-- Container with items -->
    //     <div class="container">
    //         <div class="item item1">
    //             <h2>Foo</h2>
    //             <!-- This is a
    //             multi-line comment. -->
    //             <desc>
    //                 This is very nice item.
    //             </desc>
    //             <p>It costs 100$</p>
    //         </div>
    //         <div class="item item2">
    //             <h2>Bar</h2>
    //             <desc>
    //                 This is small item
    //             </desc>
    //             <p>It costs 20$</p>
    //         </div>
    //     </div>
    // </div>
    let tree: Node = elem(
        "div".to_string(),
        {
            let mut attrs = HashMap::new();
            add_class(&mut attrs, "root");
            attrs
        },
        vec![
            heading(1, "Hello, world!".to_string()),
            one_line_comment("Container with items".to_string()),
            elem(
                "div".to_string(),
                {
                    let mut attrs = HashMap::new();
                    add_class(&mut attrs, "container");
                    attrs
                },
                vec![
                    elem(
                        "div".to_string(),
                        {
                            let mut attrs = HashMap::new();
                            add_class(&mut attrs, "item item1");
                            attrs
                        },
                        vec![
                            heading(2, "Foo".to_string()),
                            multi_line_comment("This is a\nmulti-line comment.".to_string()),
                            desc("This is very nice item.".to_string()),
                            text("It costs 100$".to_string()),
                        ],
                        String::new(),
                    ),
                    elem(
                        "div".to_string(),
                        {
                            let mut attrs = HashMap::new();
                            add_class(&mut attrs, "item item2");
                            attrs
                        },
                        vec![
                            heading(2, "Bar".to_string()),
                            desc("This is small item.".to_string()),
                            text("It costs 20$".to_string()),
                        ],
                        String::new(),
                    ),
                ],
                String::new(),
            ),
        ],
        String::new(),
    );
    let serialized = serde_json::to_string_pretty(&tree)?;
    println!("{}", serialized);
    Ok(())
}

fn main() -> Result<()> {
    print_tree()?;
    Ok(())
}
