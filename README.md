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
    - [ ] /public/unsubscribe
    - [x] /private/subscribe
    - [ ] /private/unsubscribe
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
    - [ ] /private/get_user_trades_by_currency
    - [ ] /private/get_user_trades_by_currency_and_time
    - [ ] /private/get_user_trades_by_instrument
    - [ ] /private/get_user_trades_by_instrument_and_time
    - [ ] /private/get_user_trades_by_order
    - [ ] /private/get_settlement_history_by_instrument
    - [ ] /private/get_settlement_history_by_currency

- Market Data
    - [x] /public/get_index
    - [x] /public/get_instruments
- Wallet
    - Not implemented
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
