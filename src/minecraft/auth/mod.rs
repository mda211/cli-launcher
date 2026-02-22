use open;

const TOKEN_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";

pub mod server;
pub mod util;
pub mod web;

pub struct AuthResult {
    pub code: String,
    pub state: String,
    pub code_verifier: String,
}

pub async fn authenticate() -> Result<AuthResult, Box<dyn std::error::Error>> {
    let (auth_url, state, code_verifier) = web::construct_auth_url();

    open::that(&auth_url)?;
    let callback_params = server::start_auth_server().await?;
    if let Some(error) = callback_params.error {
        return Err(format!("Microsoft authentication error: {}", error).into());
    }

    let code = callback_params
        .code
        .ok_or("No authorization code received")?;
    let callback_state = callback_params.state.ok_or("No state parameter received")?;

    if callback_state != state {
        return Err("State mismatch".into());
    }

    println!("Authentication successful!");

    Ok(AuthResult {
        code,
        state: callback_state,
        code_verifier,
    })
}
