use anyhow::Error;
use deribit::models::jsonrpc::JSONRPCSuccessResponse;
use fehler::throws;
use serde_json::from_str;

#[throws(Error)]
fn main() {
    let payload = r#"{"jsonrpc":"2.0","id":1,"result":{"creation_timestamp":1556701017624,"delta_total":0.0,"spot_reserve":0.0,"email":"youngw@sfu.ca","delta_total_map":{},"available_withdrawal_funds":5.00053339,"options_session_rpl":0.0,"session_rpl":0.0,"futures_session_upl":0.0,"projected_delta_total":0.0,"id":5864,"security_keys_enabled":false,"options_pl":0.0,"estimated_liquidation_ratio":0.0,"options_value":0.0,"currency":"BTC","projected_initial_margin":0.0,"portfolio_margining_enabled":false,"referrer_id":null,"available_funds":5.00053339,"estimated_liquidation_ratio_map":{},"margin_balance":5.00053339,"options_delta":0.0,"balance":5.00053339,"username":"derirsbit","options_theta":0.0,"fee_balance":0.0,"total_pl":0.0,"futures_session_rpl":0.0,"futures_pl":0.0,"options_vega":0.0,"initial_margin":0.0,"type":"main","maintenance_margin":0.0,"equity":5.00053339,"session_upl":0.0,"options_gamma":0.0,"interuser_transfers_enabled":false,"limits":{"non_matching_engine":{"rate":20,"burst":100},"matching_engine":{"rate":5,"burst":20}},"projected_maintenance_margin":0.0,"options_session_upl":0.0,"system_name":"derirsbit"},"usIn":1680395184951913,"usOut":1680395184955201,"usDiff":3288,"testnet":true}"#;

    let data: JSONRPCSuccessResponse<deribit::models::account::GetAccountSummaryResponse> =
        from_str(payload)?;
    println!("{data:?}")
}
