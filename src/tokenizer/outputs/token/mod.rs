mod attributes;

pub use self::attributes::*;
use super::AttributeView;
use crate::base::Bytes;

#[derive(Getters, Debug)]
pub struct CharacterToken<'i> {
    #[get = "pub"]
    text: Bytes<'i>,
}

#[derive(Getters, Debug)]
pub struct CommentToken<'i> {
    #[get = "pub"]
    text: Bytes<'i>,
}

#[derive(Getters, Debug)]
pub struct StartTagToken<'i> {
    #[get = "pub"]
    name: Bytes<'i>,

    #[get = "pub"]
    attributes: Attributes<'i>,

    self_closing: bool,
}

impl<'i> StartTagToken<'i> {
    #[inline]
    pub fn self_closing(&self) -> bool {
        self.self_closing
    }
}

#[derive(Getters, Debug)]
pub struct EndTagToken<'i> {
    #[get = "pub"]
    name: Bytes<'i>,
}

#[derive(Debug)]
pub struct DoctypeToken<'i> {
    name: Option<Bytes<'i>>,
    public_id: Option<Bytes<'i>>,
    system_id: Option<Bytes<'i>>,
    force_quirks: bool,
}

impl<'i> DoctypeToken<'i> {
    #[inline]
    pub fn name(&self) -> Option<&Bytes<'i>> {
        self.name.as_ref()
    }

    #[inline]
    pub fn public_id(&self) -> Option<&Bytes<'i>> {
        self.public_id.as_ref()
    }

    #[inline]
    pub fn system_id(&self) -> Option<&Bytes<'i>> {
        self.system_id.as_ref()
    }

    #[inline]
    pub fn force_quirks(&self) -> bool {
        self.force_quirks
    }
}

#[derive(Debug)]
pub enum Token<'i> {
    Character(CharacterToken<'i>),
    Comment(CommentToken<'i>),
    StartTag(StartTagToken<'i>),
    EndTag(EndTagToken<'i>),
    Doctype(DoctypeToken<'i>),
    Eof,
}

impl<'i> Token<'i> {
    pub fn new_character(text: Bytes<'i>) -> Self {
        Token::Character(CharacterToken { text })
    }

    pub fn new_comment(text: Bytes<'i>) -> Self {
        Token::Comment(CommentToken { text })
    }

    pub fn new_start_tag(
        name: Bytes<'i>,
        attributes: ParsedAttributeList<'i>,
        self_closing: bool,
    ) -> Self {
        Token::StartTag(StartTagToken {
            name,
            attributes: Box::new(attributes),
            self_closing,
        })
    }

    pub fn new_end_tag(name: Bytes<'i>) -> Self {
        Token::EndTag(EndTagToken { name })
    }

    pub fn new_doctype(
        name: Option<Bytes<'i>>,
        public_id: Option<Bytes<'i>>,
        system_id: Option<Bytes<'i>>,
        force_quirks: bool,
    ) -> Self {
        Token::Doctype(DoctypeToken {
            name,
            public_id,
            system_id,
            force_quirks,
        })
    }
}