use crate::minecraft::metadata::arguments;
use crate::minecraft::metadata::{Environment, Features, ResolvedArguments};

pub fn send_arguments(
    arguments: &arguments::Arguments,
    environment: &Environment,
    features: &Features,
) -> ResolvedArguments {
    let game = resolve_argument_list(&arguments.game, &environment, &features);
    let jvm = resolve_argument_list(&arguments.jvm, &environment, &features);

    ResolvedArguments { game, jvm }
}

fn resolve_argument_list(
    list: &[arguments::Argument],
    environment: &Environment,
    features: &Features,
) -> Vec<String> {
    let mut out = Vec::new();

    for arg in list {
        match arg {
            arguments::Argument::Simple(s) => out.push(s.clone()),

            arguments::Argument::RuleBased { rules, value } => {
                let include = rules.iter().all(|r| r.matches(environment, features));

                if include {
                    match value {
                        arguments::ArgValue::Single(s) => out.push(s.clone()),
                        arguments::ArgValue::Multiple(v) => out.extend(v.clone()),
                    }
                }
            }
        }
    }
    out
}
