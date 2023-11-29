use leptos::*;
use leptos_meta::*;
// use leptos_router::*;

use crate::{
    api::converse,
    app::components::{chat_box_section::ChatBoxSection, history_section::HistorySection},
    model::conversation::{Conversation, Message},
};
mod components;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            is_user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });
        converse(conversation.get())
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            // TODO: Disable submit button when response is loading

            let model_message = Message {
                text: "Thinking...".to_string(),
                is_user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().clone().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Rust Talk - AI Powered Chatbot built with Rust 🦀"/>
        <HistorySection conversation />
        <ChatBoxSection send />
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
