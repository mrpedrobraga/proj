use ::std::{collections::HashMap, hash::Hash};

pub mod implementations;

#[derive(Debug)]
pub struct ActionSet {
    actions: HashMap<String, ActionDefinition>
}

pub struct ActionDefinition {
    label: String,
    description: String,
    parameters: Vec<ActionParameterDefinition>,
    maker: ActionDraftReifier,
}

pub type ActionDraftReifier = Box<dyn Fn(&ActionDraft) -> Result<Box<dyn Action>, DraftError>>;

#[derive(Clone)]
pub struct ActionParameterDefinition {
    id: String,
    label: String,
    description: String,
    ty: ValueType
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueType {
    String,
    Bool,
    Integer,
    Float,
    List(Box<ValueType>)
}

impl Hash for ActionDefinition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Bool(bool),
    Integer(i32),
    Float(f32),
    List(ValueType, Vec<Value>)
}

#[derive(Debug, Clone)]
pub struct ActionDraft {
    label: String,
    parameters: HashMap<String, Value>,
}

#[derive(Debug, Clone, Copy)]
pub enum DraftError {
    NoSuchAction,
    NoSuchParameter,
    WrongParameterType,
    Incomplete,
}

pub trait Action {
    fn execute(&self);
}