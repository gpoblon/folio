use dioxus::prelude::*;

#[component]
pub fn Experience() -> Element {
    rsx! {
        section {
            class: "container center-content p-32 lg:w-[60%] m-auto",
            id: "experience",
            features::experiences::Experiences {}
        }
    }
}
