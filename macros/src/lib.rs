use darling::FromMeta;
use proc_macro::*;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs};

#[derive(FromMeta)]
struct Event {
    name: String, //event name
    #[darling(default, rename="return")]
    ret: bool,
}

/// Mark function in struct as an event. Assumes that struct implements Observable.
/// # Fields
/// 
/// * `name` [String](std::string::String) - must be the same as the `T`'s [to_string](std::string::ToString::to_string) representation in `Observable<Evt = T>`
/// 
/// * `return` [bool](core::primitive::bool) - provide the actual event when dispatching the event to the callbacks. Return `observable::Return` to provide both the event and function return value, the macro will deal with putting the values is the correct spots. 
/// 
#[proc_macro_attribute]
pub fn event(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);

    let Event { name, ret } = match Event::from_list(&args) {
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

    let dispatch = if ret {
        quote!(self.dispatch(#name.to_string(), Some(res.evt));)
    } else {
        quote!(self.dispatch(#name.to_string(), None);)
    };
    let aux = if ret {
        quote!(return res.ret;)
    } else {
        quote!(return res;)
    };

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

///TODO: observable proc macro to automatically turn struct into an observable by:
///adding a field for a map (of some type) and automatically implementing Observable trait
#[proc_macro_attribute]
pub fn observable(attr: TokenStream, stream: TokenStream) -> TokenStream {
    stream
}
