use std::sync::Arc;

use crate::language::repr::Representation;
use iced::{
    event::listen_with,
    executor,
    keyboard::{self, key::Named, Key},
    widget::{
        self, column, container, horizontal_rule, horizontal_space, row, scrollable, text,
        text_editor::{self, Content},
        vertical_rule, vertical_space,
    },
    Application, Command, Event, Font, Settings, Theme,
};
use language::vm::VM;

mod language;

#[derive(Default)]
struct App {
    content: Content,
    err: Content,
    vm: VM,
    history: Vec<Arc<String>>,
    history_idx: Option<usize>,
    has_err: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Evaluate,
    HistoryBack,
    HistoryForward,
}
impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Stackulator".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::HistoryForward => {
                let idx;
                (self.history_idx, idx) = match self.history_idx {
                    Some(i) => {
                        let r = (i + 1) % self.history.len();
                        (Some(r), r)
                    }
                    None => (Some(0), 0),
                };
                self.content
                    .perform(text_editor::Action::Move(text_editor::Motion::DocumentEnd));
                self.content.perform(text_editor::Action::Select(
                    text_editor::Motion::DocumentStart,
                ));
                self.content
                    .perform(text_editor::Action::Edit(text_editor::Edit::Delete));
                let prog = &self.history[idx];
                self.content
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        prog.to_owned(),
                    )));
                Command::none()
            }
            Message::HistoryBack => {
                let idx;
                (self.history_idx, idx) = match self.history_idx {
                    Some(i) => {
                        let val = if i == 0 {
                            self.history.len() - 1
                        } else {
                            i - 1
                        };
                        (Some(val), val)
                    }
                    None => (Some(self.history.len() - 1), self.history.len() - 1),
                };
                self.content
                    .perform(text_editor::Action::Move(text_editor::Motion::DocumentEnd));
                self.content.perform(text_editor::Action::Select(
                    text_editor::Motion::DocumentStart,
                ));
                self.content
                    .perform(text_editor::Action::Edit(text_editor::Edit::Delete));
                let prog = &self.history[idx];
                self.content
                    .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                        prog.to_owned(),
                    )));
                Command::none()
            }
            Message::Evaluate => {
                if self.has_err {
                    self.err
                        .perform(text_editor::Action::Move(text_editor::Motion::DocumentEnd));
                    self.err.perform(text_editor::Action::Select(
                        text_editor::Motion::DocumentStart,
                    ));
                    self.err
                        .perform(text_editor::Action::Edit(text_editor::Edit::Delete));
                }
                self.has_err = false;
                self.history_idx = None;

                let prog = self.content.text();
                let parse_snippet_result = self.vm.parse_snippet(&prog);
                let parse_program_result = self.vm.parse_full_program(&prog);
                match (parse_snippet_result, parse_program_result) {
                    (Ok(_), _) | (_, Ok(_)) => {
                        let old_stack = self.vm.stack.clone();
                        match self.vm.eval() {
                            Ok(_) => (),
                            Err(err) => {
                                self.has_err = true;
                                let err = err.get_repr(&self.vm.parse_ctx);
                                self.err.perform(text_editor::Action::Edit(
                                    text_editor::Edit::Paste(Arc::new(err)),
                                ));
                                self.vm.stack = old_stack;
                            }
                        };

                        self.history.push(Arc::new(prog));
                        self.content
                            .perform(text_editor::Action::Move(text_editor::Motion::DocumentEnd));
                        self.content.perform(text_editor::Action::Select(
                            text_editor::Motion::DocumentStart,
                        ));

                        self.content
                            .perform(text_editor::Action::Edit(text_editor::Edit::Delete));
                    }
                    (Err(err1), Err(err2)) => {
                        self.has_err = true;
                        let err = (err1.to_string(), err2.to_string());
                        println!("nerr :\n {} \n or \n {},", &err.0, &err.1,);
                        self.err
                            .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                                Arc::new(format!("{}\n-*-*-*-*-*-*-*-*-*-*-*-*-*-* Or -*-*-*-*-*-*-*-*-*-*-*-*-*-*\n{},",&err.0,&err.1)
                               )),
                            ));
                    }
                }
                Command::none()
                //self.content.perform(action);
            }
            Message::Edit(action) => {
                //dbg!(&action);
                //self.history_idx = None;
                self.content.perform(action);
                Command::none()
            }
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        listen_with(|event, _| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if modifiers.control() =>
            {
                match key {
                    Key::Named(Named::Enter) => Some(Message::Evaluate),
                    Key::Named(Named::ArrowUp) => Some(Message::HistoryBack),
                    Key::Named(Named::ArrowDown) => Some(Message::HistoryForward),
                    _ => None,
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::F4),
                ..
            }) => Some(Message::Evaluate),
            _ => None,
        })
    }
    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let input = container(if self.has_err {
            row!(
                container(
                    iced::widget::text_editor(&self.content)
                        .on_action(Message::Edit)
                        .padding(10)
                        .font(Font::MONOSPACE)
                ),
                container(
                    iced::widget::text_editor(&self.err)
                        .padding(10)
                        .font(Font::MONOSPACE)
                ),
            )
        } else {
            row!(container(
                iced::widget::text_editor(&self.content)
                    .on_action(Message::Edit)
                    .padding(10)
            ))
        });

        let value_stack = container(scrollable(
            widget::column(self.vm.stack.iter().rev().map(|val| {
                scrollable(container(text(val.get_repr(&self.vm.parse_ctx))).padding(10)).into()
            }))
            .align_items(iced::Alignment::Center),
        ));
        let definitions = container(scrollable(widget::column(
            self.vm.get_definitons().iter().map(|(name, body)| {
                row!(
                    container(text(name)),
                    container(text(" = ")),
                    scrollable(widget::column(
                        body.iter().map(|x| container(text(x)).padding(10).into())
                    ))
                )
                .into()
            }),
        )));
        let structs = container(scrollable(widget::column(
            self.vm.get_structs().iter().map(|(name, body)| {
                row!(
                    container(text(" struct ")),
                    container(text(name)),
                    container(text("{ ")),
                    scrollable(widget::column(
                        body.iter().map(|x| container(text(x)).padding(10).into())
                            .chain(Some( container(text("}")).into()).into_iter())
                    )),


                )
                    .into()
            }),
        )));

        let enums = container(scrollable(widget::column(
            self.vm.get_enums().iter().map(|(name, body)| {
                row!(
                    container(text(" enum ")),
                    container(text(name)),
                    container(text(" { ")),
                    scrollable(widget::column(
                        body.iter().map(|(x,body)|
                                        row![
                                            container(text("| ")),
                                            container(text(x)),
                                            container(text("(")),
                                            scrollable(widget::column(
                                                    body.iter().map(|x| container(text(x))
                                                                    .padding(10).into())
                                                    .chain(Some( container(text(")")).into()).into_iter())

                                                )),

                                        ].padding(10).into())

                            .chain(Some( container(text("}")).into()).into_iter())

                    )

                    )
                )
                    .into()
            }),
        )));


        column![
            row![
                horizontal_space(),
                value_stack,
                vertical_rule(30),
                column![
                    vertical_space(),
                    definitions,
                    horizontal_rule(20),
                    row![
                        horizontal_space(),

                        structs,
                        vertical_rule(20),
                        enums,
                        horizontal_space(),

                    ]
                 ],
                horizontal_space(),
            ],
            horizontal_rule(30),
            input,
        ]
        .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())?;
    Ok(())
}
