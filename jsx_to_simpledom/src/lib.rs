#![feature(plugin, plugin_registrar, rustc_private)]
extern crate proc_macro_tokens;
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use syntax::symbol::Symbol;
use syntax::ext::base::SyntaxExtension;
//use syntax::ext::proc_macro_shim::prelude::*;

//use syntax::ext::base::ProcMacro;
use syntax::tokenstream::TokenStream;

use proc_macro_tokens::prelude::*;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(Symbol::intern("jsx"), SyntaxExtension::ProcMacro(Box::new(compile_jsx)));
}

fn compile_jsx(ts: TokenStream) -> TokenStream {
    //lex("fn f1() -> bool { true }")
    let source = ts.to_string();
    println!("=======================> {}",&source.clone());

    //TODO write JSX to Rust AST transformation
    let sample = r#"vec![
                el("div",
                    Some(vec![Attr("id","id1"),]),
                    Some(vec![
                        el("div",
                            None,
                            Some(vec![ Element::text("content".to_string()) ]),
                        )
                    ])
                )
            ]"#;
    lex(&sample)

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
