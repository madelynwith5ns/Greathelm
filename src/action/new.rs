use crate::term::*;

use super::Action;

/**
 * Built-in (io.github.greathelm.greathelm:Script) action to run a script in the current
 * project.
 */
pub struct NewAction {}
impl NewAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for NewAction {
    fn get_name(&self) -> String {
        "New".into()
    }
    fn get_aliases(&self) -> Vec<String> {
        vec!["new".into()]
    }
    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.greathelm.greathelm".into(),
            identifier: "New".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        match state.cli_args.get(2) {
            Some(v) => {
                crate::template::generate_from_template(&state.manifest, v.to_owned());
            }
            None => {
                error!("Please provide a template identifier.");
            }
        }
    }
}
