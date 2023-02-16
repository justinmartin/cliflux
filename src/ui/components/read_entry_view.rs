use tuirealm::{Props, MockComponent, event::{KeyEvent, Key, KeyModifiers}, Component, State, StateValue, tui::widgets::Paragraph, command::{Cmd, CmdResult}, Event, Sub, SubClause};

use crate::{libminiflux::{FeedEntry, ReadStatus}, ui::{ComponentIds, Message}};
use unicode_segmentation::UnicodeSegmentation;
use stringreader::StringReader;

pub struct ReadEntryView {
    props: Props,
    entry: Option<FeedEntry>
}

impl ReadEntryView {
    pub fn new(entry: Option<FeedEntry>) -> Self {
        return Self {
            props: Props::default(),
            entry
        }
    }

    pub fn subscriptions() -> Vec<Sub<ComponentIds, KeyEvent>> {
        return vec![
            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('q'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('b'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('u'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                tuirealm::SubEventClause::Keyboard(KeyEvent {
                    code: Key::Char('o'),
                    modifiers: KeyModifiers::NONE
                }), 
                SubClause::Always
            ),

            Sub::new(
                tuirealm::SubEventClause::Tick,
                SubClause::Always
            )
        ]
    }

    fn format_entry_text(entry: &FeedEntry) -> String {
        return [
            entry.title.to_owned(),
            "-".repeat(entry.title.graphemes(true).count()),
            html2text::from_read(
                StringReader::new(&entry.content), 
                120
            )
        ].join("\n")
    }
}

impl MockComponent for ReadEntryView {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        if self.entry.is_some() {
            frame.render_widget(
                Paragraph::new(
                    ReadEntryView::format_entry_text(&self.entry.to_owned().unwrap())
                ),
                area
            )
        }
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        return self.props.get(attr);
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        match attr {
            tuirealm::Attribute::Content => {
                let unwrapped = value.clone().unwrap_string();
                let new_entry = serde_json::from_str::<FeedEntry>(&unwrapped).unwrap();
                self.entry = Some(new_entry)
            }
            _ => {}
        }
        return self.props.set(attr, value);
    }

    fn state(&self) -> tuirealm::State {
        match &self.entry {
            Some(e) => State::One(StateValue::String(ReadEntryView::format_entry_text(&e))),
            None => State::None
        }
    }

    fn perform(&mut self, cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        match cmd {
            Cmd::Custom("back") => {
                self.entry = None;
                return CmdResult::Custom("back");
            }
            Cmd::Custom("mark_as_unread") => {
                return CmdResult::Custom("mark_as_unread");
            }
            Cmd::Custom("open_in_browser") => {
                if let Some(e) = &self.entry {
                    let _ = open::that(&e.url);
                }
                return CmdResult::Custom("open_in_browser");
            }
            _ => CmdResult::None
        }
    }
}

impl Component<Message, KeyEvent> for ReadEntryView {
    fn on(&mut self, ev: tuirealm::Event<KeyEvent>) -> Option<Message> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                ..
            }) => Cmd::Custom("back"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('u'),
                ..
            }) => Cmd::Custom("mark_as_unread"),

            Event::Keyboard(KeyEvent {
                code: Key::Char('o'),
                ..
            }) => Cmd::Custom("open_in_browser"),
            _ => Cmd::None
        };

        match self.perform(cmd) {
            CmdResult::Custom("back") => {
                return Some(Message::ReadEntryViewClosed)
            },
            CmdResult::Custom("mark_as_unread") => {
                match &mut self.entry {
                    Some(e) => {
                        e.status = ReadStatus::Unread;
                        return Some(Message::ChangeEntryReadStatus(e.id, ReadStatus::Unread))
                    }
                    None => None
                }
            }
            CmdResult::Changed(_) => Some(Message::Tick),
            _ => None
        }
    }
}
