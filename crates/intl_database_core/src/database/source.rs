use thiserror::Error;

use crate::{KeySymbol, MessageMeta, MessageValue, SourceFileKind, SourceFileMeta};

#[derive(Debug, Error)]
pub enum MessageSourceError {
    #[error("Failed to parse message {0} source: {1}")]
    ParseError(SourceFileKind, String),
    #[error("Semantic restriction for definitions was violated: {0}")]
    DefinitionRestrictionViolated(String),
    #[error("Semantic restriction for translations was violated: {0}")]
    TranslationRestrictionViolated(String),
    #[error("Message {0} did not contain a message value")]
    NoMessageValue(KeySymbol),
    #[error("Source file meta descriptor is invalid")]
    InvalidSourceFileMeta,
    #[error("Message meta descriptor for message {0} is invalid")]
    InvalidMessageMeta(KeySymbol),
    #[error("Expected to encounter at least 1 definition in the source file, but none were found")]
    NoMessagesFound,
}

pub type MessageSourceResult<T> = Result<T, MessageSourceError>;

pub trait RawMessage {
    fn name(&self) -> KeySymbol;
}

pub struct RawMessageDefinition {
    pub name: KeySymbol,
    pub value: MessageValue,
    pub offset: u32,
    pub meta: MessageMeta,
}

impl RawMessageDefinition {
    pub fn new<V: AsRef<str>>(name: KeySymbol, offset: u32, value: V, meta: MessageMeta) -> Self {
        let value = MessageValue::from_raw(value.as_ref());
        Self {
            name,
            value,
            offset,
            meta,
        }
    }
}

impl RawMessage for RawMessageDefinition {
    fn name(&self) -> KeySymbol {
        self.name
    }
}

pub struct RawMessageTranslation {
    pub name: KeySymbol,
    pub offset: u32,
    pub value: MessageValue,
}

impl RawMessageTranslation {
    pub fn new<V: AsRef<str>>(name: KeySymbol, offset: u32, value: V) -> Self {
        let value = MessageValue::from_raw(value.as_ref());
        Self {
            name,
            offset,
            value,
        }
    }
}

impl RawMessage for RawMessageTranslation {
    fn name(&self) -> KeySymbol {
        self.name
    }
}

pub trait MessageDefinitionSource {
    /// Return an [`Iterator`] over all of the message definitions contained in the source file.
    /// Any kind of iterator is valid, so long as it yields complete [`RawMessageDefinition`]
    /// structs for the database to handle inserting and updating as needed.
    fn extract_definitions(
        self,
        file_name: KeySymbol,
        content: &str,
    ) -> MessageSourceResult<(
        SourceFileMeta,
        impl Iterator<Item = RawMessageDefinition> + '_,
    )>;
}

pub trait MessageTranslationSource {
    /// Return an [`Iterator`] over all of the message translations contained in the source file.
    /// Any kind of iterator is valid, so long as it yields complete [`RawMessageTranslation`]
    /// structs for the database to handle inserting and updating as needed.
    fn extract_translations(
        self,
        file_name: KeySymbol,
        content: &str,
    ) -> MessageSourceResult<impl Iterator<Item = RawMessageTranslation> + '_>;
}