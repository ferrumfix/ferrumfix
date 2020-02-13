#![allow(dead_code)]

mod components {
    use super::*;

    pub struct BidDescReqGrp {
        t_no_bid_descriptors: Option<i64>,
        t_bid_descriptor_type: Option<i64>,
        t_bid_descriptor: Option<String>,
        t_side_value_ind: Option<i64>,
        t_liquidity_value: Option<f32>,
        t_liquidity_num_securities: Option<i64>,
        t_liquidity_pct_low: Option<f32>,
        t_liquidity_pct_high: Option<f32>,
        t_efp_tracking_error: Option<f32>,
        t_fair_value: Option<f32>,
        t_outside_index_pct: Option<f32>,
        t_value_of_futures: Option<f32>,
    }

    pub struct CompIDReqGrp {
        t_no_comp_i_ds: Option<i64>,
        t_ref_comp_id: Option<String>,
        t_ref_sub_id: Option<String>,
        t_location_id: Option<String>,
        t_desk_id: Option<String>,
    }

    pub struct ListOrdGrp {
        t_no_orders: i64,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_list_seq_no: i64,
        t_cl_ord_link_id: Option<String>,
        t_settl_inst_mode: Option<char>,
        c_parties: Option<components::Parties>,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_day_booking_inst: Option<char>,
        t_booking_unit: Option<char>,
        t_alloc_id: Option<String>,
        t_prealloc_method: Option<char>,
        c_pre_alloc_grp: Option<components::PreAllocGrp>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_cash_margin: Option<char>,
        t_clearing_fee_indicator: Option<String>,
        t_handl_inst: Option<char>,
        t_exec_inst: Option<String>,
        t_min_qty: Option<f32>,
        t_max_floor: Option<f32>,
        t_ex_destination: Option<String>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_process_code: Option<char>,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_prev_close_px: Option<f32>,
        t_side: char,
        t_side_value_ind: Option<i64>,
        t_locate_reqd: Option<char>,
        t_transact_time: Option<String>,
        c_stipulations: Option<components::Stipulations>,
        t_qty_type: Option<i64>,
        c_order_qty_data: components::OrderQtyData,
        t_ord_type: Option<char>,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_stop_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        t_currency: Option<String>,
        t_compliance_id: Option<String>,
        t_solicited_flag: Option<char>,
        t_ioiid: Option<String>,
        t_quote_id: Option<String>,
        t_time_in_force: Option<char>,
        t_effective_time: Option<String>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_gt_booking_inst: Option<i64>,
        c_commission_data: Option<components::CommissionData>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_forex_req: Option<char>,
        t_settl_currency: Option<String>,
        t_booking_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_price_2: Option<f32>,
        t_position_effect: Option<char>,
        t_covered_or_uncovered: Option<i64>,
        t_max_show: Option<f32>,
        c_peg_instructions: Option<components::PegInstructions>,
        c_discretion_instructions: Option<components::DiscretionInstructions>,
        t_target_strategy: Option<i64>,
        t_target_strategy_parameters: Option<String>,
        t_participation_rate: Option<f32>,
        t_designation: Option<String>,
    }

    pub struct MiscFeesGrp {
        t_no_misc_fees: Option<i64>,
        t_misc_fee_amt: Option<f32>,
        t_misc_fee_curr: Option<String>,
        t_misc_fee_type: Option<char>,
        t_misc_fee_basis: Option<i64>,
    }

    pub struct CommissionData {
        t_commission: Option<f32>,
        t_comm_type: Option<char>,
        t_comm_currency: Option<String>,
        t_fund_renew_waiv: Option<char>,
    }

    pub struct QuotQualGrp {
        t_no_quote_qualifiers: Option<i64>,
        t_quote_qualifier: Option<char>,
    }

    pub struct UndInstrmtStrkPxGrp {
        t_no_underlyings: Option<i64>,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_prev_close_px: Option<f32>,
        t_cl_ord_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        t_side: Option<char>,
        t_price: f32,
        t_currency: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct LegSecAltIDGrp {
        t_no_leg_security_alt_id: Option<i64>,
        t_leg_security_alt_id: Option<String>,
        t_leg_security_alt_id_source: Option<String>,
    }

    pub struct InstrumentExtension {
        t_delivery_form: Option<i64>,
        t_pct_at_risk: Option<f32>,
        c_attrb_grp: Option<components::AttrbGrp>,
    }

    pub struct QuotReqGrp {
        t_no_related_sym: i64,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_prev_close_px: Option<f32>,
        t_quote_request_type: Option<i64>,
        t_quote_type: Option<i64>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_trade_origination_date: Option<String>,
        t_side: Option<char>,
        t_qty_type: Option<i64>,
        c_order_qty_data: Option<components::OrderQtyData>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_currency: Option<String>,
        c_stipulations: Option<components::Stipulations>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        c_quot_req_legs_grp: Option<components::QuotReqLegsGrp>,
        c_quot_qual_grp: Option<components::QuotQualGrp>,
        t_quote_price_type: Option<i64>,
        t_ord_type: Option<char>,
        t_valid_until_time: Option<String>,
        t_expire_time: Option<String>,
        t_transact_time: Option<String>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_price_2: Option<f32>,
        c_yield_data: Option<components::YieldData>,
        c_parties: Option<components::Parties>,
    }

    pub struct UndInstrmtGrp {
        t_no_underlyings: Option<i64>,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
    }

    pub struct EvntGrp {
        t_no_events: Option<i64>,
        t_event_type: Option<i64>,
        t_event_date: Option<String>,
        t_event_px: Option<f32>,
        t_event_text: Option<String>,
    }

    pub struct TrdInstrmtLegGrp {
        t_no_legs: Option<i64>,
        c_instrument_leg: Option<components::InstrumentLeg>,
        t_leg_qty: Option<f32>,
        t_leg_swap_type: Option<i64>,
        c_leg_stipulations: Option<components::LegStipulations>,
        t_leg_position_effect: Option<char>,
        t_leg_covered_or_uncovered: Option<i64>,
        c_nested_parties: Option<components::NestedParties>,
        t_leg_ref_id: Option<String>,
        t_leg_price: Option<f32>,
        t_leg_settl_type: Option<char>,
        t_leg_settl_date: Option<String>,
        t_leg_last_px: Option<f32>,
    }

    pub struct YieldData {
        t_yield_type: Option<String>,
        t_yield: Option<f32>,
        t_yield_calc_date: Option<String>,
        t_yield_redemption_date: Option<String>,
        t_yield_redemption_price: Option<f32>,
        t_yield_redemption_price_type: Option<i64>,
    }

    pub struct UnderlyingStipulations {
        t_no_underlying_stips: Option<i64>,
        t_underlying_stip_type: Option<String>,
        t_underlying_stip_value: Option<String>,
    }

    pub struct DlvyInstGrp {
        t_no_dlvy_inst: Option<i64>,
        t_settl_inst_source: Option<char>,
        t_dlvy_inst_type: Option<char>,
        c_settl_parties: Option<components::SettlParties>,
    }

    pub struct SettlPtysSubGrp {
        t_no_settl_party_sub_i_ds: Option<i64>,
        t_settl_party_sub_id: Option<String>,
        t_settl_party_sub_id_type: Option<i64>,
    }

    pub struct ContraGrp {
        t_no_contra_brokers: Option<i64>,
        t_contra_broker: Option<String>,
        t_contra_trader: Option<String>,
        t_contra_trade_qty: Option<f32>,
        t_contra_trade_time: Option<String>,
        t_contra_leg_ref_id: Option<String>,
    }

    pub struct RFQReqGrp {
        t_no_related_sym: i64,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_prev_close_px: Option<f32>,
        t_quote_request_type: Option<i64>,
        t_quote_type: Option<i64>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
    }

    pub struct SettlInstGrp {
        t_no_settl_inst: Option<i64>,
        t_settl_inst_id: Option<String>,
        t_settl_inst_trans_type: Option<char>,
        t_settl_inst_ref_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_side: Option<char>,
        t_product: Option<i64>,
        t_security_type: Option<String>,
        t_cfi_code: Option<String>,
        t_effective_time: Option<String>,
        t_expire_time: Option<String>,
        t_last_update_time: Option<String>,
        c_settl_instructions_data: Option<components::SettlInstructionsData>,
        t_payment_method: Option<i64>,
        t_payment_ref: Option<String>,
        t_card_holder_name: Option<String>,
        t_card_number: Option<String>,
        t_card_start_date: Option<String>,
        t_card_exp_date: Option<String>,
        t_card_iss_num: Option<String>,
        t_payment_date: Option<String>,
        t_payment_remitter_id: Option<String>,
    }

    pub struct InstrumentLeg {
        t_leg_symbol: Option<String>,
        t_leg_symbol_sfx: Option<String>,
        t_leg_security_id: Option<String>,
        t_leg_security_id_source: Option<String>,
        c_leg_sec_alt_id_grp: Option<components::LegSecAltIDGrp>,
        t_leg_product: Option<i64>,
        t_leg_cfi_code: Option<String>,
        t_leg_security_type: Option<String>,
        t_leg_security_sub_type: Option<String>,
        t_leg_maturity_month_year: Option<String>,
        t_leg_maturity_date: Option<String>,
        t_leg_coupon_payment_date: Option<String>,
        t_leg_issue_date: Option<String>,
        t_leg_repo_collateral_security_type: Option<String>,
        t_leg_repurchase_term: Option<i64>,
        t_leg_repurchase_rate: Option<f32>,
        t_leg_factor: Option<f32>,
        t_leg_credit_rating: Option<String>,
        t_leg_instr_registry: Option<String>,
        t_leg_country_of_issue: Option<String>,
        t_leg_state_or_province_of_issue: Option<String>,
        t_leg_locale_of_issue: Option<String>,
        t_leg_redemption_date: Option<String>,
        t_leg_strike_price: Option<f32>,
        t_leg_strike_currency: Option<String>,
        t_leg_opt_attribute: Option<char>,
        t_leg_contract_multiplier: Option<f32>,
        t_leg_coupon_rate: Option<f32>,
        t_leg_security_exchange: Option<String>,
        t_leg_issuer: Option<String>,
        t_encoded_leg_issuer_len: Option<i64>,
        t_encoded_leg_issuer: Option<String>,
        t_leg_security_desc: Option<String>,
        t_encoded_leg_security_desc_len: Option<i64>,
        t_encoded_leg_security_desc: Option<String>,
        t_leg_ratio_qty: Option<f32>,
        t_leg_side: Option<char>,
        t_leg_currency: Option<String>,
        t_leg_pool: Option<String>,
        t_leg_dated_date: Option<String>,
        t_leg_contract_settl_month: Option<String>,
        t_leg_interest_accrual_date: Option<String>,
    }

    pub struct PositionAmountData {
        t_no_pos_amt: Option<i64>,
        t_pos_amt_type: Option<String>,
        t_pos_amt: Option<f32>,
    }

    pub struct ExecCollGrp {
        t_no_execs: Option<i64>,
        t_exec_id: Option<String>,
    }

    pub struct InstrmtLegIOIGrp {
        t_no_legs: Option<i64>,
        c_instrument_leg: Option<components::InstrumentLeg>,
        t_leg_ioi_qty: Option<String>,
        c_leg_stipulations: Option<components::LegStipulations>,
    }

    pub struct PreAllocGrp {
        t_no_allocs: Option<i64>,
        t_alloc_account: Option<String>,
        t_alloc_acct_id_source: Option<i64>,
        t_alloc_settl_currency: Option<String>,
        t_individual_alloc_id: Option<String>,
        c_nested_parties: Option<components::NestedParties>,
        t_alloc_qty: Option<f32>,
    }

    pub struct RgstDistInstGrp {
        t_no_distrib_insts: Option<i64>,
        t_distrib_payment_method: Option<i64>,
        t_distrib_percentage: Option<f32>,
        t_cash_distrib_curr: Option<String>,
        t_cash_distrib_agent_name: Option<String>,
        t_cash_distrib_agent_code: Option<String>,
        t_cash_distrib_agent_acct_number: Option<String>,
        t_cash_distrib_pay_ref: Option<String>,
        t_cash_distrib_agent_acct_name: Option<String>,
    }

    pub struct StandardTrailer {
        t_signature_length: Option<i64>,
        t_signature: Option<String>,
        t_check_sum: String,
    }

    pub struct TrdAllocGrp {
        t_no_allocs: Option<i64>,
        t_alloc_account: Option<String>,
        t_alloc_acct_id_source: Option<i64>,
        t_alloc_settl_currency: Option<String>,
        t_individual_alloc_id: Option<String>,
        c_nested_parties_2: Option<components::NestedParties2>,
        t_alloc_qty: Option<f32>,
    }

    pub struct AllocGrp {
        t_no_allocs: Option<i64>,
        t_alloc_account: Option<String>,
        t_alloc_acct_id_source: Option<i64>,
        t_match_status: Option<char>,
        t_alloc_price: Option<f32>,
        t_alloc_qty: Option<f32>,
        t_individual_alloc_id: Option<String>,
        t_process_code: Option<char>,
        c_nested_parties: Option<components::NestedParties>,
        t_notify_broker_of_credit: Option<char>,
        t_alloc_handl_inst: Option<i64>,
        t_alloc_text: Option<String>,
        t_encoded_alloc_text_len: Option<i64>,
        t_encoded_alloc_text: Option<String>,
        c_commission_data: Option<components::CommissionData>,
        t_alloc_avg_px: Option<f32>,
        t_alloc_net_money: Option<f32>,
        t_settl_curr_amt: Option<f32>,
        t_alloc_settl_curr_amt: Option<f32>,
        t_settl_currency: Option<String>,
        t_alloc_settl_currency: Option<String>,
        t_settl_curr_fx_rate: Option<f32>,
        t_settl_curr_fx_rate_calc: Option<char>,
        t_alloc_accrued_interest_amt: Option<f32>,
        t_alloc_interest_at_maturity: Option<f32>,
        c_misc_fees_grp: Option<components::MiscFeesGrp>,
        c_clr_inst_grp: Option<components::ClrInstGrp>,
        t_alloc_settl_inst_type: Option<i64>,
        c_settl_instructions_data: Option<components::SettlInstructionsData>,
    }

    pub struct CompIDStatGrp {
        t_no_comp_i_ds: i64,
        t_ref_comp_id: Option<String>,
        t_ref_sub_id: Option<String>,
        t_location_id: Option<String>,
        t_desk_id: Option<String>,
        t_status_value: Option<i64>,
        t_status_text: Option<String>,
    }

    pub struct InstrmtLegGrp {
        t_no_legs: Option<i64>,
        c_instrument_leg: Option<components::InstrumentLeg>,
    }

