use super::util::{encode_base64, encode_url, generate_string, hash_sha256};

static AUTH_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
static CLIENT_ID: &str = "bed4de5f-cd1f-4dd1-bb40-45abc197dd61";
static REDIRECT_URI: &str = "http://localhost:8080/auth/callback";

pub fn construct_auth_url() -> (String, String, String) {
    let state = generate_string(43);
    let code_verifier = generate_string(128);

    let hash = hash_sha256(code_verifier.clone());
    let code_challenge = encode_base64(hash.as_slice());

    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope=XboxLive.signin%20offline_access&state={}&code_challenge={}&code_challenge_method=S256",
        AUTH_URL,
        CLIENT_ID,
        encode_url(REDIRECT_URI),
        encode_url(&state),
        encode_url(&code_challenge)
    );

    (auth_url, state, code_verifier)
}
