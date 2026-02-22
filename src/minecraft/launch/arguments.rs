use crate::config::Config;
use crate::minecraft::launch::environment;
use crate::minecraft::metadata::arguments;
use crate::minecraft::metadata::{Features, Metadata, ResolvedArguments, rules_allow};

pub fn send_arguments(arguments: &arguments::Arguments, features: &Features) -> ResolvedArguments {
    let game = resolve_argument_list(&arguments.game, &features);
    let jvm = resolve_argument_list(&arguments.jvm, &features);

    ResolvedArguments { game, jvm }
}

pub fn replace_variables(args: Vec<String>, config: &Config, metadata: &Metadata) -> Vec<String> {
    args.into_iter()
        .map(|arg| {
            let mut result = arg;

            result = result.replace("${auth_player_name}", &config.user.username);
            result = result.replace("${auth_uuid}", &config.user.uuid);
            result = result.replace("${version_name}", &config.user.version);
            result = result.replace("${game_directory}", &config.directories.instance);

            result = result.replace("${assets_index_name}", &metadata.asset_index.id);
            let version_type = metadata
                .r#type
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_else(|| "unknown".to_string());
            result = result.replace("${version_type}", &version_type);

            result
        })
        .collect()
}

fn resolve_argument_list(list: &[arguments::Argument], features: &Features) -> Vec<String> {
    let mut out = Vec::new();

    for arg in list {
        match arg {
            arguments::Argument::Simple(s) => out.push(s.clone()),

            arguments::Argument::RuleBased { rules, value } => {
                let include =
                    rules_allow(rules, &environment::Environment::detect(), Some(features));

                if include {
                    match value {
                        arguments::ArgValue::Single(s) => out.push(s.clone()),
                        arguments::ArgValue::Multiple(v) => out.extend(v.clone()),
                    }
                }
            }
        }
    }
    out.join(" ");

    out
}
