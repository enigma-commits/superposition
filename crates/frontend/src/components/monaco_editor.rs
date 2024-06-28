use leptos::*;
use monaco::sys::editor::{IEditorMinimapOptions, IStandaloneEditorConstructionOptions};
#[derive(Debug, Clone, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Languages {
    Javascript,
    Json,
}

#[component]
pub fn monaco_editor(
    node_id: &'static str,
    data_rs: ReadSignal<String>,
    data_ws: WriteSignal<String>,
    #[prop(default = Languages::Javascript)] language: Languages,
    #[prop(default = vec!["min-h-50"])] classes: Vec<&'static str>,
    #[prop(default = false)] _auto_complete: bool,
    #[prop(default = false)] _validation: bool,
    #[prop(default = false)] read_only: bool,
) -> impl IntoView {
    let editor_ref = create_node_ref::<html::Div>();
    let styling = classes.join(" ");
    create_effect(move |_| {
        if let Some(node) = editor_ref.get() {
            // node.set_inner_html("");
            logging::log!("No editor found, creating a new editor!");
            let editor_settings = IStandaloneEditorConstructionOptions::default();
            let minimap_settings = IEditorMinimapOptions::default();
            minimap_settings.set_enabled(Some(false));
            editor_settings.set_language(Some(language.to_string().as_str()));
            editor_settings.set_automatic_layout(Some(true));
            editor_settings.set_value(Some(data_rs.get().as_str()));
            editor_settings.set_render_final_newline(Some(true));
            editor_settings.set_read_only(Some(read_only));
            editor_settings.set_minimap(Some(&minimap_settings));
            let editor = monaco::api::CodeEditor::create(&node, Some(editor_settings));
            on_cleanup(move || drop(editor));
        }
    });
    view! {
        <div id={node_id} class={styling} node_ref=editor_ref on:change=move |event| {
            let new_data = event_target_value(&event);
            logging::log!("You entered some code!");
            data_ws.set(new_data);
        }>
        </div>
    }
}
