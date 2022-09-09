use rhai::{packages::Package, Engine, EvalAltResult, ImmutableString};
use rhai_env::EnvPackage;
use std::env;

#[test]
fn test_get_env() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(EnvPackage::new().as_shared_module());

    env::set_var("TEST", "hello");

    let variable_result = engine.eval::<rhai::ImmutableString>("get_env(\"TEST\")")?;

    let compare: ImmutableString = "hello".into();
    assert!(compare == variable_result);

    Ok(())
}

#[test]
fn test_set_env() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(EnvPackage::new().as_shared_module());

    engine.eval::<()>("set_env(\"TEST\", \"hello\")")?;

    let compare: ImmutableString = env::var("TEST").expect("").into();
    let expected_result: ImmutableString = "hello".into();
    assert!(compare == expected_result);

    Ok(())
}