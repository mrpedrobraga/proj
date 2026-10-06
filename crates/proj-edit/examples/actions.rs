use ::proj_edit::actions::{
    Action, ActionDefinition, ActionDraft, ActionParameterDefinition, ActionSet, DraftError, Value,
    ValueType,
};

fn main() {
    let mut action_set = ActionSet::new();

    struct GreetAction {
        whom: String,
    }

    impl Action for GreetAction {
        fn execute(&self) {
            println!("Hello, {}! Welcome!", self.whom);
        }
    }

    action_set.insert(
        String::from("greet_person"),
        ActionDefinition::new(
            String::from("Greet {person}!"),
            String::from("Prints out a greeting message for {person}."),
            &[ActionParameterDefinition::new(
                String::from("person"),
                String::from("person"),
                String::from("The person that should be greeted."),
                ValueType::String,
            )],
            Box::new(|draft| {
                let whom = draft
                    .parameter(String::from("person"))?
                    .as_string()
                    .ok_or(DraftError::WrongParameterType)?
                    .clone();

                Ok(Box::new(GreetAction { whom }))
            }),
        ),
    );
    let action_set = action_set;

    let mut draft = ActionDraft::new(String::from("greet_person"));
    draft
        .set_parameter(
            &action_set,
            String::from("person"),
            Value::String(String::from("Pedro Braga")),
        )
        .unwrap();
    let action = draft.to_action(&action_set).unwrap();

    dbg!(action_set);
    dbg!(draft);

    action.execute();
}
