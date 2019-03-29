Deribit API V2 Client for Rust Language
=================

This is experimental and currently under heavy development. Use at your own risk.

This library uses Rust nightly features extensively like async/await and futures api. You need a nightly Rust to make it run.

The current plan is to only implement the websocket communication, which includes call api through websocket 
and websocket subscription. I will first implement these APIs used for my own trading purpose, however, if you want some APIs 
to be prioritly implemented please open an issue or just throw me a PR (this is more welcome :P).

# Donate

![donationqr](assets/donationqr.png)

16PeVqncfWoQ94M4pxnitkYnnW8agQBBZB

# Implementation Status

- Authentication
    - [x] /public/auth
    - [ ] /private/logout
- Session Management
    - [ ] /public/set_heartbeat
    - [ ] /public/disable_heartbeat
    - [ ] /private/enable_cancel_on_disconnect
    - [ ] /private/disable_cancel_on_disconnect
- Supporting
    - [x] /public/get_time
    - [x] /public/hello
    - [x] /public/test
- Subscription Management
    - [ ] /public/subscribe
    - [ ] /public/unsubscribe
    - [ ] /private/subscribe
    - [ ] /private/unsubscribe
- Account Management
    - Not implemented
- Trading
    - Not implemented
- Market Data
    - Not implemented
- Wallet
    - Not implemented
- Subscriptions
    - [ ] announcements
    - [ ] book.{instrument_name}.{group}.{depth}.{interval}
    - [x] book.{instrument_name}.{interval}
    - [ ] deribit_price_index.{index_name}
    - [ ] deribit_price_ranking.{index_name}
    - [ ] estimated_expiration_price.{index_name}
    - [ ] markprice.options.{index_name}
    - [ ] perpetual.{instrument_name}.{interval}
    - [ ] quote.{instrument_name}
    - [ ] ticker.{instrument_name}.{interval}
    - [ ] trades.{instrument_name}.{interval}
    - [ ] user.orders.{instrument_name}.{interval}
    - [ ] user.orders.{kind}.{currency}.{interval}
    - [x] user.portfolio.{currency}
    - [x] user.trades.{instrument_name}.{interval}
    - [ ] user.trades.{kind}.{currency}.{interval}]