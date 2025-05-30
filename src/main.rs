use crate::language::repr::Representation;
use dioxus::prelude::*;
use language::vm::VM;
use std::sync::Arc;

mod language;

fn main() {
    launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Stack,
    Definitions,
    Structs,
    Enums,
    Examples,
}

#[component]
fn App() -> Element {
    let mut content = use_signal(String::new);
    let mut err = use_signal(String::new);
    let mut vm = use_signal(VM::default);
    let mut history = use_signal(Vec::<Arc<String>>::new);
    let mut history_idx = use_signal(|| None::<usize>);
    let mut has_err = use_signal(|| false);
    let mut tab = use_signal(|| Tab::Stack);

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

    rsx! {
        div {
            style: "
                min-height: 100vh;
                background: linear-gradient(135deg, #667eea, #764ba2);
                color: white;
                font-family: system-ui, sans-serif;
                padding: 1rem;
            ",

            div {
                style: "max-width: 1200px; margin: 0 auto; display: grid; gap: 1rem; grid-template-columns: 1fr;",

                // Input
                div {
                    style: "display: flex; flex-direction: column; gap: 1rem;",

                    div {
                        style: "display: flex; gap: 0.5rem; flex-wrap: wrap;",
                        button {
                            style: "padding: 0.5rem 1rem; background: #059669; border-radius: 6px; border: none; color: white; font-size: 0.875rem; font-weight: 500; cursor: pointer;",
                            onclick: move |_| eval(()),
                            "⚡ Run"
                        }
                        button {
                            style: "padding: 0.5rem 1rem; background: #4b5563; border-radius: 6px; border: none; color: white; font-size: 0.875rem; cursor: pointer;",
                            onclick: move |_| prev(()),
                            "↑"
                        }
                        button {
                            style: "padding: 0.5rem 1rem; background: #4b5563; border-radius: 6px; border: none; color: white; font-size: 0.875rem; cursor: pointer;",
                            onclick: move |_| next(()),
                            "↓"
                        }
                    }

                    textarea {
                        style: "
                            width: 100%;
                            height: 12rem;
                            padding: 0.75rem;
                            background: rgba(0, 0, 0, 0.3);
                            border: 1px solid rgba(255, 255, 255, 0.2);
                            border-radius: 6px;
                            font-family: monospace;
                            font-size: 0.875rem;
                            resize: vertical;
                            outline: none;
                            color: white;
                        ",
                        placeholder: "Enter code... Ctrl+Enter to run",
                        value: "{content}",
                        oninput: move |e| content.set(e.value()),
                        onkeydown: keydown,
                    }

                    if *has_err.read() {
                        div {
                            style: "padding: 0.75rem; background: rgba(185, 28, 28, 0.5); border: 1px solid rgba(239, 68, 68, 0.5); border-radius: 6px;",
                            pre {
                                style: "font-size: 0.875rem; font-family: monospace; color: #fecaca; white-space: pre-wrap; margin: 0;",
                                "{err.read()}"
                            }
                        }
                    }
                }

                // Tabs
                div {
                    style: "background: rgba(255, 255, 255, 0.1); border-radius: 8px; overflow: hidden; backdrop-filter: blur(10px);",

                    nav {
                        style: "display: flex; background: rgba(0, 0, 0, 0.2); border-bottom: 1px solid rgba(255, 255, 255, 0.1); overflow-x: auto;",
                        for (t, icon, name) in [
                            (Tab::Stack, "📚", "Stack"),
                            (Tab::Definitions, "🔧", "Defs"),
                            (Tab::Structs, "🏗", "Structs"),
                            (Tab::Enums, "🔀", "Enums"),
                            (Tab::Examples, "💡", "Examples"),
                        ] {
                            button {
                                key: "{name}",
                                style: format!("
                                    flex: 1;
                                    padding: 0.5rem;
                                    font-size: 0.75rem;
                                    font-weight: 500;
                                    white-space: nowrap;
                                    border: none;
                                    cursor: pointer;
                                    background: {};
                                    color: {};
                                ", if *tab.read() == t { "rgba(255, 255, 255, 0.2)" } else { "transparent" },
                                   if *tab.read() == t { "#ffffff" } else { "rgba(255, 255, 255, 0.7)" }),
                                onclick: move |_| tab.set(t),
                                "{icon} {name}"
                            }
                        }
                    }

                    div {
                        style: "padding: 0.75rem; max-height: 24rem; overflow-y: auto;",
                        match *tab.read() {
                            Tab::Stack => rsx! { Stack { vm } },
                            Tab::Definitions => rsx! { Definitions { vm } },
                            Tab::Structs => rsx! { Structs { vm } },
                            Tab::Enums => rsx! { Enums { vm } },
                            Tab::Examples => rsx! { Examples { content } },
                        }
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
                style: "text-center py-8 text-white/60",
                div { style: "text-2xl mb-2", "📭" }
                "Stack empty"
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column-reverse; gap: 0.5rem;",
            for (i, value) in vm_ref.stack.iter().enumerate() {
                div {
                    key: "{i}",
                    style: "padding: 0.5rem; background: rgba(255, 255, 255, 0.1); border-radius: 4px; border-left: 2px solid #10b981;",
                    div {
                        style: "font-size: 0.75rem; color: rgba(255, 255, 255, 0.6); margin-bottom: 0.25rem;",
                        "{i}"
                    }
                    div {
                        style: "font-family: monospace; font-size: 0.875rem; word-break: break-all;",
                        "{value.get_repr(&vm_ref.parse_ctx)}"
                    }
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
                style: "text-center py-8 text-white/60",
                div { style: "text-2xl mb-2", "🔧" }
                "No definitions"
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.5rem;",
            for (name, body) in defs.iter() {
                div {
                    key: "{name}",
                    style: "padding: 0.5rem; background: rgba(255, 255, 255, 0.05); border-radius: 4px; border-left: 2px solid #3b82f6;",
                    div {
                        style: "font-weight: 600; color: #93c5fd; margin-bottom: 0.25rem;",
                        "{name}"
                    }
                    div {
                        style: "font-family: monospace; font-size: 0.75rem; color: rgba(255, 255, 255, 0.8);",
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
                style: "text-center py-8 text-white/60",
                div { style: "text-2xl mb-2", "🏗️" }
                "No structs"
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.5rem;",
            for (name, body) in structs.iter() {
                div {
                    key: "{name}",
                    style: "padding: 0.5rem; background: rgba(255, 255, 255, 0.05); border-radius: 4px; border-left: 2px solid #f59e0b;",
                    div {
                        style: "font-weight: 600; color: #fbbf24; margin-bottom: 0.25rem;",
                        "struct {name} {{"
                    }
                    div {
                        style: "font-family: monospace; font-size: 0.75rem; color: rgba(255, 255, 255, 0.8); margin-left: 0.5rem;",
                        for (i, field) in body.iter().enumerate() {
                            div { key: "{i}", "{field}" }
                        }
                    }
                    div { style: "color: rgba(255, 255, 255, 0.6);", "}}" }
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
                style: "text-center py-8 text-white/60",
                div { style: "text-2xl mb-2", "🔀" }
                "No enums"
            }
        };
    }

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.5rem;",
            for (name, variants) in enums.iter() {
                div {
                    key: "{name}",
                    style: "padding: 0.5rem; background: rgba(255, 255, 255, 0.05); border-radius: 4px; border-left: 2px solid #8b5cf6;",
                    div {
                        style: "font-weight: 600; color: #a78bfa; margin-bottom: 0.25rem;",
                        "enum {name} {{"
                    }
                    div {
                        style: "margin-left: 0.5rem;",
                        for (variant_name, variant_body) in variants.iter() {
                            div {
                                key: "{variant_name}",
                                style: "margin-bottom: 0.25rem;",
                                div {
                                    style: "color: #fbbf24; font-family: monospace; font-size: 0.875rem;",
                                    "| {variant_name}("
                                }
                                if !variant_body.is_empty() {
                                    div {
                                        style: "font-family: monospace; font-size: 0.75rem; color: rgba(255, 255, 255, 0.8); margin-left: 1rem;",
                                        for (i, field) in variant_body.iter().enumerate() {
                                            div { key: "{i}", "{field}" }
                                        }
                                    }
                                }
                                div { style: "color: rgba(255, 255, 255, 0.6); margin-left: 0.5rem;", ")" }
                            }
                        }
                    }
                    div { style: "color: rgba(255, 255, 255, 0.6);", "}}" }
                }
            }
        }
    }
}

#[component]
fn Examples(content: Signal<String>) -> Element {
    let examples = [
        ("And", "true false and"),
        ("Or", "true false or"),
        ("Not", "true not"),
        ("Add", "5 3 add"),
        ("Subtract", "5 3 sub"),
        ("Multiply", "5 3 mul"),
        ("Divide", "5 3 div"),
        ("Equality", "5 3 eq"),
        ("Greater", "5 3 ge"),
        ("Less", "5 3 le"),
        ("Greater or Equal", "5 3 geq"),
        ("Less or Equal", "5 3 leq"),
        ("List","List(1 3 4)"),
        ("List Push","List(1 3 4) 1 push"),
        ("List Pop","List(1 3 4) pop"),
        ("Stack length", "stack_size stack_size stack_size"),
        ("Quotation/Lambdas", "[1 2 add]"),
        ("Quotation/Lambdas Call", " 1 [ 2 add] apply"),
        ("Take", "1 2 | _x _y | {_y _x _y}"),
        ("Match suceeds", "1 | _x => _x 2 add, "),
        ("Match fails", "1 | 3 => _x 2 add, "),
        ("Match", "1 | 3 => _x 2 add,\n  | _x => _x _x add, "),
        (
            "Loops",
            "dup = |_x| = { _x _x };\n\n 1 while dup 10 le { dup 1 add  };  ",
        ),
        ("Function Defintion", "dup = |_x| {_x}"),
        ("Struct Defintion", "struct Pair{ Int 'X}"),
        ("Struct Instance", "struct Pair{ Int 'X};\nPair(1 Pair(2 3));"),
        (
            "Match Struct Instance",
            "struct Pair{ Int 'X};\nPair(1 Pair(2 3)) | Pair(_x _y) => _y,;",
        ),
        (
            "Enum Definition",
            "enum Either{
   | Left( 'A )
   | Right( 'B)
}",
        ),
        (
            "Enum Instance",
            "enum Either{
   | Left( 'A )
   | Right( 'B)
};\nEither::Left(1);",
        ),
        (
            "Match Enum Instance",
            "enum Either{
   | Left( 'A )
   | Right( 'B)
};\n\n Either::Left(1)\n\t | Either::Left(_x) => _x ",
        ),
        ("Protocol defintnion","struct Dummy{Int} ;

add(Dummy Dummy) = | Dummy(_x) Dummy(_y) => Dummy(_x _y add), ;

Dummy(1) Dummy(2) add;"),


        (
            "Fibonacci",
            "drop = |_x| {};
drop2 = |_x _y| { };

dup = |_x| {_x _x} ;

swap = |_x _y|{ _y _x};

fib_step = | _x _y _z |{
  _x _y add
  _x
  _z 1 sub
 };

fib = | _n |{ 1 1 _n }
      while dup 1 eq not{
          fib_step
      }
      drop2 ;

0 while dup 100 leq  {
    1 add dup fib swap
} ;
",
        ),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0.5rem;",
            for (title, code) in examples.iter() {
                div {
                    key: "{title}",
                    style: "padding: 0.5rem; background: rgba(255, 255, 255, 0.05); border-radius: 4px; border-left: 2px solid #06b6d4; cursor: pointer;",
                    onclick: {
                        let code = code.to_string();
                        move |_| content.set(code.clone())
                    },
                    div {
                        style: "font-weight: 600; color: #67e8f9; font-size: 0.875rem; margin-bottom: 0.25rem;",
                        "{title}"
                    }
                    div {
                        style: "font-family: monospace; font-size: 0.75rem; background: rgba(0, 0, 0, 0.3); padding: 0.25rem; border-radius: 3px;",
                        "{code}"
                    }
                }
            }
        }
    }
}
