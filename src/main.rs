use crate::language::repr::Representation;
use dioxus::prelude::*;
use language::vm::VM;
use std::sync::Arc;

mod language;

fn main() {
    launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum SidebarTab {
    Protocols,
    Definitions,
    Structs,
    Enums,
    Examples,
    Tutorial,
}

#[component]
fn App() -> Element {
    let mut content = use_signal(String::new);
    let mut err = use_signal(String::new);
    let mut vm = use_signal(VM::default);
    let mut history = use_signal(Vec::<Arc<String>>::new);
    let mut history_idx = use_signal(|| None::<usize>);
    let mut has_err = use_signal(|| false);
    let mut sidebar_open = use_signal(|| false);
    let mut sidebar_pinned = use_signal(|| false);
    let mut active_tab = use_signal(|| SidebarTab::Examples);

    let mut eval = move |_| {
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
                    err.set(format!("{}\n--- OR ---\n{}", err1, err2));
                }
            }
        });
    };

    let mut prev = move |_| {
        let hist = history.read();
        if hist.is_empty() {
            return;
        }
        let idx = match *history_idx.read() {
            Some(i) if i == 0 => hist.len() - 1,
            Some(i) => i - 1,
            None => hist.len() - 1,
        };
        history_idx.set(Some(idx));
        content.set(hist[idx].to_string());
    };

    let mut next = move |_| {
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

    let keydown = move |evt: KeyboardEvent| {
        if evt.modifiers().ctrl() {
            match evt.key() {
                Key::Enter => eval(()),
                Key::ArrowUp => prev(()),
                Key::ArrowDown => next(()),
                _ => {}
            }
        } else if evt.key() == Key::F4 {
            eval(());
        }
    };

    let css = r#"
                .example-item:hover {
                    border-left-color: #58a6ff !important;
                    background: #1c2128 !important;
                }
                "#;
    rsx! {
        head {
            style {
                "{css}"
            }
        }
        div {
            style: "
                height: 100vh;
                background: #0d1117;
                color: #f0f6fc;
                font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', monospace;
                display: flex;
                overflow: hidden;
            ",

            // Sidebar
            div {
                style: format!("
                    position: fixed;
                    top: 0;
                    left: 0;
                    height: 100vh;
                    width: 320px;
                    background: #161b22;
                    border-right: 1px solid #21262d;
                    transform: translateX({});
                    transition: transform 0.2s ease;
                    z-index: 100;
                    display: flex;
                    flex-direction: column;
                ", if *sidebar_open.read() || *sidebar_pinned.read() { "0" } else { "-100%" }),
                onmouseenter: move |_| if !*sidebar_pinned.read() { sidebar_open.set(true) },
                onmouseleave: move |_| if !*sidebar_pinned.read() { sidebar_open.set(false) },

                // Sidebar header
                div {
                    style: "
                        padding: 1rem;
                        border-bottom: 1px solid #21262d;
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                    ",
                    h2 {
                        style: "font-size: 1rem; font-weight: 600; color: #f0f6fc; margin: 0;",
                        "Stackulator"
                    }
                    button {
                        style: format!("
                            background: none;
                            border: none;
                            color: {};
                            cursor: pointer;
                            padding: 0.25rem;
                            border-radius: 4px;
                        ", if *sidebar_pinned.read() { "#58a6ff" } else { "#8b949e" }),
                        onclick: move |_| {
                            let current = *sidebar_pinned.read();
                            sidebar_pinned.set(!current);
                        },
                        if *sidebar_pinned.read() { "📌" } else { "📍" }
                    }
                }

                // Sidebar tabs
                div {
                    style: "
                        display: flex;
                        border-bottom: 1px solid #21262d;
                        overflow-x: auto;
                    ",
                    for (tab, name) in [
                        (SidebarTab::Tutorial, "Tutorial"),
                        (SidebarTab::Examples, "Examples"),
                        (SidebarTab::Protocols, "Protocols"),
                        (SidebarTab::Definitions, "Defs"),
                        (SidebarTab::Structs, "Structs"),
                        (SidebarTab::Enums, "Enums"),
                    ] {
                        button {
                            key: "{name}",
                            style: format!("
                                flex: 1;
                                padding: 0.5rem;
                                font-size: 0.75rem;
                                font-weight: 500;
                                border: none;
                                cursor: pointer;
                                background: {};
                                color: {};
                                border-bottom: 2px solid {};
                            ",
                                if *active_tab.read() == tab { "#21262d" } else { "transparent" },
                                if *active_tab.read() == tab { "#f0f6fc" } else { "#8b949e" },
                                if *active_tab.read() == tab { "#58a6ff" } else { "transparent" }
                            ),
                            onclick: move |_| active_tab.set(tab),
                            "{name}"
                        }
                    }
                }

                // Sidebar content
                div {
                    style: "flex: 1; overflow-y: auto; padding: 1rem;",
                    match *active_tab.read() {
                        SidebarTab::Tutorial => rsx! { Tutorial {} },
                        SidebarTab::Examples => rsx! { Examples { content } },
                        SidebarTab::Protocols => rsx! { Protocols { vm } },
                        SidebarTab::Definitions => rsx! { Definitions { vm } },
                        SidebarTab::Structs => rsx! { Structs { vm } },
                        SidebarTab::Enums => rsx! { Enums { vm } },
                    }
                }
            }

            // Sidebar trigger area
            div {
                style: format!("
                    position: fixed;
                    top: 50%;
                    left: {};
                    width: 20px;
                    height: 60px;
                    background: #21262d;
                    border-radius: 0 8px 8px 0;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    cursor: pointer;
                    z-index: 101;
                    transform: translateY(-50%);
                    opacity: {};
                ",
                    if *sidebar_open.read() || *sidebar_pinned.read() { "320px" } else { "0" },
                    if *sidebar_open.read() || *sidebar_pinned.read() { "0.3" } else { "0.6" }
                ),
                onmouseenter: move |_| if !*sidebar_pinned.read() { sidebar_open.set(true) },
                "⋮"
            }

            // Main content
            div {
                style: format!("
                    flex: 1;
                    display: grid;
                    grid-template-columns: 1fr 400px;
                    gap: 1px;
                    background: #21262d;
                    margin-left: {};
                ", if *sidebar_pinned.read() { "320px" } else { "0" }),

                // Left panel - Code editor
                div {
                    style: "
                        background: #0d1117;
                        display: flex;
                        flex-direction: column;
                    ",

                    // Toolbar
                    div {
                        style: "
                            padding: 0.75rem;
                            background: #161b22;
                            border-bottom: 1px solid #21262d;
                            display: flex;
                            gap: 0.5rem;
                            align-items: center;
                        ",

                        button {
                            style: "
                                padding: 0.5rem 1rem;
                                background: #238636;
                                border: 1px solid #2ea043;
                                border-radius: 6px;
                                color: #fff;
                                font-size: 0.875rem;
                                font-weight: 500;
                                cursor: pointer;
                                display: flex;
                                align-items: center;
                                gap: 0.25rem;
                            ",
                            onclick: move |_| eval(()),
                            "▶ Run"
                        }

                        div {
                            style: "display: flex; gap: 0.25rem;",
                            button {
                                style: "
                                    padding: 0.5rem;
                                    background: #21262d;
                                    border: 1px solid #30363d;
                                    border-radius: 6px;
                                    color: #f0f6fc;
                                    cursor: pointer;
                                ",
                                onclick: move |_| prev(()),
                                "↑"
                            }
                            button {
                                style: "
                                    padding: 0.5rem;
                                    background: #21262d;
                                    border: 1px solid #30363d;
                                    border-radius: 6px;
                                    color: #f0f6fc;
                                    cursor: pointer;
                                ",
                                onclick: move |_| next(()),
                                "↓"
                            }
                        }

                        div {
                            style: "flex: 1; text-align: right; font-size: 0.75rem; color: #8b949e;",
                            "Ctrl+Enter to run • F4 to execute"
                        }
                    }

                    // Code editor
                    textarea {
                        style: "
                            flex: 1;
                            padding: 1rem;
                            background: #0d1117;
                            border: none;
                            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', monospace;
                            font-size: 14px;
                            line-height: 1.5;
                            color: #f0f6fc;
                            resize: none;
                            outline: none;
                        ",
                        placeholder: "Enter your code here...",
                        value: "{content}",
                        oninput: move |e| content.set(e.value()),
                        onkeydown: keydown,
                    }

                    // Error display
                    if *has_err.read() {
                        div {
                            style: "
                                padding: 1rem;
                                background: #2d1b1b;
                                border-top: 1px solid #f85149;
                                border-left: 4px solid #f85149;
                            ",
                            div {
                                style: "font-size: 0.75rem; color: #f85149; font-weight: 600; margin-bottom: 0.5rem;",
                                "⚠ Error"
                            }
                            pre {
                                style: "
                                    font-size: 0.75rem;
                                    font-family: monospace;
                                    color: #ffa198;
                                    white-space: pre-wrap;
                                    margin: 0;
                                    line-height: 1.4;
                                ",
                                "{err.read()}"
                            }
                        }
                    }
                }

                // Right panel - Stack
                div {
                    style: "
                        background: #0d1117;
                        display: flex;
                        flex-direction: column;
                    ",

                    // Stack header
                    div {
                        style: "
                            padding: 0.75rem;
                            background: #161b22;
                            border-bottom: 1px solid #21262d;
                            font-weight: 600;
                            font-size: 0.875rem;
                            color: #f0f6fc;
                        ",
                        "Stack"
                    }

                    // Stack content
                    div {
                        style: "flex: 1; overflow-y: auto;",
                        Stack { vm }
                    }
                }
            }
        }
    }
}

#[component]
fn Stack(vm: Signal<VM>) -> Element {
    let vm_ref = vm.read();

    if vm_ref.stack.is_empty() {
        return rsx! {
            div {
                style: "
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    height: 200px;
                    color: #6e7681;
                ",
                div { style: "font-size: 2rem; margin-bottom: 0.5rem;", "📭" }
                div { "Stack is empty" }
            }
        };
    }

    rsx! {
        div {
            style: "padding: 1rem;",
            div {
                style: "display: flex; flex-direction: column-reverse; gap: 0.5rem;",
                for (i, value) in vm_ref.stack.iter().enumerate() {
                    div {
                        key: "{i}",
                        style: "
                            padding: 0.75rem;
                            background: #161b22;
                            border: 1px solid #21262d;
                            border-radius: 6px;
                            border-left: 3px solid #58a6ff;
                        ",
                        div {
                            style: "
                                font-size: 0.75rem;
                                color: #8b949e;
                                margin-bottom: 0.5rem;
                                font-weight: 500;
                            ",
                            "#{i}"
                        }
                        div {
                            style: "
                                font-family: monospace;
                                font-size: 0.875rem;
                                color: #f0f6fc;
                                word-break: break-all;
                                line-height: 1.4;
                            ",
                            "{value.get_repr(&vm_ref.parse_ctx)}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Tutorial() -> Element {
    rsx! {
        div {
            style: "color: #f0f6fc; line-height: 1.6;",

            div {
                style: "margin-bottom: 1.5rem;",
                h3 {
                    style: "color: #58a6ff; font-size: 1rem; margin-bottom: 0.5rem;",
                    "Getting Started"
                }
                p {
                    style: "font-size: 0.875rem; color: #8b949e; margin-bottom: 1rem;",
                    "Stackulator is a stack-based calculator/programming language. Values are pushed onto a stack and operations consume values from the top."
                }
            }

            div {
                style: "margin-bottom: 1.5rem;",
                h3 {
                    style: "color: #58a6ff; font-size: 1rem; margin-bottom: 0.5rem;",
                    "Basic Operations"
                }
                div {
                    div {
                        style: "font-size: 0.75rem; margin-bottom: 0.5rem;",
                        code { style: "background: #21262d; padding: 0.25rem; border-radius: 3px;", "5 3 add" }
                        span { style: "color: #8b949e; margin-left: 0.5rem;", "→ pushes 8" }
                    }
                    div {
                        style: "font-size: 0.75rem; margin-bottom: 0.5rem;",
                        code { style: "background: #21262d; padding: 0.25rem; border-radius: 3px;", "true false and" }
                        span { style: "color: #8b949e; margin-left: 0.5rem;", "→ pushes false" }
                    }
                }
            }

            div {
                style: "margin-bottom: 1.5rem;",
                h3 {
                    style: "color: #58a6ff; font-size: 1rem; margin-bottom: 0.5rem;",
                    "Functions"
                }
                div {
                    style: "font-size: 0.75rem; margin-bottom: 0.5rem;",
                    code {
                        style: "background: #21262d; padding: 0.5rem; border-radius: 3px; display: block;",
                        "dup = |_x| {{_x _x}};"
                    }
                }
                p {
                    style: "font-size: 0.75rem; color: #8b949e;",
                    "Defines a function that duplicates the top stack value."
                }
            }

            div {
                style: "margin-bottom: 1.5rem;",
                h3 {
                    style: "color: #58a6ff; font-size: 1rem; margin-bottom: 0.5rem;",
                    "Data Structures"
                }
                div {
                    style: "font-size: 0.75rem; margin-bottom: 0.5rem;",
                    code {
                        style: "background: #21262d; padding: 0.5rem; border-radius: 3px; display: block;",
                        "List(1 2 3)"
                    }
                    p { style: "color: #8b949e; margin-top: 0.25rem;", "Creates a list" }
                }
                div {
                    style: "font-size: 0.75rem; margin-bottom: 0.5rem;",
                    code {
                        style: "background: #21262d; padding: 0.5rem; border-radius: 3px; display: block;",
                        "Set(1 2 3)"
                    }
                    p { style: "color: #8b949e; margin-top: 0.25rem;", "Creates a set" }
                }
            }

            div {
                h3 {
                    style: "color: #58a6ff; font-size: 1rem; margin-bottom: 0.5rem;",
                    "Pattern Matching"
                }
                div {
                    style: "font-size: 0.75rem;",
                    code {
                        style: "background: #21262d; padding: 0.5rem; border-radius: 3px; display: block; white-space: pre-wrap;",
                        "5 | _x => _x 2 add, "
                    }
                    p { style: "color: #8b949e; margin-top: 0.25rem;", "Matches value and adds 2" }
                }
            }
        }
    }
}

#[component]
fn Definitions(vm: Signal<VM>) -> Element {
    let vm_ref = vm.read();
    let defs = vm_ref.get_definitons();

    if defs.is_empty() {
        return rsx! {
            div {
                style: "text-center py-8 color: #6e7681;",
                div { style: "font-size: 1.5rem; margin-bottom: 0.5rem;", "⚙️" }
                div { "No definitions yet" }
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem;",
            for (name, body) in defs.iter() {
                div {
                    key: "{name}",
                    style: "
                        padding: 0.75rem;
                        background: #161b22;
                        border: 1px solid #21262d;
                        border-radius: 6px;
                        border-left: 3px solid #a5a5a5;
                    ",
                    div {
                        style: "font-weight: 600; color: #f0f6fc; margin-bottom: 0.5rem; font-size: 0.875rem;",
                        "{name}"
                    }
                    div {
                        style: "font-family: monospace; font-size: 0.75rem; color: #8b949e;",
                        for (i, item) in body.iter().enumerate() {
                            div { key: "{i}", "{item}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Structs(vm: Signal<VM>) -> Element {
    let vm_ref = vm.read();
    let structs = vm_ref.get_structs();

    if structs.is_empty() {
        return rsx! {
            div {
                style: "text-center py-8 color: #6e7681;",
                div { style: "font-size: 1.5rem; margin-bottom: 0.5rem;", "🏗️" }
                div { "No structs defined" }
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem;",
            for (name, body) in structs.iter() {
                div {
                    key: "{name}",
                    style: "
                        padding: 0.75rem;
                        background: #161b22;
                        border: 1px solid #21262d;
                        border-radius: 6px;
                        border-left: 3px solid #f79000;
                    ",
                    div {
                        style: "font-weight: 600; color: #f79000; margin-bottom: 0.5rem; font-size: 0.875rem;",
                        "struct {name}"
                    }
                    div {
                        style: "font-family: monospace; font-size: 0.75rem; color: #8b949e; margin-left: 0.5rem;",
                        for (i, field) in body.iter().enumerate() {
                            div { key: "{i}", "{field}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Enums(vm: Signal<VM>) -> Element {
    let vm_ref = vm.read();
    let enums = vm_ref.get_enums();

    if enums.is_empty() {
        return rsx! {
            div {
                style: "text-center py-8 color: #6e7681;",
                div { style: "font-size: 1.5rem; margin-bottom: 0.5rem;", "🔀" }
                div { "No enums defined" }
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem;",
            for (name, variants) in enums.iter() {
                div {
                    key: "{name}",
                    style: "
                        padding: 0.75rem;
                        background: #161b22;
                        border: 1px solid #21262d;
                        border-radius: 6px;
                        border-left: 3px solid #a855f7;
                    ",
                    div {
                        style: "font-weight: 600; color: #a855f7; margin-bottom: 0.5rem; font-size: 0.875rem;",
                        "enum {name}"
                    }
                    div {
                        style: "margin-left: 0.5rem;",
                        for (variant_name, variant_body) in variants.iter() {
                            div {
                                key: "{variant_name}",
                                style: "margin-bottom: 0.5rem;",
                                div {
                                    style: "color: #f79000; font-family: monospace; font-size: 0.75rem;",
                                    "| {variant_name}("
                                }
                                if !variant_body.is_empty() {
                                    div {
                                        style: "font-family: monospace; font-size: 0.75rem; color: #8b949e; margin-left: 1rem;",
                                        for (i, field) in variant_body.iter().enumerate() {
                                            div { key: "{i}", "{field}" }
                                        }
                                    }
                                }
                                div { style: "color: #8b949e; margin-left: 0.5rem;", ")" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Examples(content: Signal<String>) -> Element {
    let examples = [
        (
            "Basics",
            vec![
                ("Push Integers", "5 3 2"),
                ("Push Rationals", "5.2 0.3 1.2"),
                ("Add", "5 3 add"),
                ("Boolean Logic", "true false and"),
                ("Comparison", "5 3 ge"),
                ("Stack Operations", "1 2 3 stack_size"),
            ],
        ),
        (
            "Take Operations",
            vec![
                ("Simple Take", "5 10 |x y| { x y add }"),
                ("Multiple Values", "1 2 3 |a b c| { c b a }"),
                ("Variable Reuse", "42 |x| { x x x }"),
                ("Nested Take", "1 2 |x y| { x |z| { z y add } }"),
            ],
        ),
        (
            "Data Structures",
            vec![
                ("Create List", "List(1 2 3 4)"),
                ("List Operations", "List(1 2 3) 4 push"),
                ("List with Variables", "5 10 |x y| { List(x y x) }"),
                ("Create Set", "Set(1 2 3 2)"),
                ("Set Operations", "Set(1 2) Set(2 3) union"),
                ("List Access", "List(10 20 30) 1 get"),
            ],
        ),
        (
            "Functions",
            vec![
                ("Simple Function", "dup = |x| {x x};"),
                ("Using Function", "5 dup"),
                ("Swap Function", "swap = |x y| {y x};\n1 2 swap"),
                ("Drop Function", "drop = |x| {};\n42 drop"),
                ("Over Function", "over = |x y| {y x y};\n1 2 over"),
            ],
        ),
        (
            "Pattern Matching",
            vec![
                ("Simple Match", "5 | x => x 2 add,"),
                ("Literal Match", "7 | 5 => true,\n  | x => false,"),
                ("List Pattern", "List(1 2 3) | List(x y z) => x y add,"),
                ("Rest Pattern", "List(1 2 3 4) | List(x $rest) => rest,"),
                (
                    "Middle Pattern",
                    "List(1 2 3 4 5) | List(first $middle last) => middle,",
                ),
                ("Set Pattern", "Set(1 2 3) | Set(x $rest) => x,"),
                (
                    "Type Patterns",
                    "42 | Int(_) => true, 42.0 | Rat(_) => false,",
                ),
            ],
        ),
        (
            "Control Flow",
            vec![
                ("Conditional", "true ? { 42 }"),
                ("Simple Loop", "1 while dup 5 le { dup 1 add }"),
                (
                    "Break/Return",
                    "1 while true { dup 10 ge ? { break } dup 1 add }",
                ),
            ],
        ),
        (
            "Struct and Enums",
            vec![
                ("Struct Definition", "struct Point { Int Int };"),
                ("Struct Usage", "struct Point { Int Int };\nPoint(10 20);"),
                (
                    "Struct Pattern",
                    "struct Point { Int Int };\nPoint(5 10) | Point(x y) => x y add,",
                ),
                (
                    "Enum Definition",
                    "enum Option {\n  | Some('T)\n  | None()\n};",
                ),
                (
                    "Enum Usage",
                    "enum Option { | Some('T) | None() };\nOption::Some(42);",
                ),
                (
                    "Enum Pattern",
                    "enum Option { | Some('T) | None() };\nOption::Some(42) | Option::Some(x) => x,",
                ),
            ],
        ),
        (
            "Protocols",
            vec![
                (
                    "Some 'functions' can be used on different types",
                    "1 2 add \n0.1 0.3 add\n true false or\n 3 2 or\n",
                ),
                (
                    "Defining addition on a custom Type",
                    "struct Dummy{Int} ;

add(Dummy Dummy) = | Dummy(x) Dummy(y) => Dummy(x y add), ;

Dummy(1) Dummy(2) add;",
                ),
            ],
        ),
        (
            "Algorithms",
            vec![(
                "Fibonacci",
                "drop = |x| {};

drop2 = |x y| { };

dup = |x| {x x} ;

swap = |x y|{ y x};

fib_step = | x y z |{
  x y add
  x
  z 1 sub
 };

fib = | n |{ 1 1 n }
      while dup 1 eq not{
          fib_step
      }
      drop2 ;

0 while dup 100 leq  {
    1 add dup fib swap
} ;",
            )],
        ),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 1rem;",
            for (category, examples) in examples.iter() {
                div {
                    key: "{category}",
                    h4 {
                        style: "color: #58a6ff; font-size: 0.875rem; margin-bottom: 0.5rem; font-weight: 600;",
                        "{category}"
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 0.5rem;",
                        for (title, code) in examples.iter() {
                            div {
                                key: "{title}",
                                style: "
                                    padding: 0.75rem;
                                    background: #161b22;
                                    border: 1px solid #21262d;
                                    border-radius: 6px;
                                    cursor: pointer;
                                    border-left: 3px solid #21262d;
                                    transition: all 0.2s;
                                ",
                                class: "example-item",
                                onclick: {
                                    let code = code.to_string();
                                    move |_| content.set(code.clone())
                                },
                                onmouseenter: move |_| {},
                                onmouseleave: move |_|{},
                                div {
                                    style: "font-weight: 500; color: #f0f6fc; font-size: 0.75rem; margin-bottom: 0.5rem;",
                                    "{title}"
                                }
                                pre {
                                    style: "
                                        font-family: monospace;
                                        font-size: 0.75rem;
                                        background: #0d1117;
                                        padding: 0.5rem;
                                        border-radius: 4px;
                                        margin: 0;
                                        color: #8b949e;
                                        white-space: pre-wrap;
                                        line-height: 1.4;
                                    ",
                                    "{code}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Protocols(vm: Signal<VM>) -> Element {
    let vm_ref = vm.read();
    let defs = vm_ref.get_protocols();

    if defs.is_empty() {
        return rsx! {
            div {
                style: "text-center py-8 color: #6e7681;",
                div { style: "font-size: 1.5rem; margin-bottom: 0.5rem;", "⚙️" }
                div { "No definitions yet" }
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.75rem;",
            for (name, signatures) in defs.into_iter() {
                div {
                    key: "{name}",
                    style: "
                        padding: 0.75rem;
                        background: #161b22;
                        border: 1px solid #21262d;
                        border-radius: 6px;
                        border-left: 3px solid #a5a5a5;
                    ",
                    div {
                        style: "font-weight: 600; color: #f0f6fc; margin-bottom: 0.5rem; font-size: 0.875rem;",
                        "{name}"
                    }
                    for (i,signature) in signatures.into_iter().enumerate(){
                     div {
                        key: "{i}",
                        "{signature}"

                    }
                    }
                }
            }
        }
    }
}
