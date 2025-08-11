#[cfg(test)]
mod tests {
    use wave::parser::*;    
    use pest_ascii_tree::print_ascii_tree;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn test_parse_html() {
        let input = r#"
            <div class="root">
                <h1>Hello, world!</h1>
                <!-- Container with items -->
                <div class="container">
                    <div class="item item1">
                        <h2>Foo</h2>
                        <!-- This is a
                        multi-line comment. -->
                        <desc>
                            This is very nice item.
                        </desc>
                        <p>It costs 100$</p>
                    </div>
                    <div class="item item2">
                        <h2>Bar</h2>
                        <desc>
                            This is small item
                        </desc>
                        <p>It costs 20$</p>
                    </div>
                </div>
            </div>
        "#;
        let expected_result = elem(
        "div".to_string(),
        {
            let mut attrs = HashMap::new();
            attrs.insert("class".to_string(), "root".to_string());
            attrs
        },
        Children::Nodes(vec![
            heading(1, "Hello, world!".to_string()),
            comment("Container with items".to_string()),
            elem(
                "div".to_string(),
                {
                    let mut attrs = HashMap::new();
                    attrs.insert("class".to_string(), "container".to_string());
                    attrs
                },
                Children::Nodes(vec![
                    elem(
                        "div".to_string(),
                        {
                            let mut attrs = HashMap::new();
                            attrs.insert("class".to_string(), "item item1".to_string());
                            attrs
                        },
                            Children::Nodes(vec![
                                heading(2, "Foo".to_string()),
                                comment("This is a\nmulti-line comment.".to_string()),
                                desc("This is very nice item.".to_string()),
                                text("It costs 100$".to_string())
                            ])
                    ),
                    elem(
                        "div".to_string(),
                        {
                            let mut attrs = HashMap::new();
                            attrs.insert("class".to_string(), "item item2".to_string());
                            attrs
                        },
                        Children::Nodes(vec![
                            heading(2, "Bar".to_string()),
                            desc("This is small item".to_string()),
                            text("It costs 20$".to_string())
                        ])
                    )
                ])
            )
        ])
    );

        let dom = parse_html(input).unwrap();

        print_ascii_tree(Ok(dom.clone().into_inner()));

        let result = parse_nodes(dom).unwrap();

        assert_eq!(&result, &expected_result);
    }
}