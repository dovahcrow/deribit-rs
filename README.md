Deribit API V2 Client for Rust Language
=================

[![Build Status]][travis] [![Latest Version]][crates.io] [![Rustc Version nightly]][rustc] [![license]][license-content] [![keybase logo]][keybase] ![btc]

[Build Status]: https://img.shields.io/travis/dovahcrow/deribit-rs.svg?style=flat-square
[travis]: https://travis-ci.org/dovahcrow/deribit-rs

[Latest Version]: https://img.shields.io/crates/v/deribit.svg?style=flat-square
[crates.io]: https://crates.io/crates/deribit

[Rustc Version nightly]: https://img.shields.io/badge/rustc-nightly-lightgray.svg?style=flat-square
[rustc]: #

[license]: https://img.shields.io/crates/l/deribit.svg?style=flat-square
[license-content]: LICENSE

[keybase logo]: https://img.shields.io/keybase/pgp/dovahcrow.svg?style=flat-square
[keybase]: #

[btc]: https://img.shields.io/keybase/btc/dovahcrow.svg?style=flat-square

This is experimental and currently under heavy development. Use at your own risk.

This library uses Rust nightly features extensively like async/await and futures api. You need a nightly Rust to make it run.

The current plan is to only implement the websocket communication, which includes call api through websocket 
and websocket subscription. I will first implement these APIs used for my own trading purpose, however, if you want some APIs 
to be prioritly implemented please open an issue or just throw me a PR (this is more welcome :P).

# Basic usage

```
    // This will gives you a Deribit instance, which the only purpose is creating connection.
    let drb = deribit::DeribitBuilder::default().build().expect("Cannot create deribit client");

    // "Deribit::connect" will connect deribit server with websocket as well as
    // spin up a task in the backgroud polling message from the websocket
    // and dispatch them to subscription channel or RPC channel respectively.
    // "Deribit::connect" returns a "(DeribitAPIClient, DeribitSubscriptionClient)" tuple, where
    // the former is used to send out RPC requests, and the later is used to receive notifications.
    let (mut client, mut subscription) = drb.connect().await?;

    // All the request models reside in "deribit::models" module, with the
    // naming convention of "camelCase(method)+Request", e.g. "/public/test" would be
    // "TestRequest" in deribit-rs.
    let req = deribit::models::TestRequest::default();

    // Calls to deribit server is made by giving "DeribitAPIClient::call" the request object.
    // The return type of "DeribitAPIClient::call" is "impl Future<Output=impl Future<Output=R>>", 
    // where the first layer future denotes the send process, and the second one denotes the receive process. This brings
    // find grained control of the communication. The response type R depends on your request, with similar naming convention: 
    // "TestRequest" will have "TestResponse".
    let _ = client.call(req).await?.await?;
    
    // Subscription is made by calling with "PublicSubscribeRequest" or "PrivateSubscribeRequest".
    let req = PublicSubscribeRequest::new(&["book.BTC-PERPETUAL.raw".into()]);

    // You can avoid the second ".await" to save some time - no worries, the request will still be received by the deribit server.
    let _ = client.call(req).await?;

    // In order to get your subscriptions, just poll the subscription stream. The "DeribitSubscriptionClient" implements the "Stream" trait.
    while let Some(message) = subscription.next().await {
        println!("Subscription message received {:?}", message);
    }

```
# Implementation Status

- Authentication
    - [x] /public/auth
    - [ ] /private/logout
- Session Management
    - [x] /public/set_heartbeat
    - [x] /public/disable_heartbeat
    - [ ] /private/enable_cancel_on_disconnect
    - [ ] /private/disable_cancel_on_disconnect
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
    - [ ] /private/change_subaccount_name
    - [ ] /private/create_subaccount
    - [ ] /private/disable_tfa_for_subaccount
    - [x] /private/get_account_summary
    - [ ] /private/get_email_language
    - [ ] /private/get_new_announcements
    - [ ] /private/get_position
    - [x] /private/get_positions
    - [x] /private/get_subaccounts
    - [ ] /private/set_announcement_as_read
    - [ ] /private/set_email_for_subaccount
    - [ ] /private/set_email_language 
    - [ ] /private/set_password_for_subaccount
    - [ ] /private/toggle_notifications_from_subaccount
    - [ ] /private/toggle_subaccount_login
- Trading
    - [x] /private/buy
    - [x] /private/sell
    - [x] /private/edit
    - [x] /private/cancel
    - [x] /private/cancel_all
    - [x] /private/cancel_all_by_currency
    - [x] /private/cancel_all_by_instrument
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
    - [ ] /private/get_transfers
    - [ ] /private/get_withdrawals
    - [ ] /private/withdraw
- Subscriptions
    - [x] announcements
    - [x] book.{instrument_name}.{group}.{depth}.{interval}
    - [x] book.{instrument_name}.{interval}
    - [x] deribit_price_index.{index_name}
    - [x] deribit_price_ranking.{index_name}
    - [x] estimated_expiration_price.{index_name}
    - [x] markprice.options.{index_name}
    - [x] perpetual.{instrument_name}.{interval}
    - [x] quote.{instrument_name}
    - [x] ticker.{instrument_name}.{interval}
    - [x] trades.{instrument_name}.{interval}
    - [x] user.orders.{instrument_name}.{interval}
    - [x] user.orders.{kind}.{currency}.{interval}
    - [x] user.portfolio.{currency}
    - [x] user.trades.{instrument_name}.{interval}
    - [x] user.trades.{kind}.{currency}.{interval}]

# Donate

![donationqr](assets/donationqr.png)

16PeVqncfWoQ94M4pxnitkYnnW8agQBBZB
