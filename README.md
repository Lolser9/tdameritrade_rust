# tdameritrade_rust

An unofficial rust library for [TD Ameritrade's API](https://developer.tdameritrade.com/apis)

## Features
- tdameritrade_rust supports both sync/async
- tdameritrade_rust automatically handles authentification
- tdameritrade_rust eases creating token file from TD Ameritrade

## Installation

Add this to your Cargo.toml

```toml
[dependencies]
tdameritrade_rust = "0.1.3"
```

## Getting Started

- Register at the [TD Ameritrade API Website](https://developer.tdameritrade.com/apis)
- Go to the My Apps section and add a new app
- Click on your app and copy the consumer key
- Click on the details section of your app and copy the callback url
- Download [chromedriver](https://chromedriver.chromium.org/downloads)
- Copy code below, replace_fields, run, and login when prompted to receive a token file from TD Ameritrade
```
use tdameritrade_rust::init;

fn main() {
    // Create Token File
    init::create_token_file(
        "chrome_driver_path".into(), // Path To Chromedriver
        "client_id@AMER.OAUTHAP".into(), // Client Id (Consumer Key)
        "redirect_uri".into(), // Redirect URI (Callback URL)
        "token_file_path".into(), // Where To Put Token File After Completion 
    )
}
```

## Synchronous
- After creating the token file, create a TD Ameritrade Client to access the API endpoints. Here's an example with the synchronous client
```
use tdameritrade_rust::{
    output::quotes::{QuoteType, Quotes},
    SyncTDAClient, TDAClientError,
};

fn main() -> Result<(), TDAClientError> {
    // Create Synchronous TDAClient
    let mut client = SyncTDAClient::new(
        "client_id@AMER.OAUTHAP".into(), // Client Id (Consumer Key)
        "redirect_uri".into(), // Redirect URI (Callback URL)
        "token_file_path".into(), // Token File Location
    );

    // Get Quote
    let symbol = "AAPL";
    let res = client.get_quote("AAPL")?;
    let res_json = serde_json::from_str::<Quotes>(&res)?;

    if let QuoteType::Equity(equity) = &res_json.symbol[symbol] {
        println!("{}", equity.close_price);
    }

    // Get Quotes
    let symbols = vec!["AAPL", "AMZN", "AMD", "NVDA"];
    let res = client.get_quotes(&symbols)?;
    let res_json = serde_json::from_str::<Quotes>(&res)?;

    for symbol in symbols.into_iter() {
        if let QuoteType::Equity(equity) = &res_json.symbol[symbol] {
            println!("{}", equity.close_price)
        }
    }

    Ok(())
}
```

## Asynchronous
- After creating the token file, create a TD Ameritrade Client to access the API endpoints. Here's an example with the asynchronous client
```
use tdameritrade_rust::{
    output::quotes::{QuoteType, Quotes},
    AsyncTDAClient, TDAClientError,
};

#[tokio::main]
async fn main() -> Result<(), TDAClientError> {
    // Create Asynchronous TDAClient
    let mut client = AsyncTDAClient::new(
        "client_id@AMER.OAUTHAP".into(), // Client Id (Consumer Key)
        "redirect_uri".into(), // Redirect URI (Callback URL)
        "token_file_path".into(), // Token File Location
    );

    // Get Quote
    let symbol = "AAPL";
    let res = client.get_quote(symbol).await?;
    let res_json = serde_json::from_str::<Quotes>(&res)?;

    if let QuoteType::Equity(equity) = &res_json.symbol[symbol] {
        println!("{}", equity.close_price);
    }

    // Get Quotes
    let symbols = vec!["AAPL", "AMZN", "AMD", "NVDA"];
    let res = client.get_quotes(&symbols).await?;
    let res_json = serde_json::from_str::<Quotes>(&res)?;

    for symbol in symbols.into_iter() {
        if let QuoteType::Equity(equity) = &res_json.symbol[symbol] {
            println!("{}", equity.close_price)
        }
    }

    Ok(())
}
```

## Future Plans

- I plan to make the watchlist and order endpoints easier to use
- I plan to decode output into custom structs instead of relying on serde_json::Value
- I plan to support streaming 

## Disclaimer

tdameritrade_rust is released under the [MIT license](https://github.com/Lolser9/tdameritrade_rust-async/blob/main/LICENSE.md)

tdameritrade_rust is an unofficial API wrapper. It is in no way endorsed by or affiliated with TD Ameritrade or any associated organization. The authors will accept no responsibility for any damage that might stem from use of this package. See the LICENSE file for more details.
