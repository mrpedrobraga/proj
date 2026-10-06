use super::{
    Action, ActionDefinition, ActionDraft, ActionDraftReifier, ActionParameterDefinition,
    ActionSet,
    DraftError::{self, NoSuchParameter},
    Value, ValueType,
};
use ::std::collections::HashMap;

impl ActionSet {
    pub fn new() -> Self {
        Self {
            actions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: String, definition: ActionDefinition) {
        self.actions.insert(id, definition);
    }
}

impl Default for ActionSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionDefinition {
    pub fn new(
        label: String,
        description: String,
        parameters: &[ActionParameterDefinition],
        maker: ActionDraftReifier,
    ) -> Self {
        Self {
            label,
            description,
            parameters: parameters.to_vec(),
            maker,
        }
    }
}

impl std::fmt::Debug for ActionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionDefinition")
            .field("label", &self.label)
            .field("description", &self.description)
            .field("parameters", &self.parameters)
            .finish()
    }
}

impl ActionParameterDefinition {
    pub fn new(id: String, label: String, description: String, ty: ValueType) -> Self {
        Self {
            id,
            label,
            description,
            ty,
        }
    }
}

impl std::fmt::Debug for ActionParameterDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionParameterDefinition")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("description", &self.description)
            .field("ty", &self.ty)
            .finish()
    }
}

impl ActionDraft {
    pub fn new(label: String) -> Self {
        Self {
            label,
            parameters: HashMap::new(),
        }
    }

    /// Attempts to set the given parameter, failing with a descriptive error if the
    /// action or the parameter doesn't exist or the type doesn't match.
    pub fn set_parameter(
        &mut self,
        action_set: &ActionSet,
        id: String,
        value: Value,
    ) -> Result<(), DraftError> {
        let action_def = action_set
            .actions
            .get(&self.label)
            .ok_or(DraftError::NoSuchAction)?;

        let parameter_def = action_def
            .parameters
            .iter()
            .find(|p| p.id == id)
            .ok_or(NoSuchParameter)?;

        if !(parameter_def.ty == value.ty()) {
            return Err(DraftError::WrongParameterType);
        }

        self.set_parameter_unchecked(id, value);
        Ok(())
    }

    pub fn set_parameter_unchecked(&mut self, id: String, value: Value) {
        self.parameters.insert(id, value);
    }

    pub fn parameter(&self, id: String) -> Result<Value, DraftError> {
        self.parameters
            .get(&id)
            .ok_or(DraftError::NoSuchParameter)
            .cloned()
    }

    pub fn to_action(&self, action_set: &ActionSet) -> Result<Box<dyn Action>, DraftError> {
        let action_def = action_set
            .actions
            .get(&self.label)
            .ok_or(DraftError::NoSuchAction)?;

        (action_def.maker)(self)
    }
}

impl Value {
    /// Returns the value type of this value.
    pub fn ty(&self) -> ValueType {
        match self {
            Value::String(_) => ValueType::String,
            Value::Bool(_) => ValueType::Bool,
            Value::Integer(_) => ValueType::Integer,
            Value::Float(_) => ValueType::Float,
            Value::List(value_type, _) => ValueType::List(Box::new(value_type.clone())),
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
}