    pub struct SpreadOrBenchmarkCurveData {
        t_spread: Option<f32>,
        t_benchmark_curve_currency: Option<String>,
        t_benchmark_curve_name: Option<String>,
        t_benchmark_curve_point: Option<String>,
        t_benchmark_price: Option<f32>,
        t_benchmark_price_type: Option<i64>,
        t_benchmark_security_id: Option<String>,
        t_benchmark_security_id_source: Option<String>,
    }

    pub struct ExecAllocGrp {
        t_no_execs: Option<i64>,
        t_last_qty: Option<f32>,
        t_exec_id: Option<String>,
        t_secondary_exec_id: Option<String>,
        t_last_px: Option<f32>,
        t_last_par_px: Option<f32>,
        t_last_capacity: Option<char>,
    }

    pub struct Instrument {
        t_symbol: Option<String>,
        t_symbol_sfx: Option<String>,
        t_security_id: Option<String>,
        t_security_id_source: Option<String>,
        c_sec_alt_id_grp: Option<components::SecAltIDGrp>,
        t_product: Option<i64>,
        t_cfi_code: Option<String>,
        t_security_type: Option<String>,
        t_security_sub_type: Option<String>,
        t_maturity_month_year: Option<String>,
        t_maturity_date: Option<String>,
        t_put_or_call: Option<i64>,
        t_coupon_payment_date: Option<String>,
        t_issue_date: Option<String>,
        t_repo_collateral_security_type: Option<String>,
        t_repurchase_term: Option<i64>,
        t_repurchase_rate: Option<f32>,
        t_factor: Option<f32>,
        t_credit_rating: Option<String>,
        t_instr_registry: Option<String>,
        t_country_of_issue: Option<String>,
        t_state_or_province_of_issue: Option<String>,
        t_locale_of_issue: Option<String>,
        t_redemption_date: Option<String>,
        t_strike_price: Option<f32>,
        t_strike_currency: Option<String>,
        t_opt_attribute: Option<char>,
        t_contract_multiplier: Option<f32>,
        t_coupon_rate: Option<f32>,
        t_security_exchange: Option<String>,
        t_issuer: Option<String>,
        t_encoded_issuer_len: Option<i64>,
        t_encoded_issuer: Option<String>,
        t_security_desc: Option<String>,
        t_encoded_security_desc_len: Option<i64>,
        t_encoded_security_desc: Option<String>,
        t_pool: Option<String>,
        t_contract_settl_month: Option<String>,
        t_cp_program: Option<i64>,
        t_cp_reg_type: Option<String>,
        c_evnt_grp: Option<components::EvntGrp>,
        t_dated_date: Option<String>,
        t_interest_accrual_date: Option<String>,
    }

    pub struct InstrmtMDReqGrp {
        t_no_related_sym: i64,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
    }

    pub struct OrdListStatGrp {
        t_no_orders: i64,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cum_qty: f32,
        t_ord_status: char,
        t_working_indicator: Option<char>,
        t_leaves_qty: f32,
        t_cxl_qty: f32,
        t_avg_px: f32,
        t_ord_rej_reason: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct TrdCapDtGrp {
        t_no_dates: Option<i64>,
        t_trade_date: Option<String>,
        t_transact_time: Option<String>,
    }

    pub struct SecAltIDGrp {
        t_no_security_alt_id: Option<i64>,
        t_security_alt_id: Option<String>,
        t_security_alt_id_source: Option<String>,
    }

    pub struct UndSecAltIDGrp {
        t_no_underlying_security_alt_id: Option<i64>,
        t_underlying_security_alt_id: Option<String>,
        t_underlying_security_alt_id_source: Option<String>,
    }

    pub struct DiscretionInstructions {
        t_discretion_inst: Option<char>,
        t_discretion_offset_value: Option<f32>,
        t_discretion_move_type: Option<i64>,
        t_discretion_offset_type: Option<i64>,
        t_discretion_limit_type: Option<i64>,
        t_discretion_round_direction: Option<i64>,
        t_discretion_scope: Option<i64>,
    }

    pub struct InstrmtLegSecListGrp {
        t_no_legs: Option<i64>,
        c_instrument_leg: Option<components::InstrumentLeg>,
        t_leg_swap_type: Option<i64>,
        t_leg_settl_type: Option<char>,
        c_leg_stipulations: Option<components::LegStipulations>,
        c_leg_benchmark_curve_data: Option<components::LegBenchmarkCurveData>,
    }

    pub struct PreAllocMlegGrp {
        t_no_allocs: Option<i64>,
        t_alloc_account: Option<String>,
        t_alloc_acct_id_source: Option<i64>,
        t_alloc_settl_currency: Option<String>,
        t_individual_alloc_id: Option<String>,
        c_nested_parties_3: Option<components::NestedParties3>,
        t_alloc_qty: Option<f32>,
    }

    pub struct SideCrossOrdCxlGrp {
        t_no_sides: i64,
        t_side: char,
        t_orig_cl_ord_id: String,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        t_orig_ord_mod_time: Option<String>,
        c_parties: Option<components::Parties>,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        c_order_qty_data: components::OrderQtyData,
        t_compliance_id: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct Hop {
        t_no_hops: Option<i64>,
        t_hop_comp_id: Option<String>,
        t_hop_sending_time: Option<String>,
        t_hop_ref_id: Option<i64>,
    }

    pub struct NstdPtys2SubGrp {
        t_no_nested_2_party_sub_i_ds: Option<i64>,
        t_nested_2_party_sub_id: Option<String>,
        t_nested_2_party_sub_id_type: Option<i64>,
    }

    pub struct NstdPtys3SubGrp {
        t_no_nested_3_party_sub_i_ds: Option<i64>,
        t_nested_3_party_sub_id: Option<String>,
        t_nested_3_party_sub_id_type: Option<i64>,
    }

    pub struct ExecsGrp {
        t_no_execs: Option<i64>,
        t_exec_id: Option<String>,
    }

    pub struct SettlInstructionsData {
        t_settl_delivery_type: Option<i64>,
        t_stand_inst_db_type: Option<i64>,
        t_stand_inst_db_name: Option<String>,
        t_stand_inst_db_id: Option<String>,
        c_dlvy_inst_grp: Option<components::DlvyInstGrp>,
    }

    pub struct QuotEntryGrp {
        t_no_quote_entries: i64,
        t_quote_entry_id: String,
        c_instrument: Option<components::Instrument>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_bid_px: Option<f32>,
        t_offer_px: Option<f32>,
        t_bid_size: Option<f32>,
        t_offer_size: Option<f32>,
        t_valid_until_time: Option<String>,
        t_bid_spot_rate: Option<f32>,
        t_offer_spot_rate: Option<f32>,
        t_bid_forward_points: Option<f32>,
        t_offer_forward_points: Option<f32>,
        t_mid_px: Option<f32>,
        t_bid_yield: Option<f32>,
        t_mid_yield: Option<f32>,
        t_offer_yield: Option<f32>,
        t_transact_time: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_settl_date: Option<String>,
        t_ord_type: Option<char>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_bid_forward_points_2: Option<f32>,
        t_offer_forward_points_2: Option<f32>,
        t_currency: Option<String>,
    }

    pub struct InstrmtGrp {
        t_no_related_sym: Option<i64>,
        c_instrument: Option<components::Instrument>,
    }

    pub struct PosUndInstrmtGrp {
        t_no_underlyings: Option<i64>,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_underlying_settl_price: f32,
        t_underlying_settl_price_type: i64,
    }

    pub struct SecListGrp {
        t_no_related_sym: Option<i64>,
        c_instrument: Option<components::Instrument>,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_currency: Option<String>,
        c_stipulations: Option<components::Stipulations>,
        c_instrmt_leg_sec_list_grp: Option<components::InstrmtLegSecListGrp>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        t_round_lot: Option<f32>,
        t_min_trade_vol: Option<f32>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_expiration_cycle: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct UndInstrmtCollGrp {
        t_no_underlyings: Option<i64>,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_coll_action: Option<i64>,
    }

    pub struct BidCompRspGrp {
        t_no_bid_components: i64,
        c_commission_data: components::CommissionData,
        t_list_id: Option<String>,
        t_country: Option<String>,
        t_side: Option<char>,
        t_price: Option<f32>,
        t_price_type: Option<i64>,
        t_fair_value: Option<f32>,
        t_net_gross_ind: Option<i64>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct QuotEntryAckGrp {
        t_no_quote_entries: Option<i64>,
        t_quote_entry_id: Option<String>,
        c_instrument: Option<components::Instrument>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_bid_px: Option<f32>,
        t_offer_px: Option<f32>,
        t_bid_size: Option<f32>,
        t_offer_size: Option<f32>,
        t_valid_until_time: Option<String>,
        t_bid_spot_rate: Option<f32>,
        t_offer_spot_rate: Option<f32>,
        t_bid_forward_points: Option<f32>,
        t_offer_forward_points: Option<f32>,
        t_mid_px: Option<f32>,
        t_bid_yield: Option<f32>,
        t_mid_yield: Option<f32>,
        t_offer_yield: Option<f32>,
        t_transact_time: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_settl_date: Option<String>,
        t_ord_type: Option<char>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_bid_forward_points_2: Option<f32>,
        t_offer_forward_points_2: Option<f32>,
        t_currency: Option<String>,
        t_quote_entry_reject_reason: Option<i64>,
    }

    pub struct CpctyConfGrp {
        t_no_capacities: i64,
        t_order_capacity: char,
        t_order_restrictions: Option<String>,
        t_order_capacity_qty: f32,
    }

    pub struct InstrmtStrkPxGrp {
        t_no_strikes: i64,
        c_instrument: components::Instrument,
    }

    pub struct NestedParties2 {
        t_no_nested_2_party_i_ds: Option<i64>,
        t_nested_2_party_id: Option<String>,
        t_nested_2_party_id_source: Option<char>,
        t_nested_2_party_role: Option<i64>,
        c_nstd_ptys_2_sub_grp: Option<components::NstdPtys2SubGrp>,
    }

    pub struct LegBenchmarkCurveData {
        t_leg_benchmark_curve_currency: Option<String>,
        t_leg_benchmark_curve_name: Option<String>,
        t_leg_benchmark_curve_point: Option<String>,
        t_leg_benchmark_price: Option<f32>,
        t_leg_benchmark_price_type: Option<i64>,
    }

    pub struct LegQuotGrp {
        t_no_legs: Option<i64>,
        c_instrument_leg: Option<components::InstrumentLeg>,
        t_leg_qty: Option<f32>,
        t_leg_swap_type: Option<i64>,
        t_leg_settl_type: Option<char>,
        t_leg_settl_date: Option<String>,
        c_leg_stipulations: Option<components::LegStipulations>,
        c_nested_parties: Option<components::NestedParties>,
        t_leg_price_type: Option<i64>,
        t_leg_bid_px: Option<f32>,
        t_leg_offer_px: Option<f32>,
        c_leg_benchmark_curve_data: Option<components::LegBenchmarkCurveData>,
    }

    pub struct LegQuotStatGrp {
        t_no_legs: Option<i64>,
        c_instrument_leg: Option<components::InstrumentLeg>,
        t_leg_qty: Option<f32>,
        t_leg_swap_type: Option<i64>,
        t_leg_settl_type: Option<char>,
        t_leg_settl_date: Option<String>,
        c_leg_stipulations: Option<components::LegStipulations>,
        c_nested_parties: Option<components::NestedParties>,
    }

    pub struct AllocAckGrp {
        t_no_allocs: Option<i64>,
        t_alloc_account: Option<String>,
        t_alloc_acct_id_source: Option<i64>,
        t_alloc_price: Option<f32>,
        t_individual_alloc_id: Option<String>,
        t_individual_alloc_rej_code: Option<i64>,
        t_alloc_text: Option<String>,
        t_encoded_alloc_text_len: Option<i64>,
        t_encoded_alloc_text: Option<String>,
    }

    pub struct MDRjctGrp {
        t_no_alt_md_source: Option<i64>,
        t_alt_md_source_id: Option<String>,
    }

    pub struct QuotReqLegsGrp {
        t_no_legs: Option<i64>,
        c_instrument_leg: Option<components::InstrumentLeg>,
        t_leg_qty: Option<f32>,
        t_leg_swap_type: Option<i64>,
        t_leg_settl_type: Option<char>,
        t_leg_settl_date: Option<String>,
        c_leg_stipulations: Option<components::LegStipulations>,
        c_nested_parties: Option<components::NestedParties>,
        c_leg_benchmark_curve_data: Option<components::LegBenchmarkCurveData>,
    }

    pub struct QuotReqRjctGrp {
        t_no_related_sym: i64,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_prev_close_px: Option<f32>,
        t_quote_request_type: Option<i64>,
        t_quote_type: Option<i64>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_trade_origination_date: Option<String>,
        t_side: Option<char>,
        t_qty_type: Option<i64>,
        c_order_qty_data: Option<components::OrderQtyData>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_currency: Option<String>,
        c_stipulations: Option<components::Stipulations>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        c_quot_req_legs_grp: Option<components::QuotReqLegsGrp>,
        c_quot_qual_grp: Option<components::QuotQualGrp>,
        t_quote_price_type: Option<i64>,
        t_ord_type: Option<char>,
        t_expire_time: Option<String>,
        t_transact_time: Option<String>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_price_2: Option<f32>,
        c_yield_data: Option<components::YieldData>,
        c_parties: Option<components::Parties>,
    }

    pub struct SettlParties {
        t_no_settl_party_i_ds: Option<i64>,
        t_settl_party_id: Option<String>,
        t_settl_party_id_source: Option<char>,
        t_settl_party_role: Option<i64>,
        c_settl_ptys_sub_grp: Option<components::SettlPtysSubGrp>,
    }

    pub struct MDReqGrp {
        t_no_md_entry_types: i64,
        t_md_entry_type: char,
    }

    pub struct LegStipulations {
        t_no_leg_stipulations: Option<i64>,
        t_leg_stipulation_type: Option<String>,
        t_leg_stipulation_value: Option<String>,
    }

