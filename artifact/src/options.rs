use bitcoincore_rpc::{Auth, Client};
use std::env;
use std::path::PathBuf;

pub struct BitcoinRpcOptions {
    pub rpc_url: String,
    pub rpc_user: Option<String>,
    pub rpc_pass: Option<String>,
    pub cookie_file: Option<PathBuf>,
}

impl BitcoinRpcOptions {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Self {
            rpc_url: env::var("BITCOIN_RPC_URL").unwrap_or_else(|_| "http://localhost:8332".to_string()),
            rpc_user: env::var("BITCOIN_RPC_USER").ok(),
            rpc_pass: env::var("BITCOIN_RPC_PASS").ok(),
            cookie_file: env::var("BITCOIN_RPC_COOKIE_FILE").ok().map(PathBuf::from),
        }
    }

    pub fn create_rpc_client(&self) -> bitcoincore_rpc::Result<Client> {
        let auth = match (&self.rpc_user, &self.rpc_pass, &self.cookie_file) {
            (Some(user), Some(pass), _) => Auth::UserPass(user.clone(), pass.clone()),
            (_, _, Some(cookie_file)) => Auth::CookieFile(cookie_file.clone()),
            _ => return Err(bitcoincore_rpc::Error::ReturnedError("Missing authentication data for RPC client".into())),
        };

        Client::new(&self.rpc_url, auth)
    }
}
