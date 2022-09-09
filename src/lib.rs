use rhai::def_package;
use rhai::plugin::*;
use rhai::{EvalAltResult, Position, ImmutableString};
use std::env;

def_package! {
    /// Package for read and write env variables
    pub EnvPackage(lib) {
        combine_with_exported_module!(lib, "env", env_functions);
    }
}

#[export_module]
mod env_functions{
    #[rhai_fn(name = "get_env", return_raw)]
    pub fn get_env(key: &str) -> Result<ImmutableString, Box<EvalAltResult>> {
        if key.is_empty() {
            Err(EvalAltResult::ErrorVariableNotFound(
                format!("Env variable not found: {:?}", key),
                Position::NONE,
            )
            .into())
        } else {
            Ok(env::var(key).expect("").into())
        }
    }

    #[rhai_fn(name = "set_env")]
    pub fn set_env(key: &str, val: ImmutableString) {
        env::set_var(key, val.to_string());
    }
}