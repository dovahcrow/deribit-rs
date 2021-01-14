Deribit API V2 Client for Rust Language
=================
[CI](https://github.com/dovahcrow/treerite/workflows/deribit-rs%20CI/badge.svg) [![Latest Version]][crates.io] [![Rustc Version nightly]][rustc] [![license]][license-content] [![keybase logo]][keybase] ![btc]

[Latest Version]: https://img.shields.io/crates/v/deribit.svg?style=flat-square
[crates.io]: https://crates.io/crates/deribit

[Rustc Version nightly]: https://img.shields.io/badge/rustc-nightly-lightgray.svg?style=flat-square
[rustc]: #

[license]: https://img.shields.io/crates/l/deribit.svg?style=flat-square
[license-content]: LICENSE

[keybase logo]: https://img.shields.io/keybase/pgp/dovahcrow.svg?style=flat-square
[keybase]: #

[btc]: https://img.shields.io/keybase/btc/dovahcrow.svg?style=flat-square

Use this library for trading at your own risk.

The current plan is to only implement the websocket communication, which includes call api through websocket 
and websocket subscription. I will first implement these APIs used for my own trading purpose, however, if you want some APIs 
to be prioritly implemented please open an issue or just throw me a PR (this is more welcome :P).

# Basic usage

```rust
// This will give you a Deribit instance, which the only purpose is to create connection.
let drb = deribit::DeribitBuilder::default().build().expect("Cannot create deribit client");

// "Deribit::connect" will connect to the deribit server with websocket as well as
// spin up a task in the backgroud polling message and dispatch them to subscription channel or RPC channel respectively.
// "Deribit::connect" returns a "(DeribitAPIClient, DeribitSubscriptionClient)" tuple, where
// the former is used for sending out RPC requests, and the later is used for receiving notifications.
let (mut client, mut subscription) = drb.connect().await?;

// All the request models reside in "deribit::models" module, with the
// naming convention of "camelCase(method)+Request", e.g. "/public/test" would be
// "TestRequest" in deribit-rs.
let req = deribit::models::TestRequest::default();

// Calls to deribit server is made by giving "DeribitAPIClient::call" the request object.
// The return type of "DeribitAPIClient::call" is "impl Future<Output=impl Future<Output=R>>", 
// where the first layer future denotes the send process, and the second one denotes the receive process. This brings
// fine grained control of the communication. The response type "R" depends on your request, with similar naming convention: 
// "TestRequest" will have "TestResponse".
let _ = client.call(req).await?.await?;

// Subscription is made by calling with "PublicSubscribeRequest" or "PrivateSubscribeRequest".
let req = PublicSubscribeRequest::new(&["book.BTC-PERPETUAL.raw".into()]);

// You can avoid the second ".await" to save some time - no worries, the request will still be received by the deribit server.
let _ = client.call(req).await?;

// In order to get your subscriptions, just poll the subscription stream. The "DeribitSubscriptionClient" implements the "futures::Stream" trait.
while let Some(message) = subscription.next().await {
    println!("Subscription message received {:?}", message);
}

```
# Implementation Status

- Authentication
    - [x] /public/auth
    - [x] /public/exchange_token
    - [x] /public/fork_token
    - [x] /private/logout
- Session Management
    - [x] /public/set_heartbeat
    - [x] /public/disable_heartbeat
    - [x] /private/enable_cancel_on_disconnect
    - [x] /private/disable_cancel_on_disconnect
    - [x] /private/get_cancel_on_disconnect
- Supporting
    - [x] /public/get_time
    - [x] /public/hello
    - [x] /public/test
- Subscription Management
    - [x] /public/subscribe
    - [x] /public/unsubscribe
    - [x] /private/subscribe
    - [x] /private/unsubscribe
- Account Management
    - [ ] /public/get_announcements
    - [ ] /private/change_api_key_name
    - [ ] /private/change_scope_in_api_key
    - [ ] /private/change_subaccount_name
    - [ ] /private/create_api_key
    - [ ] /private/create_subaccount
    - [ ] /private/disable_api_key
    - [ ] /private/disable_tfa_for_subaccount
    - [ ] /private/enable_api_key
    - [x] /private/get_account_summary
    - [ ] /private/get_email_language
    - [ ] /private/get_new_announcements
    - [ ] /private/get_position
    - [x] /private/get_positions
    - [x] /private/get_subaccounts
    - [ ] /private/list_api_keys
    - [ ] /private/remove_api_key
    - [ ] /private/reset_api_key
    - [ ] /private/set_announcement_as_read
    - [ ] /private/set_api_key_as_default
    - [ ] /private/set_email_for_subaccount
    - [ ] /private/set_email_language 
    - [ ] /private/set_password_for_subaccount
    - [ ] /private/toggle_notifications_from_subaccount
    - [ ] /private/toggle_subaccount_login
- Block Trading
    - [ ] /private/execute_block_trade
    - [ ] /private/get_block_trade
    - [ ] /private/get_last_block_trades_by_currency
    - [ ] /private/invalidate_block_trade_signature
    - [ ] /private/verify_block_trade
- Trading
    - [x] /private/buy
    - [x] /private/sell
    - [x] /private/edit
    - [x] /private/cancel
    - [x] /private/cancel_all
    - [x] /private/cancel_all_by_currency
    - [x] /private/cancel_all_by_instrument
    - [ ] /private/cancel_by_label
    - [ ] /private/close_position
    - [ ] /private/get_margins
    - [x] /private/get_open_orders_by_currency
    - [x] /private/get_open_orders_by_instrument
    - [ ] /private/get_order_history_by_currency
    - [ ] /private/get_order_history_by_instrument
    - [ ] /private/get_order_margin_by_ids
    - [x] /private/get_order_state
    - [ ] /private/get_stop_order_history
    - [ ] /private/get_user_trades_by_currency
    - [ ] /private/get_user_trades_by_currency_and_time
    - [ ] /private/get_user_trades_by_instrument
    - [ ] /private/get_user_trades_by_instrument_and_time
    - [ ] /private/get_user_trades_by_order
    - [ ] /private/get_settlement_history_by_instrument
    - [ ] /private/get_settlement_history_by_currency
- Market Data
    - [x] /public/get_book_summary_by_currency
    - [ ] /public/get_book_summary_by_instrument
    - [ ] /public/get_contract_size
    - [ ] /public/get_currencies
    - [ ] /public/get_funding_chart_data
    - [ ] /public/get_funding_rate_history
    - [ ] /public/get_funding_rate_value
    - [ ] /public/get_historical_volatility
    - [x] /public/get_index
    - [x] /public/get_instruments
    - [ ] /public/get_last_settlements_by_currency
    - [ ] /public/get_last_settlements_by_instrument
    - [ ] /public/get_last_trades_by_currency
    - [ ] /public/get_last_trades_by_currency_and_time
    - [ ] /public/get_last_trades_by_instrument
    - [ ] /public/get_last_trades_by_instrument_and_time
    - [ ] /public/get_order_book
    - [ ] /public/get_trade_volumes
    - [ ] /public/get_tradingview_chart_data
    - [ ] /public/ticker
- Wallet
    - [ ] /private/cancel_transfer_by_id
    - [ ] /private/cancel_withdrawal
    - [ ] /private/create_deposit_address
    - [ ] /private/get_current_deposit_address
    - [ ] /private/get_deposits
    - [x] /private/get_transfers
    - [ ] /private/get_withdrawals
    - [x] /private/submit_transfer_to_subaccount
    - [x] /private/submit_transfer_to_user
    - [x] /private/withdraw
- Subscriptions
    - [x] announcements
    - [x] book.{instrument_name}.{group}.{depth}.{interval}
    - [x] book.{instrument_name}.{interval}
    - [ ] chart.trades.{instrument_name}.{resolution}
    - [x] deribit_price_index.{index_name}
    - [x] deribit_price_ranking.{index_name}
    - [x] estimated_expiration_price.{index_name}
    - [x] markprice.options.{index_name}
    - [x] perpetual.{instrument_name}.{interval}
    - [ ] platform_state
    - [x] quote.{instrument_name}
    - [x] ticker.{instrument_name}.{interval}
    - [x] trades.{instrument_name}.{interval}
    - [ ] trades.{kind}.{currency}.{interval}
    - [ ] user.changes.{instrument_name}.{interval}
    - [ ] user.changes.{kind}.{currency}.{interval}
    - [x] user.orders.{instrument_name}.{interval}
    - [x] user.orders.{kind}.{currency}.{interval}
    - [x] user.portfolio.{currency}
    - [x] user.trades.{instrument_name}.{interval}
    - [x] user.trades.{kind}.{currency}.{interval}

# Donate

![donationqr](assets/donationqr.png)

16PeVqncfWoQ94M4pxnitkYnnW8agQBBZB