    pub struct TrdCapRptSideGrp {
        t_no_sides: i64,
        t_side: char,
        t_order_id: String,
        t_secondary_order_id: Option<String>,
        t_cl_ord_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        t_list_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_process_code: Option<char>,
        t_odd_lot: Option<char>,
        c_clr_inst_grp: Option<components::ClrInstGrp>,
        t_trade_input_source: Option<String>,
        t_trade_input_device: Option<String>,
        t_order_input_device: Option<String>,
        t_currency: Option<String>,
        t_compliance_id: Option<String>,
        t_solicited_flag: Option<char>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_ord_type: Option<char>,
        t_exec_inst: Option<String>,
        t_trans_bkd_time: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_time_bracket: Option<String>,
        c_commission_data: Option<components::CommissionData>,
        t_gross_trade_amt: Option<f32>,
        t_num_days_interest: Option<i64>,
        t_ex_date: Option<String>,
        t_accrued_interest_rate: Option<f32>,
        t_accrued_interest_amt: Option<f32>,
        t_interest_at_maturity: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        t_concession: Option<f32>,
        t_total_takedown: Option<f32>,
        t_net_money: Option<f32>,
        t_settl_curr_amt: Option<f32>,
        t_settl_currency: Option<String>,
        t_settl_curr_fx_rate: Option<f32>,
        t_settl_curr_fx_rate_calc: Option<char>,
        t_position_effect: Option<char>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_side_multi_leg_reporting_type: Option<i64>,
        c_cont_amt_grp: Option<components::ContAmtGrp>,
        c_stipulations: Option<components::Stipulations>,
        c_misc_fees_grp: Option<components::MiscFeesGrp>,
        t_exchange_rule: Option<String>,
        t_trade_alloc_indicator: Option<i64>,
        t_prealloc_method: Option<char>,
        t_alloc_id: Option<String>,
        c_trd_alloc_grp: Option<components::TrdAllocGrp>,
    }

    pub struct PegInstructions {
        t_peg_offset_value: Option<f32>,
        t_peg_move_type: Option<i64>,
        t_peg_offset_type: Option<i64>,
        t_peg_limit_type: Option<i64>,
        t_peg_round_direction: Option<i64>,
        t_peg_scope: Option<i64>,
    }

    pub struct UnderlyingInstrument {
        t_underlying_symbol: Option<String>,
        t_underlying_symbol_sfx: Option<String>,
        t_underlying_security_id: Option<String>,
        t_underlying_security_id_source: Option<String>,
        c_und_sec_alt_id_grp: Option<components::UndSecAltIDGrp>,
        t_underlying_product: Option<i64>,
        t_underlying_cfi_code: Option<String>,
        t_underlying_security_type: Option<String>,
        t_underlying_security_sub_type: Option<String>,
        t_underlying_maturity_month_year: Option<String>,
        t_underlying_maturity_date: Option<String>,
        t_underlying_put_or_call: Option<i64>,
        t_underlying_coupon_payment_date: Option<String>,
        t_underlying_issue_date: Option<String>,
        t_underlying_repo_collateral_security_type: Option<String>,
        t_underlying_repurchase_term: Option<i64>,
        t_underlying_repurchase_rate: Option<f32>,
        t_underlying_factor: Option<f32>,
        t_underlying_credit_rating: Option<String>,
        t_underlying_instr_registry: Option<String>,
        t_underlying_country_of_issue: Option<String>,
        t_underlying_state_or_province_of_issue: Option<String>,
        t_underlying_locale_of_issue: Option<String>,
        t_underlying_redemption_date: Option<String>,
        t_underlying_strike_price: Option<f32>,
        t_underlying_strike_currency: Option<String>,
        t_underlying_opt_attribute: Option<char>,
        t_underlying_contract_multiplier: Option<f32>,
        t_underlying_coupon_rate: Option<f32>,
        t_underlying_security_exchange: Option<String>,
        t_underlying_issuer: Option<String>,
        t_encoded_underlying_issuer_len: Option<i64>,
        t_encoded_underlying_issuer: Option<String>,
        t_underlying_security_desc: Option<String>,
        t_encoded_underlying_security_desc_len: Option<i64>,
        t_encoded_underlying_security_desc: Option<String>,
        t_underlying_cp_program: Option<String>,
        t_underlying_cp_reg_type: Option<String>,
        t_underlying_currency: Option<String>,
        t_underlying_qty: Option<f32>,
        t_underlying_px: Option<f32>,
        t_underlying_dirty_price: Option<f32>,
        t_underlying_end_price: Option<f32>,
        t_underlying_start_value: Option<f32>,
        t_underlying_current_value: Option<f32>,
        t_underlying_end_value: Option<f32>,
        c_underlying_stipulations: Option<components::UnderlyingStipulations>,
    }

    pub struct TrdCollGrp {
        t_no_trades: Option<i64>,
        t_trade_report_id: Option<String>,
        t_secondary_trade_report_id: Option<String>,
    }

    pub struct TrdgSesGrp {
        t_no_trading_sessions: Option<i64>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
    }

    pub struct ClrInstGrp {
        t_no_clearing_instructions: Option<i64>,
        t_clearing_instruction: Option<i64>,
    }

    pub struct SideCrossOrdModGrp {
        t_no_sides: i64,
        t_side: char,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_day_booking_inst: Option<char>,
        t_booking_unit: Option<char>,
        t_prealloc_method: Option<char>,
        t_alloc_id: Option<String>,
        c_pre_alloc_grp: Option<components::PreAllocGrp>,
        t_qty_type: Option<i64>,
        c_order_qty_data: components::OrderQtyData,
        c_commission_data: Option<components::CommissionData>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_forex_req: Option<char>,
        t_settl_currency: Option<String>,
        t_booking_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_position_effect: Option<char>,
        t_covered_or_uncovered: Option<i64>,
        t_cash_margin: Option<char>,
        t_clearing_fee_indicator: Option<String>,
        t_solicited_flag: Option<char>,
        t_side_compliance_id: Option<String>,
    }

    pub struct PositionQty {
        t_no_positions: Option<i64>,
        t_pos_type: Option<String>,
        t_long_qty: Option<f32>,
        t_short_qty: Option<f32>,
        t_pos_qty_status: Option<i64>,
        c_nested_parties: Option<components::NestedParties>,
    }

    pub struct LinesOfTextGrp {
        t_no_lines_of_text: i64,
        t_text: String,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct CollInqQualGrp {
        t_no_coll_inquiry_qualifier: Option<i64>,
        t_coll_inquiry_qualifier: Option<i64>,
    }

    pub struct MDFullGrp {
        t_no_md_entries: i64,
        t_md_entry_type: char,
        t_md_entry_px: Option<f32>,
        t_currency: Option<String>,
        t_md_entry_size: Option<f32>,
        t_md_entry_date: Option<String>,
        t_md_entry_time: Option<String>,
        t_tick_direction: Option<char>,
        t_md_mkt: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_quote_condition: Option<String>,
        t_trade_condition: Option<String>,
        t_md_entry_originator: Option<String>,
        t_location_id: Option<String>,
        t_desk_id: Option<String>,
        t_open_close_settl_flag: Option<String>,
        t_time_in_force: Option<char>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_min_qty: Option<f32>,
        t_exec_inst: Option<String>,
        t_seller_days: Option<i64>,
        t_order_id: Option<String>,
        t_quote_entry_id: Option<String>,
        t_md_entry_buyer: Option<String>,
        t_md_entry_seller: Option<String>,
        t_number_of_orders: Option<i64>,
        t_md_entry_position_no: Option<i64>,
        t_scope: Option<String>,
        t_price_delta: Option<f32>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct PtysSubGrp {
        t_no_party_sub_i_ds: Option<i64>,
        t_party_sub_id: Option<String>,
        t_party_sub_id_type: Option<i64>,
    }

    pub struct LegPreAllocGrp {
        t_no_leg_allocs: Option<i64>,
        t_leg_alloc_account: Option<String>,
        t_leg_individual_alloc_id: Option<String>,
        c_nested_parties_2: Option<components::NestedParties2>,
        t_leg_alloc_qty: Option<f32>,
        t_leg_alloc_acct_id_source: Option<String>,
        t_leg_settl_currency: Option<String>,
    }

    pub struct QuotCxlEntriesGrp {
        t_no_quote_entries: Option<i64>,
        c_instrument: Option<components::Instrument>,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
    }

    pub struct ContAmtGrp {
        t_no_cont_amts: Option<i64>,
        t_cont_amt_type: Option<i64>,
        t_cont_amt_value: Option<f32>,
        t_cont_amt_curr: Option<String>,
    }

    pub struct LegOrdGrp {
        t_no_legs: i64,
        c_instrument_leg: Option<components::InstrumentLeg>,
        t_leg_qty: Option<f32>,
        t_leg_swap_type: Option<i64>,
        c_leg_stipulations: Option<components::LegStipulations>,
        c_leg_pre_alloc_grp: Option<components::LegPreAllocGrp>,
        t_leg_position_effect: Option<char>,
        t_leg_covered_or_uncovered: Option<i64>,
        c_nested_parties: Option<components::NestedParties>,
        t_leg_ref_id: Option<String>,
        t_leg_price: Option<f32>,
        t_leg_settl_type: Option<char>,
        t_leg_settl_date: Option<String>,
    }

    pub struct QuotSetAckGrp {
        t_no_quote_sets: Option<i64>,
        t_quote_set_id: Option<String>,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_tot_no_quote_entries: Option<i64>,
        t_last_fragment: Option<char>,
        c_quot_entry_ack_grp: Option<components::QuotEntryAckGrp>,
    }

    pub struct BidCompReqGrp {
        t_no_bid_components: Option<i64>,
        t_list_id: Option<String>,
        t_side: Option<char>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_net_gross_ind: Option<i64>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
    }

    pub struct MDIncGrp {
        t_no_md_entries: i64,
        t_md_update_action: char,
        t_delete_reason: Option<char>,
        t_md_entry_type: Option<char>,
        t_md_entry_id: Option<String>,
        t_md_entry_ref_id: Option<String>,
        c_instrument: Option<components::Instrument>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_financial_status: Option<String>,
        t_corporate_action: Option<String>,
        t_md_entry_px: Option<f32>,
        t_currency: Option<String>,
        t_md_entry_size: Option<f32>,
        t_md_entry_date: Option<String>,
        t_md_entry_time: Option<String>,
        t_tick_direction: Option<char>,
        t_md_mkt: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_quote_condition: Option<String>,
        t_trade_condition: Option<String>,
        t_md_entry_originator: Option<String>,
        t_location_id: Option<String>,
        t_desk_id: Option<String>,
        t_open_close_settl_flag: Option<String>,
        t_time_in_force: Option<char>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_min_qty: Option<f32>,
        t_exec_inst: Option<String>,
        t_seller_days: Option<i64>,
        t_order_id: Option<String>,
        t_quote_entry_id: Option<String>,
        t_md_entry_buyer: Option<String>,
        t_md_entry_seller: Option<String>,
        t_number_of_orders: Option<i64>,
        t_md_entry_position_no: Option<i64>,
        t_scope: Option<String>,
        t_price_delta: Option<f32>,
        t_net_chg_prev_day: Option<f32>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct TrdRegTimestamps {
        t_no_trd_reg_timestamps: Option<i64>,
        t_trd_reg_timestamp: Option<String>,
        t_trd_reg_timestamp_type: Option<i64>,
        t_trd_reg_timestamp_origin: Option<String>,
    }

    pub struct FinancingDetails {
        t_agreement_desc: Option<String>,
        t_agreement_id: Option<String>,
        t_agreement_date: Option<String>,
        t_agreement_currency: Option<String>,
        t_termination_type: Option<i64>,
        t_start_date: Option<String>,
        t_end_date: Option<String>,
        t_delivery_type: Option<i64>,
        t_margin_ratio: Option<f32>,
    }

    pub struct AffectedOrdGrp {
        t_no_affected_orders: Option<i64>,
        t_orig_cl_ord_id: Option<String>,
        t_affected_order_id: Option<String>,
        t_affected_secondary_order_id: Option<String>,
    }

    pub struct AttrbGrp {
        t_no_instr_attrib: Option<i64>,
        t_instr_attrib_type: Option<i64>,
        t_instr_attrib_value: Option<String>,
    }

    pub struct RgstDtlsGrp {
        t_no_regist_dtls: Option<i64>,
        t_regist_dtls: Option<String>,
        t_regist_email: Option<String>,
        t_mailing_dtls: Option<String>,
        t_mailing_inst: Option<String>,
        c_nested_parties: Option<components::NestedParties>,
        t_owner_type: Option<i64>,
        t_date_of_birth: Option<String>,
        t_investor_country_of_residence: Option<String>,
    }

    pub struct IOIQualGrp {
        t_no_ioi_qualifiers: Option<i64>,
        t_ioi_qualifier: Option<char>,
    }

    pub struct NstdPtysSubGrp {
        t_no_nested_party_sub_i_ds: Option<i64>,
        t_nested_party_sub_id: Option<String>,
        t_nested_party_sub_id_type: Option<i64>,
    }

    pub struct RelSymDerivSecGrp {
        t_no_related_sym: Option<i64>,
        c_instrument: Option<components::Instrument>,
        t_currency: Option<String>,
        t_expiration_cycle: Option<i64>,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
    }

    pub struct InstrmtLegExecGrp {
        t_no_legs: Option<i64>,
        c_instrument_leg: Option<components::InstrumentLeg>,
        t_leg_qty: Option<f32>,
        t_leg_swap_type: Option<i64>,
        c_leg_stipulations: Option<components::LegStipulations>,
        t_leg_position_effect: Option<char>,
        t_leg_covered_or_uncovered: Option<i64>,
        c_nested_parties: Option<components::NestedParties>,
        t_leg_ref_id: Option<String>,
        t_leg_price: Option<f32>,
        t_leg_settl_type: Option<char>,
        t_leg_settl_date: Option<String>,
        t_leg_last_px: Option<f32>,
    }

    pub struct Parties {
        t_no_party_i_ds: Option<i64>,
        t_party_id: Option<String>,
        t_party_id_source: Option<char>,
        t_party_role: Option<i64>,
        c_ptys_sub_grp: Option<components::PtysSubGrp>,
    }

    pub struct RoutingGrp {
        t_no_routing_i_ds: Option<i64>,
        t_routing_type: Option<i64>,
        t_routing_id: Option<String>,
    }

    pub struct NestedParties {
        t_no_nested_party_i_ds: Option<i64>,
        t_nested_party_id: Option<String>,
        t_nested_party_id_source: Option<char>,
        t_nested_party_role: Option<i64>,
        c_nstd_ptys_sub_grp: Option<components::NstdPtysSubGrp>,
    }

    pub struct OrdAllocGrp {
        t_no_orders: Option<i64>,
        t_cl_ord_id: Option<String>,
        t_order_id: Option<String>,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        t_list_id: Option<String>,
        c_nested_parties_2: Option<components::NestedParties2>,
        t_order_qty: Option<f32>,
        t_order_avg_px: Option<f32>,
        t_order_booking_qty: Option<f32>,
    }

    pub struct QuotSetGrp {
        t_no_quote_sets: i64,
        t_quote_set_id: String,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_quote_set_valid_until_time: Option<String>,
        t_tot_no_quote_entries: i64,
        t_last_fragment: Option<char>,
        c_quot_entry_grp: components::QuotEntryGrp,
    }

    pub struct SecTypesGrp {
        t_no_security_types: Option<i64>,
        t_security_type: Option<String>,
        t_security_sub_type: Option<String>,
        t_product: Option<i64>,
        t_cfi_code: Option<String>,
    }

