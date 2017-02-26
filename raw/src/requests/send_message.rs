use std::ops::Not;

use types::chat::*;
use types::primitive::*;
use types::message::*;
use requests::_base::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendMessage<'c, 's> {
    pub chat_id: ChatId<'c>,
    pub text: &'s str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'c, 's> SendMessage<'c, 's> {
    pub fn new<C>(chat: C, text: &'s str) -> Self where C: Into<ChatId<'c>> {
        SendMessage {
            chat_id: chat.into(),
            text: text,
            parse_mode: None,
            disable_web_page_preview: false,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn disable_web_page_preview(mut self) -> Self { // TODO(knsd): Rename to disable_preview?
        self.disable_web_page_preview = true;
        self
    }

    pub fn disable_notification(mut self) -> Self {
        self.disable_notification = true;
        self
    }

    pub fn reply_to<R>(mut self, to: R) -> Self where R: Into<ReplyTo> {
        self.reply_to_message_id = Some(to.into().0);
        self
    }

    pub fn reply_markup(mut self, reply_markup: ReplyMarkup) -> Self { // TODO(knsd): nice builder?
        self.reply_markup = Some(reply_markup);
        self
    }
}

impl Chat {
    pub fn text<'c, 's>(&'c self, text: &'s str) -> SendMessage<'c, 's> {
        SendMessage::new(self, text)
    }
}

macro_rules! send_chat_type {
    ($chat: ident) => {
        impl $chat {
            pub fn text<'c, 's>(&'c self, text: &'s str) -> SendMessage<'c, 's> {
                SendMessage::new(self, text)
            }
        }
    };
}

send_chat_type!(User);
send_chat_type!(Group);
send_chat_type!(Supergroup);
send_chat_type!(Channel);

impl Message {
    pub fn text_reply<'c, 's>(&'c self, text: &'s str) -> SendMessage<'c, 's> {
        self.chat.text(text).reply_to(self)
    }
}