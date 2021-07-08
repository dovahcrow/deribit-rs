use deribit::models::subscription::UserChangesData;

static P: &'static str = r#"{"trades":[],"positions":[{"total_profit_loss":-0.014649126,"size_currency":-0.243313388,"size":-540.0,"settlement_price":2208.04,"realized_profit_loss":0.0,"open_orders_margin":0.0,"mark_price":2219.36,"maintenance_margin":0.002433252,"leverage":50,"kind":"future","instrument_name":"ETH-24SEP21","initial_margin":0.004866386,"index_price":2203.67,"floating_profit_loss":-0.001247399,"direction":"sell","delta":-0.243313388,"average_price":2093.33}],"orders":[{"web":false,"time_in_force":"good_til_cancelled","replaced":false,"reduce_only":false,"profit_loss":0.0,"price":2216.9,"post_only":true,"order_type":"limit","order_state":"open","order_id":"ETH-1416712080","max_show":50.0,"last_update_timestamp":1625304899374,"label":"","is_liquidation":false,"instrument_name":"ETH-24SEP21","filled_amount":0.0,"direction":"buy","creation_timestamp":1625304899374,"commission":0.0,"average_price":0.0,"api":true,"amount":50.0}],"instrument_name":"ETH-24SEP21"}"#;

fn main() {
    let _: UserChangesData = serde_json::from_str(P).unwrap();
}
