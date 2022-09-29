use tdameritrade_rust::init;
mod config;

fn main() {
    // Create Token File
    init::create_token_file(
        config::chromedriver_path(),
        config::client_id(),
        config::redirect_uri(),
        config::token_path(),
    );
}
