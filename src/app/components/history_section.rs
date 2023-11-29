use crate::model::conversation::Conversation;
use leptos::{html::Div, *};

const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-gray-200 text-black";

#[component]
pub fn HistorySection(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>();
    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });
    view! {
        <div class="h-screen pb-24 w-full flex flex-col overflow-y-auto border border-gray-300 rounded p-5 bg-gray-100" node_ref=chat_div_ref>
          {move || conversation.get().messages.iter().map(move |message| {
              let class_str = if message.is_user { USER_MESSAGE_CLASS } else { MODEL_MESSAGE_CLASS };
              view! {
                <div class={class_str}>
                  {message.text.clone()}
                </div>
              }
            }).collect::<Vec<_>>()
          }
        </div>
    }
}