    pub struct NestedParties3 {
        t_no_nested_3_party_i_ds: Option<i64>,
        t_nested_3_party_id: Option<String>,
        t_nested_3_party_id_source: Option<char>,
        t_nested_3_party_role: Option<i64>,
        c_nstd_ptys_3_sub_grp: Option<components::NstdPtys3SubGrp>,
    }

    pub struct Stipulations {
        t_no_stipulations: Option<i64>,
        t_stipulation_type: Option<String>,
        t_stipulation_value: Option<String>,
    }

    pub struct StandardHeader {
        t_begin_string: String,
        t_body_length: i64,
        t_msg_type: String,
        t_sender_comp_id: String,
        t_target_comp_id: String,
        t_on_behalf_of_comp_id: Option<String>,
        t_deliver_to_comp_id: Option<String>,
        t_secure_data_len: Option<i64>,
        t_secure_data: Option<String>,
        t_msg_seq_num: i64,
        t_sender_sub_id: Option<String>,
        t_sender_location_id: Option<String>,
        t_target_sub_id: Option<String>,
        t_target_location_id: Option<String>,
        t_on_behalf_of_sub_id: Option<String>,
        t_on_behalf_of_location_id: Option<String>,
        t_deliver_to_sub_id: Option<String>,
        t_deliver_to_location_id: Option<String>,
        t_poss_dup_flag: Option<char>,
        t_poss_resend: Option<char>,
        t_sending_time: String,
        t_orig_sending_time: Option<String>,
        t_xml_data_len: Option<i64>,
        t_xml_data: Option<String>,
        t_message_encoding: Option<String>,
        t_last_msg_seq_num_processed: Option<i64>,
        c_hop: Option<components::Hop>,
    }

    pub struct OrderQtyData {
        t_order_qty: Option<f32>,
        t_cash_order_qty: Option<f32>,
        t_order_percent: Option<f32>,
        t_rounding_direction: Option<char>,
        t_rounding_modulus: Option<f32>,
    }
}

mod messages {
    use super::*;

