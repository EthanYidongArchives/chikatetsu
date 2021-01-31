use proc_macro::TokenStream;

use syn::*;
use syn::punctuated::Punctuated;

use quote::*;

fn parse_handles(attrs: Vec<Attribute>) -> (Vec<Ident>, Vec<Ident>) {
    let mut message_idents = Vec::new();
    let mut reply_idents = Vec::new();
    for attr in attrs {
        if attr.path.is_ident("handles") {
            if let Ok(idents) = attr.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated) {
                if idents.len() == 2 {
                    let mut idents = idents.into_iter();
                    message_idents.push(idents.next().unwrap());
                    reply_idents.push(idents.next().unwrap());
                }
            }
        }
        
    }
    (message_idents, reply_idents)
}

#[proc_macro_derive(Actor, attributes(handles))]
pub fn derive_message_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_ident = input.ident;

    let messages_ident = format_ident!("{}Messages", input_ident);
    let replies_ident = format_ident!("{}Replies", input_ident);

    let (messages, replies) = parse_handles(input.attrs);

    let message_impl = quote! {
        #(impl ::chikatetsu::message::Message for #messages {
            type Group = #messages_ident;
            type Reply = #replies;

            fn to_group(self) -> #messages_ident {
                #messages_ident::#messages(self)
            }
        })*
    };

    let message_group_enum = quote! {
        pub enum #messages_ident {
            #(#messages(#messages)),*
        }

        impl ::chikatetsu::message::MessageGroup for #messages_ident {
            type ReplyGroup = #replies_ident;
        }
    };

    let reply_group_enum = quote! {
        pub enum #replies_ident {
            #(#replies(#replies)),*
        }

        #(
            impl ::chikatetsu::message::Reply for #replies {
                type Group = #replies_ident;

                fn from_group(group: #replies_ident) -> Option<Self> {
                    match group {
                        #replies_ident::#replies(inner) => Some(inner),
                        _ => None,
                    }
                }
            }
        )*
    };

    let actor_impl = quote! {
        #[::chikatetsu::async_trait::async_trait]
        impl ::chikatetsu::actor::Actor for #input_ident {
            type MessageGroup = #messages_ident;
            async fn handle_all(&mut self, msg: #messages_ident) -> #replies_ident {
                match msg {
                    #(#messages_ident::#messages(inner) => #replies_ident::#replies(self.handle(inner).await)),*
                }
            }
        }
    };

    let output = quote!{
        #message_impl
        #message_group_enum
        #reply_group_enum
        #actor_impl
    };
    TokenStream::from(output)
}
