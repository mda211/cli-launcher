use crate::minecraft::launch::environment::{Arch, Environment, OS};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub struct Rule {
    pub action: RuleAction,
    #[serde(default)]
    pub os: Option<OperatingSystem>,
    #[serde(default)]
    pub features: Option<Features>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    Allow,
    Disallow,
}

#[derive(Debug, Deserialize)]
pub struct OperatingSystem {
    #[serde(default)]
    pub name: Option<OS>,
    #[serde(default)]
    pub arch: Option<Arch>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Features {
    #[serde(default)]
    pub is_demo_user: bool,
    #[serde(default)]
    pub has_custom_resolution: bool,
    #[serde(default)]
    pub has_quick_plays_support: bool,
    #[serde(default)]
    pub is_quick_play_singleplayer: bool,
    #[serde(default)]
    pub is_quick_play_multiplayer: bool,
    #[serde(default)]
    pub is_quick_play_realms: bool,
}

#[derive(Debug)]
pub struct ResolvedArguments {
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}

impl Rule {
    pub fn applies(&self, environment: &Environment, features: Option<&Features>) -> bool {
        if let Some(os_rule) = &self.os {
            if let Some(name) = &os_rule.name {
                if *name != environment.os {
                    return false;
                }
            }

            if let Some(arch) = &os_rule.arch {
                if *arch != environment.arch {
                    return false;
                }
            }
        }

        if let (Some(rule_features), Some(active_features)) = (&self.features, features) {
            if rule_features.is_demo_user && !active_features.is_demo_user {
                return false;
            }
            if rule_features.has_custom_resolution && !active_features.has_custom_resolution {
                return false;
            }
            if rule_features.has_quick_plays_support && !active_features.has_quick_plays_support {
                return false;
            }
            if rule_features.is_quick_play_singleplayer
                && !active_features.is_quick_play_singleplayer
            {
                return false;
            }
            if rule_features.is_quick_play_multiplayer && !active_features.is_quick_play_multiplayer
            {
                return false;
            }
            if rule_features.is_quick_play_realms && !active_features.is_quick_play_realms {
                return false;
            }
        }

        true
    }
}

pub fn rules_allow(rules: &[Rule], environment: &Environment, features: Option<&Features>) -> bool {
    if rules.is_empty() {
        return true;
    }

    let mut allowed = false;

    for rule in rules {
        if rule.applies(environment, features) {
            allowed = rule.action == RuleAction::Allow;
        }
    }

    allowed
}
