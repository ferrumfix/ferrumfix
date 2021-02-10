#![allow(dead_code, missing_docs)]

use fefix_derive::*;

pub mod components {
    use super::*;

    /// Component information: StandardHeader
    #[fefix(msg_type = "TODO")]
    #[derive(Debug, Clone, TsrMessage)]
    pub struct StandardHeader {
        #[fefix(tag = 8, rust_type = "", opt = false)]
        pub begin_string: String,
        #[fefix(tag = 9, rust_type = "", opt = false)]
        pub body_length: i64,
        #[fefix(tag = 35, rust_type = "", opt = false)]
        pub msg_type: String,
        #[fefix(tag = 49, rust_type = "", opt = false)]
        pub sender_comp_id: String,
        #[fefix(tag = 56, rust_type = "", opt = false)]
        pub target_comp_id: String,
        #[fefix(tag = 115, rust_type = "", opt = true)]
        pub on_behalf_of_comp_id: ::std::option::Option<String>,
        #[fefix(tag = 128, rust_type = "", opt = true)]
        pub deliver_to_comp_id: ::std::option::Option<String>,
        #[fefix(tag = 90, rust_type = "", opt = true)]
        pub secure_data_len: ::std::option::Option<usize>,
        #[fefix(tag = 91, rust_type = "", opt = true)]
        pub secure_data: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 34, rust_type = "", opt = false)]
        pub msg_seq_num: i64,
        #[fefix(tag = 50, rust_type = "", opt = true)]
        pub sender_sub_id: ::std::option::Option<String>,
        #[fefix(tag = 142, rust_type = "", opt = true)]
        pub sender_location_id: ::std::option::Option<String>,
        #[fefix(tag = 57, rust_type = "", opt = true)]
        pub target_sub_id: ::std::option::Option<String>,
        #[fefix(tag = 143, rust_type = "", opt = true)]
        pub target_location_id: ::std::option::Option<String>,
        #[fefix(tag = 116, rust_type = "", opt = true)]
        pub on_behalf_of_sub_id: ::std::option::Option<String>,
        #[fefix(tag = 144, rust_type = "", opt = true)]
        pub on_behalf_of_location_id: ::std::option::Option<String>,
        #[fefix(tag = 129, rust_type = "", opt = true)]
        pub deliver_to_sub_id: ::std::option::Option<String>,
        #[fefix(tag = 145, rust_type = "", opt = true)]
        pub deliver_to_location_id: ::std::option::Option<String>,
        #[fefix(tag = 43, rust_type = "", opt = true)]
        pub poss_dup_flag: ::std::option::Option<bool>,
        #[fefix(tag = 97, rust_type = "", opt = true)]
        pub poss_resend: ::std::option::Option<bool>,
        #[fefix(tag = 52, rust_type = "", opt = false)]
        pub sending_time: Vec<u8>,
        #[fefix(tag = 122, rust_type = "", opt = true)]
        pub orig_sending_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 212, rust_type = "", opt = true)]
        pub xml_data_len: ::std::option::Option<usize>,
        #[fefix(tag = 213, rust_type = "", opt = true)]
        pub xml_data: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 347, rust_type = "", opt = true)]
        pub message_encoding: ::std::option::Option<String>,
        #[fefix(tag = 369, rust_type = "", opt = true)]
        pub last_msg_seq_num_processed: ::std::option::Option<i64>,
        #[fefix(tag = 370, rust_type = "", opt = true)]
        pub on_behalf_of_sending_time: ::std::option::Option<Vec<u8>>,
    }

    /// Component information: StandardTrailer
    #[fefix(msg_type = "TODO")]
    #[derive(Debug, Clone, TsrMessage)]
    pub struct StandardTrailer {
        #[fefix(tag = 93, rust_type = "", opt = true)]
        pub signature_length: ::std::option::Option<usize>,
        #[fefix(tag = 89, rust_type = "", opt = true)]
        pub signature: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 10, rust_type = "", opt = false)]
        pub check_sum: String,
    }
}

pub mod messages {
    use super::*;

