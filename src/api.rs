use crate::model::conversation::Conversation;
use leptos::*;

#[server(Converse "/api")]
pub async fn converse(cs: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use llm::models::Llama;

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner();
    })
    .await
    .unwrap();

    use llm::KnownMoodel;

    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an AI assistant.";
    let mut history = format!(
        "{character_name}: Hello - How may I help you today?\n\
    {user_name}: What is the capital of USA?\n\
    {character_name}: Washington D.C. is the capital of USA.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{character_name}: {msg}\n")
        } else {
            format!("{user_name}: {msg}\n")
        };

        history.push_str(&curr_line);
    }

    Ok(String::from(""))
}
