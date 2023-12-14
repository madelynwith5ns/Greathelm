use std::{collections::HashMap, path::PathBuf, str::FromStr};

use action::Action;
use builder::ProjectBuilder;
use generator::ProjectGenerator;
use identify::NamespacedIdentifier;
use state::GreathelmState;

use crate::term::error;

mod action;
mod builder;
mod config;
mod generator;
mod ibht;
mod identify;
mod manifest;
mod module;
mod plugin;
mod script;
mod state;
mod term;

fn main() {
    if std::env::args().len() <= 1 {
        println!("Usage: greathelm <action> [args]");
        std::process::exit(0);
    }
    let args: Vec<String> = std::env::args().collect();

    let mut action = match args.get(1) {
        Some(arg) => arg.clone(),
        None => {
            panic!("Insufficient arguments but check was bypassed. How?")
        }
    };
    let mut flags = HashMap::new();

    if args.len() >= 3 {
        for arg in &args {
            if arg.starts_with("--") && arg.contains("=") {
                let flag: String = arg.split_once("--").unwrap().1.into();
                let flag: String = flag.split_once("=").unwrap().0.into();
                let value: String = arg.split_once("=").unwrap().1.into();
                flags.insert(flag, value);
            }
        }
    }

    config::ensure_config_dirs();

    let mut builders: Vec<Box<dyn ProjectBuilder>> = Vec::new();
    let mut generators: Vec<Box<dyn ProjectGenerator>> = Vec::new();
    let mut manifest = manifest::ProjectManifest::new();
    let mut actions: Vec<Box<dyn Action>> = Vec::new();

    // user manifest
    let path = PathBuf::from_str(
        format!(
            "{}/UserManifest.ghm",
            config::get_config_base_dir().to_str().unwrap()
        )
        .as_str(),
    )
    .unwrap();
    if path.exists() {
        manifest.read_and_append(&path);
    }
    let path = PathBuf::from_str("Project.ghm").unwrap();
    if path.exists() {
        manifest.read_and_append(&path);
    }
    let path = PathBuf::from_str("Project.local.ghm").unwrap();
    if path.exists() {
        manifest.read_and_append(&path);
    }

    for f in flags.keys() {
        (&mut manifest)
            .properties
            .insert(f.clone(), flags.get(f).unwrap().clone());
    }

    let mut pt = match manifest.properties.get("Project-Type") {
        Some(t) => t.clone(),
        None => "%".into(),
    };
    if pt != "" {
        for a in manifest.aliases.keys() {
            if &pt == a {
                pt = manifest.aliases.get(a).unwrap().clone();
            }
        }
    }
    manifest.properties.insert("Project-Type".into(), pt);

    // builtins
    builders.push(Box::new(builder::c::CBuilder::create()));
    builders.push(Box::new(builder::cpp::CPPBuilder::create()));
    builders.push(Box::new(builder::custom::CustomBuilder::create()));

    generators.push(Box::new(generator::c::CGenerator::create()));
    generators.push(Box::new(generator::cpp::CPPGenerator::create()));
    generators.push(Box::new(generator::custom::CustomGenerator::create()));

    actions.push(Box::new(action::init::InitAction::create()));
    actions.push(Box::new(action::build::BuildAction::create()));
    actions.push(Box::new(action::script::ScriptAction::create()));

    // load plugins here..
    let plugins = plugin::load_plugins();
    for plugin in plugins {
        for b in plugin.builders {
            builders.push(b);
        }
        for g in plugin.generators {
            generators.push(g);
        }
        for a in plugin.actions {
            actions.push(a);
        }
    }

    let state = GreathelmState {
        builders: builders,
        generators: generators,
        manifest: manifest,
        actions: actions,
        cli_args: std::env::args().collect(),
    };

    for a in state.manifest.aliases.keys() {
        if &action == a {
            action = state.manifest.aliases.get(a).unwrap().clone();
        }
    }
    let mut use_action: Option<&Box<dyn Action>> = None;
    let namespaced = NamespacedIdentifier::parse_text(&action);
    for a in &state.actions {
        if a.get_aliases().contains(&action.to_lowercase()) {
            if use_action.is_some() {
                error(format!(
                    "Action name \"{}\" is ambiguous in your configuration.",
                    action
                ));
                error(format!("Please specify which one you would like to use."));
                error(format!("Like so: greathelm <full.namespaced:Identifier>"));
                std::process::exit(1);
            } else {
                use_action = Some(a);
            }
        } else if namespaced.namespace != "unnamespaced" && a.get_identifier() == namespaced {
            use_action = Some(a);
        }
    }

    match use_action {
        Some(a) => {
            a.execute(&state);
        }
        None => {
            error(format!("Action {action} could not be resolved."));
            error(format!("Are you missing a plugin?"));
        }
    }
}
