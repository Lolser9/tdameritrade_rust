// Just So Config Doesn't Give An Error
fn main() {}

// Return Client Id
pub fn client_id() -> String {
    return "client_id@AMER.OAUTHAP".to_string();
}

// Return Account Id
pub fn acct_id() -> i64 {
    return 0;
}

// Return Redirect URI
pub fn redirect_uri() -> String {
    return "redirect_uri".into();
}

// Return Token Path
pub fn token_path() -> String {
    return "token_path".into();
}

// Return Chrome Driver Path
pub fn chromedriver_path() -> String {
    return "chrome_driver_path".to_string();
}
