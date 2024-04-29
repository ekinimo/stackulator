
use std::sync::Arc;

use iced::{
    executor,
    widget::{
        self, column, row, scrollable, text,
        text_editor::{self, Content},
        vertical_rule, vertical_space, container, horizontal_space,
    },
    Application, Command, Font, Settings, Theme,
};
use language::vm::VM;
use crate::language::repr::Representation;

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
            Message::Edit(text_editor::Action::Move(text_editor::Motion::PageUp)) => {
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
            }
            Message::Edit(text_editor::Action::Move(text_editor::Motion::PageDown)) => {
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
            }
            Message::Edit(text_editor::Action::Edit(text_editor::Edit::Enter)) => {
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
                let parse_result = self.vm.parse_snippet(&prog);
                match parse_result {
                    Ok(_) => {
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
                    Err(err) => {
                        self.has_err = true;
                        let path = err.path().unwrap_or("");
                        let line = err.line();
                        let err = err.to_string();
                        println!("path:\n{}\n line :\n {}\nerr :\n {}",&path,line ,&err,);
                        self.err
                            .perform(text_editor::Action::Edit(text_editor::Edit::Paste(
                                Arc::new(err),
                            )));
                    }
                }
                //self.content.perform(action);
            }
            Message::Edit(action) => {
                dbg!(&action);
                self.history_idx = None;
                self.content.perform(action);
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let input = if self.has_err {
            row!(
                iced::widget::text_editor(&self.content)
                    .on_action(Message::Edit)
                    .padding(10)
                    .font(Font::MONOSPACE),
                iced::widget::text_editor(&self.err)
                    .padding(10)
                    .font(Font::MONOSPACE),
            )
        } else {
            row!(iced::widget::text_editor(&self.content)
                .on_action(Message::Edit)
                .padding(10))
        };

        let value_stack = scrollable(
            widget::column(self.vm.stack.iter().map(|val| {
                container(text(val.get_repr(&self.vm.parse_ctx))).padding(10).into()
            }))
            .align_items(iced::Alignment::Center),
        );
        let definitions = scrollable(widget::column(self.vm.get_definitons().iter().map(
            |(name, body)| {
                row!(
                    text(name),
                    text(" = "),
                    widget::column(body.iter().map(|x| text(x).into()))
                )
                .into()
            },
        )));
        column![
            row![horizontal_space() , value_stack, vertical_rule(30), definitions],
            vertical_space(),
            input,
        ]
        .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())?;
    return Ok(());
}


