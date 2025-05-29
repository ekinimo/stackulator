use crate::language::repr::Representation;
use dioxus::prelude::*;
use language::vm::VM;
use std::sync::Arc;

mod language;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut content = use_signal(String::new);
    let mut err = use_signal(String::new);
    let mut vm = use_signal(VM::default);
    let mut history = use_signal(Vec::<Arc<String>>::new);
    let mut history_idx = use_signal(|| None::<usize>);
    let mut has_err = use_signal(|| false);

    let mut evaluate = move |_| {
        if *has_err.read() {
            err.set(String::new());
        }
        has_err.set(false);
        history_idx.set(None);

        let prog = content.read().clone();

        vm.with_mut(|vm| {
            let parse_snippet_result = vm.parse_snippet(&prog);
            let parse_program_result = vm.parse_full_program(&prog);

            match (parse_snippet_result, parse_program_result) {
                (Ok(_), _) | (_, Ok(_)) => {
                    let old_stack = vm.stack.clone();
                    match vm.eval() {
                        Ok(_) => {
                            history.with_mut(|h| h.push(Arc::new(prog)));
                            content.set(String::new());
                        }
                        Err(error) => {
                            has_err.set(true);
                            err.set(error.get_repr(&vm.parse_ctx));
                            vm.stack = old_stack;
                        }
                    }
                }
                (Err(err1), Err(err2)) => {
                    has_err.set(true);
                    err.set(format!(
                        "{}\n-*-*-*-*-*-*-*-*-*-*-*-*-*-* Or -*-*-*-*-*-*-*-*-*-*-*-*-*-*\n{}",
                        err1, err2
                    ));
                }
            }
        });
    };

    let mut history_back = move |_| {
        let hist = history.read();
        if hist.is_empty() {
            return;
        }

        let idx = match *history_idx.read() {
            Some(i) => {
                if i == 0 {
                    hist.len() - 1
                } else {
                    i - 1
                }
            }
            None => hist.len() - 1,
        };

        history_idx.set(Some(idx));
        content.set(hist[idx].to_string());
    };

    let mut history_forward = move |_| {
        let hist = history.read();
        if hist.is_empty() {
            return;
        }

        let idx = match *history_idx.read() {
            Some(i) => (i + 1) % hist.len(),
            None => 0,
        };

        history_idx.set(Some(idx));
        content.set(hist[idx].to_string());
    };

    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.modifiers().ctrl() {
            match evt.key() {
                Key::Enter => evaluate(()),
                Key::ArrowUp => history_back(()),
                Key::ArrowDown => history_forward(()),
                _ => {}
            }
        } else if evt.key() == Key::F4 {
            evaluate(());
        }
    };

    rsx! {
        div {
            class: "app-container",
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                background-color: #2b2b2b;
                color: #ffffff;
                font-family: 'Courier New', monospace;
            ",

            // Main content area
            div {
                class: "main-content",
                style: "
                    display: flex;
                    flex: 1;
                    gap: 20px;
                    padding: 20px;
                ",

                // Left spacer
                div { style: "flex: 1;" }

                // Value stack
                StackDisplay { vm: vm.cloned() }

                // Vertical separator
                div { style: "width: 2px; background-color: #555; margin: 0 10px;" }

                // Right panel (definitions, structs, enums)
                RightPanel { vm: vm.cloned() }

                // Right spacer
                div { style: "flex: 1;" }
            }

            // Horizontal separator
            div { style: "height: 2px; background-color: #555; margin: 20px;" }

            // Input area
            InputArea {
                content: content,
                err: err.read().clone(),
                has_err: *has_err.read(),
                on_keydown: handle_keydown,
            }

            // Control buttons
            ControlButtons {
                on_evaluate: evaluate,
                on_history_back: history_back,
                on_history_forward: history_forward,
            }
        }
    }
}

#[component]
fn StackDisplay(vm: ReadOnlySignal<VM>) -> Element {
    let vm_ref = vm.read();
    rsx! {
        div {
            class: "value-stack",
            style: "
                flex: 2;
                background-color: #3c3c3c;
                border-radius: 8px;
                padding: 15px;
                overflow-y: auto;
                max-height: 60vh;
            ",
            h3 {
                style: "margin-top: 0; text-align: center;",
                "Stack"
            }
            div {
                style: "display: flex; flex-direction: column-reverse; align-items: center;",
                for (i, value) in vm_ref.stack.iter().enumerate() {
                    div {
                        key: "{i}",
                        style: "
                            padding: 8px;
                            margin: 2px 0;
                            background-color: #4a4a4a;
                            border-radius: 4px;
                            width: 100%;
                            text-align: center;
                        ",
                        "{value.get_repr(&vm_ref.parse_ctx)}"
                    }
                }
            }
        }
    }
}

#[component]
fn RightPanel(vm: ReadOnlySignal<VM>) -> Element {
    rsx! {
        div {
            class: "right-panel",
            style: "flex: 3; display: flex; flex-direction: column; gap: 20px;",

            DefinitionsPanel { vm: vm }

            div { style: "height: 2px; background-color: #555; margin: 10px 0;" }

            // Structs and Enums container
            div {
                style: "display: flex; gap: 20px; flex: 1;",

                div { style: "flex: 1;" }

                StructsPanel { vm: vm }

                div { style: "width: 2px; background-color: #555; margin: 0 10px;" }

                EnumsPanel { vm: vm }

                div { style: "flex: 1;" }
            }
        }
    }
}

