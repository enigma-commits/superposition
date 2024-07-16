use leptos::*;

#[component]
pub fn boolean_toggle(
    config_value: bool,
    update_value: Callback<bool, ()>,
    #[prop(default = String::new())] class: String,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let (flag, set_flag) = create_signal(config_value);
    view! {
        <input
            disabled=disabled
            on:click=move |_| {
                set_flag.update(|val| *val = !*val);
                logging::log!("<<>> {}", flag.get());
                update_value.call(flag.get());
            }

            type="checkbox"
            class=format!("toggle toggle-[#ffffff] flex items-center {class}")
            checked=flag
        />
    }
}
