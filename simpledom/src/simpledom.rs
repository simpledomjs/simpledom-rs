use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Write;

#[derive(Clone)]
pub struct Element{
    pub name: Option<&'static str>,
    pub attrs: Option<Vec<Attribute>>, 
    pub content: Option<ElementContent>,
}

#[derive(Clone)]
pub enum ElementContent{
    Text(String),
    Html(&'static str),
    Children(Vec<Element>),
}

impl Element {
    pub fn new(name: &'static str, attrs: Option<Vec<Attribute>>, content: Option<ElementContent>) -> Element {
        Element {
            name: Some(name),
            attrs: attrs,
            content: content,
        }
    }

    pub fn text(content: String) -> Element {
        Element {
            name: None,
            attrs: None,
            content: Some(ElementContent::Text(content))
        }
    }

    pub fn html(content: &'static str) -> Element {
        Element {
            name: None,
            attrs: None,
            content: Some(ElementContent::Html(content))
        }
    }
}

#[derive(Clone)]
pub enum Attribute{
    Attr(&'static str, &'static str),
    FnAttr(&'static str, Rc<RefCell<Box<Fn()>>>)
}

pub fn render_to_string(elements: Vec<Element>) -> String {
    let mut result = String::new();

    for element in elements {
        match element.name{
            Some(name) => {
                write!(&mut result, "<{}{}>{}</{}>", 
                    name, 
                    write_attr(element.attrs.clone()),
                    write_elem_content(element.content.clone()),
                    name
                ).unwrap();
            },
            None => write!(&mut result, "{}", write_elem_content(element.content.clone())).unwrap(),
        };

    }

    return result;
}

fn write_elem_content(element_content: Option<ElementContent>) -> String{
    let mut result = String::new(); 
    match element_content{
        Some(content) => {
            match content {
                ElementContent::Text(text) => write!(&mut result, "{}", text).unwrap(),
                ElementContent::Html(html) => write!(&mut result, "{}", html).unwrap(),
                ElementContent::Children(elts) => write!(&mut result, "{}", render_to_string(elts)).unwrap(),
            };
        },
        None => (),
    };        
    result
}

fn write_attr(attrs: Option<Vec<Attribute>>) -> String{
    let mut result = String::new(); 
    match attrs{
        Some(attrs) => {
            for attr in attrs {
                match attr {
                    Attribute::Attr(name, value) => write!(&mut result, r#" {}="{}""#, name, value).unwrap(),
                    //Attribute::FnAttr(event_name, fn_block) => write!(&mut result, " {}='{}'", event_name, "TODO").unwrap(),
                    Attribute::FnAttr(event_name, fn_block) => (),
                };
            }
        },
        None => ()
    };
    result  
}

pub fn el(name: &'static str, attrs: Option<Vec<Attribute>>, childrens: Option<Vec<Element>>) -> Element {
    /*
    if (isFunction(name)) {
        return name(attrs, ...children);
    }
    return {
        name,
        attrs: attrs || {},
        children: (flatten(children) || []).filter(child => child !== null && child !== undefined),
        isElem: true
    };
    */
    match childrens {
        Some(c) => Element::new(name, attrs, Some(ElementContent::Children(c))),
        None => Element::new(name, attrs, None),
    }
}

#[cfg(test)]
mod tests {
    use simpledom::{el, render_to_string};
    use simpledom::{Attribute, Element};
    use simpledom::Attribute::{Attr, FnAttr};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_render_to_string() {

        let result = render_to_string(vec![
            el("div", None, Some(vec![
                el("div",
                    Some(vec![
                        Attr("id","id1"),
                        FnAttr("onClick", Rc::new(RefCell::new( Box::new(|| {}) )))
                    ]),
                    Some(vec![
                        Element::text("content".to_string())
                    ]),
                )
            ])),
            el("div", None, Some(vec![Element::text("0".to_string())])),
            el("ul", None, Some(vec![
                el("li", None, Some(vec![Element::text("1".to_string())])),
                el("li", None, Some(vec![Element::text("2".to_string())])),
                el("li", None, Some(vec![Element::text("3".to_string())])),
                el("li", None, Some(vec![Element::text("4".to_string())])),
            ])),
            el("div", None, Some(vec![Element::html("<span>Coucou</span>")])),
        ]);

        //println!("---------------------- \n {} \n ----------------------", result);
        
        assert_eq!(r#"<div><div id="id1">content</div></div><div>0</div><ul><li>1</li><li>2</li><li>3</li><li>4</li></ul><div><span>Coucou</span></div>"#, result);
    }

    #[test]
    fn test_render_to_string_with_component() {
        
        pub mod another_component{
            use simpledom::{el, render_to_string};
            use simpledom::Element;

            pub struct MyDivAttr{
                pub content: String
            }

            pub fn MyDiv(attr: MyDivAttr) -> String{
                return render_to_string(vec![
                    el("div",
                        None,
                        Some(vec![
                            Element::text(attr.content)
                        ])
                    )
                ]);
            }
        }
         
        let result = render_to_string(vec![
            el("div",
                Some(vec![Attr("id","id1"),]),
                Some(vec![
                    Element::text( another_component::MyDiv(another_component::MyDivAttr{content:"content".to_string()}) )
                ])
            )
        ]);
        
        assert_eq!(r#"<div id="id1"><div>content</div></div>"#, result);

    }

    #[test]
    fn test_jsx_render_to_string() {

        let result = render_to_string(
            jsx!(
                <div id="id1">
                    <div>content</div>
                </div>
            )
            /*
            //This procedural macro produce this one
            vec![
                el("div",
                    Some(vec![Attr("id","id1"),]),
                    Some(vec![
                        el("div",
                            None,
                            Some(vec![ Element::text("content".to_string()) ]),
                        )
                    ])
                )
            ]
            */
        );

        //println!("---------------------- \n {} \n ----------------------", result);
        
        assert_eq!(r#"<div id="id1"><div>content</div></div>"#, result);
    }
    
}