#[component]
fn DefinitionsPanel(vm: ReadOnlySignal<VM>) -> Element {
    let vm_ref = vm.read();
    let definitions = vm_ref.get_definitons();

    rsx! {
        div {
            class: "definitions",
            style: "
                background-color: #3c3c3c;
                border-radius: 8px;
                padding: 15px;
                overflow-y: auto;
                max-height: 20vh;
            ",
            h4 { style: "margin-top: 0;", "Definitions" }
            for (name, body) in definitions.iter() {
                div {
                    key: "{name}",
                    style: "margin-bottom: 10px;",
                    span { style: "font-weight: bold;", "{name}" }
                    span { " = " }
                    div {
                        style: "margin-left: 20px;",
                        for (idx, item) in body.iter().enumerate() {
                            div {
                                key: "{idx}",
                                style: "padding: 2px 0;",
                                "{item}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StructsPanel(vm: ReadOnlySignal<VM>) -> Element {
    let vm_ref = vm.read();
    let structs = vm_ref.get_structs();

    rsx!{
        div {
            class: "structs",
            style: "
                flex: 2;
                background-color: #3c3c3c;
                border-radius: 8px;
                padding: 15px;
                overflow-y: auto;
            ",
            h4 { style: "margin-top: 0;", "Structs" }
            for (name, body) in structs.iter() {
                div {
                    key: "{name}",
                    style: "margin-bottom: 15px;",
                    div {
                        span { style: "color: #ff6b6b;", "struct " }
                        span { style: "font-weight: bold;", "{name}" }
                        span { " {{" }
                    }
                    div {
                        style: "margin-left: 20px;",
                        for (idx, field) in body.iter().enumerate() {
                            div {
                                key: "{idx}",
                                style: "padding: 2px 0;",
                                "{field}"
                            }
                        }
                        div { "}}" }
                    }
                }
            }
        }
    }
}


#[component]
fn EnumsPanel(vm: ReadOnlySignal<VM>) -> Element {
    let vm_ref = vm.read();
    let enums = vm_ref.get_enums();

    rsx!{
        div {
            class: "enums",
            style: "
                flex: 2;
                background-color: #3c3c3c;
                border-radius: 8px;
                padding: 15px;
                overflow-y: auto;
            ",
            h4 { style: "margin-top: 0;", "Enums" }
            for (name, variants) in enums.iter() {
                div {
                    key: "{name}",
                    style: "margin-bottom: 15px;",
                    div {
                        span { style: "color: #4ecdc4;", "enum " }
                        span { style: "font-weight: bold;", "{name}" }
                        span { " {{" }
                    }
                    div {
                        style: "margin-left: 20px;",
                        for (variant_name, variant_body) in variants.iter() {
                            div {
                                key: "{variant_name}",
                                style: "padding: 2px 0;",
                                span { style: "color: #ffe66d;", "| " }
                                span { "{variant_name}" }
                                span { "(" }
                                div {
                                    style: "margin-left: 20px;",
                                    for (idx, field) in variant_body.iter().enumerate() {
                                        div {
                                            key: "{idx}",
                                            style: "padding: 1px 0;",
                                            "{field}"
                                        }
                                    }
                                }
                                span { ")" }
                            }
                        }
                        div { "}}" }
                    }
                }
            }
        }
    }
}

#[component]
fn InputArea(
    content: Signal<String>,
    err: String,
    has_err: bool,
    on_keydown: EventHandler<KeyboardEvent>,
) -> Element {
    rsx! {
        div {
            class: "input-area",
            style: "
                padding: 20px;
                background-color: #3c3c3c;
                display: flex;
                gap: 20px;
            ",

            // Input editor
            div {
                style: "flex: 1;",
                textarea {
                    style: "
                        width: 100%;
                        height: 120px;
                        background-color: #4a4a4a;
                        color: #ffffff;
                        border: 2px solid #666;
                        border-radius: 4px;
                        padding: 10px;
                        font-family: 'Courier New', monospace;
                        font-size: 14px;
                        resize: vertical;
                    ",
                    placeholder: "Enter your code here... (Ctrl+Enter or F4 to evaluate)",
                    value: "{content}",
                    oninput: move |evt| content.set(evt.value()),
                    onkeydown: on_keydown,
                }
            }

            // Error display
            if has_err {
                div {
                    style: "
                        flex: 1;
                        background-color: #5a2d2d;
                        border: 2px solid #aa4444;
                        border-radius: 4px;
                        padding: 10px;
                        font-family: 'Courier New', monospace;
                        font-size: 14px;
                        color: #ffaaaa;
                        overflow-y: auto;
                        max-height: 120px;
                        white-space: pre-wrap;
                    ",
                    "{err}"
                }
            }
        }
    }
}

#[component]
fn ControlButtons(
    on_evaluate: EventHandler<()>,
    on_history_back: EventHandler<()>,
    on_history_forward: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "controls",
            style: "
                padding: 10px 20px;
                background-color: #2b2b2b;
                display: flex;
                gap: 10px;
                justify-content: center;
            ",
            button {
                style: "
                    background-color: #4a90e2;
                    color: white;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    font-family: inherit;
                ",
                onclick: move |_| on_evaluate.call(()),
                "Evaluate (Ctrl+Enter / F4)"
            }
            button {
                style: "
                    background-color: #666;
                    color: white;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    font-family: inherit;
                ",
                onclick: move |_| on_history_back.call(()),
                "← History (Ctrl+↑)"
            }
            button {
                style: "
                    background-color: #666;
                    color: white;
                    border: none;
                    padding: 8px 16px;
                    border-radius: 4px;
                    cursor: pointer;
                    font-family: inherit;
                ",
                onclick: move |_| on_history_forward.call(()),
                "History → (Ctrl+↓)"
            }
        }
    }
}
