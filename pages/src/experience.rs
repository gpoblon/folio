use dioxus::prelude::*;
#[server]
fn echo_from_server() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
#[component]
pub fn Experience() -> Element {
    let mut response = use_signal(String::new);
    let call_server = move |_| {
        spawn(async move {
            match echo_from_server().await {
                Ok(msg) => response.set(msg),
                Err(e) => response.set(format!("Error: {e}")),
            }
        });
    };
    rsx! {
        section { id: "experience", class: "space-y-8",
            h1 { class: "text-3xl", "Experience" }
            div { class: "space-y-4",
                button { class: "bg-experience text-black", onclick: call_server, "Call Server Function" }
                if !response().is_empty() {
                    p { class: "text-sm text-muted", "Server says: {response()}" }
                }
            }
        }
    }
}
