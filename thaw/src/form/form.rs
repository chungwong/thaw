use crate::FieldContextInjection;
use leptos::{context::Provider, ev, prelude::*};
use thaw_utils::{class_list, mount_style, BoxOneCallback, ComponentRef};

#[component]
pub fn Form(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Called on form submit after validation passes.
    /// The submit event has `prevent_default()` already called.
    #[prop(optional, into)]
    on_submit: Option<BoxOneCallback<ev::SubmitEvent>>,
    /// Component ref for imperative `validate()` / `validate_field()`.
    #[prop(optional)]
    comp_ref: ComponentRef<FormRef>,
    children: Children,
) -> impl IntoView {
    mount_style("form", include_str!("./form.css"));
    let field_context = FieldContextInjection::new();

    comp_ref.load(FormRef {
        field_context: field_context.clone(),
    });

    let on_submit_handler = {
        let field_context = field_context.clone();
        move |ev: ev::SubmitEvent| {
            ev.prevent_default();
            if field_context.validate() {
                if let Some(ref on_submit) = on_submit {
                    on_submit(ev);
                }
            }
        }
    };

    view! {
        <form class=class_list!["thaw-form", class] novalidate on:submit=on_submit_handler>
            <Provider value=field_context>{children()}</Provider>
        </form>
    }
}

/// Imperative handle for the `Form` component.
#[derive(Clone)]
pub struct FormRef {
    field_context: FieldContextInjection,
}

impl FormRef {
    /// Validate all registered fields. Returns `true` if all pass.
    pub fn validate(&self) -> bool {
        self.field_context.validate()
    }

    /// Validate a specific field by name. Returns `true` if it passes.
    pub fn validate_field(&self, name: String) -> bool {
        self.field_context.validate_field(name)
    }
}