    /// # Message information:
    /// 
    /// Message type: W
    pub struct MarketDataSnapshotFullRefresh {
        c_standard_header: components::StandardHeader,
        t_md_req_id: Option<String>,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_financial_status: Option<String>,
        t_corporate_action: Option<String>,
        t_net_chg_prev_day: Option<f32>,
        c_md_full_grp: components::MDFullGrp,
        t_appl_queue_depth: Option<i64>,
        t_appl_queue_resolution: Option<i64>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: i
    pub struct MassQuote {
        c_standard_header: components::StandardHeader,
        t_quote_req_id: Option<String>,
        t_quote_id: String,
        t_quote_type: Option<i64>,
        t_quote_response_level: Option<i64>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_def_bid_size: Option<f32>,
        t_def_offer_size: Option<f32>,
        c_quot_set_grp: components::QuotSetGrp,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: S
    pub struct Quote {
        c_standard_header: components::StandardHeader,
        t_quote_req_id: Option<String>,
        t_quote_id: String,
        t_quote_resp_id: Option<String>,
        t_quote_type: Option<i64>,
        c_quot_qual_grp: Option<components::QuotQualGrp>,
        t_quote_response_level: Option<i64>,
        c_parties: Option<components::Parties>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_side: Option<char>,
        c_order_qty_data: Option<components::OrderQtyData>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_currency: Option<String>,
        c_stipulations: Option<components::Stipulations>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        c_leg_quot_grp: Option<components::LegQuotGrp>,
        t_bid_px: Option<f32>,
        t_offer_px: Option<f32>,
        t_mkt_bid_px: Option<f32>,
        t_mkt_offer_px: Option<f32>,
        t_min_bid_size: Option<f32>,
        t_bid_size: Option<f32>,
        t_min_offer_size: Option<f32>,
        t_offer_size: Option<f32>,
        t_valid_until_time: Option<String>,
        t_bid_spot_rate: Option<f32>,
        t_offer_spot_rate: Option<f32>,
        t_bid_forward_points: Option<f32>,
        t_offer_forward_points: Option<f32>,
        t_mid_px: Option<f32>,
        t_bid_yield: Option<f32>,
        t_mid_yield: Option<f32>,
        t_offer_yield: Option<f32>,
        t_transact_time: Option<String>,
        t_ord_type: Option<char>,
        t_bid_forward_points_2: Option<f32>,
        t_offer_forward_points_2: Option<f32>,
        t_settl_curr_bid_fx_rate: Option<f32>,
        t_settl_curr_offer_fx_rate: Option<f32>,
        t_settl_curr_fx_rate_calc: Option<char>,
        t_comm_type: Option<char>,
        t_commission: Option<f32>,
        t_cust_order_capacity: Option<i64>,
        t_ex_destination: Option<String>,
        t_order_capacity: Option<char>,
        t_price_type: Option<i64>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AU
    pub struct ConfirmationAck {
        c_standard_header: components::StandardHeader,
        t_confirm_id: String,
        t_trade_date: String,
        t_transact_time: String,
        t_affirm_status: i64,
        t_confirm_rej_reason: Option<i64>,
        t_match_status: Option<char>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: k
    pub struct BidRequest {
        c_standard_header: components::StandardHeader,
        t_bid_id: Option<String>,
        t_client_bid_id: String,
        t_bid_request_trans_type: char,
        t_list_name: Option<String>,
        t_tot_no_related_sym: i64,
        t_bid_type: i64,
        t_num_tickets: Option<i64>,
        t_currency: Option<String>,
        t_side_value_1: Option<f32>,
        t_side_value_2: Option<f32>,
        c_bid_desc_req_grp: Option<components::BidDescReqGrp>,
        c_bid_comp_req_grp: Option<components::BidCompReqGrp>,
        t_liquidity_ind_type: Option<i64>,
        t_wt_average_liquidity: Option<f32>,
        t_exchange_for_physical: Option<char>,
        t_out_main_cntry_u_index: Option<f32>,
        t_cross_percent: Option<f32>,
        t_prog_rpt_reqs: Option<i64>,
        t_prog_period_interval: Option<i64>,
        t_inc_tax_ind: Option<i64>,
        t_forex_req: Option<char>,
        t_num_bidders: Option<i64>,
        t_trade_date: Option<String>,
        t_bid_trade_type: char,
        t_basis_px_type: char,
        t_strike_time: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: L
    pub struct ListExecute {
        c_standard_header: components::StandardHeader,
        t_list_id: String,
        t_client_bid_id: Option<String>,
        t_bid_id: Option<String>,
        t_transact_time: String,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: n
    pub struct XMLnonFIX {
        c_standard_header: components::StandardHeader,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: z
    pub struct DerivativeSecurityListRequest {
        c_standard_header: components::StandardHeader,
        t_security_req_id: String,
        t_security_list_request_type: i64,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_security_sub_type: Option<String>,
        t_currency: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_subscription_request_type: Option<char>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: f
    pub struct SecurityStatus {
        c_standard_header: components::StandardHeader,
        t_security_status_req_id: Option<String>,
        c_instrument: components::Instrument,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_currency: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_unsolicited_indicator: Option<char>,
        t_security_trading_status: Option<i64>,
        t_financial_status: Option<String>,
        t_corporate_action: Option<String>,
        t_halt_reason: Option<char>,
        t_in_view_of_common: Option<char>,
        t_due_to_related: Option<char>,
        t_buy_volume: Option<f32>,
        t_sell_volume: Option<f32>,
        t_high_px: Option<f32>,
        t_low_px: Option<f32>,
        t_last_px: Option<f32>,
        t_transact_time: Option<String>,
        t_adjustment: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: Z
    pub struct QuoteCancel {
        c_standard_header: components::StandardHeader,
        t_quote_req_id: Option<String>,
        t_quote_id: String,
        t_quote_cancel_type: i64,
        t_quote_response_level: Option<i64>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_quot_cxl_entries_grp: Option<components::QuotCxlEntriesGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: w
    pub struct SecurityTypes {
        c_standard_header: components::StandardHeader,
        t_security_req_id: String,
        t_security_response_id: String,
        t_security_response_type: i64,
        t_tot_no_security_types: Option<i64>,
        t_last_fragment: Option<char>,
        c_sec_types_grp: Option<components::SecTypesGrp>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_subscription_request_type: Option<char>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: V
    pub struct MarketDataRequest {
        c_standard_header: components::StandardHeader,
        t_md_req_id: String,
        t_subscription_request_type: char,
        t_market_depth: i64,
        t_md_update_type: Option<i64>,
        t_aggregated_book: Option<char>,
        t_open_close_settl_flag: Option<String>,
        t_scope: Option<String>,
        t_md_implicit_delete: Option<char>,
        c_md_req_grp: components::MDReqGrp,
        c_instrmt_md_req_grp: components::InstrmtMDReqGrp,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_appl_queue_action: Option<i64>,
        t_appl_queue_max: Option<i64>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: y
    pub struct SecurityList {
        c_standard_header: components::StandardHeader,
        t_security_req_id: String,
        t_security_response_id: String,
        t_security_request_result: i64,
        t_tot_no_related_sym: Option<i64>,
        t_last_fragment: Option<char>,
        c_sec_list_grp: Option<components::SecListGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: Y
    pub struct MarketDataRequestReject {
        c_standard_header: components::StandardHeader,
        t_md_req_id: String,
        t_md_req_rej_reason: Option<char>,
        c_md_rjct_grp: Option<components::MDRjctGrp>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AI
    pub struct QuoteStatusReport {
        c_standard_header: components::StandardHeader,
        t_quote_status_req_id: Option<String>,
        t_quote_req_id: Option<String>,
        t_quote_id: String,
        t_quote_resp_id: Option<String>,
        t_quote_type: Option<i64>,
        c_parties: Option<components::Parties>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_side: Option<char>,
        c_order_qty_data: Option<components::OrderQtyData>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_currency: Option<String>,
        c_stipulations: Option<components::Stipulations>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        c_leg_quot_stat_grp: Option<components::LegQuotStatGrp>,
        c_quot_qual_grp: Option<components::QuotQualGrp>,
        t_expire_time: Option<String>,
        t_price: Option<f32>,
        t_price_type: Option<i64>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        t_bid_px: Option<f32>,
        t_offer_px: Option<f32>,
        t_mkt_bid_px: Option<f32>,
        t_mkt_offer_px: Option<f32>,
        t_min_bid_size: Option<f32>,
        t_bid_size: Option<f32>,
        t_min_offer_size: Option<f32>,
        t_offer_size: Option<f32>,
        t_valid_until_time: Option<String>,
        t_bid_spot_rate: Option<f32>,
        t_offer_spot_rate: Option<f32>,
        t_bid_forward_points: Option<f32>,
        t_offer_forward_points: Option<f32>,
        t_mid_px: Option<f32>,
        t_bid_yield: Option<f32>,
        t_mid_yield: Option<f32>,
        t_offer_yield: Option<f32>,
        t_transact_time: Option<String>,
        t_ord_type: Option<char>,
        t_bid_forward_points_2: Option<f32>,
        t_offer_forward_points_2: Option<f32>,
        t_settl_curr_bid_fx_rate: Option<f32>,
        t_settl_curr_offer_fx_rate: Option<f32>,
        t_settl_curr_fx_rate_calc: Option<char>,
        t_comm_type: Option<char>,
        t_commission: Option<f32>,
        t_cust_order_capacity: Option<i64>,
        t_ex_destination: Option<String>,
        t_quote_status: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: r
    pub struct OrderMassCancelReport {
        c_standard_header: components::StandardHeader,
        t_cl_ord_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        t_order_id: String,
        t_secondary_order_id: Option<String>,
        t_mass_cancel_request_type: char,
        t_mass_cancel_response: char,
        t_mass_cancel_reject_reason: Option<char>,
        t_total_affected_orders: Option<i64>,
        c_affected_ord_grp: Option<components::AffectedOrdGrp>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_instrument: Option<components::Instrument>,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_side: Option<char>,
        t_transact_time: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: BF
    pub struct UserResponse {
        c_standard_header: components::StandardHeader,
        t_user_request_id: String,
        t_username: String,
        t_user_status: Option<i64>,
        t_user_status_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: K
    pub struct ListCancelRequest {
        c_standard_header: components::StandardHeader,
        t_list_id: String,
        t_transact_time: String,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: A
    pub struct Logon {
        c_standard_header: components::StandardHeader,
        t_encrypt_method: i64,
        t_heart_bt_int: i64,
        t_raw_data_length: Option<i64>,
        t_raw_data: Option<String>,
        t_reset_seq_num_flag: Option<char>,
        t_next_expected_msg_seq_num: Option<i64>,
        t_max_message_size: Option<i64>,
        t_no_msg_types: Option<i64>,
        t_ref_msg_type: Option<String>,
        t_msg_direction: Option<char>,
        t_test_message_indicator: Option<char>,
        t_username: Option<String>,
        t_password: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: Q
    pub struct DontKnowTrade {
        c_standard_header: components::StandardHeader,
        t_order_id: String,
        t_secondary_order_id: Option<String>,
        t_exec_id: String,
        t_dk_reason: char,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_side: char,
        c_order_qty_data: components::OrderQtyData,
        t_last_qty: Option<f32>,
        t_last_px: Option<f32>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 0
    pub struct Heartbeat {
        c_standard_header: components::StandardHeader,
        t_test_req_id: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AW
    pub struct AssignmentReport {
        c_standard_header: components::StandardHeader,
        t_asgn_rpt_id: String,
        t_tot_num_assignment_reports: Option<i64>,
        t_last_rpt_requested: Option<char>,
        c_parties: components::Parties,
        t_account: Option<String>,
        t_account_type: i64,
        c_instrument: Option<components::Instrument>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_position_qty: components::PositionQty,
        c_position_amount_data: components::PositionAmountData,
        t_threshold_amount: Option<f32>,
        t_settl_price: f32,
        t_settl_price_type: i64,
        t_underlying_settl_price: f32,
        t_expire_date: Option<String>,
        t_assignment_method: char,
        t_assignment_unit: Option<f32>,
        t_open_interest: f32,
        t_exercise_method: char,
        t_settl_sess_id: String,
        t_settl_sess_sub_id: String,
        t_clearing_business_date: String,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: R
    pub struct QuoteRequest {
        c_standard_header: components::StandardHeader,
        t_quote_req_id: String,
        t_rfq_req_id: Option<String>,
        t_cl_ord_id: Option<String>,
        t_order_capacity: Option<char>,
        c_quot_req_grp: components::QuotReqGrp,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AM
    pub struct PositionMaintenanceReport {
        c_standard_header: components::StandardHeader,
        t_pos_maint_rpt_id: String,
        t_pos_trans_type: i64,
        t_pos_req_id: Option<String>,
        t_pos_maint_action: i64,
        t_orig_pos_req_ref_id: String,
        t_pos_maint_status: i64,
        t_pos_maint_result: Option<i64>,
        t_clearing_business_date: String,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_account: String,
        t_acct_id_source: Option<i64>,
        t_account_type: i64,
        c_instrument: components::Instrument,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_transact_time: String,
        c_position_qty: components::PositionQty,
        c_position_amount_data: components::PositionAmountData,
        t_adjustment_type: Option<i64>,
        t_threshold_amount: Option<f32>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: BD
    pub struct NetworkCounterpartySystemStatusResponse {
        c_standard_header: components::StandardHeader,
        t_network_status_response_type: i64,
        t_network_request_id: Option<String>,
        t_network_response_id: String,
        t_last_network_response_id: Option<String>,
        c_comp_id_stat_grp: components::CompIDStatGrp,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AS
    pub struct AllocationReport {
        c_standard_header: components::StandardHeader,
        t_alloc_report_id: String,
        t_alloc_id: Option<String>,
        t_alloc_trans_type: char,
        t_alloc_report_ref_id: Option<String>,
        t_alloc_canc_replace_reason: Option<i64>,
        t_secondary_alloc_id: Option<String>,
        t_alloc_report_type: i64,
        t_alloc_status: i64,
        t_alloc_rej_code: Option<i64>,
        t_ref_alloc_id: Option<String>,
        t_alloc_intermed_req_type: Option<i64>,
        t_alloc_link_id: Option<String>,
        t_alloc_link_type: Option<i64>,
        t_booking_ref_id: Option<String>,
        t_alloc_no_orders_type: i64,
        c_ord_alloc_grp: Option<components::OrdAllocGrp>,
        c_exec_alloc_grp: Option<components::ExecAllocGrp>,
        t_previously_reported: Option<char>,
        t_reversal_indicator: Option<char>,
        t_match_type: Option<String>,
        t_side: char,
        c_instrument: components::Instrument,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_quantity: f32,
        t_qty_type: Option<i64>,
        t_last_mkt: Option<String>,
        t_trade_origination_date: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_price_type: Option<i64>,
        t_avg_px: f32,
        t_avg_par_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        t_currency: Option<String>,
        t_avg_px_precision: Option<i64>,
        c_parties: Option<components::Parties>,
        t_trade_date: String,
        t_transact_time: Option<String>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_booking_type: Option<i64>,
        t_gross_trade_amt: Option<f32>,
        t_concession: Option<f32>,
        t_total_takedown: Option<f32>,
        t_net_money: Option<f32>,
        t_position_effect: Option<char>,
        t_auto_accept_indicator: Option<char>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_num_days_interest: Option<i64>,
        t_accrued_interest_rate: Option<f32>,
        t_accrued_interest_amt: Option<f32>,
        t_total_accrued_interest_amt: Option<f32>,
        t_interest_at_maturity: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        t_legal_confirm: Option<char>,
        c_stipulations: Option<components::Stipulations>,
        c_yield_data: Option<components::YieldData>,
        t_tot_no_allocs: Option<i64>,
        t_last_fragment: Option<char>,
        c_alloc_grp: Option<components::AllocGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AX
    pub struct CollateralRequest {
        c_standard_header: components::StandardHeader,
        t_coll_req_id: String,
        t_coll_asgn_reason: i64,
        t_transact_time: String,
        t_expire_time: Option<String>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_account_type: Option<i64>,
        t_cl_ord_id: Option<String>,
        t_order_id: Option<String>,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        c_exec_coll_grp: Option<components::ExecCollGrp>,
        c_trd_coll_grp: Option<components::TrdCollGrp>,
        c_instrument: Option<components::Instrument>,
        c_financing_details: Option<components::FinancingDetails>,
        t_settl_date: Option<String>,
        t_quantity: Option<f32>,
        t_qty_type: Option<i64>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_coll_grp: Option<components::UndInstrmtCollGrp>,
        t_margin_excess: Option<f32>,
        t_total_net_value: Option<f32>,
        t_cash_outstanding: Option<f32>,
        c_trd_reg_timestamps: Option<components::TrdRegTimestamps>,
        t_side: Option<char>,
        c_misc_fees_grp: Option<components::MiscFeesGrp>,
        t_price: Option<f32>,
        t_price_type: Option<i64>,
        t_accrued_interest_amt: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_stipulations: Option<components::Stipulations>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        t_clearing_business_date: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AE
    pub struct TradeCaptureReport {
        c_standard_header: components::StandardHeader,
        t_trade_report_id: String,
        t_trade_report_trans_type: Option<i64>,
        t_trade_report_type: Option<i64>,
        t_trade_request_id: Option<String>,
        t_trd_type: Option<i64>,
        t_trd_sub_type: Option<i64>,
        t_secondary_trd_type: Option<i64>,
        t_transfer_reason: Option<String>,
        t_exec_type: Option<char>,
        t_tot_num_trade_reports: Option<i64>,
        t_last_rpt_requested: Option<char>,
        t_unsolicited_indicator: Option<char>,
        t_subscription_request_type: Option<char>,
        t_trade_report_ref_id: Option<String>,
        t_secondary_trade_report_ref_id: Option<String>,
        t_secondary_trade_report_id: Option<String>,
        t_trade_link_id: Option<String>,
        t_trd_match_id: Option<String>,
        t_exec_id: Option<String>,
        t_ord_status: Option<char>,
        t_secondary_exec_id: Option<String>,
        t_exec_restatement_reason: Option<i64>,
        t_previously_reported: char,
        t_price_type: Option<i64>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_order_qty_data: Option<components::OrderQtyData>,
        t_qty_type: Option<i64>,
        c_yield_data: Option<components::YieldData>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_underlying_trading_session_id: Option<String>,
        t_underlying_trading_session_sub_id: Option<String>,
        t_last_qty: f32,
        t_last_px: f32,
        t_last_par_px: Option<f32>,
        t_last_spot_rate: Option<f32>,
        t_last_forward_points: Option<f32>,
        t_last_mkt: Option<String>,
        t_trade_date: String,
        t_clearing_business_date: Option<String>,
        t_avg_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        t_avg_px_indicator: Option<i64>,
        c_position_amount_data: Option<components::PositionAmountData>,
        t_multi_leg_reporting_type: Option<char>,
        t_trade_leg_ref_id: Option<String>,
        c_trd_instrmt_leg_grp: Option<components::TrdInstrmtLegGrp>,
        t_transact_time: String,
        c_trd_reg_timestamps: Option<components::TrdRegTimestamps>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_match_status: Option<char>,
        t_match_type: Option<String>,
        c_trd_cap_rpt_side_grp: components::TrdCapRptSideGrp,
        t_copy_msg_indicator: Option<char>,
        t_publish_trd_indicator: Option<char>,
        t_short_sale_reason: Option<i64>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 8
    pub struct ExecutionReport {
        c_standard_header: components::StandardHeader,
        t_order_id: String,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        t_secondary_exec_id: Option<String>,
        t_cl_ord_id: Option<String>,
        t_orig_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        t_quote_resp_id: Option<String>,
        t_ord_status_req_id: Option<String>,
        t_mass_status_req_id: Option<String>,
        t_tot_num_reports: Option<i64>,
        t_last_rpt_requested: Option<char>,
        c_parties: Option<components::Parties>,
        t_trade_origination_date: Option<String>,
        c_contra_grp: Option<components::ContraGrp>,
        t_list_id: Option<String>,
        t_cross_id: Option<String>,
        t_orig_cross_id: Option<String>,
        t_cross_type: Option<i64>,
        t_exec_id: String,
        t_exec_ref_id: Option<String>,
        t_exec_type: char,
        t_ord_status: char,
        t_working_indicator: Option<char>,
        t_ord_rej_reason: Option<i64>,
        t_exec_restatement_reason: Option<i64>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_day_booking_inst: Option<char>,
        t_booking_unit: Option<char>,
        t_prealloc_method: Option<char>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_cash_margin: Option<char>,
        t_clearing_fee_indicator: Option<String>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_side: char,
        c_stipulations: Option<components::Stipulations>,
        t_qty_type: Option<i64>,
        c_order_qty_data: Option<components::OrderQtyData>,
        t_ord_type: Option<char>,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_stop_px: Option<f32>,
        c_peg_instructions: Option<components::PegInstructions>,
        c_discretion_instructions: Option<components::DiscretionInstructions>,
        t_pegged_price: Option<f32>,
        t_discretion_price: Option<f32>,
        t_target_strategy: Option<i64>,
        t_target_strategy_parameters: Option<String>,
        t_participation_rate: Option<f32>,
        t_target_strategy_performance: Option<f32>,
        t_currency: Option<String>,
        t_compliance_id: Option<String>,
        t_solicited_flag: Option<char>,
        t_time_in_force: Option<char>,
        t_effective_time: Option<String>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_exec_inst: Option<String>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_last_qty: Option<f32>,
        t_underlying_last_qty: Option<f32>,
        t_last_px: Option<f32>,
        t_underlying_last_px: Option<f32>,
        t_last_par_px: Option<f32>,
        t_last_spot_rate: Option<f32>,
        t_last_forward_points: Option<f32>,
        t_last_mkt: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_time_bracket: Option<String>,
        t_last_capacity: Option<char>,
        t_leaves_qty: f32,
        t_cum_qty: f32,
        t_avg_px: f32,
        t_day_order_qty: Option<f32>,
        t_day_cum_qty: Option<f32>,
        t_day_avg_px: Option<f32>,
        t_gt_booking_inst: Option<i64>,
        t_trade_date: Option<String>,
        t_transact_time: Option<String>,
        t_report_to_exch: Option<char>,
        c_commission_data: Option<components::CommissionData>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        t_gross_trade_amt: Option<f32>,
        t_num_days_interest: Option<i64>,
        t_ex_date: Option<String>,
        t_accrued_interest_rate: Option<f32>,
        t_accrued_interest_amt: Option<f32>,
        t_interest_at_maturity: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        t_traded_flat_switch: Option<char>,
        t_basis_feature_date: Option<String>,
        t_basis_feature_price: Option<f32>,
        t_concession: Option<f32>,
        t_total_takedown: Option<f32>,
        t_net_money: Option<f32>,
        t_settl_curr_amt: Option<f32>,
        t_settl_currency: Option<String>,
        t_settl_curr_fx_rate: Option<f32>,
        t_settl_curr_fx_rate_calc: Option<char>,
        t_handl_inst: Option<char>,
        t_min_qty: Option<f32>,
        t_max_floor: Option<f32>,
        t_position_effect: Option<char>,
        t_max_show: Option<f32>,
        t_booking_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_last_forward_points_2: Option<f32>,
        t_multi_leg_reporting_type: Option<char>,
        t_cancellation_rights: Option<char>,
        t_money_laundering_status: Option<char>,
        t_regist_id: Option<String>,
        t_designation: Option<String>,
        t_trans_bkd_time: Option<String>,
        t_exec_valuation_point: Option<String>,
        t_exec_price_type: Option<char>,
        t_exec_price_adjustment: Option<f32>,
        t_priority_indicator: Option<i64>,
        t_price_improvement: Option<f32>,
        t_last_liquidity_ind: Option<i64>,
        c_cont_amt_grp: Option<components::ContAmtGrp>,
        c_instrmt_leg_exec_grp: Option<components::InstrmtLegExecGrp>,
        t_copy_msg_indicator: Option<char>,
        c_misc_fees_grp: Option<components::MiscFeesGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: v
    pub struct SecurityTypeRequest {
        c_standard_header: components::StandardHeader,
        t_security_req_id: String,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_product: Option<i64>,
        t_security_type: Option<String>,
        t_security_sub_type: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: o
    pub struct RegistrationInstructions {
        c_standard_header: components::StandardHeader,
        t_regist_id: String,
        t_regist_trans_type: char,
        t_regist_ref_id: String,
        t_cl_ord_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_regist_acct_type: Option<String>,
        t_tax_advantage_type: Option<i64>,
        t_ownership_type: Option<char>,
        c_rgst_dtls_grp: Option<components::RgstDtlsGrp>,
        c_rgst_dist_inst_grp: Option<components::RgstDistInstGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: M
    pub struct ListStatusRequest {
        c_standard_header: components::StandardHeader,
        t_list_id: String,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: x
    pub struct SecurityListRequest {
        c_standard_header: components::StandardHeader,
        t_security_req_id: String,
        t_security_list_request_type: i64,
        c_instrument: Option<components::Instrument>,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_currency: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_subscription_request_type: Option<char>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: l
    pub struct BidResponse {
        c_standard_header: components::StandardHeader,
        t_bid_id: Option<String>,
        t_client_bid_id: Option<String>,
        c_bid_comp_rsp_grp: components::BidCompRspGrp,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AV
    pub struct SettlementInstructionRequest {
        c_standard_header: components::StandardHeader,
        t_settl_inst_req_id: String,
        t_transact_time: String,
        c_parties: Option<components::Parties>,
        t_alloc_account: Option<String>,
        t_alloc_acct_id_source: Option<i64>,
        t_side: Option<char>,
        t_product: Option<i64>,
        t_security_type: Option<String>,
        t_cfi_code: Option<String>,
        t_effective_time: Option<String>,
        t_expire_time: Option<String>,
        t_last_update_time: Option<String>,
        t_stand_inst_db_type: Option<i64>,
        t_stand_inst_db_name: Option<String>,
        t_stand_inst_db_id: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 1
    pub struct TestRequest {
        c_standard_header: components::StandardHeader,
        t_test_req_id: String,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: u
    pub struct CrossOrderCancelRequest {
        c_standard_header: components::StandardHeader,
        t_order_id: Option<String>,
        t_cross_id: String,
        t_orig_cross_id: String,
        t_cross_type: i64,
        t_cross_prioritization: i64,
        c_side_cross_ord_cxl_grp: components::SideCrossOrdCxlGrp,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_transact_time: String,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AG
    pub struct QuoteRequestReject {
        c_standard_header: components::StandardHeader,
        t_quote_req_id: String,
        t_rfq_req_id: Option<String>,
        t_quote_request_reject_reason: i64,
        c_quot_req_rjct_grp: components::QuotReqRjctGrp,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AN
    pub struct RequestForPositions {
        c_standard_header: components::StandardHeader,
        t_pos_req_id: String,
        t_pos_req_type: i64,
        t_match_status: Option<char>,
        t_subscription_request_type: Option<char>,
        c_parties: components::Parties,
        t_account: String,
        t_acct_id_source: Option<i64>,
        t_account_type: i64,
        c_instrument: Option<components::Instrument>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_clearing_business_date: String,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_transact_time: String,
        t_response_transport_type: Option<i64>,
        t_response_destination: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AO
    pub struct RequestForPositionsAck {
        c_standard_header: components::StandardHeader,
        t_pos_maint_rpt_id: String,
        t_pos_req_id: Option<String>,
        t_total_num_pos_reports: Option<i64>,
        t_unsolicited_indicator: Option<char>,
        t_pos_req_result: i64,
        t_pos_req_status: i64,
        c_parties: components::Parties,
        t_account: String,
        t_acct_id_source: Option<i64>,
        t_account_type: i64,
        c_instrument: Option<components::Instrument>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_response_transport_type: Option<i64>,
        t_response_destination: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: D
    pub struct NewOrderSingle {
        c_standard_header: components::StandardHeader,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_day_booking_inst: Option<char>,
        t_booking_unit: Option<char>,
        t_prealloc_method: Option<char>,
        t_alloc_id: Option<String>,
        c_pre_alloc_grp: Option<components::PreAllocGrp>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_cash_margin: Option<char>,
        t_clearing_fee_indicator: Option<String>,
        t_handl_inst: Option<char>,
        t_exec_inst: Option<String>,
        t_min_qty: Option<f32>,
        t_max_floor: Option<f32>,
        t_ex_destination: Option<String>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_process_code: Option<char>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_prev_close_px: Option<f32>,
        t_side: char,
        t_locate_reqd: Option<char>,
        t_transact_time: String,
        c_stipulations: Option<components::Stipulations>,
        t_qty_type: Option<i64>,
        c_order_qty_data: components::OrderQtyData,
        t_ord_type: char,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_stop_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        t_currency: Option<String>,
        t_compliance_id: Option<String>,
        t_solicited_flag: Option<char>,
        t_ioiid: Option<String>,
        t_quote_id: Option<String>,
        t_time_in_force: Option<char>,
        t_effective_time: Option<String>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_gt_booking_inst: Option<i64>,
        c_commission_data: Option<components::CommissionData>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_forex_req: Option<char>,
        t_settl_currency: Option<String>,
        t_booking_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_price_2: Option<f32>,
        t_position_effect: Option<char>,
        t_covered_or_uncovered: Option<i64>,
        t_max_show: Option<f32>,
        c_peg_instructions: Option<components::PegInstructions>,
        c_discretion_instructions: Option<components::DiscretionInstructions>,
        t_target_strategy: Option<i64>,
        t_target_strategy_parameters: Option<String>,
        t_participation_rate: Option<f32>,
        t_cancellation_rights: Option<char>,
        t_money_laundering_status: Option<char>,
        t_regist_id: Option<String>,
        t_designation: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: BH
    pub struct ConfirmationRequest {
        c_standard_header: components::StandardHeader,
        t_confirm_req_id: String,
        t_confirm_type: i64,
        c_ord_alloc_grp: Option<components::OrdAllocGrp>,
        t_alloc_id: Option<String>,
        t_secondary_alloc_id: Option<String>,
        t_individual_alloc_id: Option<String>,
        t_transact_time: String,
        t_alloc_account: Option<String>,
        t_alloc_acct_id_source: Option<i64>,
        t_alloc_account_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: P
    pub struct AllocationInstructionAck {
        c_standard_header: components::StandardHeader,
        t_alloc_id: String,
        c_parties: Option<components::Parties>,
        t_secondary_alloc_id: Option<String>,
        t_trade_date: Option<String>,
        t_transact_time: String,
        t_alloc_status: i64,
        t_alloc_rej_code: Option<i64>,
        t_alloc_type: Option<i64>,
        t_alloc_intermed_req_type: Option<i64>,
        t_match_status: Option<char>,
        t_product: Option<i64>,
        t_security_type: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_alloc_ack_grp: Option<components::AllocAckGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AK
    pub struct Confirmation {
        c_standard_header: components::StandardHeader,
        t_confirm_id: String,
        t_confirm_ref_id: Option<String>,
        t_confirm_req_id: Option<String>,
        t_confirm_trans_type: i64,
        t_confirm_type: i64,
        t_copy_msg_indicator: Option<char>,
        t_legal_confirm: Option<char>,
        t_confirm_status: i64,
        c_parties: Option<components::Parties>,
        c_ord_alloc_grp: Option<components::OrdAllocGrp>,
        t_alloc_id: Option<String>,
        t_secondary_alloc_id: Option<String>,
        t_individual_alloc_id: Option<String>,
        t_transact_time: String,
        t_trade_date: String,
        c_trd_reg_timestamps: Option<components::TrdRegTimestamps>,
        c_instrument: components::Instrument,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: components::UndInstrmtGrp,
        c_instrmt_leg_grp: components::InstrmtLegGrp,
        c_yield_data: Option<components::YieldData>,
        t_alloc_qty: f32,
        t_qty_type: Option<i64>,
        t_side: char,
        t_currency: Option<String>,
        t_last_mkt: Option<String>,
        c_cpcty_conf_grp: components::CpctyConfGrp,
        t_alloc_account: String,
        t_alloc_acct_id_source: Option<i64>,
        t_alloc_account_type: Option<i64>,
        t_avg_px: f32,
        t_avg_px_precision: Option<i64>,
        t_price_type: Option<i64>,
        t_avg_par_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        t_reported_px: Option<f32>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_process_code: Option<char>,
        t_gross_trade_amt: f32,
        t_num_days_interest: Option<i64>,
        t_ex_date: Option<String>,
        t_accrued_interest_rate: Option<f32>,
        t_accrued_interest_amt: Option<f32>,
        t_interest_at_maturity: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        t_concession: Option<f32>,
        t_total_takedown: Option<f32>,
        t_net_money: f32,
        t_maturity_net_money: Option<f32>,
        t_settl_curr_amt: Option<f32>,
        t_settl_currency: Option<String>,
        t_settl_curr_fx_rate: Option<f32>,
        t_settl_curr_fx_rate_calc: Option<char>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        c_settl_instructions_data: Option<components::SettlInstructionsData>,
        c_commission_data: Option<components::CommissionData>,
        t_shared_commission: Option<f32>,
        c_stipulations: Option<components::Stipulations>,
        c_misc_fees_grp: Option<components::MiscFeesGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: E
    pub struct NewOrderList {
        c_standard_header: components::StandardHeader,
        t_list_id: String,
        t_bid_id: Option<String>,
        t_client_bid_id: Option<String>,
        t_prog_rpt_reqs: Option<i64>,
        t_bid_type: i64,
        t_prog_period_interval: Option<i64>,
        t_cancellation_rights: Option<char>,
        t_money_laundering_status: Option<char>,
        t_regist_id: Option<String>,
        t_list_exec_inst_type: Option<char>,
        t_list_exec_inst: Option<String>,
        t_encoded_list_exec_inst_len: Option<i64>,
        t_encoded_list_exec_inst: Option<String>,
        t_allowable_one_sidedness_pct: Option<f32>,
        t_allowable_one_sidedness_value: Option<f32>,
        t_allowable_one_sidedness_curr: Option<String>,
        t_tot_no_orders: i64,
        t_last_fragment: Option<char>,
        c_list_ord_grp: components::ListOrdGrp,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AL
    pub struct PositionMaintenanceRequest {
        c_standard_header: components::StandardHeader,
        t_pos_req_id: String,
        t_pos_trans_type: i64,
        t_pos_maint_action: i64,
        t_orig_pos_req_ref_id: Option<String>,
        t_pos_maint_rpt_ref_id: Option<String>,
        t_clearing_business_date: String,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        c_parties: components::Parties,
        t_account: String,
        t_acct_id_source: Option<i64>,
        t_account_type: i64,
        c_instrument: components::Instrument,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_transact_time: String,
        c_position_qty: components::PositionQty,
        t_adjustment_type: Option<i64>,
        t_contrary_instruction_indicator: Option<char>,
        t_prior_spread_indicator: Option<char>,
        t_threshold_amount: Option<f32>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AJ
    pub struct QuoteResponse {
        c_standard_header: components::StandardHeader,
        t_quote_resp_id: String,
        t_quote_id: Option<String>,
        t_quote_resp_type: i64,
        t_cl_ord_id: Option<String>,
        t_order_capacity: Option<char>,
        t_ioiid: Option<String>,
        t_quote_type: Option<i64>,
        c_quot_qual_grp: Option<components::QuotQualGrp>,
        c_parties: Option<components::Parties>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_side: Option<char>,
        c_order_qty_data: Option<components::OrderQtyData>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_currency: Option<String>,
        c_stipulations: Option<components::Stipulations>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        c_leg_quot_grp: Option<components::LegQuotGrp>,
        t_bid_px: Option<f32>,
        t_offer_px: Option<f32>,
        t_mkt_bid_px: Option<f32>,
        t_mkt_offer_px: Option<f32>,
        t_min_bid_size: Option<f32>,
        t_bid_size: Option<f32>,
        t_min_offer_size: Option<f32>,
        t_offer_size: Option<f32>,
        t_valid_until_time: Option<String>,
        t_bid_spot_rate: Option<f32>,
        t_offer_spot_rate: Option<f32>,
        t_bid_forward_points: Option<f32>,
        t_offer_forward_points: Option<f32>,
        t_mid_px: Option<f32>,
        t_bid_yield: Option<f32>,
        t_mid_yield: Option<f32>,
        t_offer_yield: Option<f32>,
        t_transact_time: Option<String>,
        t_ord_type: Option<char>,
        t_bid_forward_points_2: Option<f32>,
        t_offer_forward_points_2: Option<f32>,
        t_settl_curr_bid_fx_rate: Option<f32>,
        t_settl_curr_offer_fx_rate: Option<f32>,
        t_settl_curr_fx_rate_calc: Option<char>,
        t_commission: Option<f32>,
        t_comm_type: Option<char>,
        t_cust_order_capacity: Option<i64>,
        t_ex_destination: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_price: Option<f32>,
        t_price_type: Option<i64>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: H
    pub struct OrderStatusRequest {
        c_standard_header: components::StandardHeader,
        t_order_id: Option<String>,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_ord_status_req_id: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_side: char,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AP
    pub struct PositionReport {
        c_standard_header: components::StandardHeader,
        t_pos_maint_rpt_id: String,
        t_pos_req_id: Option<String>,
        t_pos_req_type: Option<i64>,
        t_subscription_request_type: Option<char>,
        t_total_num_pos_reports: Option<i64>,
        t_unsolicited_indicator: Option<char>,
        t_pos_req_result: i64,
        t_clearing_business_date: String,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        c_parties: components::Parties,
        t_account: String,
        t_acct_id_source: Option<i64>,
        t_account_type: i64,
        c_instrument: Option<components::Instrument>,
        t_currency: Option<String>,
        t_settl_price: f32,
        t_settl_price_type: i64,
        t_prior_settl_price: f32,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_pos_und_instrmt_grp: Option<components::PosUndInstrmtGrp>,
        c_position_qty: components::PositionQty,
        c_position_amount_data: components::PositionAmountData,
        t_regist_status: Option<char>,
        t_delivery_date: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: e
    pub struct SecurityStatusRequest {
        c_standard_header: components::StandardHeader,
        t_security_status_req_id: String,
        c_instrument: components::Instrument,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_currency: Option<String>,
        t_subscription_request_type: char,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: BC
    pub struct NetworkCounterpartySystemStatusRequest {
        c_standard_header: components::StandardHeader,
        t_network_request_type: i64,
        t_network_request_id: String,
        c_comp_id_req_grp: Option<components::CompIDReqGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: c
    pub struct SecurityDefinitionRequest {
        c_standard_header: components::StandardHeader,
        t_security_req_id: String,
        t_security_request_type: i64,
        c_instrument: Option<components::Instrument>,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_currency: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_expiration_cycle: Option<i64>,
        t_subscription_request_type: Option<char>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: C
    pub struct Email {
        c_standard_header: components::StandardHeader,
        t_email_thread_id: String,
        t_email_type: char,
        t_orig_time: Option<String>,
        t_subject: String,
        t_encoded_subject_len: Option<i64>,
        t_encoded_subject: Option<String>,
        c_routing_grp: Option<components::RoutingGrp>,
        c_instrmt_grp: Option<components::InstrmtGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_order_id: Option<String>,
        t_cl_ord_id: Option<String>,
        c_lines_of_text_grp: components::LinesOfTextGrp,
        t_raw_data_length: Option<i64>,
        t_raw_data: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: F
    pub struct OrderCancelRequest {
        c_standard_header: components::StandardHeader,
        t_orig_cl_ord_id: String,
        t_order_id: Option<String>,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        t_list_id: Option<String>,
        t_orig_ord_mod_time: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        c_parties: Option<components::Parties>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_side: char,
        t_transact_time: String,
        c_order_qty_data: components::OrderQtyData,
        t_compliance_id: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: J
    pub struct AllocationInstruction {
        c_standard_header: components::StandardHeader,
        t_alloc_id: String,
        t_alloc_trans_type: char,
        t_alloc_type: i64,
        t_secondary_alloc_id: Option<String>,
        t_ref_alloc_id: Option<String>,
        t_alloc_canc_replace_reason: Option<i64>,
        t_alloc_intermed_req_type: Option<i64>,
        t_alloc_link_id: Option<String>,
        t_alloc_link_type: Option<i64>,
        t_booking_ref_id: Option<String>,
        t_alloc_no_orders_type: i64,
        c_ord_alloc_grp: Option<components::OrdAllocGrp>,
        c_exec_alloc_grp: Option<components::ExecAllocGrp>,
        t_previously_reported: Option<char>,
        t_reversal_indicator: Option<char>,
        t_match_type: Option<String>,
        t_side: char,
        c_instrument: components::Instrument,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_quantity: f32,
        t_qty_type: Option<i64>,
        t_last_mkt: Option<String>,
        t_trade_origination_date: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_price_type: Option<i64>,
        t_avg_px: f32,
        t_avg_par_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        t_currency: Option<String>,
        t_avg_px_precision: Option<i64>,
        c_parties: Option<components::Parties>,
        t_trade_date: String,
        t_transact_time: Option<String>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_booking_type: Option<i64>,
        t_gross_trade_amt: Option<f32>,
        t_concession: Option<f32>,
        t_total_takedown: Option<f32>,
        t_net_money: Option<f32>,
        t_position_effect: Option<char>,
        t_auto_accept_indicator: Option<char>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_num_days_interest: Option<i64>,
        t_accrued_interest_rate: Option<f32>,
        t_accrued_interest_amt: Option<f32>,
        t_total_accrued_interest_amt: Option<f32>,
        t_interest_at_maturity: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        t_legal_confirm: Option<char>,
        c_stipulations: Option<components::Stipulations>,
        c_yield_data: Option<components::YieldData>,
        t_tot_no_allocs: Option<i64>,
        t_last_fragment: Option<char>,
        c_alloc_grp: Option<components::AllocGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: N
    pub struct ListStatus {
        c_standard_header: components::StandardHeader,
        t_list_id: String,
        t_list_status_type: i64,
        t_no_rpts: i64,
        t_list_order_status: i64,
        t_rpt_seq: i64,
        t_list_status_text: Option<String>,
        t_encoded_list_status_text_len: Option<i64>,
        t_encoded_list_status_text: Option<String>,
        t_transact_time: Option<String>,
        t_tot_no_orders: i64,
        t_last_fragment: Option<char>,
        c_ord_list_stat_grp: components::OrdListStatGrp,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: h
    pub struct TradingSessionStatus {
        c_standard_header: components::StandardHeader,
        t_trad_ses_req_id: Option<String>,
        t_trading_session_id: String,
        t_trading_session_sub_id: Option<String>,
        t_trad_ses_method: Option<i64>,
        t_trad_ses_mode: Option<i64>,
        t_unsolicited_indicator: Option<char>,
        t_trad_ses_status: i64,
        t_trad_ses_status_rej_reason: Option<i64>,
        t_trad_ses_start_time: Option<String>,
        t_trad_ses_open_time: Option<String>,
        t_trad_ses_pre_close_time: Option<String>,
        t_trad_ses_close_time: Option<String>,
        t_trad_ses_end_time: Option<String>,
        t_total_volume_traded: Option<f32>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 7
    pub struct Advertisement {
        c_standard_header: components::StandardHeader,
        t_adv_id: String,
        t_adv_trans_type: String,
        t_adv_ref_id: Option<String>,
        c_instrument: components::Instrument,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_adv_side: char,
        t_quantity: f32,
        t_qty_type: Option<i64>,
        t_price: Option<f32>,
        t_currency: Option<String>,
        t_trade_date: Option<String>,
        t_transact_time: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_url_link: Option<String>,
        t_last_mkt: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: X
    pub struct MarketDataIncrementalRefresh {
        c_standard_header: components::StandardHeader,
        t_md_req_id: Option<String>,
        c_md_inc_grp: components::MDIncGrp,
        t_appl_queue_depth: Option<i64>,
        t_appl_queue_resolution: Option<i64>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: d
    pub struct SecurityDefinition {
        c_standard_header: components::StandardHeader,
        t_security_req_id: String,
        t_security_response_id: String,
        t_security_response_type: i64,
        c_instrument: Option<components::Instrument>,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_currency: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_expiration_cycle: Option<i64>,
        t_round_lot: Option<f32>,
        t_min_trade_vol: Option<f32>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: p
    pub struct RegistrationInstructionsResponse {
        c_standard_header: components::StandardHeader,
        t_regist_id: String,
        t_regist_trans_type: char,
        t_regist_ref_id: String,
        t_cl_ord_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_regist_status: char,
        t_regist_rej_reason_code: Option<i64>,
        t_regist_rej_reason_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: t
    pub struct CrossOrderCancelReplaceRequest {
        c_standard_header: components::StandardHeader,
        t_order_id: Option<String>,
        t_cross_id: String,
        t_orig_cross_id: String,
        t_cross_type: i64,
        t_cross_prioritization: i64,
        c_side_cross_ord_mod_grp: components::SideCrossOrdModGrp,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_handl_inst: Option<char>,
        t_exec_inst: Option<String>,
        t_min_qty: Option<f32>,
        t_max_floor: Option<f32>,
        t_ex_destination: Option<String>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_process_code: Option<char>,
        t_prev_close_px: Option<f32>,
        t_locate_reqd: Option<char>,
        t_transact_time: String,
        c_stipulations: Option<components::Stipulations>,
        t_ord_type: char,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_stop_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        t_currency: Option<String>,
        t_compliance_id: Option<String>,
        t_ioiid: Option<String>,
        t_quote_id: Option<String>,
        t_time_in_force: Option<char>,
        t_effective_time: Option<String>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_gt_booking_inst: Option<i64>,
        t_max_show: Option<f32>,
        c_peg_instructions: Option<components::PegInstructions>,
        c_discretion_instructions: Option<components::DiscretionInstructions>,
        t_target_strategy: Option<i64>,
        t_target_strategy_parameters: Option<String>,
        t_participation_rate: Option<f32>,
        t_cancellation_rights: Option<char>,
        t_money_laundering_status: Option<char>,
        t_regist_id: Option<String>,
        t_designation: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 5
    pub struct Logout {
        c_standard_header: components::StandardHeader,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 6
    pub struct IOI {
        c_standard_header: components::StandardHeader,
        t_ioiid: String,
        t_ioi_trans_type: char,
        t_ioi_ref_id: Option<String>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_side: char,
        t_qty_type: Option<i64>,
        c_order_qty_data: Option<components::OrderQtyData>,
        t_ioi_qty: String,
        t_currency: Option<String>,
        c_stipulations: Option<components::Stipulations>,
        c_instrmt_leg_ioi_grp: Option<components::InstrmtLegIOIGrp>,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_valid_until_time: Option<String>,
        t_ioi_qlty_ind: Option<char>,
        t_ioi_natural_flag: Option<char>,
        c_ioi_qual_grp: Option<components::IOIQualGrp>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_transact_time: Option<String>,
        t_url_link: Option<String>,
        c_routing_grp: Option<components::RoutingGrp>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: BA
    pub struct CollateralReport {
        c_standard_header: components::StandardHeader,
        t_coll_rpt_id: String,
        t_coll_inquiry_id: Option<String>,
        t_coll_status: i64,
        t_tot_num_reports: Option<i64>,
        t_last_rpt_requested: Option<char>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_account_type: Option<i64>,
        t_cl_ord_id: Option<String>,
        t_order_id: Option<String>,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        c_exec_coll_grp: Option<components::ExecCollGrp>,
        c_trd_coll_grp: Option<components::TrdCollGrp>,
        c_instrument: Option<components::Instrument>,
        c_financing_details: Option<components::FinancingDetails>,
        t_settl_date: Option<String>,
        t_quantity: Option<f32>,
        t_qty_type: Option<i64>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_margin_excess: Option<f32>,
        t_total_net_value: Option<f32>,
        t_cash_outstanding: Option<f32>,
        c_trd_reg_timestamps: Option<components::TrdRegTimestamps>,
        t_side: Option<char>,
        c_misc_fees_grp: Option<components::MiscFeesGrp>,
        t_price: Option<f32>,
        t_price_type: Option<i64>,
        t_accrued_interest_amt: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_stipulations: Option<components::Stipulations>,
        c_settl_instructions_data: Option<components::SettlInstructionsData>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        t_clearing_business_date: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: g
    pub struct TradingSessionStatusRequest {
        c_standard_header: components::StandardHeader,
        t_trad_ses_req_id: String,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_trad_ses_method: Option<i64>,
        t_trad_ses_mode: Option<i64>,
        t_subscription_request_type: char,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: m
    pub struct ListStrikePrice {
        c_standard_header: components::StandardHeader,
        t_list_id: String,
        t_tot_no_strikes: i64,
        t_last_fragment: Option<char>,
        c_instrmt_strk_px_grp: components::InstrmtStrkPxGrp,
        c_und_instrmt_strk_px_grp: Option<components::UndInstrmtStrkPxGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: b
    pub struct MassQuoteAcknowledgement {
        c_standard_header: components::StandardHeader,
        t_quote_req_id: Option<String>,
        t_quote_id: Option<String>,
        t_quote_status: i64,
        t_quote_reject_reason: Option<i64>,
        t_quote_response_level: Option<i64>,
        t_quote_type: Option<i64>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_quot_set_ack_grp: Option<components::QuotSetAckGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AA
    pub struct DerivativeSecurityList {
        c_standard_header: components::StandardHeader,
        t_security_req_id: String,
        t_security_response_id: String,
        t_security_request_result: i64,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_tot_no_related_sym: Option<i64>,
        t_last_fragment: Option<char>,
        c_rel_sym_deriv_sec_grp: Option<components::RelSymDerivSecGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: B
    pub struct News {
        c_standard_header: components::StandardHeader,
        t_orig_time: Option<String>,
        t_urgency: Option<char>,
        t_headline: String,
        t_encoded_headline_len: Option<i64>,
        t_encoded_headline: Option<String>,
        c_routing_grp: Option<components::RoutingGrp>,
        c_instrmt_grp: Option<components::InstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_lines_of_text_grp: components::LinesOfTextGrp,
        t_url_link: Option<String>,
        t_raw_data_length: Option<i64>,
        t_raw_data: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AB
    pub struct NewOrderMultileg {
        c_standard_header: components::StandardHeader,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_day_booking_inst: Option<char>,
        t_booking_unit: Option<char>,
        t_prealloc_method: Option<char>,
        t_alloc_id: Option<String>,
        c_pre_alloc_mleg_grp: Option<components::PreAllocMlegGrp>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_cash_margin: Option<char>,
        t_clearing_fee_indicator: Option<String>,
        t_handl_inst: Option<char>,
        t_exec_inst: Option<String>,
        t_min_qty: Option<f32>,
        t_max_floor: Option<f32>,
        t_ex_destination: Option<String>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_process_code: Option<char>,
        t_side: char,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_prev_close_px: Option<f32>,
        c_leg_ord_grp: components::LegOrdGrp,
        t_locate_reqd: Option<char>,
        t_transact_time: String,
        t_qty_type: Option<i64>,
        c_order_qty_data: components::OrderQtyData,
        t_ord_type: char,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_stop_px: Option<f32>,
        t_currency: Option<String>,
        t_compliance_id: Option<String>,
        t_solicited_flag: Option<char>,
        t_ioiid: Option<String>,
        t_quote_id: Option<String>,
        t_time_in_force: Option<char>,
        t_effective_time: Option<String>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_gt_booking_inst: Option<i64>,
        c_commission_data: Option<components::CommissionData>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_forex_req: Option<char>,
        t_settl_currency: Option<String>,
        t_booking_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_position_effect: Option<char>,
        t_covered_or_uncovered: Option<i64>,
        t_max_show: Option<f32>,
        c_peg_instructions: Option<components::PegInstructions>,
        c_discretion_instructions: Option<components::DiscretionInstructions>,
        t_target_strategy: Option<i64>,
        t_target_strategy_parameters: Option<String>,
        t_participation_rate: Option<f32>,
        t_cancellation_rights: Option<char>,
        t_money_laundering_status: Option<char>,
        t_regist_id: Option<String>,
        t_designation: Option<String>,
        t_multi_leg_rpt_type_req: Option<i64>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AR
    pub struct TradeCaptureReportAck {
        c_standard_header: components::StandardHeader,
        t_trade_report_id: String,
        t_trade_report_trans_type: Option<i64>,
        t_trade_report_type: Option<i64>,
        t_trd_type: Option<i64>,
        t_trd_sub_type: Option<i64>,
        t_secondary_trd_type: Option<i64>,
        t_transfer_reason: Option<String>,
        t_exec_type: char,
        t_trade_report_ref_id: Option<String>,
        t_secondary_trade_report_ref_id: Option<String>,
        t_trd_rpt_status: Option<i64>,
        t_trade_report_reject_reason: Option<i64>,
        t_secondary_trade_report_id: Option<String>,
        t_subscription_request_type: Option<char>,
        t_trade_link_id: Option<String>,
        t_trd_match_id: Option<String>,
        t_exec_id: Option<String>,
        t_secondary_exec_id: Option<String>,
        c_instrument: components::Instrument,
        t_transact_time: Option<String>,
        c_trd_reg_timestamps: Option<components::TrdRegTimestamps>,
        t_response_transport_type: Option<i64>,
        t_response_destination: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_trd_instrmt_leg_grp: Option<components::TrdInstrmtLegGrp>,
        t_clearing_fee_indicator: Option<String>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_position_effect: Option<char>,
        t_prealloc_method: Option<char>,
        c_trd_alloc_grp: Option<components::TrdAllocGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 4
    pub struct SequenceReset {
        c_standard_header: components::StandardHeader,
        t_gap_fill_flag: Option<char>,
        t_new_seq_no: i64,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: q
    pub struct OrderMassCancelRequest {
        c_standard_header: components::StandardHeader,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_mass_cancel_request_type: char,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_instrument: Option<components::Instrument>,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_side: Option<char>,
        t_transact_time: String,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AY
    pub struct CollateralAssignment {
        c_standard_header: components::StandardHeader,
        t_coll_asgn_id: String,
        t_coll_req_id: Option<String>,
        t_coll_asgn_reason: i64,
        t_coll_asgn_trans_type: i64,
        t_coll_asgn_ref_id: Option<String>,
        t_transact_time: String,
        t_expire_time: Option<String>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_account_type: Option<i64>,
        t_cl_ord_id: Option<String>,
        t_order_id: Option<String>,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        c_exec_coll_grp: Option<components::ExecCollGrp>,
        c_trd_coll_grp: Option<components::TrdCollGrp>,
        c_instrument: Option<components::Instrument>,
        c_financing_details: Option<components::FinancingDetails>,
        t_settl_date: Option<String>,
        t_quantity: Option<f32>,
        t_qty_type: Option<i64>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_coll_grp: Option<components::UndInstrmtCollGrp>,
        t_margin_excess: Option<f32>,
        t_total_net_value: Option<f32>,
        t_cash_outstanding: Option<f32>,
        c_trd_reg_timestamps: Option<components::TrdRegTimestamps>,
        t_side: Option<char>,
        c_misc_fees_grp: Option<components::MiscFeesGrp>,
        t_price: Option<f32>,
        t_price_type: Option<i64>,
        t_accrued_interest_amt: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_stipulations: Option<components::Stipulations>,
        c_settl_instructions_data: Option<components::SettlInstructionsData>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        t_clearing_business_date: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: BE
    pub struct UserRequest {
        c_standard_header: components::StandardHeader,
        t_user_request_id: String,
        t_user_request_type: i64,
        t_username: String,
        t_password: Option<String>,
        t_new_password: Option<String>,
        t_raw_data_length: Option<i64>,
        t_raw_data: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AC
    pub struct MultilegOrderCancelReplace {
        c_standard_header: components::StandardHeader,
        t_order_id: Option<String>,
        t_orig_cl_ord_id: String,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        t_orig_ord_mod_time: Option<String>,
        c_parties: Option<components::Parties>,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_day_booking_inst: Option<char>,
        t_booking_unit: Option<char>,
        t_prealloc_method: Option<char>,
        t_alloc_id: Option<String>,
        c_pre_alloc_mleg_grp: Option<components::PreAllocMlegGrp>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_cash_margin: Option<char>,
        t_clearing_fee_indicator: Option<String>,
        t_handl_inst: Option<char>,
        t_exec_inst: Option<String>,
        t_min_qty: Option<f32>,
        t_max_floor: Option<f32>,
        t_ex_destination: Option<String>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_process_code: Option<char>,
        t_side: char,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_prev_close_px: Option<f32>,
        c_leg_ord_grp: components::LegOrdGrp,
        t_locate_reqd: Option<char>,
        t_transact_time: String,
        t_qty_type: Option<i64>,
        c_order_qty_data: components::OrderQtyData,
        t_ord_type: char,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_stop_px: Option<f32>,
        t_currency: Option<String>,
        t_compliance_id: Option<String>,
        t_solicited_flag: Option<char>,
        t_ioiid: Option<String>,
        t_quote_id: Option<String>,
        t_time_in_force: Option<char>,
        t_effective_time: Option<String>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_gt_booking_inst: Option<i64>,
        c_commission_data: Option<components::CommissionData>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_forex_req: Option<char>,
        t_settl_currency: Option<String>,
        t_booking_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_position_effect: Option<char>,
        t_covered_or_uncovered: Option<i64>,
        t_max_show: Option<f32>,
        c_peg_instructions: Option<components::PegInstructions>,
        c_discretion_instructions: Option<components::DiscretionInstructions>,
        t_target_strategy: Option<i64>,
        t_target_strategy_parameters: Option<String>,
        t_participation_rate: Option<f32>,
        t_cancellation_rights: Option<char>,
        t_money_laundering_status: Option<char>,
        t_regist_id: Option<String>,
        t_designation: Option<String>,
        t_multi_leg_rpt_type_req: Option<i64>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AF
    pub struct OrderMassStatusRequest {
        c_standard_header: components::StandardHeader,
        t_mass_status_req_id: String,
        t_mass_status_req_type: i64,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        c_instrument: Option<components::Instrument>,
        c_underlying_instrument: Option<components::UnderlyingInstrument>,
        t_side: Option<char>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AH
    pub struct RFQRequest {
        c_standard_header: components::StandardHeader,
        t_rfq_req_id: String,
        c_rfq_req_grp: components::RFQReqGrp,
        t_subscription_request_type: Option<char>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: j
    pub struct BusinessMessageReject {
        c_standard_header: components::StandardHeader,
        t_ref_seq_num: Option<i64>,
        t_ref_msg_type: String,
        t_business_reject_ref_id: Option<String>,
        t_business_reject_reason: i64,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AZ
    pub struct CollateralResponse {
        c_standard_header: components::StandardHeader,
        t_coll_resp_id: String,
        t_coll_asgn_id: String,
        t_coll_req_id: Option<String>,
        t_coll_asgn_reason: i64,
        t_coll_asgn_trans_type: Option<i64>,
        t_coll_asgn_resp_type: i64,
        t_coll_asgn_reject_reason: Option<i64>,
        t_transact_time: String,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_account_type: Option<i64>,
        t_cl_ord_id: Option<String>,
        t_order_id: Option<String>,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        c_exec_coll_grp: Option<components::ExecCollGrp>,
        c_trd_coll_grp: Option<components::TrdCollGrp>,
        c_instrument: Option<components::Instrument>,
        c_financing_details: Option<components::FinancingDetails>,
        t_settl_date: Option<String>,
        t_quantity: Option<f32>,
        t_qty_type: Option<i64>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_coll_grp: Option<components::UndInstrmtCollGrp>,
        t_margin_excess: Option<f32>,
        t_total_net_value: Option<f32>,
        t_cash_outstanding: Option<f32>,
        c_trd_reg_timestamps: Option<components::TrdRegTimestamps>,
        t_side: Option<char>,
        c_misc_fees_grp: Option<components::MiscFeesGrp>,
        t_price: Option<f32>,
        t_price_type: Option<i64>,
        t_accrued_interest_amt: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_stipulations: Option<components::Stipulations>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 9
    pub struct OrderCancelReject {
        c_standard_header: components::StandardHeader,
        t_order_id: String,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_id: String,
        t_cl_ord_link_id: Option<String>,
        t_orig_cl_ord_id: String,
        t_ord_status: char,
        t_working_indicator: Option<char>,
        t_orig_ord_mod_time: Option<String>,
        t_list_id: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        t_transact_time: Option<String>,
        t_cxl_rej_response_to: char,
        t_cxl_rej_reason: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: BB
    pub struct CollateralInquiry {
        c_standard_header: components::StandardHeader,
        t_coll_inquiry_id: Option<String>,
        c_coll_inq_qual_grp: Option<components::CollInqQualGrp>,
        t_subscription_request_type: Option<char>,
        t_response_transport_type: Option<i64>,
        t_response_destination: Option<String>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_account_type: Option<i64>,
        t_cl_ord_id: Option<String>,
        t_order_id: Option<String>,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        c_exec_coll_grp: Option<components::ExecCollGrp>,
        c_trd_coll_grp: Option<components::TrdCollGrp>,
        c_instrument: Option<components::Instrument>,
        c_financing_details: Option<components::FinancingDetails>,
        t_settl_date: Option<String>,
        t_quantity: Option<f32>,
        t_qty_type: Option<i64>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_margin_excess: Option<f32>,
        t_total_net_value: Option<f32>,
        t_cash_outstanding: Option<f32>,
        c_trd_reg_timestamps: Option<components::TrdRegTimestamps>,
        t_side: Option<char>,
        t_price: Option<f32>,
        t_price_type: Option<i64>,
        t_accrued_interest_amt: Option<f32>,
        t_end_accrued_interest_amt: Option<f32>,
        t_start_cash: Option<f32>,
        t_end_cash: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_stipulations: Option<components::Stipulations>,
        c_settl_instructions_data: Option<components::SettlInstructionsData>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        t_clearing_business_date: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AQ
    pub struct TradeCaptureReportRequestAck {
        c_standard_header: components::StandardHeader,
        t_trade_request_id: String,
        t_trade_request_type: i64,
        t_subscription_request_type: Option<char>,
        t_tot_num_trade_reports: Option<i64>,
        t_trade_request_result: i64,
        t_trade_request_status: i64,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_multi_leg_reporting_type: Option<char>,
        t_response_transport_type: Option<i64>,
        t_response_destination: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: T
    pub struct SettlementInstructions {
        c_standard_header: components::StandardHeader,
        t_settl_inst_msg_id: String,
        t_settl_inst_req_id: Option<String>,
        t_settl_inst_mode: char,
        t_settl_inst_req_rej_code: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_cl_ord_id: Option<String>,
        t_transact_time: String,
        c_settl_inst_grp: Option<components::SettlInstGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: BG
    pub struct CollateralInquiryAck {
        c_standard_header: components::StandardHeader,
        t_coll_inquiry_id: String,
        t_coll_inquiry_status: i64,
        t_coll_inquiry_result: Option<i64>,
        c_coll_inq_qual_grp: Option<components::CollInqQualGrp>,
        t_tot_num_reports: Option<i64>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_account_type: Option<i64>,
        t_cl_ord_id: Option<String>,
        t_order_id: Option<String>,
        t_secondary_order_id: Option<String>,
        t_secondary_cl_ord_id: Option<String>,
        c_exec_coll_grp: Option<components::ExecCollGrp>,
        c_trd_coll_grp: Option<components::TrdCollGrp>,
        c_instrument: Option<components::Instrument>,
        c_financing_details: Option<components::FinancingDetails>,
        t_settl_date: Option<String>,
        t_quantity: Option<f32>,
        t_qty_type: Option<i64>,
        t_currency: Option<String>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_settl_sess_id: Option<String>,
        t_settl_sess_sub_id: Option<String>,
        t_clearing_business_date: Option<String>,
        t_response_transport_type: Option<i64>,
        t_response_destination: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 3
    pub struct Reject {
        c_standard_header: components::StandardHeader,
        t_ref_seq_num: i64,
        t_ref_tag_id: Option<i64>,
        t_ref_msg_type: Option<String>,
        t_session_reject_reason: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: 2
    pub struct ResendRequest {
        c_standard_header: components::StandardHeader,
        t_begin_seq_no: i64,
        t_end_seq_no: i64,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: G
    pub struct OrderCancelReplaceRequest {
        c_standard_header: components::StandardHeader,
        t_order_id: Option<String>,
        c_parties: Option<components::Parties>,
        t_trade_origination_date: Option<String>,
        t_trade_date: Option<String>,
        t_orig_cl_ord_id: String,
        t_cl_ord_id: String,
        t_secondary_cl_ord_id: Option<String>,
        t_cl_ord_link_id: Option<String>,
        t_list_id: Option<String>,
        t_orig_ord_mod_time: Option<String>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_day_booking_inst: Option<char>,
        t_booking_unit: Option<char>,
        t_prealloc_method: Option<char>,
        t_alloc_id: Option<String>,
        c_pre_alloc_grp: Option<components::PreAllocGrp>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_cash_margin: Option<char>,
        t_clearing_fee_indicator: Option<String>,
        t_handl_inst: Option<char>,
        t_exec_inst: Option<String>,
        t_min_qty: Option<f32>,
        t_max_floor: Option<f32>,
        t_ex_destination: Option<String>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        t_side: char,
        t_transact_time: String,
        t_qty_type: Option<i64>,
        c_order_qty_data: components::OrderQtyData,
        t_ord_type: char,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_stop_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        c_peg_instructions: Option<components::PegInstructions>,
        c_discretion_instructions: Option<components::DiscretionInstructions>,
        t_target_strategy: Option<i64>,
        t_target_strategy_parameters: Option<String>,
        t_participation_rate: Option<f32>,
        t_compliance_id: Option<String>,
        t_solicited_flag: Option<char>,
        t_currency: Option<String>,
        t_time_in_force: Option<char>,
        t_effective_time: Option<String>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_gt_booking_inst: Option<i64>,
        c_commission_data: Option<components::CommissionData>,
        t_order_capacity: Option<char>,
        t_order_restrictions: Option<String>,
        t_cust_order_capacity: Option<i64>,
        t_forex_req: Option<char>,
        t_settl_currency: Option<String>,
        t_booking_type: Option<i64>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        t_settl_date_2: Option<String>,
        t_order_qty_2: Option<f32>,
        t_price_2: Option<f32>,
        t_position_effect: Option<char>,
        t_covered_or_uncovered: Option<i64>,
        t_max_show: Option<f32>,
        t_locate_reqd: Option<char>,
        t_cancellation_rights: Option<char>,
        t_money_laundering_status: Option<char>,
        t_regist_id: Option<String>,
        t_designation: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: a
    pub struct QuoteStatusRequest {
        c_standard_header: components::StandardHeader,
        t_quote_status_req_id: Option<String>,
        t_quote_id: Option<String>,
        c_instrument: components::Instrument,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_parties: Option<components::Parties>,
        t_account: Option<String>,
        t_acct_id_source: Option<i64>,
        t_account_type: Option<i64>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_subscription_request_type: Option<char>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AD
    pub struct TradeCaptureReportRequest {
        c_standard_header: components::StandardHeader,
        t_trade_request_id: String,
        t_trade_request_type: i64,
        t_subscription_request_type: Option<char>,
        t_trade_report_id: Option<String>,
        t_secondary_trade_report_id: Option<String>,
        t_exec_id: Option<String>,
        t_exec_type: Option<char>,
        t_order_id: Option<String>,
        t_cl_ord_id: Option<String>,
        t_match_status: Option<char>,
        t_trd_type: Option<i64>,
        t_trd_sub_type: Option<i64>,
        t_transfer_reason: Option<String>,
        t_secondary_trd_type: Option<i64>,
        t_trade_link_id: Option<String>,
        t_trd_match_id: Option<String>,
        c_parties: Option<components::Parties>,
        c_instrument: Option<components::Instrument>,
        c_instrument_extension: Option<components::InstrumentExtension>,
        c_financing_details: Option<components::FinancingDetails>,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        c_trd_cap_dt_grp: Option<components::TrdCapDtGrp>,
        t_clearing_business_date: Option<String>,
        t_trading_session_id: Option<String>,
        t_trading_session_sub_id: Option<String>,
        t_time_bracket: Option<String>,
        t_side: Option<char>,
        t_multi_leg_reporting_type: Option<char>,
        t_trade_input_source: Option<String>,
        t_trade_input_device: Option<String>,
        t_response_transport_type: Option<i64>,
        t_response_destination: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: AT
    pub struct AllocationReportAck {
        c_standard_header: components::StandardHeader,
        t_alloc_report_id: String,
        t_alloc_id: String,
        c_parties: Option<components::Parties>,
        t_secondary_alloc_id: Option<String>,
        t_trade_date: Option<String>,
        t_transact_time: String,
        t_alloc_status: i64,
        t_alloc_rej_code: Option<i64>,
        t_alloc_report_type: Option<i64>,
        t_alloc_intermed_req_type: Option<i64>,
        t_match_status: Option<char>,
        t_product: Option<i64>,
        t_security_type: Option<String>,
        t_text: Option<String>,
        t_encoded_text_len: Option<i64>,
        t_encoded_text: Option<String>,
        c_alloc_ack_grp: Option<components::AllocAckGrp>,
        c_standard_trailer: components::StandardTrailer,
    }

