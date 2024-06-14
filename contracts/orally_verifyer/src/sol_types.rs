use alloy_sol_types::sol;

sol! {


    struct Meta {
        string feedId;      // The identifier for the data feed
        uint256 timestamp;  // The timestamp HTTP Gateway response happened
        uint256 fee;        // The update fee in ether (could be zero)
//        string fee_symbol;  // The symbol of the fee token
    }


}
