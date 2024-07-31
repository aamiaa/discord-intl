use std::ops::Deref;

use rustc_hash::FxHashSet;
use serde::Serialize;

use intl_markdown::{BlockNode, Document, Icu, InlineContent, TextOrPlaceholder};

use crate::messages::symbols::KeySymbolMap;

use super::{global_intern_string, KeySymbol, MessagesResult};

#[derive(Clone, Debug, Serialize, Hash, PartialEq, Eq)]
pub enum MessageVariableType {
    /// Any value is accepted for this variable. Generally used when the
    /// required type of the variable can't be determined.
    Any,
    /// Any type of numeric value is valid. Accepts both integers and floats.
    Number,
    /// A value used for a Plural evaluation. Generally a number, or something
    /// that can be directly cast to a number.
    Plural,
    /// A value that must match one of the defined values in this vec. Enums
    /// that support fallbacks are determined by the runtime, but most use the
    /// option `"other"` to represent that.
    Enum(Vec<String>),
    /// A Date type must be supplied. The runtime can decide whether the type
    /// can be parsed from a String or must be a Date object.
    Date,
    /// A Time type must be supplied. The runtime can decide whether the type
    /// can be parsed from a String or must be a specific Time object.
    Time,
    /// A function that provides some structured replacement of content,
    /// normally used for applying styles or injecting custom objects into the
    /// result string.
    HookFunction,
    /// A specialization of [MessageVariableType::HookFunction] that represents
    /// a Link, which requires specific handling in most cases.
    LinkFunction,
}

/// A representation of a single _instance_ of a variable in a message. Each
/// time a variable appears in a message, even if it is a variable that has
/// already been seen, a new MessageVariable is created.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MessageVariableInstance {
    /// The location in the message where this variable is used. Each instance
    /// of a variable in a string has its own struct, so each stores its own
    /// span as well.
    /// TODO: Add this back
    pub span: Option<usize>,
    /// The specific kind of the variable, used for generating types.
    pub kind: MessageVariableType,
}

#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct MessageVariables {
    variables: KeySymbolMap<Vec<MessageVariableInstance>>,
}

impl MessageVariables {
    pub fn new() -> Self {
        Self {
            variables: KeySymbolMap::default(),
        }
    }
    /// Add a new instance of a variable to the set of variables in a message.
    /// If this is the first instance of that variable, a new entry will be
    /// allocated for it, otherwise it will be appended to the list of
    /// instances for that name.
    pub fn add_instance(
        &mut self,
        name: KeySymbol,
        kind: MessageVariableType,
        span: Option<usize>,
    ) {
        let instance = MessageVariableInstance { kind, span };
        self.variables
            .entry(name)
            .or_insert_with(|| vec![])
            .push(instance);
    }

    /// Merge the variables from `other` into self by copying them over.
    pub fn merge(&mut self, other: &Self) {
        for (symbol, instances) in other.iter() {
            self.variables
                .entry(*symbol)
                .and_modify(|existing| existing.extend(instances.clone()))
                .or_insert(instances.clone());
        }
    }

    /// Returns a HashSet of the names of all variables in this message.
    pub fn get_keys(&self) -> FxHashSet<&KeySymbol> {
        self.variables.keys().collect::<FxHashSet<&KeySymbol>>()
    }

    /// Returns the count of _uniquely-named_ variables found in the message
    pub fn count(&self) -> usize {
        self.variables.len()
    }

    pub fn get(&self, key: &KeySymbol) -> Option<&Vec<MessageVariableInstance>> {
        self.variables.get(key)
    }
}

impl Deref for MessageVariables {
    type Target = KeySymbolMap<Vec<MessageVariableInstance>>;

    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}

pub struct MessageVariablesVisitor;

impl MessageVariablesVisitor {
    pub fn visit(ast: &Document, variables: &mut MessageVariables) -> MessagesResult<()> {
        for child in ast.blocks() {
            Self::visit_block(child, variables)?;
        }
        Ok(())
    }

