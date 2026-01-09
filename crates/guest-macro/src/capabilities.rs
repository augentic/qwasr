use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
// use syn::spanned::Spanned;
use syn::{Ident, LitStr, Path, Result, Token};

use crate::guest::{Config, handler_name};

pub struct Messaging {
    pub topics: Vec<Topic>,
}

impl Parse for Messaging {
    fn parse(input: ParseStream) -> Result<Self> {
        let topics = Punctuated::<Topic, Token![,]>::parse_terminated(input)?;
        Ok(Self {
            topics: topics.into_iter().collect(),
        })
    }
}

pub struct Topic {
    pub pattern: LitStr,
    pub message: Path,
    pub handler: Ident,
}

impl Parse for Topic {
    fn parse(input: ParseStream) -> Result<Self> {
        let pattern: LitStr = input.parse()?;
        input.parse::<Token![:]>()?;

        let message: Path = input.parse()?;
        let handler = handler_name(&pattern);

        Ok(Self {
            pattern,
            message,
            handler,
        })
    }
}

pub fn expand(messaging: &Messaging, config: &Config) -> TokenStream {
    let topic_arms = messaging.topics.iter().map(expand_topic);
    let processors = messaging.topics.iter().map(|t| expand_handler(t, config));

    quote! {
        mod messaging {
            use warp_sdk::wasi_messaging::types::{Error, Message};
            use warp_sdk::{wasi_messaging, wasi_otel};
            use warp_sdk::Handler;

            use super::*;

            pub struct Messaging;
            wasi_messaging::export!(Messaging with_types_in wasi_messaging);

            impl wasi_messaging::incoming_handler::Guest for Messaging {
                #[wasi_otel::instrument]
                async fn handle(message: Message) -> Result<(), Error> {
                    let topic = message.topic().unwrap_or_default();

                    // check we're processing topics for the correct environment
                    // FIXME: this should be done in macro body instead of here
                    let env = std::env::var("ENV").unwrap_or_default();
                    let Some(topic) = topic.strip_prefix(&format!("{env}-")) else {
                        return Err(wasi_messaging::types::Error::Other("Incorrect environment".to_string()));
                    };

                    if let Err(e) = match &topic {
                        #(#topic_arms)*
                        _ => return Err(Error::Other("Unhandled topic".to_string())),
                    } {
                        return Err(Error::Other(e.to_string()));
                    }

                    Ok(())
                }
            }

            #(#processors)*
        }
    }
}

fn expand_topic(topic: &Topic) -> TokenStream {
    let pattern = &topic.pattern;
    let handler = &topic.handler;

    quote! {
        t if t.contains(#pattern) => #handler(message.data()).await,
    }
}

fn expand_handler(topic: &Topic, config: &Config) -> TokenStream {
    let handler_fn = &topic.handler;
    let message = &topic.message;
    let owner = &config.owner;
    let provider = &config.provider;

    quote! {
        #[wasi_otel::instrument]
        async fn #handler_fn(payload: Vec<u8>) -> Result<()> {
             #message::handler(payload)?
                 .provider(&#provider::new())
                 .owner(#owner)
                 .await
                 .map(|_| ())
                 .map_err(Into::into)
        }
    }
}