    /// Message information: Heartbeat
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "0")]
    pub struct Heartbeat {
        #[fefix(tag = 112, rust_type = "", opt = true)]
        pub test_req_id: ::std::option::Option<String>,
    }

    /// Message information: TestRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "1")]
    pub struct TestRequest {
        #[fefix(tag = 112, rust_type = "", opt = false)]
        pub test_req_id: String,
    }

    /// Message information: ResendRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "2")]
    pub struct ResendRequest {
        #[fefix(tag = 7, rust_type = "", opt = false)]
        pub begin_seq_no: i64,
        #[fefix(tag = 16, rust_type = "", opt = false)]
        pub end_seq_no: i64,
    }

    /// Message information: Reject
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "3")]
    pub struct Reject {
        #[fefix(tag = 45, rust_type = "", opt = false)]
        pub ref_seq_num: i64,
        #[fefix(tag = 371, rust_type = "", opt = true)]
        pub ref_tag_id: ::std::option::Option<i64>,
        #[fefix(tag = 372, rust_type = "", opt = true)]
        pub ref_msg_type: ::std::option::Option<String>,
        #[fefix(tag = 373, rust_type = "", opt = true)]
        pub session_reject_reason: ::std::option::Option<i64>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: SequenceReset
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "4")]
    pub struct SequenceReset {
        #[fefix(tag = 123, rust_type = "", opt = true)]
        pub gap_fill_flag: ::std::option::Option<bool>,
        #[fefix(tag = 36, rust_type = "", opt = false)]
        pub new_seq_no: i64,
    }

    /// Message information: Logout
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "5")]
    pub struct Logout {
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: IOI
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "6")]
    pub struct IOI {
        #[fefix(tag = 23, rust_type = "", opt = false)]
        pub io_iid: String,
        #[fefix(tag = 28, rust_type = "", opt = false)]
        pub ioi_trans_type: char,
        #[fefix(tag = 26, rust_type = "", opt = true)]
        pub ioi_ref_id: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 54, rust_type = "", opt = false)]
        pub side: char,
        #[fefix(tag = 27, rust_type = "", opt = false)]
        pub ioi_shares: String,
        #[fefix(tag = 44, rust_type = "", opt = true)]
        pub price: ::std::option::Option<f64>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 62, rust_type = "", opt = true)]
        pub valid_until_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 25, rust_type = "", opt = true)]
        pub ioi_qlty_ind: ::std::option::Option<char>,
        #[fefix(tag = 130, rust_type = "", opt = true)]
        pub ioi_natural_flag: ::std::option::Option<bool>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 149, rust_type = "", opt = true)]
        pub url_link: ::std::option::Option<String>,
        #[fefix(tag = 218, rust_type = "", opt = true)]
        pub spread_to_benchmark: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 219, rust_type = "", opt = true)]
        pub benchmark: ::std::option::Option<char>,
    }

    /// Message information: Advertisement
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "7")]
    pub struct Advertisement {
        #[fefix(tag = 2, rust_type = "", opt = false)]
        pub adv_id: String,
        #[fefix(tag = 5, rust_type = "", opt = false)]
        pub adv_trans_type: String,
        #[fefix(tag = 3, rust_type = "", opt = true)]
        pub adv_ref_id: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 4, rust_type = "", opt = false)]
        pub adv_side: char,
        #[fefix(tag = 53, rust_type = "", opt = false)]
        pub shares: Vec<u8>,
        #[fefix(tag = 44, rust_type = "", opt = true)]
        pub price: ::std::option::Option<f64>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 75, rust_type = "", opt = true)]
        pub trade_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 149, rust_type = "", opt = true)]
        pub url_link: ::std::option::Option<String>,
        #[fefix(tag = 30, rust_type = "", opt = true)]
        pub last_mkt: ::std::option::Option<String>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
    }

    /// Message information: ExecutionReport
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "8")]
    pub struct ExecutionReport {
        #[fefix(tag = 37, rust_type = "", opt = false)]
        pub order_id: String,
        #[fefix(tag = 198, rust_type = "", opt = true)]
        pub secondary_order_id: ::std::option::Option<String>,
        #[fefix(tag = 11, rust_type = "", opt = true)]
        pub cl_ord_id: ::std::option::Option<String>,
        #[fefix(tag = 41, rust_type = "", opt = true)]
        pub orig_cl_ord_id: ::std::option::Option<String>,
        #[fefix(tag = 109, rust_type = "", opt = true)]
        pub client_id: ::std::option::Option<String>,
        #[fefix(tag = 76, rust_type = "", opt = true)]
        pub exec_broker: ::std::option::Option<String>,
        #[fefix(tag = 66, rust_type = "", opt = true)]
        pub list_id: ::std::option::Option<String>,
        #[fefix(tag = 17, rust_type = "", opt = false)]
        pub exec_id: String,
        #[fefix(tag = 20, rust_type = "", opt = false)]
        pub exec_trans_type: char,
        #[fefix(tag = 19, rust_type = "", opt = true)]
        pub exec_ref_id: ::std::option::Option<String>,
        #[fefix(tag = 150, rust_type = "", opt = false)]
        pub exec_type: char,
        #[fefix(tag = 39, rust_type = "", opt = false)]
        pub ord_status: char,
        #[fefix(tag = 103, rust_type = "", opt = true)]
        pub ord_rej_reason: ::std::option::Option<i64>,
        #[fefix(tag = 378, rust_type = "", opt = true)]
        pub exec_restatement_reason: ::std::option::Option<i64>,
        #[fefix(tag = 1, rust_type = "", opt = true)]
        pub account: ::std::option::Option<String>,
        #[fefix(tag = 63, rust_type = "", opt = true)]
        pub settlmnt_typ: ::std::option::Option<char>,
        #[fefix(tag = 64, rust_type = "", opt = true)]
        pub fut_sett_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 54, rust_type = "", opt = false)]
        pub side: char,
        #[fefix(tag = 38, rust_type = "", opt = true)]
        pub order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 152, rust_type = "", opt = true)]
        pub cash_order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 40, rust_type = "", opt = true)]
        pub ord_type: ::std::option::Option<char>,
        #[fefix(tag = 44, rust_type = "", opt = true)]
        pub price: ::std::option::Option<f64>,
        #[fefix(tag = 99, rust_type = "", opt = true)]
        pub stop_px: ::std::option::Option<f64>,
        #[fefix(tag = 211, rust_type = "", opt = true)]
        pub peg_difference: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 388, rust_type = "", opt = true)]
        pub discretion_inst: ::std::option::Option<char>,
        #[fefix(tag = 389, rust_type = "", opt = true)]
        pub discretion_offset: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 376, rust_type = "", opt = true)]
        pub compliance_id: ::std::option::Option<String>,
        #[fefix(tag = 377, rust_type = "", opt = true)]
        pub solicited_flag: ::std::option::Option<bool>,
        #[fefix(tag = 59, rust_type = "", opt = true)]
        pub time_in_force: ::std::option::Option<char>,
        #[fefix(tag = 168, rust_type = "", opt = true)]
        pub effective_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 432, rust_type = "", opt = true)]
        pub expire_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 126, rust_type = "", opt = true)]
        pub expire_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 18, rust_type = "", opt = true)]
        pub exec_inst: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 47, rust_type = "", opt = true)]
        pub rule_80a: ::std::option::Option<char>,
        #[fefix(tag = 32, rust_type = "", opt = true)]
        pub last_shares: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 31, rust_type = "", opt = true)]
        pub last_px: ::std::option::Option<f64>,
        #[fefix(tag = 194, rust_type = "", opt = true)]
        pub last_spot_rate: ::std::option::Option<f64>,
        #[fefix(tag = 195, rust_type = "", opt = true)]
        pub last_forward_points: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 30, rust_type = "", opt = true)]
        pub last_mkt: ::std::option::Option<String>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
        #[fefix(tag = 29, rust_type = "", opt = true)]
        pub last_capacity: ::std::option::Option<char>,
        #[fefix(tag = 151, rust_type = "", opt = false)]
        pub leaves_qty: Vec<u8>,
        #[fefix(tag = 14, rust_type = "", opt = false)]
        pub cum_qty: Vec<u8>,
        #[fefix(tag = 6, rust_type = "", opt = false)]
        pub avg_px: f64,
        #[fefix(tag = 424, rust_type = "", opt = true)]
        pub day_order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 425, rust_type = "", opt = true)]
        pub day_cum_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 426, rust_type = "", opt = true)]
        pub day_avg_px: ::std::option::Option<f64>,
        #[fefix(tag = 427, rust_type = "", opt = true)]
        pub gt_booking_inst: ::std::option::Option<i64>,
        #[fefix(tag = 75, rust_type = "", opt = true)]
        pub trade_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 113, rust_type = "", opt = true)]
        pub report_to_exch: ::std::option::Option<bool>,
        #[fefix(tag = 12, rust_type = "", opt = true)]
        pub commission: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 13, rust_type = "", opt = true)]
        pub comm_type: ::std::option::Option<char>,
        #[fefix(tag = 381, rust_type = "", opt = true)]
        pub gross_trade_amt: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 119, rust_type = "", opt = true)]
        pub settl_curr_amt: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 120, rust_type = "", opt = true)]
        pub settl_currency: ::std::option::Option<String>,
        #[fefix(tag = 155, rust_type = "", opt = true)]
        pub settl_curr_fx_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 156, rust_type = "", opt = true)]
        pub settl_curr_fx_rate_calc: ::std::option::Option<char>,
        #[fefix(tag = 21, rust_type = "", opt = true)]
        pub handl_inst: ::std::option::Option<char>,
        #[fefix(tag = 110, rust_type = "", opt = true)]
        pub min_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 111, rust_type = "", opt = true)]
        pub max_floor: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 77, rust_type = "", opt = true)]
        pub open_close: ::std::option::Option<char>,
        #[fefix(tag = 210, rust_type = "", opt = true)]
        pub max_show: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 193, rust_type = "", opt = true)]
        pub fut_sett_date_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 192, rust_type = "", opt = true)]
        pub order_qty_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 439, rust_type = "", opt = true)]
        pub clearing_firm: ::std::option::Option<String>,
        #[fefix(tag = 440, rust_type = "", opt = true)]
        pub clearing_account: ::std::option::Option<String>,
        #[fefix(tag = 442, rust_type = "", opt = true)]
        pub multi_leg_reporting_type: ::std::option::Option<char>,
    }

    /// Message information: OrderCancelReject
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "9")]
    pub struct OrderCancelReject {
        #[fefix(tag = 37, rust_type = "", opt = false)]
        pub order_id: String,
        #[fefix(tag = 198, rust_type = "", opt = true)]
        pub secondary_order_id: ::std::option::Option<String>,
        #[fefix(tag = 11, rust_type = "", opt = false)]
        pub cl_ord_id: String,
        #[fefix(tag = 41, rust_type = "", opt = false)]
        pub orig_cl_ord_id: String,
        #[fefix(tag = 39, rust_type = "", opt = false)]
        pub ord_status: char,
        #[fefix(tag = 109, rust_type = "", opt = true)]
        pub client_id: ::std::option::Option<String>,
        #[fefix(tag = 76, rust_type = "", opt = true)]
        pub exec_broker: ::std::option::Option<String>,
        #[fefix(tag = 66, rust_type = "", opt = true)]
        pub list_id: ::std::option::Option<String>,
        #[fefix(tag = 1, rust_type = "", opt = true)]
        pub account: ::std::option::Option<String>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 434, rust_type = "", opt = false)]
        pub cxl_rej_response_to: char,
        #[fefix(tag = 102, rust_type = "", opt = true)]
        pub cxl_rej_reason: ::std::option::Option<i64>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: Logon
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "A")]
    pub struct Logon {
        #[fefix(tag = 98, rust_type = "", opt = false)]
        pub encrypt_method: i64,
        #[fefix(tag = 108, rust_type = "", opt = false)]
        pub heart_bt_int: i64,
        #[fefix(tag = 95, rust_type = "", opt = true)]
        pub raw_data_length: ::std::option::Option<usize>,
        #[fefix(tag = 96, rust_type = "", opt = true)]
        pub raw_data: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 141, rust_type = "", opt = true)]
        pub reset_seq_num_flag: ::std::option::Option<bool>,
        #[fefix(tag = 383, rust_type = "", opt = true)]
        pub max_message_size: ::std::option::Option<i64>,
    }

    /// Message information: News
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "B")]
    pub struct News {
        #[fefix(tag = 42, rust_type = "", opt = true)]
        pub orig_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 61, rust_type = "", opt = true)]
        pub urgency: ::std::option::Option<char>,
        #[fefix(tag = 148, rust_type = "", opt = false)]
        pub headline: String,
        #[fefix(tag = 358, rust_type = "", opt = true)]
        pub encoded_headline_len: ::std::option::Option<usize>,
        #[fefix(tag = 359, rust_type = "", opt = true)]
        pub encoded_headline: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 149, rust_type = "", opt = true)]
        pub url_link: ::std::option::Option<String>,
        #[fefix(tag = 95, rust_type = "", opt = true)]
        pub raw_data_length: ::std::option::Option<usize>,
        #[fefix(tag = 96, rust_type = "", opt = true)]
        pub raw_data: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: Email
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "C")]
    pub struct Email {
        #[fefix(tag = 164, rust_type = "", opt = false)]
        pub email_thread_id: String,
        #[fefix(tag = 94, rust_type = "", opt = false)]
        pub email_type: char,
        #[fefix(tag = 42, rust_type = "", opt = true)]
        pub orig_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 147, rust_type = "", opt = false)]
        pub subject: String,
        #[fefix(tag = 356, rust_type = "", opt = true)]
        pub encoded_subject_len: ::std::option::Option<usize>,
        #[fefix(tag = 357, rust_type = "", opt = true)]
        pub encoded_subject: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 37, rust_type = "", opt = true)]
        pub order_id: ::std::option::Option<String>,
        #[fefix(tag = 11, rust_type = "", opt = true)]
        pub cl_ord_id: ::std::option::Option<String>,
        #[fefix(tag = 95, rust_type = "", opt = true)]
        pub raw_data_length: ::std::option::Option<usize>,
        #[fefix(tag = 96, rust_type = "", opt = true)]
        pub raw_data: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: NewOrderSingle
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "D")]
    pub struct NewOrderSingle {
        #[fefix(tag = 11, rust_type = "", opt = false)]
        pub cl_ord_id: String,
        #[fefix(tag = 109, rust_type = "", opt = true)]
        pub client_id: ::std::option::Option<String>,
        #[fefix(tag = 76, rust_type = "", opt = true)]
        pub exec_broker: ::std::option::Option<String>,
        #[fefix(tag = 1, rust_type = "", opt = true)]
        pub account: ::std::option::Option<String>,
        #[fefix(tag = 63, rust_type = "", opt = true)]
        pub settlmnt_typ: ::std::option::Option<char>,
        #[fefix(tag = 64, rust_type = "", opt = true)]
        pub fut_sett_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 21, rust_type = "", opt = false)]
        pub handl_inst: char,
        #[fefix(tag = 18, rust_type = "", opt = true)]
        pub exec_inst: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 110, rust_type = "", opt = true)]
        pub min_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 111, rust_type = "", opt = true)]
        pub max_floor: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 100, rust_type = "", opt = true)]
        pub ex_destination: ::std::option::Option<String>,
        #[fefix(tag = 81, rust_type = "", opt = true)]
        pub process_code: ::std::option::Option<char>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 140, rust_type = "", opt = true)]
        pub prev_close_px: ::std::option::Option<f64>,
        #[fefix(tag = 54, rust_type = "", opt = false)]
        pub side: char,
        #[fefix(tag = 114, rust_type = "", opt = true)]
        pub locate_reqd: ::std::option::Option<bool>,
        #[fefix(tag = 60, rust_type = "", opt = false)]
        pub transact_time: Vec<u8>,
        #[fefix(tag = 38, rust_type = "", opt = true)]
        pub order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 152, rust_type = "", opt = true)]
        pub cash_order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 40, rust_type = "", opt = false)]
        pub ord_type: char,
        #[fefix(tag = 44, rust_type = "", opt = true)]
        pub price: ::std::option::Option<f64>,
        #[fefix(tag = 99, rust_type = "", opt = true)]
        pub stop_px: ::std::option::Option<f64>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 376, rust_type = "", opt = true)]
        pub compliance_id: ::std::option::Option<String>,
        #[fefix(tag = 377, rust_type = "", opt = true)]
        pub solicited_flag: ::std::option::Option<bool>,
        #[fefix(tag = 23, rust_type = "", opt = true)]
        pub io_iid: ::std::option::Option<String>,
        #[fefix(tag = 117, rust_type = "", opt = true)]
        pub quote_id: ::std::option::Option<String>,
        #[fefix(tag = 59, rust_type = "", opt = true)]
        pub time_in_force: ::std::option::Option<char>,
        #[fefix(tag = 168, rust_type = "", opt = true)]
        pub effective_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 432, rust_type = "", opt = true)]
        pub expire_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 126, rust_type = "", opt = true)]
        pub expire_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 427, rust_type = "", opt = true)]
        pub gt_booking_inst: ::std::option::Option<i64>,
        #[fefix(tag = 12, rust_type = "", opt = true)]
        pub commission: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 13, rust_type = "", opt = true)]
        pub comm_type: ::std::option::Option<char>,
        #[fefix(tag = 47, rust_type = "", opt = true)]
        pub rule_80a: ::std::option::Option<char>,
        #[fefix(tag = 121, rust_type = "", opt = true)]
        pub forex_req: ::std::option::Option<bool>,
        #[fefix(tag = 120, rust_type = "", opt = true)]
        pub settl_currency: ::std::option::Option<String>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 193, rust_type = "", opt = true)]
        pub fut_sett_date_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 192, rust_type = "", opt = true)]
        pub order_qty_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 77, rust_type = "", opt = true)]
        pub open_close: ::std::option::Option<char>,
        #[fefix(tag = 203, rust_type = "", opt = true)]
        pub covered_or_uncovered: ::std::option::Option<i64>,
        #[fefix(tag = 204, rust_type = "", opt = true)]
        pub customer_or_firm: ::std::option::Option<i64>,
        #[fefix(tag = 210, rust_type = "", opt = true)]
        pub max_show: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 211, rust_type = "", opt = true)]
        pub peg_difference: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 388, rust_type = "", opt = true)]
        pub discretion_inst: ::std::option::Option<char>,
        #[fefix(tag = 389, rust_type = "", opt = true)]
        pub discretion_offset: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 439, rust_type = "", opt = true)]
        pub clearing_firm: ::std::option::Option<String>,
        #[fefix(tag = 440, rust_type = "", opt = true)]
        pub clearing_account: ::std::option::Option<String>,
    }

    /// Message information: NewOrderList
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "E")]
    pub struct NewOrderList {
        #[fefix(tag = 66, rust_type = "", opt = false)]
        pub list_id: String,
        #[fefix(tag = 390, rust_type = "", opt = true)]
        pub bid_id: ::std::option::Option<String>,
        #[fefix(tag = 391, rust_type = "", opt = true)]
        pub client_bid_id: ::std::option::Option<String>,
        #[fefix(tag = 414, rust_type = "", opt = true)]
        pub prog_rpt_reqs: ::std::option::Option<i64>,
        #[fefix(tag = 394, rust_type = "", opt = false)]
        pub bid_type: i64,
        #[fefix(tag = 415, rust_type = "", opt = true)]
        pub prog_period_interval: ::std::option::Option<i64>,
        #[fefix(tag = 433, rust_type = "", opt = true)]
        pub list_exec_inst_type: ::std::option::Option<char>,
        #[fefix(tag = 69, rust_type = "", opt = true)]
        pub list_exec_inst: ::std::option::Option<String>,
        #[fefix(tag = 352, rust_type = "", opt = true)]
        pub encoded_list_exec_inst_len: ::std::option::Option<usize>,
        #[fefix(tag = 353, rust_type = "", opt = true)]
        pub encoded_list_exec_inst: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 68, rust_type = "", opt = false)]
        pub tot_no_orders: i64,
    }

    /// Message information: OrderCancelRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "F")]
    pub struct OrderCancelRequest {
        #[fefix(tag = 41, rust_type = "", opt = false)]
        pub orig_cl_ord_id: String,
        #[fefix(tag = 37, rust_type = "", opt = true)]
        pub order_id: ::std::option::Option<String>,
        #[fefix(tag = 11, rust_type = "", opt = false)]
        pub cl_ord_id: String,
        #[fefix(tag = 66, rust_type = "", opt = true)]
        pub list_id: ::std::option::Option<String>,
        #[fefix(tag = 1, rust_type = "", opt = true)]
        pub account: ::std::option::Option<String>,
        #[fefix(tag = 109, rust_type = "", opt = true)]
        pub client_id: ::std::option::Option<String>,
        #[fefix(tag = 76, rust_type = "", opt = true)]
        pub exec_broker: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 54, rust_type = "", opt = false)]
        pub side: char,
        #[fefix(tag = 60, rust_type = "", opt = false)]
        pub transact_time: Vec<u8>,
        #[fefix(tag = 38, rust_type = "", opt = true)]
        pub order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 152, rust_type = "", opt = true)]
        pub cash_order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 376, rust_type = "", opt = true)]
        pub compliance_id: ::std::option::Option<String>,
        #[fefix(tag = 377, rust_type = "", opt = true)]
        pub solicited_flag: ::std::option::Option<bool>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: OrderCancelReplaceRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "G")]
    pub struct OrderCancelReplaceRequest {
        #[fefix(tag = 37, rust_type = "", opt = true)]
        pub order_id: ::std::option::Option<String>,
        #[fefix(tag = 109, rust_type = "", opt = true)]
        pub client_id: ::std::option::Option<String>,
        #[fefix(tag = 76, rust_type = "", opt = true)]
        pub exec_broker: ::std::option::Option<String>,
        #[fefix(tag = 41, rust_type = "", opt = false)]
        pub orig_cl_ord_id: String,
        #[fefix(tag = 11, rust_type = "", opt = false)]
        pub cl_ord_id: String,
        #[fefix(tag = 66, rust_type = "", opt = true)]
        pub list_id: ::std::option::Option<String>,
        #[fefix(tag = 1, rust_type = "", opt = true)]
        pub account: ::std::option::Option<String>,
        #[fefix(tag = 63, rust_type = "", opt = true)]
        pub settlmnt_typ: ::std::option::Option<char>,
        #[fefix(tag = 64, rust_type = "", opt = true)]
        pub fut_sett_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 21, rust_type = "", opt = false)]
        pub handl_inst: char,
        #[fefix(tag = 18, rust_type = "", opt = true)]
        pub exec_inst: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 110, rust_type = "", opt = true)]
        pub min_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 111, rust_type = "", opt = true)]
        pub max_floor: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 100, rust_type = "", opt = true)]
        pub ex_destination: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 54, rust_type = "", opt = false)]
        pub side: char,
        #[fefix(tag = 60, rust_type = "", opt = false)]
        pub transact_time: Vec<u8>,
        #[fefix(tag = 38, rust_type = "", opt = true)]
        pub order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 152, rust_type = "", opt = true)]
        pub cash_order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 40, rust_type = "", opt = false)]
        pub ord_type: char,
        #[fefix(tag = 44, rust_type = "", opt = true)]
        pub price: ::std::option::Option<f64>,
        #[fefix(tag = 99, rust_type = "", opt = true)]
        pub stop_px: ::std::option::Option<f64>,
        #[fefix(tag = 211, rust_type = "", opt = true)]
        pub peg_difference: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 388, rust_type = "", opt = true)]
        pub discretion_inst: ::std::option::Option<char>,
        #[fefix(tag = 389, rust_type = "", opt = true)]
        pub discretion_offset: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 376, rust_type = "", opt = true)]
        pub compliance_id: ::std::option::Option<String>,
        #[fefix(tag = 377, rust_type = "", opt = true)]
        pub solicited_flag: ::std::option::Option<bool>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 59, rust_type = "", opt = true)]
        pub time_in_force: ::std::option::Option<char>,
        #[fefix(tag = 168, rust_type = "", opt = true)]
        pub effective_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 432, rust_type = "", opt = true)]
        pub expire_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 126, rust_type = "", opt = true)]
        pub expire_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 427, rust_type = "", opt = true)]
        pub gt_booking_inst: ::std::option::Option<i64>,
        #[fefix(tag = 12, rust_type = "", opt = true)]
        pub commission: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 13, rust_type = "", opt = true)]
        pub comm_type: ::std::option::Option<char>,
        #[fefix(tag = 47, rust_type = "", opt = true)]
        pub rule_80a: ::std::option::Option<char>,
        #[fefix(tag = 121, rust_type = "", opt = true)]
        pub forex_req: ::std::option::Option<bool>,
        #[fefix(tag = 120, rust_type = "", opt = true)]
        pub settl_currency: ::std::option::Option<String>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 193, rust_type = "", opt = true)]
        pub fut_sett_date_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 192, rust_type = "", opt = true)]
        pub order_qty_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 77, rust_type = "", opt = true)]
        pub open_close: ::std::option::Option<char>,
        #[fefix(tag = 203, rust_type = "", opt = true)]
        pub covered_or_uncovered: ::std::option::Option<i64>,
        #[fefix(tag = 204, rust_type = "", opt = true)]
        pub customer_or_firm: ::std::option::Option<i64>,
        #[fefix(tag = 210, rust_type = "", opt = true)]
        pub max_show: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 114, rust_type = "", opt = true)]
        pub locate_reqd: ::std::option::Option<bool>,
        #[fefix(tag = 439, rust_type = "", opt = true)]
        pub clearing_firm: ::std::option::Option<String>,
        #[fefix(tag = 440, rust_type = "", opt = true)]
        pub clearing_account: ::std::option::Option<String>,
    }

    /// Message information: OrderStatusRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "H")]
    pub struct OrderStatusRequest {
        #[fefix(tag = 37, rust_type = "", opt = true)]
        pub order_id: ::std::option::Option<String>,
        #[fefix(tag = 11, rust_type = "", opt = false)]
        pub cl_ord_id: String,
        #[fefix(tag = 109, rust_type = "", opt = true)]
        pub client_id: ::std::option::Option<String>,
        #[fefix(tag = 1, rust_type = "", opt = true)]
        pub account: ::std::option::Option<String>,
        #[fefix(tag = 76, rust_type = "", opt = true)]
        pub exec_broker: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 54, rust_type = "", opt = false)]
        pub side: char,
    }

    /// Message information: Allocation
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "J")]
    pub struct Allocation {
        #[fefix(tag = 70, rust_type = "", opt = false)]
        pub alloc_id: String,
        #[fefix(tag = 71, rust_type = "", opt = false)]
        pub alloc_trans_type: char,
        #[fefix(tag = 72, rust_type = "", opt = true)]
        pub ref_alloc_id: ::std::option::Option<String>,
        #[fefix(tag = 196, rust_type = "", opt = true)]
        pub alloc_link_id: ::std::option::Option<String>,
        #[fefix(tag = 197, rust_type = "", opt = true)]
        pub alloc_link_type: ::std::option::Option<i64>,
        #[fefix(tag = 54, rust_type = "", opt = false)]
        pub side: char,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 53, rust_type = "", opt = false)]
        pub shares: Vec<u8>,
        #[fefix(tag = 30, rust_type = "", opt = true)]
        pub last_mkt: ::std::option::Option<String>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
        #[fefix(tag = 6, rust_type = "", opt = false)]
        pub avg_px: f64,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 74, rust_type = "", opt = true)]
        pub avg_prx_precision: ::std::option::Option<i64>,
        #[fefix(tag = 75, rust_type = "", opt = false)]
        pub trade_date: Vec<u8>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 63, rust_type = "", opt = true)]
        pub settlmnt_typ: ::std::option::Option<char>,
        #[fefix(tag = 64, rust_type = "", opt = true)]
        pub fut_sett_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 381, rust_type = "", opt = true)]
        pub gross_trade_amt: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 118, rust_type = "", opt = true)]
        pub net_money: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 77, rust_type = "", opt = true)]
        pub open_close: ::std::option::Option<char>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 157, rust_type = "", opt = true)]
        pub num_days_interest: ::std::option::Option<i64>,
        #[fefix(tag = 158, rust_type = "", opt = true)]
        pub accrued_interest_rate: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: ListCancelRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "K")]
    pub struct ListCancelRequest {
        #[fefix(tag = 66, rust_type = "", opt = false)]
        pub list_id: String,
        #[fefix(tag = 60, rust_type = "", opt = false)]
        pub transact_time: Vec<u8>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: ListExecute
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "L")]
    pub struct ListExecute {
        #[fefix(tag = 66, rust_type = "", opt = false)]
        pub list_id: String,
        #[fefix(tag = 391, rust_type = "", opt = true)]
        pub client_bid_id: ::std::option::Option<String>,
        #[fefix(tag = 390, rust_type = "", opt = true)]
        pub bid_id: ::std::option::Option<String>,
        #[fefix(tag = 60, rust_type = "", opt = false)]
        pub transact_time: Vec<u8>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: ListStatusRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "M")]
    pub struct ListStatusRequest {
        #[fefix(tag = 66, rust_type = "", opt = false)]
        pub list_id: String,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: ListStatus
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "N")]
    pub struct ListStatus {
        #[fefix(tag = 66, rust_type = "", opt = false)]
        pub list_id: String,
        #[fefix(tag = 429, rust_type = "", opt = false)]
        pub list_status_type: i64,
        #[fefix(tag = 82, rust_type = "", opt = false)]
        pub no_rpts: i64,
        #[fefix(tag = 431, rust_type = "", opt = false)]
        pub list_order_status: i64,
        #[fefix(tag = 83, rust_type = "", opt = false)]
        pub rpt_seq: i64,
        #[fefix(tag = 444, rust_type = "", opt = true)]
        pub list_status_text: ::std::option::Option<String>,
        #[fefix(tag = 445, rust_type = "", opt = true)]
        pub encoded_list_status_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 446, rust_type = "", opt = true)]
        pub encoded_list_status_text: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 68, rust_type = "", opt = false)]
        pub tot_no_orders: i64,
    }

    /// Message information: AllocationInstructionAck
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "P")]
    pub struct AllocationInstructionAck {
        #[fefix(tag = 109, rust_type = "", opt = true)]
        pub client_id: ::std::option::Option<String>,
        #[fefix(tag = 76, rust_type = "", opt = true)]
        pub exec_broker: ::std::option::Option<String>,
        #[fefix(tag = 70, rust_type = "", opt = false)]
        pub alloc_id: String,
        #[fefix(tag = 75, rust_type = "", opt = false)]
        pub trade_date: Vec<u8>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 87, rust_type = "", opt = false)]
        pub alloc_status: i64,
        #[fefix(tag = 88, rust_type = "", opt = true)]
        pub alloc_rej_code: ::std::option::Option<i64>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: DontKnowTrade
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "Q")]
    pub struct DontKnowTrade {
        #[fefix(tag = 37, rust_type = "", opt = false)]
        pub order_id: String,
        #[fefix(tag = 17, rust_type = "", opt = false)]
        pub exec_id: String,
        #[fefix(tag = 127, rust_type = "", opt = false)]
        pub dk_reason: char,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 54, rust_type = "", opt = false)]
        pub side: char,
        #[fefix(tag = 38, rust_type = "", opt = true)]
        pub order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 152, rust_type = "", opt = true)]
        pub cash_order_qty: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 32, rust_type = "", opt = true)]
        pub last_shares: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 31, rust_type = "", opt = true)]
        pub last_px: ::std::option::Option<f64>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: QuoteRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "R")]
    pub struct QuoteRequest {
        #[fefix(tag = 131, rust_type = "", opt = false)]
        pub quote_req_id: String,
    }

    /// Message information: Quote
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "S")]
    pub struct Quote {
        #[fefix(tag = 131, rust_type = "", opt = true)]
        pub quote_req_id: ::std::option::Option<String>,
        #[fefix(tag = 117, rust_type = "", opt = false)]
        pub quote_id: String,
        #[fefix(tag = 301, rust_type = "", opt = true)]
        pub quote_response_level: ::std::option::Option<i64>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 132, rust_type = "", opt = true)]
        pub bid_px: ::std::option::Option<f64>,
        #[fefix(tag = 133, rust_type = "", opt = true)]
        pub offer_px: ::std::option::Option<f64>,
        #[fefix(tag = 134, rust_type = "", opt = true)]
        pub bid_size: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 135, rust_type = "", opt = true)]
        pub offer_size: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 62, rust_type = "", opt = true)]
        pub valid_until_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 188, rust_type = "", opt = true)]
        pub bid_spot_rate: ::std::option::Option<f64>,
        #[fefix(tag = 190, rust_type = "", opt = true)]
        pub offer_spot_rate: ::std::option::Option<f64>,
        #[fefix(tag = 189, rust_type = "", opt = true)]
        pub bid_forward_points: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 191, rust_type = "", opt = true)]
        pub offer_forward_points: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 64, rust_type = "", opt = true)]
        pub fut_sett_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 40, rust_type = "", opt = true)]
        pub ord_type: ::std::option::Option<char>,
        #[fefix(tag = 193, rust_type = "", opt = true)]
        pub fut_sett_date_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 192, rust_type = "", opt = true)]
        pub order_qty_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
    }

    /// Message information: SettlementInstructions
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "T")]
    pub struct SettlementInstructions {
        #[fefix(tag = 162, rust_type = "", opt = false)]
        pub settl_inst_id: String,
        #[fefix(tag = 163, rust_type = "", opt = false)]
        pub settl_inst_trans_type: char,
        #[fefix(tag = 214, rust_type = "", opt = false)]
        pub settl_inst_ref_id: String,
        #[fefix(tag = 160, rust_type = "", opt = false)]
        pub settl_inst_mode: char,
        #[fefix(tag = 165, rust_type = "", opt = false)]
        pub settl_inst_source: char,
        #[fefix(tag = 79, rust_type = "", opt = false)]
        pub alloc_account: String,
        #[fefix(tag = 166, rust_type = "", opt = true)]
        pub settl_location: ::std::option::Option<String>,
        #[fefix(tag = 75, rust_type = "", opt = true)]
        pub trade_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 70, rust_type = "", opt = true)]
        pub alloc_id: ::std::option::Option<String>,
        #[fefix(tag = 30, rust_type = "", opt = true)]
        pub last_mkt: ::std::option::Option<String>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
        #[fefix(tag = 54, rust_type = "", opt = true)]
        pub side: ::std::option::Option<char>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 168, rust_type = "", opt = true)]
        pub effective_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 60, rust_type = "", opt = false)]
        pub transact_time: Vec<u8>,
        #[fefix(tag = 109, rust_type = "", opt = true)]
        pub client_id: ::std::option::Option<String>,
        #[fefix(tag = 76, rust_type = "", opt = true)]
        pub exec_broker: ::std::option::Option<String>,
        #[fefix(tag = 169, rust_type = "", opt = true)]
        pub stand_inst_db_type: ::std::option::Option<i64>,
        #[fefix(tag = 170, rust_type = "", opt = true)]
        pub stand_inst_db_name: ::std::option::Option<String>,
        #[fefix(tag = 171, rust_type = "", opt = true)]
        pub stand_inst_db_id: ::std::option::Option<String>,
        #[fefix(tag = 172, rust_type = "", opt = true)]
        pub settl_delivery_type: ::std::option::Option<i64>,
        #[fefix(tag = 173, rust_type = "", opt = true)]
        pub settl_depository_code: ::std::option::Option<String>,
        #[fefix(tag = 174, rust_type = "", opt = true)]
        pub settl_brkr_code: ::std::option::Option<String>,
        #[fefix(tag = 175, rust_type = "", opt = true)]
        pub settl_inst_code: ::std::option::Option<String>,
        #[fefix(tag = 176, rust_type = "", opt = true)]
        pub security_settl_agent_name: ::std::option::Option<String>,
        #[fefix(tag = 177, rust_type = "", opt = true)]
        pub security_settl_agent_code: ::std::option::Option<String>,
        #[fefix(tag = 178, rust_type = "", opt = true)]
        pub security_settl_agent_acct_num: ::std::option::Option<String>,
        #[fefix(tag = 179, rust_type = "", opt = true)]
        pub security_settl_agent_acct_name: ::std::option::Option<String>,
        #[fefix(tag = 180, rust_type = "", opt = true)]
        pub security_settl_agent_contact_name: ::std::option::Option<String>,
        #[fefix(tag = 181, rust_type = "", opt = true)]
        pub security_settl_agent_contact_phone: ::std::option::Option<String>,
        #[fefix(tag = 182, rust_type = "", opt = true)]
        pub cash_settl_agent_name: ::std::option::Option<String>,
        #[fefix(tag = 183, rust_type = "", opt = true)]
        pub cash_settl_agent_code: ::std::option::Option<String>,
        #[fefix(tag = 184, rust_type = "", opt = true)]
        pub cash_settl_agent_acct_num: ::std::option::Option<String>,
        #[fefix(tag = 185, rust_type = "", opt = true)]
        pub cash_settl_agent_acct_name: ::std::option::Option<String>,
        #[fefix(tag = 186, rust_type = "", opt = true)]
        pub cash_settl_agent_contact_name: ::std::option::Option<String>,
        #[fefix(tag = 187, rust_type = "", opt = true)]
        pub cash_settl_agent_contact_phone: ::std::option::Option<String>,
    }

    /// Message information: MarketDataRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "V")]
    pub struct MarketDataRequest {
        #[fefix(tag = 262, rust_type = "", opt = false)]
        pub md_req_id: String,
        #[fefix(tag = 263, rust_type = "", opt = false)]
        pub subscription_request_type: char,
        #[fefix(tag = 264, rust_type = "", opt = false)]
        pub market_depth: i64,
        #[fefix(tag = 265, rust_type = "", opt = true)]
        pub md_update_type: ::std::option::Option<i64>,
        #[fefix(tag = 266, rust_type = "", opt = true)]
        pub aggregated_book: ::std::option::Option<bool>,
    }

    /// Message information: MarketDataSnapshotFullRefresh
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "W")]
    pub struct MarketDataSnapshotFullRefresh {
        #[fefix(tag = 262, rust_type = "", opt = true)]
        pub md_req_id: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 291, rust_type = "", opt = true)]
        pub financial_status: ::std::option::Option<char>,
        #[fefix(tag = 292, rust_type = "", opt = true)]
        pub corporate_action: ::std::option::Option<char>,
        #[fefix(tag = 387, rust_type = "", opt = true)]
        pub total_volume_traded: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: MarketDataIncrementalRefresh
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "X")]
    pub struct MarketDataIncrementalRefresh {
        #[fefix(tag = 262, rust_type = "", opt = true)]
        pub md_req_id: ::std::option::Option<String>,
    }

    /// Message information: MarketDataRequestReject
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "Y")]
    pub struct MarketDataRequestReject {
        #[fefix(tag = 262, rust_type = "", opt = false)]
        pub md_req_id: String,
        #[fefix(tag = 281, rust_type = "", opt = true)]
        pub md_req_rej_reason: ::std::option::Option<char>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: QuoteCancel
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "Z")]
    pub struct QuoteCancel {
        #[fefix(tag = 131, rust_type = "", opt = true)]
        pub quote_req_id: ::std::option::Option<String>,
        #[fefix(tag = 117, rust_type = "", opt = false)]
        pub quote_id: String,
        #[fefix(tag = 298, rust_type = "", opt = false)]
        pub quote_cancel_type: i64,
        #[fefix(tag = 301, rust_type = "", opt = true)]
        pub quote_response_level: ::std::option::Option<i64>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
    }

    /// Message information: QuoteStatusRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "a")]
    pub struct QuoteStatusRequest {
        #[fefix(tag = 117, rust_type = "", opt = true)]
        pub quote_id: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 54, rust_type = "", opt = true)]
        pub side: ::std::option::Option<char>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
    }

    /// Message information: QuoteAcknowledgement
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "b")]
    pub struct QuoteAcknowledgement {
        #[fefix(tag = 131, rust_type = "", opt = true)]
        pub quote_req_id: ::std::option::Option<String>,
        #[fefix(tag = 117, rust_type = "", opt = true)]
        pub quote_id: ::std::option::Option<String>,
        #[fefix(tag = 297, rust_type = "", opt = false)]
        pub quote_ack_status: i64,
        #[fefix(tag = 300, rust_type = "", opt = true)]
        pub quote_reject_reason: ::std::option::Option<i64>,
        #[fefix(tag = 301, rust_type = "", opt = true)]
        pub quote_response_level: ::std::option::Option<i64>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
    }

    /// Message information: SecurityDefinitionRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "c")]
    pub struct SecurityDefinitionRequest {
        #[fefix(tag = 320, rust_type = "", opt = false)]
        pub security_req_id: String,
        #[fefix(tag = 321, rust_type = "", opt = false)]
        pub security_request_type: i64,
        #[fefix(tag = 55, rust_type = "", opt = true)]
        pub symbol: ::std::option::Option<String>,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
    }

    /// Message information: SecurityDefinition
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "d")]
    pub struct SecurityDefinition {
        #[fefix(tag = 320, rust_type = "", opt = false)]
        pub security_req_id: String,
        #[fefix(tag = 322, rust_type = "", opt = false)]
        pub security_response_id: String,
        #[fefix(tag = 323, rust_type = "", opt = true)]
        pub security_response_type: ::std::option::Option<i64>,
        #[fefix(tag = 393, rust_type = "", opt = false)]
        pub total_num_securities: i64,
        #[fefix(tag = 55, rust_type = "", opt = true)]
        pub symbol: ::std::option::Option<String>,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: SecurityStatusRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "e")]
    pub struct SecurityStatusRequest {
        #[fefix(tag = 324, rust_type = "", opt = false)]
        pub security_status_req_id: String,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 263, rust_type = "", opt = false)]
        pub subscription_request_type: char,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
    }

    /// Message information: SecurityStatus
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "f")]
    pub struct SecurityStatus {
        #[fefix(tag = 324, rust_type = "", opt = true)]
        pub security_status_req_id: ::std::option::Option<String>,
        #[fefix(tag = 55, rust_type = "", opt = false)]
        pub symbol: String,
        #[fefix(tag = 65, rust_type = "", opt = true)]
        pub symbol_sfx: ::std::option::Option<String>,
        #[fefix(tag = 48, rust_type = "", opt = true)]
        pub security_id: ::std::option::Option<String>,
        #[fefix(tag = 22, rust_type = "", opt = true)]
        pub id_source: ::std::option::Option<String>,
        #[fefix(tag = 167, rust_type = "", opt = true)]
        pub security_type: ::std::option::Option<String>,
        #[fefix(tag = 200, rust_type = "", opt = true)]
        pub maturity_month_year: ::std::option::Option<(u8, u16)>,
        #[fefix(tag = 205, rust_type = "", opt = true)]
        pub maturity_day: ::std::option::Option<u8>,
        #[fefix(tag = 201, rust_type = "", opt = true)]
        pub put_or_call: ::std::option::Option<i64>,
        #[fefix(tag = 202, rust_type = "", opt = true)]
        pub strike_price: ::std::option::Option<f64>,
        #[fefix(tag = 206, rust_type = "", opt = true)]
        pub opt_attribute: ::std::option::Option<char>,
        #[fefix(tag = 231, rust_type = "", opt = true)]
        pub contract_multiplier: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 223, rust_type = "", opt = true)]
        pub coupon_rate: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 207, rust_type = "", opt = true)]
        pub security_exchange: ::std::option::Option<String>,
        #[fefix(tag = 106, rust_type = "", opt = true)]
        pub issuer: ::std::option::Option<String>,
        #[fefix(tag = 348, rust_type = "", opt = true)]
        pub encoded_issuer_len: ::std::option::Option<usize>,
        #[fefix(tag = 349, rust_type = "", opt = true)]
        pub encoded_issuer: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 107, rust_type = "", opt = true)]
        pub security_desc: ::std::option::Option<String>,
        #[fefix(tag = 350, rust_type = "", opt = true)]
        pub encoded_security_desc_len: ::std::option::Option<usize>,
        #[fefix(tag = 351, rust_type = "", opt = true)]
        pub encoded_security_desc: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
        #[fefix(tag = 325, rust_type = "", opt = true)]
        pub unsolicited_indicator: ::std::option::Option<bool>,
        #[fefix(tag = 326, rust_type = "", opt = true)]
        pub security_trading_status: ::std::option::Option<i64>,
        #[fefix(tag = 291, rust_type = "", opt = true)]
        pub financial_status: ::std::option::Option<char>,
        #[fefix(tag = 292, rust_type = "", opt = true)]
        pub corporate_action: ::std::option::Option<char>,
        #[fefix(tag = 327, rust_type = "", opt = true)]
        pub halt_reason_char: ::std::option::Option<char>,
        #[fefix(tag = 328, rust_type = "", opt = true)]
        pub in_view_of_common: ::std::option::Option<bool>,
        #[fefix(tag = 329, rust_type = "", opt = true)]
        pub due_to_related: ::std::option::Option<bool>,
        #[fefix(tag = 330, rust_type = "", opt = true)]
        pub buy_volume: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 331, rust_type = "", opt = true)]
        pub sell_volume: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 332, rust_type = "", opt = true)]
        pub high_px: ::std::option::Option<f64>,
        #[fefix(tag = 333, rust_type = "", opt = true)]
        pub low_px: ::std::option::Option<f64>,
        #[fefix(tag = 31, rust_type = "", opt = true)]
        pub last_px: ::std::option::Option<f64>,
        #[fefix(tag = 60, rust_type = "", opt = true)]
        pub transact_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 334, rust_type = "", opt = true)]
        pub adjustment: ::std::option::Option<i64>,
    }

    /// Message information: TradingSessionStatusRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "g")]
    pub struct TradingSessionStatusRequest {
        #[fefix(tag = 335, rust_type = "", opt = false)]
        pub trad_ses_req_id: String,
        #[fefix(tag = 336, rust_type = "", opt = true)]
        pub trading_session_id: ::std::option::Option<String>,
        #[fefix(tag = 338, rust_type = "", opt = true)]
        pub trad_ses_method: ::std::option::Option<i64>,
        #[fefix(tag = 339, rust_type = "", opt = true)]
        pub trad_ses_mode: ::std::option::Option<i64>,
        #[fefix(tag = 263, rust_type = "", opt = false)]
        pub subscription_request_type: char,
    }

    /// Message information: TradingSessionStatus
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "h")]
    pub struct TradingSessionStatus {
        #[fefix(tag = 335, rust_type = "", opt = true)]
        pub trad_ses_req_id: ::std::option::Option<String>,
        #[fefix(tag = 336, rust_type = "", opt = false)]
        pub trading_session_id: String,
        #[fefix(tag = 338, rust_type = "", opt = true)]
        pub trad_ses_method: ::std::option::Option<i64>,
        #[fefix(tag = 339, rust_type = "", opt = true)]
        pub trad_ses_mode: ::std::option::Option<i64>,
        #[fefix(tag = 325, rust_type = "", opt = true)]
        pub unsolicited_indicator: ::std::option::Option<bool>,
        #[fefix(tag = 340, rust_type = "", opt = false)]
        pub trad_ses_status: i64,
        #[fefix(tag = 341, rust_type = "", opt = true)]
        pub trad_ses_start_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 342, rust_type = "", opt = true)]
        pub trad_ses_open_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 343, rust_type = "", opt = true)]
        pub trad_ses_pre_close_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 344, rust_type = "", opt = true)]
        pub trad_ses_close_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 345, rust_type = "", opt = true)]
        pub trad_ses_end_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 387, rust_type = "", opt = true)]
        pub total_volume_traded: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: MassQuote
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "i")]
    pub struct MassQuote {
        #[fefix(tag = 131, rust_type = "", opt = true)]
        pub quote_req_id: ::std::option::Option<String>,
        #[fefix(tag = 117, rust_type = "", opt = false)]
        pub quote_id: String,
        #[fefix(tag = 301, rust_type = "", opt = true)]
        pub quote_response_level: ::std::option::Option<i64>,
        #[fefix(tag = 293, rust_type = "", opt = true)]
        pub def_bid_size: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 294, rust_type = "", opt = true)]
        pub def_offer_size: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: BusinessMessageReject
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "j")]
    pub struct BusinessMessageReject {
        #[fefix(tag = 45, rust_type = "", opt = true)]
        pub ref_seq_num: ::std::option::Option<i64>,
        #[fefix(tag = 372, rust_type = "", opt = false)]
        pub ref_msg_type: String,
        #[fefix(tag = 379, rust_type = "", opt = true)]
        pub business_reject_ref_id: ::std::option::Option<String>,
        #[fefix(tag = 380, rust_type = "", opt = false)]
        pub business_reject_reason: i64,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: BidRequest
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "k")]
    pub struct BidRequest {
        #[fefix(tag = 390, rust_type = "", opt = true)]
        pub bid_id: ::std::option::Option<String>,
        #[fefix(tag = 391, rust_type = "", opt = false)]
        pub client_bid_id: String,
        #[fefix(tag = 374, rust_type = "", opt = false)]
        pub bid_request_trans_type: char,
        #[fefix(tag = 392, rust_type = "", opt = true)]
        pub list_name: ::std::option::Option<String>,
        #[fefix(tag = 393, rust_type = "", opt = false)]
        pub total_num_securities: i64,
        #[fefix(tag = 394, rust_type = "", opt = false)]
        pub bid_type: i64,
        #[fefix(tag = 395, rust_type = "", opt = true)]
        pub num_tickets: ::std::option::Option<i64>,
        #[fefix(tag = 15, rust_type = "", opt = true)]
        pub currency: ::std::option::Option<String>,
        #[fefix(tag = 396, rust_type = "", opt = true)]
        pub side_value_1: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 397, rust_type = "", opt = true)]
        pub side_value_2: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 409, rust_type = "", opt = true)]
        pub liquidity_ind_type: ::std::option::Option<i64>,
        #[fefix(tag = 410, rust_type = "", opt = true)]
        pub wt_average_liquidity: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 411, rust_type = "", opt = true)]
        pub exchange_for_physical: ::std::option::Option<bool>,
        #[fefix(tag = 412, rust_type = "", opt = true)]
        pub out_main_cntry_u_index: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 413, rust_type = "", opt = true)]
        pub cross_percent: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 414, rust_type = "", opt = true)]
        pub prog_rpt_reqs: ::std::option::Option<i64>,
        #[fefix(tag = 415, rust_type = "", opt = true)]
        pub prog_period_interval: ::std::option::Option<i64>,
        #[fefix(tag = 416, rust_type = "", opt = true)]
        pub inc_tax_ind: ::std::option::Option<i64>,
        #[fefix(tag = 121, rust_type = "", opt = true)]
        pub forex_req: ::std::option::Option<bool>,
        #[fefix(tag = 417, rust_type = "", opt = true)]
        pub num_bidders: ::std::option::Option<i64>,
        #[fefix(tag = 75, rust_type = "", opt = true)]
        pub trade_date: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 418, rust_type = "", opt = false)]
        pub trade_type: char,
        #[fefix(tag = 419, rust_type = "", opt = false)]
        pub basis_px_type: char,
        #[fefix(tag = 443, rust_type = "", opt = true)]
        pub strike_time: ::std::option::Option<Vec<u8>>,
        #[fefix(tag = 58, rust_type = "", opt = true)]
        pub text: ::std::option::Option<String>,
        #[fefix(tag = 354, rust_type = "", opt = true)]
        pub encoded_text_len: ::std::option::Option<usize>,
        #[fefix(tag = 355, rust_type = "", opt = true)]
        pub encoded_text: ::std::option::Option<Vec<u8>>,
    }

    /// Message information: BidResponse
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "l")]
    pub struct BidResponse {
        #[fefix(tag = 390, rust_type = "", opt = true)]
        pub bid_id: ::std::option::Option<String>,
        #[fefix(tag = 391, rust_type = "", opt = true)]
        pub client_bid_id: ::std::option::Option<String>,
    }

    /// Message information: ListStrikePrice
    #[derive(Debug, Clone, TsrMessage)]
    #[fefix(msg_type = "m")]
    pub struct ListStrikePrice {
        #[fefix(tag = 66, rust_type = "", opt = false)]
        pub list_id: String,
        #[fefix(tag = 422, rust_type = "", opt = false)]
        pub tot_no_strikes: i64,
    }
}