    fn visit_block(block_node: &BlockNode, variables: &mut MessageVariables) -> MessagesResult<()> {
        match block_node {
            BlockNode::InlineContent(content) => Self::visit_inline_children(content, variables),
            BlockNode::Paragraph(paragraph) => {
                variables.add_instance(
                    global_intern_string("p")?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Self::visit_inline_children(paragraph.content(), variables)
            }
            BlockNode::Heading(heading) => {
                let heading_tag = format!("h{}", heading.level());
                variables.add_instance(
                    global_intern_string(&heading_tag)?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Self::visit_inline_children(heading.content(), variables)
            }
            // This presumes that code blocks can't contain variables, which _should_ always be true
            BlockNode::CodeBlock(_) => {
                variables.add_instance(
                    global_intern_string("codeBlock")?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Ok(())
            }
            BlockNode::ThematicBreak => {
                variables.add_instance(
                    global_intern_string("hr")?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Ok(())
            }
        }
    }

    fn visit_inline_children(
        content: &Vec<InlineContent>,
        variables: &mut MessageVariables,
    ) -> MessagesResult<()> {
        for child in content {
            Self::visit_inline_content(child, variables)?;
        }
        Ok(())
    }

    fn visit_inline_content(
        element: &InlineContent,
        variables: &mut MessageVariables,
    ) -> MessagesResult<()> {
        match element {
            InlineContent::Text(_) => Ok(()),
            // # is just a reference to an existing outer variable. It doesn't add anything new.
            // TODO: Make this add an instance of the outer variable.
            InlineContent::IcuPound => Ok(()),
            InlineContent::Icu(icu) => Self::visit_icu(icu, variables),
            // Everything else introduces a new tag directly before checking the inner content.
            InlineContent::Emphasis(emphasis) => {
                variables.add_instance(
                    global_intern_string("i")?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Self::visit_inline_children(emphasis.content(), variables)
            }
            InlineContent::Strong(strong) => {
                variables.add_instance(
                    global_intern_string("b")?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Self::visit_inline_children(strong.content(), variables)
            }
            InlineContent::Strikethrough(strikethrough) => {
                variables.add_instance(
                    global_intern_string("del")?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Self::visit_inline_children(strikethrough.content(), variables)
            }
            InlineContent::HardLineBreak => {
                variables.add_instance(
                    global_intern_string("br")?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Ok(())
            }
            InlineContent::CodeSpan(_) => {
                variables.add_instance(
                    global_intern_string("code")?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Ok(())
            }
            // Links and hooks introduce known variables.
            InlineContent::Hook(hook) => {
                variables.add_instance(
                    global_intern_string(hook.name())?,
                    MessageVariableType::HookFunction,
                    None,
                );
                Self::visit_inline_children(hook.content(), variables)
            }
            InlineContent::Link(link) => {
                variables.add_instance(
                    global_intern_string("link")?,
                    MessageVariableType::LinkFunction,
                    None,
                );
                Self::visit_inline_children(link.label(), variables)?;
                match link.destination() {
                    TextOrPlaceholder::Placeholder(icu) => Self::visit_icu(icu, variables),
                    _ => Ok(()),
                }
            }
        }
    }

    fn visit_icu(icu: &Icu, variables: &mut MessageVariables) -> MessagesResult<()> {
        match icu {
            Icu::IcuVariable(variable) => {
                variables.add_instance(
                    global_intern_string(variable.name())?,
                    MessageVariableType::Any,
                    None,
                );
                Ok(())
            }
            Icu::IcuPlural(plural) => {
                variables.add_instance(
                    global_intern_string(plural.name())?,
                    MessageVariableType::Plural,
                    None,
                );
                for arm in plural.arms() {
                    Self::visit_inline_children(arm.content(), variables)?;
                }
                Ok(())
            }
            Icu::IcuDate(date) => {
                variables.add_instance(
                    global_intern_string(date.name())?,
                    MessageVariableType::Date,
                    None,
                );
                Ok(())
            }
            Icu::IcuTime(time) => {
                variables.add_instance(
                    global_intern_string(time.name())?,
                    MessageVariableType::Time,
                    None,
                );
                Ok(())
            }
            Icu::IcuNumber(number) => {
                variables.add_instance(
                    global_intern_string(number.name())?,
                    MessageVariableType::Number,
                    None,
                );
                Ok(())
            }
        }
    }
}
