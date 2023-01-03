use darling::FromMeta;
use proc_macro::*;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs};

#[derive(FromMeta)]
struct Event {
    name: String, //event name
}

// mark function in struct as an event. assumes that struct implements Observable.
#[proc_macro_attribute]
pub fn event(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);

    let Event { name } = match Event::from_list(&args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let mut item: syn::Item = syn::parse(stream).expect("Could not parse stream");

    let outer = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn"),
    };

    let block = quote!(let res =  {};);
    let mut block_parsed: syn::Stmt =
        syn::parse(block.into()).expect(format!("Failed to parse closure").as_str());
    if let syn::Stmt::Local(local) = &mut block_parsed {
        let (_, init) = &mut local.init.as_mut().expect("Local is none");
        if let syn::Expr::Block(bl) = init.as_mut() {
            bl.block.stmts = outer.block.stmts.clone()
        } else {
            panic!("Did not parse as block")
        }
    } else {
        panic!("Did not parse as local")
    }

    outer.block.stmts = Vec::new();

    let dispatch = quote!(self.dispatch(#name.to_string()););
    let aux = quote!(return res;);

    outer.block.stmts.push(block_parsed);
    outer
        .block
        .stmts
        .push(syn::parse(dispatch.into()).expect(format!("Failed to parse dispatch").as_str()));
    outer
        .block
        .stmts
        .push(syn::parse(aux.into()).expect(format!("Failed to parse aux").as_str()));

    item.into_token_stream().into()
}

//TODO: observable proc macro to automatically turn struct into an observable by:
//adding a field for a hashmap and automatically implementing Observable trait
#[proc_macro_attribute]
pub fn observable(attr: TokenStream, stream: TokenStream) -> TokenStream {
    stream
}
