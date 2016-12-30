#![feature(plugin)]
#![plugin(jsx_to_simpledom)]

extern crate webplatform;
extern crate simpledom;

fn main() {

    println!("Hello, world!");

    let document = webplatform::init();

    let body = document.element_query("body").unwrap();
    let hr = document.element_create("hr").unwrap();
    body.append(&hr);

    body.html_prepend("<h1>HELLO FROM RUST</h1>");

    /*
    let html_button = jsx!(
        <button>CLICK ME...</button>
    );
    */
    use simpledom::{el, render_to_string, Element};
    let html_button = render_to_string(vec![
        el("button",
            None,
            Some(vec![
                Element::text("CLICK ME...".to_string())
            ])
        )
    ]);
    body.html_append(&html_button);


    let button = document.element_query("button").unwrap();

    let bodyref = body.root_ref();
    let bodyref2 = body.root_ref();
    
    button.on("click", move |_| {
        bodyref2.prop_set_str("bgColor", "blue");
        println!("This should be string 'blue': {:?}", bodyref2.prop_get_str("bgColor"));
    });

    println!("This should be empty string: {:?}", bodyref.prop_get_str("bgColor"));
    println!("Width?: {:?}", bodyref.prop_get_i32("clientWidth"));

    webplatform::spin();    

}