use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

use crate::components::form::Form;
#[component]
pub fn Contact () -> Element {
    use dioxus_motion::prelude::*;
    let mut opacity = use_motion(0.0f32);
    use_effect(move || {
        opacity.animate_to(
            1.0, // target opacity
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 10.0,
                mass: 0.5,
                velocity: 0.0,
            }))
        );
    });
    rsx! {
        div {
            class:"p-10 h-[110dvh] w-full",
            style: "opacity: {opacity.get_value()}; transition: opacity 0.1s;",
            Form {}
        }
    }
}