use crate::{
    generator::ProjectGenerator,
    identify::NamespacedIdentifier,
    term::{error, info, ok},
};

use super::Action;

pub struct InitAction {}
impl InitAction {
    pub fn create() -> Self {
        Self {}
    }
}

impl Action for InitAction {
    fn get_name(&self) -> String {
        "Initialize".into()
    }

    fn get_aliases(&self) -> Vec<String> {
        vec!["init".into()]
    }

    fn get_identifier(&self) -> crate::identify::NamespacedIdentifier {
        crate::identify::NamespacedIdentifier {
            namespace: "io.github.madelynwith5ns.greathelm".into(),
            identifier: "Initialize".into(),
        }
    }

    fn execute(&self, state: &crate::state::GreathelmState) {
        let cdir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => {
                error(format!("Current directory is invalid."));
                return;
            }
        };

        let project_name: String = state.manifest.get_string_property(
            "project-name",
            match cdir.file_name() {
                Some(v) => match v.to_str() {
                    Some(v) => v,
                    None => "current-dir",
                },
                None => "current-dir",
            },
        );

        let project_type = state
            .manifest
            .get_string_property("project-type", "io.github.madelynwith5ns.greathelm:Custom");

        let mut use_generator: Option<&Box<dyn ProjectGenerator>> = None;
        let namespaced = NamespacedIdentifier::parse_text(&project_type);
        for g in &state.generators {
            if g.get_aliases().contains(&project_type.to_lowercase()) {
                if use_generator.is_some() {
                    error(format!(
                        "Generator name \"{}\" is ambiguous in your configuration.",
                        project_type
                    ));
                    error(format!(
                        "Please specify which one you would like to use on the command line"
                    ));
                    error(format!(
                        "like so: --project-type=<full.namespaced:Identifier>"
                    ));
                    std::process::exit(1);
                } else {
                    use_generator = Some(g);
                }
            } else if namespaced.namespace != "unnamespaced" && g.get_identifier() == namespaced {
                use_generator = Some(g);
            }
        }

        match use_generator {
            Some(generator) => {
                info(format!(
                    "Initializing current directory as Greathelm project \"{}\"",
                    project_name
                ));

                generator.generate(std::env::current_dir().unwrap());

                if generator.should_make_ibht_stub() {
                    info(format!(
                        "Generator requested an IBHT stub. Writing IBHT stub..."
                    ));
                    match std::fs::write("IBHT.ghd", "\n") {
                        Ok(_) => {
                            ok(format!("Blank IBHT has been written successfully."));
                        }
                        Err(e) => {
                            error(format!("Failed to write a blank IBHT. Error is below:"));
                            eprintln!("{}", e);
                        }
                    };
                }
            }
            None => {
                error(format!(
                    "Could not find requested generator \"{project_type}\""
                ));
                error(format!("Are you missing a plugin?"));
            }
        }
    }
}