    /// # Message information:
    /// 
    /// Message type: s
    pub struct NewOrderCross {
        c_standard_header: components::StandardHeader,
        t_cross_id: String,
        t_cross_type: i64,
        t_cross_prioritization: i64,
        c_side_cross_ord_mod_grp: components::SideCrossOrdModGrp,
        c_instrument: components::Instrument,
        c_und_instrmt_grp: Option<components::UndInstrmtGrp>,
        c_instrmt_leg_grp: Option<components::InstrmtLegGrp>,
        t_settl_type: Option<char>,
        t_settl_date: Option<String>,
        t_handl_inst: Option<char>,
        t_exec_inst: Option<String>,
        t_min_qty: Option<f32>,
        t_max_floor: Option<f32>,
        t_ex_destination: Option<String>,
        c_trdg_ses_grp: Option<components::TrdgSesGrp>,
        t_process_code: Option<char>,
        t_prev_close_px: Option<f32>,
        t_locate_reqd: Option<char>,
        t_transact_time: String,
        c_stipulations: Option<components::Stipulations>,
        t_ord_type: char,
        t_price_type: Option<i64>,
        t_price: Option<f32>,
        t_stop_px: Option<f32>,
        c_spread_or_benchmark_curve_data: Option<components::SpreadOrBenchmarkCurveData>,
        c_yield_data: Option<components::YieldData>,
        t_currency: Option<String>,
        t_compliance_id: Option<String>,
        t_ioiid: Option<String>,
        t_quote_id: Option<String>,
        t_time_in_force: Option<char>,
        t_effective_time: Option<String>,
        t_expire_date: Option<String>,
        t_expire_time: Option<String>,
        t_gt_booking_inst: Option<i64>,
        t_max_show: Option<f32>,
        c_peg_instructions: Option<components::PegInstructions>,
        c_discretion_instructions: Option<components::DiscretionInstructions>,
        t_target_strategy: Option<i64>,
        t_target_strategy_parameters: Option<String>,
        t_participation_rate: Option<f32>,
        t_cancellation_rights: Option<char>,
        t_money_laundering_status: Option<char>,
        t_regist_id: Option<String>,
        t_designation: Option<String>,
        c_standard_trailer: components::StandardTrailer,
    }
}