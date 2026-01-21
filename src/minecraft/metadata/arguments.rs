use serde::Deserialize;

#[derive(Debug)]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Simple(String),

    RuleBased {
        #[serde(default)]
        rules: Vec<Rule>,
        value: ArgValue,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ArgValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub action: String,

    #[serde(default)]
    pub features: Option<Features>,
}

#[derive(Debug, Deserialize)]
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

pub fn parse(json: &serde_json::Value) -> Result<Arguments, serde_json::Error> {
    #[derive(Deserialize)]
    struct Args {
        game: Vec<Argument>,
        jvm: Vec<Argument>,
    }

    let args: Args = serde_json::from_value(json["arguments"].clone())?;

    Ok(Arguments {
        game: args.game,
        jvm: args.jvm,
    })
}
