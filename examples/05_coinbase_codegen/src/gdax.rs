#![allow(dead_code)]

// Generated automatically by FerrumFIX 0.4.0 on Thu, 22 Apr 2021 13:32:22 +0000.
//
// DO NOT MODIFY MANUALLY.
// DO NOT COMMIT TO VERSION CONTROL.
// ALL CHANGES WILL BE OVERWRITTEN.

use fefix::DataField;
use fefix::{Buffer, DataType};
use fefix::{FieldDef, FieldLocation};
use std::marker::PhantomData;

pub const AGGRESSOR_INDICATOR: &FieldDef<'static, bool> = &FieldDef {
    name: "AggressorIndicator",
    tag: 1057,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRADE_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "TradeID",
    tag: 1003,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BATCH_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "BatchID",
    tag: 8014,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SELF_TRADE_PREVENTION: &FieldDef<'static, SelfTradePrevention> = &FieldDef {
    name: "SelfTradePrevention",
    tag: 7298,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SelfTradePrevention {
    #[fefix(variant = "D")]
    DecrementAndCancel,
    #[fefix(variant = "O")]
    CancelRestingOrder,
    #[fefix(variant = "N")]
    CancelIncomingOrder,
    #[fefix(variant = "B")]
    CancelBothOrders,
}

pub const ACCOUNT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "Account",
    tag: 1,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ADV_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "AdvId",
    tag: 2,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ADV_REF_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "AdvRefID",
    tag: 3,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ADV_SIDE: &FieldDef<'static, AdvSide> = &FieldDef {
    name: "AdvSide",
    tag: 4,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum AdvSide {
    #[fefix(variant = "B")]
    Buy,
    #[fefix(variant = "S")]
    Sell,
    #[fefix(variant = "T")]
    Trade,
    #[fefix(variant = "X")]
    Cross,
}

pub const ADV_TRANS_TYPE: &FieldDef<'static, AdvTransType> = &FieldDef {
    name: "AdvTransType",
    tag: 5,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum AdvTransType {
    #[fefix(variant = "C")]
    Cancel,
    #[fefix(variant = "N")]
    New,
    #[fefix(variant = "R")]
    Replace,
}

pub const AVG_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "AvgPx",
    tag: 6,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BEGIN_SEQ_NO: &FieldDef<'static, i64> = &FieldDef {
    name: "BeginSeqNo",
    tag: 7,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BEGIN_STRING: &FieldDef<'static, BeginString> = &FieldDef {
    name: "BeginString",
    tag: 8,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum BeginString {
    #[fefix(variant = "FIX.4.2")]
    Fix42,
}

pub const BODY_LENGTH: &FieldDef<'static, i64> = &FieldDef {
    name: "BodyLength",
    tag: 9,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CHECK_SUM: &FieldDef<'static, fefix::dtf::CheckSum> = &FieldDef {
    name: "CheckSum",
    tag: 10,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CL_ORD_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ClOrdID",
    tag: 11,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const COMMISSION: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "Commission",
    tag: 12,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const COMM_TYPE: &FieldDef<'static, CommType> = &FieldDef {
    name: "CommType",
    tag: 13,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum CommType {
    #[fefix(variant = "1")]
    PerShare,
    #[fefix(variant = "2")]
    Percentage,
    #[fefix(variant = "3")]
    Absolute,
}

pub const CUM_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "CumQty",
    tag: 14,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CURRENCY: &FieldDef<'static, &[u8; 3]> = &FieldDef {
    name: "Currency",
    tag: 15,
    is_group_leader: false,
    data_type: DataType::Currency,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const END_SEQ_NO: &FieldDef<'static, i64> = &FieldDef {
    name: "EndSeqNo",
    tag: 16,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EXEC_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ExecID",
    tag: 17,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EXEC_INST: &FieldDef<'static, ExecInst> = &FieldDef {
    name: "ExecInst",
    tag: 18,
    is_group_leader: false,
    data_type: DataType::MultipleCharValue,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ExecInst {
    #[fefix(variant = "0")]
    StayOnOfferside,
    #[fefix(variant = "1")]
    NotHeld,
    #[fefix(variant = "2")]
    Work,
    #[fefix(variant = "3")]
    GoAlong,
    #[fefix(variant = "4")]
    OverTheDay,
    #[fefix(variant = "5")]
    Held,
    #[fefix(variant = "6")]
    ParticipateDontInitiate,
    #[fefix(variant = "7")]
    StrictScale,
    #[fefix(variant = "8")]
    TryToScale,
    #[fefix(variant = "9")]
    StayOnBidside,
    #[fefix(variant = "A")]
    NoCross,
    #[fefix(variant = "B")]
    OkToCross,
    #[fefix(variant = "C")]
    CallFirst,
    #[fefix(variant = "D")]
    PercentOfVolume,
    #[fefix(variant = "E")]
    DoNotIncrease,
    #[fefix(variant = "F")]
    DoNotReduce,
    #[fefix(variant = "G")]
    AllOrNone,
    #[fefix(variant = "I")]
    InstitutionsOnly,
    #[fefix(variant = "L")]
    LastPeg,
    #[fefix(variant = "M")]
    MidPricePeg,
    #[fefix(variant = "N")]
    NonNegotiable,
    #[fefix(variant = "O")]
    OpeningPeg,
    #[fefix(variant = "P")]
    MarketPeg,
    #[fefix(variant = "R")]
    PrimaryPeg,
    #[fefix(variant = "S")]
    Suspend,
    #[fefix(variant = "T")]
    FixedPegToLocalBestBidOrOfferAtTimeOfOrder,
    #[fefix(variant = "U")]
    CustomerDisplayInstruction,
    #[fefix(variant = "V")]
    Netting,
    #[fefix(variant = "W")]
    PegToVwap,
}

pub const EXEC_REF_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ExecRefID",
    tag: 19,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EXEC_TRANS_TYPE: &FieldDef<'static, ExecTransType> = &FieldDef {
    name: "ExecTransType",
    tag: 20,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ExecTransType {
    #[fefix(variant = "0")]
    New,
    #[fefix(variant = "1")]
    Cancel,
    #[fefix(variant = "2")]
    Correct,
    #[fefix(variant = "3")]
    Status,
}

pub const HANDL_INST: &FieldDef<'static, HandlInst> = &FieldDef {
    name: "HandlInst",
    tag: 21,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum HandlInst {
    #[fefix(variant = "1")]
    AutomatedExecutionOrderPrivateNoBrokerIntervention,
    #[fefix(variant = "2")]
    AutomatedExecutionOrderPublicBrokerInterventionOk,
    #[fefix(variant = "3")]
    ManualOrderBestExecution,
}

pub const ID_SOURCE: &FieldDef<'static, IdSource> = &FieldDef {
    name: "IDSource",
    tag: 22,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum IdSource {
    #[fefix(variant = "1")]
    Cusip,
    #[fefix(variant = "2")]
    Sedol,
    #[fefix(variant = "3")]
    Quik,
    #[fefix(variant = "4")]
    IsinNumber,
    #[fefix(variant = "5")]
    RicCode,
    #[fefix(variant = "6")]
    IsoCurrencyCode,
    #[fefix(variant = "7")]
    IsoCountryCode,
    #[fefix(variant = "8")]
    ExchangeSymbol,
    #[fefix(variant = "9")]
    ConsolidatedTapeAssociation,
}

pub const IO_IID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "IOIid",
    tag: 23,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const IOI_OTH_SVC: &FieldDef<'static, u8> = &FieldDef {
    name: "IOIOthSvc",
    tag: 24,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const IOI_QLTY_IND: &FieldDef<'static, IoiQltyInd> = &FieldDef {
    name: "IOIQltyInd",
    tag: 25,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum IoiQltyInd {
    #[fefix(variant = "H")]
    High,
    #[fefix(variant = "L")]
    Low,
    #[fefix(variant = "M")]
    Medium,
}

pub const IOI_REF_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "IOIRefID",
    tag: 26,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const IOI_SHARES: &FieldDef<'static, IoiShares> = &FieldDef {
    name: "IOIShares",
    tag: 27,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum IoiShares {
    #[fefix(variant = "L")]
    Large,
    #[fefix(variant = "M")]
    Medium,
    #[fefix(variant = "S")]
    Small,
}

pub const IOI_TRANS_TYPE: &FieldDef<'static, IoiTransType> = &FieldDef {
    name: "IOITransType",
    tag: 28,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum IoiTransType {
    #[fefix(variant = "C")]
    Cancel,
    #[fefix(variant = "N")]
    New,
    #[fefix(variant = "R")]
    Replace,
}

pub const LAST_CAPACITY: &FieldDef<'static, LastCapacity> = &FieldDef {
    name: "LastCapacity",
    tag: 29,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum LastCapacity {
    #[fefix(variant = "1")]
    Agent,
    #[fefix(variant = "2")]
    CrossAsAgent,
    #[fefix(variant = "3")]
    CrossAsPrincipal,
    #[fefix(variant = "4")]
    Principal,
}

pub const LAST_MKT: &FieldDef<'static, &[u8; 4]> = &FieldDef {
    name: "LastMkt",
    tag: 30,
    is_group_leader: false,
    data_type: DataType::Exchange,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LAST_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LastPx",
    tag: 31,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LAST_SHARES: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LastShares",
    tag: 32,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LINES_OF_TEXT: &FieldDef<'static, i64> = &FieldDef {
    name: "LinesOfText",
    tag: 33,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MSG_SEQ_NUM: &FieldDef<'static, i64> = &FieldDef {
    name: "MsgSeqNum",
    tag: 34,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MSG_TYPE: &FieldDef<'static, MsgType> = &FieldDef {
    name: "MsgType",
    tag: 35,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MsgType {
    #[fefix(variant = "0")]
    Heartbeat,
    #[fefix(variant = "1")]
    TestRequest,
    #[fefix(variant = "2")]
    ResendRequest,
    #[fefix(variant = "3")]
    Reject,
    #[fefix(variant = "4")]
    SequenceReset,
    #[fefix(variant = "5")]
    Logout,
    #[fefix(variant = "6")]
    IndicationOfInterest,
    #[fefix(variant = "7")]
    Advertisement,
    #[fefix(variant = "8")]
    ExecutionReport,
    #[fefix(variant = "9")]
    OrderCancelReject,
    #[fefix(variant = "a")]
    QuoteStatusRequest,
    #[fefix(variant = "A")]
    Logon,
    #[fefix(variant = "B")]
    News,
    #[fefix(variant = "b")]
    QuoteAcknowledgement,
    #[fefix(variant = "C")]
    Email,
    #[fefix(variant = "c")]
    SecurityDefinitionRequest,
    #[fefix(variant = "D")]
    OrderSingle,
    #[fefix(variant = "d")]
    SecurityDefinition,
    #[fefix(variant = "E")]
    OrderList,
    #[fefix(variant = "e")]
    SecurityStatusRequest,
    #[fefix(variant = "f")]
    SecurityStatus,
    #[fefix(variant = "F")]
    OrderCancelRequest,
    #[fefix(variant = "G")]
    OrderCancelReplaceRequest,
    #[fefix(variant = "g")]
    TradingSessionStatusRequest,
    #[fefix(variant = "H")]
    OrderStatusRequest,
    #[fefix(variant = "h")]
    TradingSessionStatus,
    #[fefix(variant = "i")]
    MassQuote,
    #[fefix(variant = "j")]
    BusinessMessageReject,
    #[fefix(variant = "J")]
    Allocation,
    #[fefix(variant = "K")]
    ListCancelRequest,
    #[fefix(variant = "k")]
    BidRequest,
    #[fefix(variant = "l")]
    BidResponse,
    #[fefix(variant = "L")]
    ListExecute,
    #[fefix(variant = "m")]
    ListStrikePrice,
    #[fefix(variant = "M")]
    ListStatusRequest,
    #[fefix(variant = "N")]
    ListStatus,
    #[fefix(variant = "P")]
    AllocationAck,
    #[fefix(variant = "Q")]
    DontKnowTrade,
    #[fefix(variant = "R")]
    QuoteRequest,
    #[fefix(variant = "S")]
    Quote,
    #[fefix(variant = "T")]
    SettlementInstructions,
    #[fefix(variant = "V")]
    MarketDataRequest,
    #[fefix(variant = "W")]
    MarketDataSnapshotFullRefresh,
    #[fefix(variant = "X")]
    MarketDataIncrementalRefresh,
    #[fefix(variant = "Y")]
    MarketDataRequestReject,
    #[fefix(variant = "Z")]
    QuoteCancel,
}

pub const NEW_SEQ_NO: &FieldDef<'static, i64> = &FieldDef {
    name: "NewSeqNo",
    tag: 36,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ORDER_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "OrderID",
    tag: 37,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ORDER_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "OrderQty",
    tag: 38,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ORD_STATUS: &FieldDef<'static, OrdStatus> = &FieldDef {
    name: "OrdStatus",
    tag: 39,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum OrdStatus {
    #[fefix(variant = "0")]
    New,
    #[fefix(variant = "1")]
    PartiallyFilled,
    #[fefix(variant = "2")]
    Filled,
    #[fefix(variant = "3")]
    DoneForDay,
    #[fefix(variant = "4")]
    Canceled,
    #[fefix(variant = "5")]
    Replaced,
    #[fefix(variant = "6")]
    PendingCancel,
    #[fefix(variant = "7")]
    Stopped,
    #[fefix(variant = "8")]
    Rejected,
    #[fefix(variant = "9")]
    Suspended,
    #[fefix(variant = "A")]
    PendingNew,
    #[fefix(variant = "B")]
    Calculated,
    #[fefix(variant = "C")]
    Expired,
    #[fefix(variant = "D")]
    AcceptedForBidding,
    #[fefix(variant = "E")]
    PendingReplace,
}

pub const ORD_TYPE: &FieldDef<'static, OrdType> = &FieldDef {
    name: "OrdType",
    tag: 40,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum OrdType {
    #[fefix(variant = "1")]
    Market,
    #[fefix(variant = "2")]
    Limit,
    #[fefix(variant = "4")]
    StopLimit,
}

pub const ORIG_CL_ORD_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "OrigClOrdID",
    tag: 41,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ORIG_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "OrigTime",
    tag: 42,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const POSS_DUP_FLAG: &FieldDef<'static, PossDupFlag> = &FieldDef {
    name: "PossDupFlag",
    tag: 43,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum PossDupFlag {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const PRICE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "Price",
    tag: 44,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const REF_SEQ_NUM: &FieldDef<'static, i64> = &FieldDef {
    name: "RefSeqNum",
    tag: 45,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const RELATD_SYM: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "RelatdSym",
    tag: 46,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const RULE80_A: &FieldDef<'static, Rule80A> = &FieldDef {
    name: "Rule80A",
    tag: 47,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum Rule80A {
    #[fefix(variant = "A")]
    AgencySingleOrder,
    #[fefix(variant = "B")]
    ShortExemptTransactionB,
    #[fefix(variant = "C")]
    ProgramOrderNonIndexArbForMemberFirmOrg,
    #[fefix(variant = "D")]
    ProgramOrderIndexArbForMemberFirmOrg,
    #[fefix(variant = "E")]
    RegisteredEquityMarketMakerTrades,
    #[fefix(variant = "F")]
    ShortExemptTransactionF,
    #[fefix(variant = "H")]
    ShortExemptTransactionH,
    #[fefix(variant = "I")]
    IndividualInvestorSingleOrder,
    #[fefix(variant = "J")]
    ProgramOrderIndexArbForIndividualCustomer,
    #[fefix(variant = "K")]
    ProgramOrderNonIndexArbForIndividualCustomer,
    #[fefix(variant = "L")]
    ShortExemptTransactionForMemberCompetingMarketMakerAffiliatedWithTheFirmClearingTheTrade,
    #[fefix(variant = "M")]
    ProgramOrderIndexArbForOtherMember,
    #[fefix(variant = "N")]
    ProgramOrderNonIndexArbForOtherMember,
    #[fefix(variant = "O")]
    CompetingDealerTradesO,
    #[fefix(variant = "P")]
    Principal,
    #[fefix(variant = "R")]
    CompetingDealerTradesR,
    #[fefix(variant = "S")]
    SpecialistTrades,
    #[fefix(variant = "T")]
    CompetingDealerTradesT,
    #[fefix(variant = "U")]
    ProgramOrderIndexArbForOtherAgency,
    #[fefix(variant = "W")]
    AllOtherOrdersAsAgentForOtherMember,
    #[fefix(variant = "X")]
    ShortExemptTransactionForMemberCompetingMarketMakerNotAffiliatedWithTheFirmClearingTheTrade,
    #[fefix(variant = "Y")]
    ProgramOrderNonIndexArbForOtherAgency,
    #[fefix(variant = "Z")]
    ShortExemptTransactionForNonMemberCompetingMarketMaker,
}

pub const SECURITY_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecurityID",
    tag: 48,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SENDER_COMP_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SenderCompID",
    tag: 49,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SENDER_SUB_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SenderSubID",
    tag: 50,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SENDING_DATE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SendingDate",
    tag: 51,
    is_group_leader: false,
    data_type: DataType::LocalMktDate,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SENDING_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "SendingTime",
    tag: 52,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SHARES: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "Shares",
    tag: 53,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SIDE: &FieldDef<'static, Side> = &FieldDef {
    name: "Side",
    tag: 54,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum Side {
    #[fefix(variant = "1")]
    Buy,
    #[fefix(variant = "2")]
    Sell,
}

pub const SYMBOL: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "Symbol",
    tag: 55,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TARGET_COMP_ID: &FieldDef<'static, TargetCompId> = &FieldDef {
    name: "TargetCompID",
    tag: 56,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum TargetCompId {
    #[fefix(variant = "Coinbase")]
    Coinbase,
}

pub const TARGET_SUB_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "TargetSubID",
    tag: 57,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TEXT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "Text",
    tag: 58,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TIME_IN_FORCE: &FieldDef<'static, TimeInForce> = &FieldDef {
    name: "TimeInForce",
    tag: 59,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum TimeInForce {
    #[fefix(variant = "1")]
    GoodTillCancel,
    #[fefix(variant = "3")]
    ImmediateOrCancel,
    #[fefix(variant = "4")]
    FillOrKill,
    #[fefix(variant = "P")]
    PostOnly,
}

pub const TRANSACT_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "TransactTime",
    tag: 60,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const URGENCY: &FieldDef<'static, Urgency> = &FieldDef {
    name: "Urgency",
    tag: 61,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum Urgency {
    #[fefix(variant = "0")]
    Normal,
    #[fefix(variant = "1")]
    Flash,
    #[fefix(variant = "2")]
    Background,
}

pub const VALID_UNTIL_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "ValidUntilTime",
    tag: 62,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTLMNT_TYP: &FieldDef<'static, SettlmntTyp> = &FieldDef {
    name: "SettlmntTyp",
    tag: 63,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SettlmntTyp {
    #[fefix(variant = "0")]
    Regular,
    #[fefix(variant = "1")]
    Cash,
    #[fefix(variant = "2")]
    NextDay,
    #[fefix(variant = "3")]
    TPlus2,
    #[fefix(variant = "4")]
    TPlus3,
    #[fefix(variant = "5")]
    TPlus4,
    #[fefix(variant = "6")]
    Future,
    #[fefix(variant = "7")]
    WhenIssued,
    #[fefix(variant = "8")]
    SellersOption,
    #[fefix(variant = "9")]
    TPlus5,
}

pub const FUT_SETT_DATE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "FutSettDate",
    tag: 64,
    is_group_leader: false,
    data_type: DataType::LocalMktDate,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SYMBOL_SFX: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SymbolSfx",
    tag: 65,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIST_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ListID",
    tag: 66,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIST_SEQ_NO: &FieldDef<'static, i64> = &FieldDef {
    name: "ListSeqNo",
    tag: 67,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TOT_NO_ORDERS: &FieldDef<'static, i64> = &FieldDef {
    name: "TotNoOrders",
    tag: 68,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIST_EXEC_INST: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ListExecInst",
    tag: 69,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "AllocID",
    tag: 70,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_TRANS_TYPE: &FieldDef<'static, AllocTransType> = &FieldDef {
    name: "AllocTransType",
    tag: 71,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum AllocTransType {
    #[fefix(variant = "0")]
    New,
    #[fefix(variant = "1")]
    Replace,
    #[fefix(variant = "2")]
    Cancel,
    #[fefix(variant = "3")]
    Preliminary,
    #[fefix(variant = "4")]
    Calculated,
    #[fefix(variant = "5")]
    CalculatedWithoutPreliminary,
}

pub const REF_ALLOC_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "RefAllocID",
    tag: 72,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_ORDERS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoOrders",
    tag: 73,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const AVG_PRX_PRECISION: &FieldDef<'static, i64> = &FieldDef {
    name: "AvgPrxPrecision",
    tag: 74,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRADE_DATE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "TradeDate",
    tag: 75,
    is_group_leader: false,
    data_type: DataType::LocalMktDate,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EXEC_BROKER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ExecBroker",
    tag: 76,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const OPEN_CLOSE: &FieldDef<'static, OpenClose> = &FieldDef {
    name: "OpenClose",
    tag: 77,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum OpenClose {
    #[fefix(variant = "C")]
    Close,
    #[fefix(variant = "O")]
    Open,
}

pub const NO_ALLOCS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoAllocs",
    tag: 78,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_ACCOUNT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "AllocAccount",
    tag: 79,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_SHARES: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "AllocShares",
    tag: 80,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const PROCESS_CODE: &FieldDef<'static, ProcessCode> = &FieldDef {
    name: "ProcessCode",
    tag: 81,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ProcessCode {
    #[fefix(variant = "0")]
    Regular,
    #[fefix(variant = "1")]
    SoftDollar,
    #[fefix(variant = "2")]
    StepIn,
    #[fefix(variant = "3")]
    StepOut,
    #[fefix(variant = "4")]
    SoftDollarStepIn,
    #[fefix(variant = "5")]
    SoftDollarStepOut,
    #[fefix(variant = "6")]
    PlanSponsor,
}

pub const NO_RPTS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoRpts",
    tag: 82,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const RPT_SEQ: &FieldDef<'static, i64> = &FieldDef {
    name: "RptSeq",
    tag: 83,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CXL_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "CxlQty",
    tag: 84,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_DLVY_INST: &FieldDef<'static, i64> = &FieldDef {
    name: "NoDlvyInst",
    tag: 85,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DLVY_INST: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "DlvyInst",
    tag: 86,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_STATUS: &FieldDef<'static, AllocStatus> = &FieldDef {
    name: "AllocStatus",
    tag: 87,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum AllocStatus {
    #[fefix(variant = "0")]
    Accepted,
    #[fefix(variant = "1")]
    Rejected,
    #[fefix(variant = "2")]
    PartialAccept,
    #[fefix(variant = "3")]
    Received,
}

pub const ALLOC_REJ_CODE: &FieldDef<'static, AllocRejCode> = &FieldDef {
    name: "AllocRejCode",
    tag: 88,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum AllocRejCode {
    #[fefix(variant = "0")]
    UnknownAccount,
    #[fefix(variant = "1")]
    IncorrectQuantity,
    #[fefix(variant = "2")]
    IncorrectAveragePrice,
    #[fefix(variant = "3")]
    UnknownExecutingBrokerMnemonic,
    #[fefix(variant = "4")]
    CommissionDifference,
    #[fefix(variant = "5")]
    UnknownOrderid,
    #[fefix(variant = "6")]
    UnknownListid,
    #[fefix(variant = "7")]
    Other,
}

pub const SIGNATURE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "Signature",
    tag: 89,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURE_DATA_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "SecureDataLen",
    tag: 90,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURE_DATA: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecureData",
    tag: 91,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BROKER_OF_CREDIT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "BrokerOfCredit",
    tag: 92,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SIGNATURE_LENGTH: &FieldDef<'static, i64> = &FieldDef {
    name: "SignatureLength",
    tag: 93,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EMAIL_TYPE: &FieldDef<'static, EmailType> = &FieldDef {
    name: "EmailType",
    tag: 94,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum EmailType {
    #[fefix(variant = "0")]
    New,
    #[fefix(variant = "1")]
    Reply,
    #[fefix(variant = "2")]
    AdminReply,
}

pub const RAW_DATA_LENGTH: &FieldDef<'static, i64> = &FieldDef {
    name: "RawDataLength",
    tag: 95,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const RAW_DATA: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "RawData",
    tag: 96,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const POSS_RESEND: &FieldDef<'static, PossResend> = &FieldDef {
    name: "PossResend",
    tag: 97,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum PossResend {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const ENCRYPT_METHOD: &FieldDef<'static, EncryptMethod> = &FieldDef {
    name: "EncryptMethod",
    tag: 98,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum EncryptMethod {
    #[fefix(variant = "0")]
    None,
    #[fefix(variant = "1")]
    Pkcs,
    #[fefix(variant = "2")]
    Des,
    #[fefix(variant = "3")]
    PkcsDes,
    #[fefix(variant = "4")]
    PgpDes,
    #[fefix(variant = "5")]
    PgpDesMd5,
    #[fefix(variant = "6")]
    PemDesMd5,
}

pub const STOP_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "StopPx",
    tag: 99,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EX_DESTINATION: &FieldDef<'static, &[u8; 4]> = &FieldDef {
    name: "ExDestination",
    tag: 100,
    is_group_leader: false,
    data_type: DataType::Exchange,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CXL_REJ_REASON: &FieldDef<'static, CxlRejReason> = &FieldDef {
    name: "CxlRejReason",
    tag: 102,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum CxlRejReason {
    #[fefix(variant = "0")]
    TooLateToCancel,
    #[fefix(variant = "1")]
    UnknownOrder,
    #[fefix(variant = "2")]
    BrokerOption,
    #[fefix(variant = "3")]
    OrderAlreadyInPendingCancelOrPendingReplaceStatus,
}

pub const ORD_REJ_REASON: &FieldDef<'static, OrdRejReason> = &FieldDef {
    name: "OrdRejReason",
    tag: 103,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum OrdRejReason {
    #[fefix(variant = "0")]
    BrokerOption,
    #[fefix(variant = "1")]
    UnknownSymbol,
    #[fefix(variant = "2")]
    ExchangeClosed,
    #[fefix(variant = "3")]
    OrderExceedsLimit,
    #[fefix(variant = "4")]
    TooLateToEnter,
    #[fefix(variant = "5")]
    UnknownOrder,
    #[fefix(variant = "6")]
    DuplicateOrder,
    #[fefix(variant = "7")]
    DuplicateOfAVerballyCommunicatedOrder,
    #[fefix(variant = "8")]
    StaleOrder,
}

pub const IOI_QUALIFIER: &FieldDef<'static, IoiQualifier> = &FieldDef {
    name: "IOIQualifier",
    tag: 104,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum IoiQualifier {
    #[fefix(variant = "A")]
    AllOrNone,
    #[fefix(variant = "C")]
    AtTheClose,
    #[fefix(variant = "I")]
    InTouchWith,
    #[fefix(variant = "L")]
    Limit,
    #[fefix(variant = "M")]
    MoreBehind,
    #[fefix(variant = "O")]
    AtTheOpen,
    #[fefix(variant = "P")]
    TakingAPosition,
    #[fefix(variant = "Q")]
    AtTheMarket,
    #[fefix(variant = "R")]
    ReadyToTrade,
    #[fefix(variant = "S")]
    PortfolioShowN,
    #[fefix(variant = "T")]
    ThroughTheDay,
    #[fefix(variant = "V")]
    Versus,
    #[fefix(variant = "W")]
    Indication,
    #[fefix(variant = "X")]
    CrossingOpportunity,
    #[fefix(variant = "Y")]
    AtTheMidpoint,
    #[fefix(variant = "Z")]
    PreOpen,
}

pub const WAVE_NO: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "WaveNo",
    tag: 105,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ISSUER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "Issuer",
    tag: 106,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_DESC: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecurityDesc",
    tag: 107,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const HEART_BT_INT: &FieldDef<'static, i64> = &FieldDef {
    name: "HeartBtInt",
    tag: 108,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CLIENT_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ClientID",
    tag: 109,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MIN_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "MinQty",
    tag: 110,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MAX_FLOOR: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "MaxFloor",
    tag: 111,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TEST_REQ_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "TestReqID",
    tag: 112,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const REPORT_TO_EXCH: &FieldDef<'static, ReportToExch> = &FieldDef {
    name: "ReportToExch",
    tag: 113,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ReportToExch {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const LOCATE_REQD: &FieldDef<'static, LocateReqd> = &FieldDef {
    name: "LocateReqd",
    tag: 114,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum LocateReqd {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const ON_BEHALF_OF_COMP_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "OnBehalfOfCompID",
    tag: 115,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ON_BEHALF_OF_SUB_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "OnBehalfOfSubID",
    tag: 116,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const QUOTE_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "QuoteID",
    tag: 117,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NET_MONEY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "NetMoney",
    tag: 118,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_CURR_AMT: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "SettlCurrAmt",
    tag: 119,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_CURRENCY: &FieldDef<'static, &[u8; 3]> = &FieldDef {
    name: "SettlCurrency",
    tag: 120,
    is_group_leader: false,
    data_type: DataType::Currency,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const FOREX_REQ: &FieldDef<'static, ForexReq> = &FieldDef {
    name: "ForexReq",
    tag: 121,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ForexReq {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const ORIG_SENDING_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "OrigSendingTime",
    tag: 122,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const GAP_FILL_FLAG: &FieldDef<'static, GapFillFlag> = &FieldDef {
    name: "GapFillFlag",
    tag: 123,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum GapFillFlag {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const NO_EXECS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoExecs",
    tag: 124,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CXL_TYPE: &FieldDef<'static, u8> = &FieldDef {
    name: "CxlType",
    tag: 125,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EXPIRE_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "ExpireTime",
    tag: 126,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DK_REASON: &FieldDef<'static, DkReason> = &FieldDef {
    name: "DKReason",
    tag: 127,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum DkReason {
    #[fefix(variant = "A")]
    UnknownSymbol,
    #[fefix(variant = "B")]
    WrongSide,
    #[fefix(variant = "C")]
    QuantityExceedsOrder,
    #[fefix(variant = "D")]
    NoMatchingOrder,
    #[fefix(variant = "E")]
    PriceExceedsLimit,
    #[fefix(variant = "Z")]
    Other,
}

pub const DELIVER_TO_COMP_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "DeliverToCompID",
    tag: 128,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DELIVER_TO_SUB_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "DeliverToSubID",
    tag: 129,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const IOI_NATURAL_FLAG: &FieldDef<'static, IoiNaturalFlag> = &FieldDef {
    name: "IOINaturalFlag",
    tag: 130,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum IoiNaturalFlag {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const QUOTE_REQ_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "QuoteReqID",
    tag: 131,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BID_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "BidPx",
    tag: 132,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const OFFER_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "OfferPx",
    tag: 133,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BID_SIZE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "BidSize",
    tag: 134,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const OFFER_SIZE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "OfferSize",
    tag: 135,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_MISC_FEES: &FieldDef<'static, i64> = &FieldDef {
    name: "NoMiscFees",
    tag: 136,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MISC_FEE_AMT: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "MiscFeeAmt",
    tag: 137,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MISC_FEE_CURR: &FieldDef<'static, &[u8; 3]> = &FieldDef {
    name: "MiscFeeCurr",
    tag: 138,
    is_group_leader: false,
    data_type: DataType::Currency,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MISC_FEE_TYPE: &FieldDef<'static, MiscFeeType> = &FieldDef {
    name: "MiscFeeType",
    tag: 139,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MiscFeeType {
    #[fefix(variant = "1")]
    Regulatory,
    #[fefix(variant = "2")]
    Tax,
    #[fefix(variant = "3")]
    LocalCommission,
    #[fefix(variant = "4")]
    ExchangeFees,
    #[fefix(variant = "5")]
    Stamp,
    #[fefix(variant = "6")]
    Levy,
    #[fefix(variant = "7")]
    Other,
    #[fefix(variant = "8")]
    Markup,
    #[fefix(variant = "9")]
    ConsumptionTax,
}

pub const PREV_CLOSE_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "PrevClosePx",
    tag: 140,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const RESET_SEQ_NUM_FLAG: &FieldDef<'static, ResetSeqNumFlag> = &FieldDef {
    name: "ResetSeqNumFlag",
    tag: 141,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ResetSeqNumFlag {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const SENDER_LOCATION_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SenderLocationID",
    tag: 142,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TARGET_LOCATION_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "TargetLocationID",
    tag: 143,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ON_BEHALF_OF_LOCATION_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "OnBehalfOfLocationID",
    tag: 144,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DELIVER_TO_LOCATION_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "DeliverToLocationID",
    tag: 145,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_RELATED_SYM: &FieldDef<'static, i64> = &FieldDef {
    name: "NoRelatedSym",
    tag: 146,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SUBJECT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "Subject",
    tag: 147,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const HEADLINE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "Headline",
    tag: 148,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const URL_LINK: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "URLLink",
    tag: 149,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EXEC_TYPE: &FieldDef<'static, ExecType> = &FieldDef {
    name: "ExecType",
    tag: 150,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ExecType {
    #[fefix(variant = "0")]
    New,
    #[fefix(variant = "1")]
    PartialFill,
    #[fefix(variant = "2")]
    Fill,
    #[fefix(variant = "3")]
    DoneForDay,
    #[fefix(variant = "4")]
    Canceled,
    #[fefix(variant = "5")]
    Replace,
    #[fefix(variant = "6")]
    PendingCancel,
    #[fefix(variant = "7")]
    Stopped,
    #[fefix(variant = "8")]
    Rejected,
    #[fefix(variant = "9")]
    Suspended,
    #[fefix(variant = "A")]
    PendingNew,
    #[fefix(variant = "B")]
    Calculated,
    #[fefix(variant = "C")]
    Expired,
    #[fefix(variant = "D")]
    Restated,
    #[fefix(variant = "E")]
    PendingReplace,
}

pub const LEAVES_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LeavesQty",
    tag: 151,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CASH_ORDER_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "CashOrderQty",
    tag: 152,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_AVG_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "AllocAvgPx",
    tag: 153,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_NET_MONEY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "AllocNetMoney",
    tag: 154,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_CURR_FX_RATE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "SettlCurrFxRate",
    tag: 155,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_CURR_FX_RATE_CALC: &FieldDef<'static, SettlCurrFxRateCalc> = &FieldDef {
    name: "SettlCurrFxRateCalc",
    tag: 156,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SettlCurrFxRateCalc {
    #[fefix(variant = "M")]
    Multiply,
    #[fefix(variant = "D")]
    Divide,
}

pub const NUM_DAYS_INTEREST: &FieldDef<'static, i64> = &FieldDef {
    name: "NumDaysInterest",
    tag: 157,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ACCRUED_INTEREST_RATE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "AccruedInterestRate",
    tag: 158,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ACCRUED_INTEREST_AMT: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "AccruedInterestAmt",
    tag: 159,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_INST_MODE: &FieldDef<'static, SettlInstMode> = &FieldDef {
    name: "SettlInstMode",
    tag: 160,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SettlInstMode {
    #[fefix(variant = "0")]
    Default,
    #[fefix(variant = "1")]
    StandingInstructionsProvided,
    #[fefix(variant = "2")]
    SpecificAllocationAccountOverriding,
    #[fefix(variant = "3")]
    SpecificAllocationAccountStanding,
}

pub const ALLOC_TEXT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "AllocText",
    tag: 161,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_INST_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SettlInstID",
    tag: 162,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_INST_TRANS_TYPE: &FieldDef<'static, SettlInstTransType> = &FieldDef {
    name: "SettlInstTransType",
    tag: 163,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SettlInstTransType {
    #[fefix(variant = "C")]
    Cancel,
    #[fefix(variant = "N")]
    New,
    #[fefix(variant = "R")]
    Replace,
}

pub const EMAIL_THREAD_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EmailThreadID",
    tag: 164,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_INST_SOURCE: &FieldDef<'static, SettlInstSource> = &FieldDef {
    name: "SettlInstSource",
    tag: 165,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SettlInstSource {
    #[fefix(variant = "1")]
    BrokersInstructions,
    #[fefix(variant = "2")]
    InstitutionsInstructions,
}

pub const SETTL_LOCATION: &FieldDef<'static, SettlLocation> = &FieldDef {
    name: "SettlLocation",
    tag: 166,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SettlLocation {
    #[fefix(variant = "CED")]
    Cedel,
    #[fefix(variant = "DTC")]
    DepositoryTrustCompany,
    #[fefix(variant = "EUR")]
    Euroclear,
    #[fefix(variant = "FED")]
    FederalBookEntry,
    #[fefix(variant = "ISO Country Code")]
    LocalMarketSettleLocation,
    #[fefix(variant = "PNY")]
    Physical,
    #[fefix(variant = "PTC")]
    ParticipantTrustCompany,
}

pub const SECURITY_TYPE: &FieldDef<'static, SecurityType> = &FieldDef {
    name: "SecurityType",
    tag: 167,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SecurityType {
    #[fefix(variant = "?")]
    WildcardEntry,
    #[fefix(variant = "BA")]
    BankersAcceptance,
    #[fefix(variant = "CB")]
    ConvertibleBond,
    #[fefix(variant = "CD")]
    CertificateOfDeposit,
    #[fefix(variant = "CMO")]
    CollateralizeMortgageObligation,
    #[fefix(variant = "CORP")]
    CorporateBond,
    #[fefix(variant = "CP")]
    CommercialPaper,
    #[fefix(variant = "CPP")]
    CorporatePrivatePlacement,
    #[fefix(variant = "CS")]
    CommonStock,
    #[fefix(variant = "FHA")]
    FederalHousingAuthority,
    #[fefix(variant = "FHL")]
    FederalHomeLoan,
    #[fefix(variant = "FN")]
    FederalNationalMortgageAssociation,
    #[fefix(variant = "FOR")]
    ForeignExchangeContract,
    #[fefix(variant = "FUT")]
    Future,
    #[fefix(variant = "GN")]
    GovernmentNationalMortgageAssociation,
    #[fefix(variant = "GOVT")]
    TreasuriesPlusAgencyDebenture,
    #[fefix(variant = "IET")]
    MortgageIoette,
    #[fefix(variant = "MF")]
    MutualFund,
    #[fefix(variant = "MIO")]
    MortgageInterestOnly,
    #[fefix(variant = "MPO")]
    MortgagePrincipalOnly,
    #[fefix(variant = "MPP")]
    MortgagePrivatePlacement,
    #[fefix(variant = "MPT")]
    MiscellaneousPassThru,
    #[fefix(variant = "MUNI")]
    MunicipalBond,
    #[fefix(variant = "NONE")]
    NoIsitcSecurityType,
    #[fefix(variant = "OPT")]
    Option,
    #[fefix(variant = "PS")]
    PreferredStock,
    #[fefix(variant = "RP")]
    RepurchaseAgreement,
    #[fefix(variant = "RVRP")]
    ReverseRepurchaseAgreement,
    #[fefix(variant = "SL")]
    StudentLoanMarketingAssociation,
    #[fefix(variant = "TD")]
    TimeDeposit,
    #[fefix(variant = "USTB")]
    UsTreasuryBill,
    #[fefix(variant = "WAR")]
    Warrant,
    #[fefix(variant = "ZOO")]
    CatsTigersLions,
}

pub const EFFECTIVE_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "EffectiveTime",
    tag: 168,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const STAND_INST_DB_TYPE: &FieldDef<'static, StandInstDbType> = &FieldDef {
    name: "StandInstDbType",
    tag: 169,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum StandInstDbType {
    #[fefix(variant = "0")]
    Other,
    #[fefix(variant = "1")]
    DtcSid,
    #[fefix(variant = "2")]
    ThomsonAlert,
    #[fefix(variant = "3")]
    AGlobalCustodian,
}

pub const STAND_INST_DB_NAME: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "StandInstDbName",
    tag: 170,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const STAND_INST_DB_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "StandInstDbID",
    tag: 171,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_DELIVERY_TYPE: &FieldDef<'static, i64> = &FieldDef {
    name: "SettlDeliveryType",
    tag: 172,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_DEPOSITORY_CODE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SettlDepositoryCode",
    tag: 173,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_BRKR_CODE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SettlBrkrCode",
    tag: 174,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_INST_CODE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SettlInstCode",
    tag: 175,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_SETTL_AGENT_NAME: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecuritySettlAgentName",
    tag: 176,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_SETTL_AGENT_CODE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecuritySettlAgentCode",
    tag: 177,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_SETTL_AGENT_ACCT_NUM: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecuritySettlAgentAcctNum",
    tag: 178,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_SETTL_AGENT_ACCT_NAME: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecuritySettlAgentAcctName",
    tag: 179,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_SETTL_AGENT_CONTACT_NAME: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecuritySettlAgentContactName",
    tag: 180,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_SETTL_AGENT_CONTACT_PHONE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecuritySettlAgentContactPhone",
    tag: 181,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CASH_SETTL_AGENT_NAME: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "CashSettlAgentName",
    tag: 182,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CASH_SETTL_AGENT_CODE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "CashSettlAgentCode",
    tag: 183,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CASH_SETTL_AGENT_ACCT_NUM: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "CashSettlAgentAcctNum",
    tag: 184,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CASH_SETTL_AGENT_ACCT_NAME: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "CashSettlAgentAcctName",
    tag: 185,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CASH_SETTL_AGENT_CONTACT_NAME: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "CashSettlAgentContactName",
    tag: 186,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CASH_SETTL_AGENT_CONTACT_PHONE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "CashSettlAgentContactPhone",
    tag: 187,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BID_SPOT_RATE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "BidSpotRate",
    tag: 188,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BID_FORWARD_POINTS: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "BidForwardPoints",
    tag: 189,
    is_group_leader: false,
    data_type: DataType::PriceOffset,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const OFFER_SPOT_RATE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "OfferSpotRate",
    tag: 190,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const OFFER_FORWARD_POINTS: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "OfferForwardPoints",
    tag: 191,
    is_group_leader: false,
    data_type: DataType::PriceOffset,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ORDER_QTY2: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "OrderQty2",
    tag: 192,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const FUT_SETT_DATE2: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "FutSettDate2",
    tag: 193,
    is_group_leader: false,
    data_type: DataType::LocalMktDate,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LAST_SPOT_RATE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LastSpotRate",
    tag: 194,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LAST_FORWARD_POINTS: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LastForwardPoints",
    tag: 195,
    is_group_leader: false,
    data_type: DataType::PriceOffset,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_LINK_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "AllocLinkID",
    tag: 196,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_LINK_TYPE: &FieldDef<'static, AllocLinkType> = &FieldDef {
    name: "AllocLinkType",
    tag: 197,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum AllocLinkType {
    #[fefix(variant = "0")]
    FXNetting,
    #[fefix(variant = "1")]
    FXSwap,
}

pub const SECONDARY_ORDER_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecondaryOrderID",
    tag: 198,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_IOI_QUALIFIERS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoIOIQualifiers",
    tag: 199,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MATURITY_MONTH_YEAR: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "MaturityMonthYear",
    tag: 200,
    is_group_leader: false,
    data_type: DataType::MonthYear,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const PUT_OR_CALL: &FieldDef<'static, PutOrCall> = &FieldDef {
    name: "PutOrCall",
    tag: 201,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum PutOrCall {
    #[fefix(variant = "0")]
    Put,
    #[fefix(variant = "1")]
    Call,
}

pub const STRIKE_PRICE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "StrikePrice",
    tag: 202,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const COVERED_OR_UNCOVERED: &FieldDef<'static, CoveredOrUncovered> = &FieldDef {
    name: "CoveredOrUncovered",
    tag: 203,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum CoveredOrUncovered {
    #[fefix(variant = "0")]
    Covered,
    #[fefix(variant = "1")]
    Uncovered,
}

pub const CUSTOMER_OR_FIRM: &FieldDef<'static, CustomerOrFirm> = &FieldDef {
    name: "CustomerOrFirm",
    tag: 204,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum CustomerOrFirm {
    #[fefix(variant = "0")]
    Customer,
    #[fefix(variant = "1")]
    Firm,
}

pub const MATURITY_DAY: &FieldDef<'static, u32> = &FieldDef {
    name: "MaturityDay",
    tag: 205,
    is_group_leader: false,
    data_type: DataType::DayOfMonth,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const OPT_ATTRIBUTE: &FieldDef<'static, u8> = &FieldDef {
    name: "OptAttribute",
    tag: 206,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_EXCHANGE: &FieldDef<'static, &[u8; 4]> = &FieldDef {
    name: "SecurityExchange",
    tag: 207,
    is_group_leader: false,
    data_type: DataType::Exchange,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NOTIFY_BROKER_OF_CREDIT: &FieldDef<'static, NotifyBrokerOfCredit> = &FieldDef {
    name: "NotifyBrokerOfCredit",
    tag: 208,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum NotifyBrokerOfCredit {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const ALLOC_HANDL_INST: &FieldDef<'static, AllocHandlInst> = &FieldDef {
    name: "AllocHandlInst",
    tag: 209,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum AllocHandlInst {
    #[fefix(variant = "1")]
    Match,
    #[fefix(variant = "2")]
    Forward,
    #[fefix(variant = "3")]
    ForwardAndMatch,
}

pub const MAX_SHOW: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "MaxShow",
    tag: 210,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const PEG_DIFFERENCE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "PegDifference",
    tag: 211,
    is_group_leader: false,
    data_type: DataType::PriceOffset,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const XML_DATA_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "XmlDataLen",
    tag: 212,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const XML_DATA: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "XmlData",
    tag: 213,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SETTL_INST_REF_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SettlInstRefID",
    tag: 214,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_ROUTING_I_DS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoRoutingIDs",
    tag: 215,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ROUTING_TYPE: &FieldDef<'static, RoutingType> = &FieldDef {
    name: "RoutingType",
    tag: 216,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum RoutingType {
    #[fefix(variant = "1")]
    TargetFirm,
    #[fefix(variant = "2")]
    TargetList,
    #[fefix(variant = "3")]
    BlockFirm,
    #[fefix(variant = "4")]
    BlockList,
}

pub const ROUTING_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "RoutingID",
    tag: 217,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SPREAD_TO_BENCHMARK: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "SpreadToBenchmark",
    tag: 218,
    is_group_leader: false,
    data_type: DataType::PriceOffset,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BENCHMARK: &FieldDef<'static, Benchmark> = &FieldDef {
    name: "Benchmark",
    tag: 219,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum Benchmark {
    #[fefix(variant = "1")]
    Curve,
    #[fefix(variant = "2")]
    N5Yr,
    #[fefix(variant = "3")]
    Old5,
    #[fefix(variant = "4")]
    N10Yr,
    #[fefix(variant = "5")]
    Old10,
    #[fefix(variant = "6")]
    N30Yr,
    #[fefix(variant = "7")]
    Old30,
    #[fefix(variant = "8")]
    N3MoLibor,
    #[fefix(variant = "9")]
    N6MoLibor,
}

pub const COUPON_RATE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "CouponRate",
    tag: 223,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CONTRACT_MULTIPLIER: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "ContractMultiplier",
    tag: 231,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_REQ_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "MDReqID",
    tag: 262,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SUBSCRIPTION_REQUEST_TYPE: &FieldDef<'static, SubscriptionRequestType> = &FieldDef {
    name: "SubscriptionRequestType",
    tag: 263,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SubscriptionRequestType {
    #[fefix(variant = "0")]
    Snapshot,
    #[fefix(variant = "1")]
    SnapshotPlusUpdates,
    #[fefix(variant = "2")]
    DisablePreviousSnapshotPlusUpdateRequest,
}

pub const MARKET_DEPTH: &FieldDef<'static, i64> = &FieldDef {
    name: "MarketDepth",
    tag: 264,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_UPDATE_TYPE: &FieldDef<'static, MdUpdateType> = &FieldDef {
    name: "MDUpdateType",
    tag: 265,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MdUpdateType {
    #[fefix(variant = "0")]
    FullRefresh,
    #[fefix(variant = "1")]
    IncrementalRefresh,
}

pub const AGGREGATED_BOOK: &FieldDef<'static, AggregatedBook> = &FieldDef {
    name: "AggregatedBook",
    tag: 266,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum AggregatedBook {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const NO_MD_ENTRY_TYPES: &FieldDef<'static, i64> = &FieldDef {
    name: "NoMDEntryTypes",
    tag: 267,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_MD_ENTRIES: &FieldDef<'static, i64> = &FieldDef {
    name: "NoMDEntries",
    tag: 268,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_ENTRY_TYPE: &FieldDef<'static, MdEntryType> = &FieldDef {
    name: "MDEntryType",
    tag: 269,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MdEntryType {
    #[fefix(variant = "0")]
    Bid,
    #[fefix(variant = "1")]
    Offer,
    #[fefix(variant = "2")]
    Trade,
    #[fefix(variant = "3")]
    IndexValue,
    #[fefix(variant = "4")]
    OpeningPrice,
    #[fefix(variant = "5")]
    ClosingPrice,
    #[fefix(variant = "6")]
    SettlementPrice,
    #[fefix(variant = "7")]
    TradingSessionHighPrice,
    #[fefix(variant = "8")]
    TradingSessionLowPrice,
    #[fefix(variant = "9")]
    TradingSessionVwapPrice,
}

pub const MD_ENTRY_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "MDEntryPx",
    tag: 270,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_ENTRY_SIZE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "MDEntrySize",
    tag: 271,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_ENTRY_DATE: &FieldDef<'static, fefix::dtf::Date> = &FieldDef {
    name: "MDEntryDate",
    tag: 272,
    is_group_leader: false,
    data_type: DataType::UtcDateOnly,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_ENTRY_TIME: &FieldDef<'static, fefix::dtf::Time> = &FieldDef {
    name: "MDEntryTime",
    tag: 273,
    is_group_leader: false,
    data_type: DataType::UtcTimeOnly,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TICK_DIRECTION: &FieldDef<'static, TickDirection> = &FieldDef {
    name: "TickDirection",
    tag: 274,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum TickDirection {
    #[fefix(variant = "0")]
    PlusTick,
    #[fefix(variant = "1")]
    ZeroPlusTick,
    #[fefix(variant = "2")]
    MinusTick,
    #[fefix(variant = "3")]
    ZeroMinusTick,
}

pub const MD_MKT: &FieldDef<'static, &[u8; 4]> = &FieldDef {
    name: "MDMkt",
    tag: 275,
    is_group_leader: false,
    data_type: DataType::Exchange,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const QUOTE_CONDITION: &FieldDef<'static, QuoteCondition> = &FieldDef {
    name: "QuoteCondition",
    tag: 276,
    is_group_leader: false,
    data_type: DataType::MultipleCharValue,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum QuoteCondition {
    #[fefix(variant = "A")]
    Open,
    #[fefix(variant = "B")]
    Closed,
    #[fefix(variant = "C")]
    ExchangeBest,
    #[fefix(variant = "D")]
    ConsolidatedBest,
    #[fefix(variant = "E")]
    Locked,
    #[fefix(variant = "F")]
    Crossed,
    #[fefix(variant = "G")]
    Depth,
    #[fefix(variant = "H")]
    FastTrading,
    #[fefix(variant = "I")]
    NonFirm,
}

pub const TRADE_CONDITION: &FieldDef<'static, TradeCondition> = &FieldDef {
    name: "TradeCondition",
    tag: 277,
    is_group_leader: false,
    data_type: DataType::MultipleCharValue,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum TradeCondition {
    #[fefix(variant = "A")]
    Cash,
    #[fefix(variant = "B")]
    AveragePriceTrade,
    #[fefix(variant = "C")]
    CashTrade,
    #[fefix(variant = "D")]
    NextDay,
    #[fefix(variant = "E")]
    Opening,
    #[fefix(variant = "F")]
    IntradayTradeDetail,
    #[fefix(variant = "G")]
    Rule127Trade,
    #[fefix(variant = "H")]
    Rule155Trade,
    #[fefix(variant = "I")]
    SoldLast,
    #[fefix(variant = "J")]
    NextDayTrade,
    #[fefix(variant = "K")]
    Opened,
    #[fefix(variant = "L")]
    Seller,
    #[fefix(variant = "M")]
    Sold,
    #[fefix(variant = "N")]
    StoppedStock,
}

pub const MD_ENTRY_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "MDEntryID",
    tag: 278,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_UPDATE_ACTION: &FieldDef<'static, MdUpdateAction> = &FieldDef {
    name: "MDUpdateAction",
    tag: 279,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MdUpdateAction {
    #[fefix(variant = "0")]
    New,
    #[fefix(variant = "1")]
    Change,
    #[fefix(variant = "2")]
    Delete,
}

pub const MD_ENTRY_REF_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "MDEntryRefID",
    tag: 280,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_REQ_REJ_REASON: &FieldDef<'static, MdReqRejReason> = &FieldDef {
    name: "MDReqRejReason",
    tag: 281,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MdReqRejReason {
    #[fefix(variant = "0")]
    UnknownSymbol,
    #[fefix(variant = "1")]
    DuplicateMdreqid,
    #[fefix(variant = "2")]
    InsufficientBandwidth,
    #[fefix(variant = "3")]
    InsufficientPermissions,
    #[fefix(variant = "4")]
    UnsupportedSubscriptionrequesttype,
    #[fefix(variant = "5")]
    UnsupportedMarketdepth,
    #[fefix(variant = "6")]
    UnsupportedMdupdatetype,
    #[fefix(variant = "7")]
    UnsupportedAggregatedbook,
    #[fefix(variant = "8")]
    UnsupportedMdentrytype,
}

pub const MD_ENTRY_ORIGINATOR: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "MDEntryOriginator",
    tag: 282,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LOCATION_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "LocationID",
    tag: 283,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DESK_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "DeskID",
    tag: 284,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DELETE_REASON: &FieldDef<'static, DeleteReason> = &FieldDef {
    name: "DeleteReason",
    tag: 285,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum DeleteReason {
    #[fefix(variant = "0")]
    Cancelation,
    #[fefix(variant = "1")]
    Error,
}

pub const OPEN_CLOSE_SETTLE_FLAG: &FieldDef<'static, OpenCloseSettleFlag> = &FieldDef {
    name: "OpenCloseSettleFlag",
    tag: 286,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum OpenCloseSettleFlag {
    #[fefix(variant = "0")]
    DailyOpen,
    #[fefix(variant = "1")]
    SessionOpen,
    #[fefix(variant = "2")]
    DeliverySettlementPrice,
}

pub const SELLER_DAYS: &FieldDef<'static, i64> = &FieldDef {
    name: "SellerDays",
    tag: 287,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_ENTRY_BUYER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "MDEntryBuyer",
    tag: 288,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_ENTRY_SELLER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "MDEntrySeller",
    tag: 289,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MD_ENTRY_POSITION_NO: &FieldDef<'static, i64> = &FieldDef {
    name: "MDEntryPositionNo",
    tag: 290,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const FINANCIAL_STATUS: &FieldDef<'static, FinancialStatus> = &FieldDef {
    name: "FinancialStatus",
    tag: 291,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum FinancialStatus {
    #[fefix(variant = "1")]
    Bankrupt,
}

pub const CORPORATE_ACTION: &FieldDef<'static, CorporateAction> = &FieldDef {
    name: "CorporateAction",
    tag: 292,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum CorporateAction {
    #[fefix(variant = "A")]
    ExDividend,
    #[fefix(variant = "B")]
    ExDistribution,
    #[fefix(variant = "C")]
    ExRights,
    #[fefix(variant = "D")]
    New,
    #[fefix(variant = "E")]
    ExInterest,
}

pub const DEF_BID_SIZE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "DefBidSize",
    tag: 293,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DEF_OFFER_SIZE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "DefOfferSize",
    tag: 294,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_QUOTE_ENTRIES: &FieldDef<'static, i64> = &FieldDef {
    name: "NoQuoteEntries",
    tag: 295,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_QUOTE_SETS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoQuoteSets",
    tag: 296,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const QUOTE_ACK_STATUS: &FieldDef<'static, QuoteAckStatus> = &FieldDef {
    name: "QuoteAckStatus",
    tag: 297,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum QuoteAckStatus {
    #[fefix(variant = "0")]
    Accepted,
    #[fefix(variant = "1")]
    CanceledForSymbol,
    #[fefix(variant = "2")]
    CanceledForSecurityType,
    #[fefix(variant = "3")]
    CanceledForUnderlying,
    #[fefix(variant = "4")]
    CanceledAll,
    #[fefix(variant = "5")]
    Rejected,
}

pub const QUOTE_CANCEL_TYPE: &FieldDef<'static, QuoteCancelType> = &FieldDef {
    name: "QuoteCancelType",
    tag: 298,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum QuoteCancelType {
    #[fefix(variant = "1")]
    CancelForSymbol,
    #[fefix(variant = "2")]
    CancelForSecurityType,
    #[fefix(variant = "3")]
    CancelForUnderlyingSymbol,
    #[fefix(variant = "4")]
    CancelForAllQuotes,
}

pub const QUOTE_ENTRY_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "QuoteEntryID",
    tag: 299,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const QUOTE_REJECT_REASON: &FieldDef<'static, QuoteRejectReason> = &FieldDef {
    name: "QuoteRejectReason",
    tag: 300,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum QuoteRejectReason {
    #[fefix(variant = "1")]
    UnknownSymbol,
    #[fefix(variant = "2")]
    Exchange,
    #[fefix(variant = "3")]
    QuoteRequestExceedsLimit,
    #[fefix(variant = "4")]
    TooLateToEnter,
    #[fefix(variant = "5")]
    UnknownQuote,
    #[fefix(variant = "6")]
    DuplicateQuote,
    #[fefix(variant = "7")]
    InvalidBidAskSpread,
    #[fefix(variant = "8")]
    InvalidPrice,
    #[fefix(variant = "9")]
    NotAuthorizedToQuoteSecurity,
}

pub const QUOTE_RESPONSE_LEVEL: &FieldDef<'static, QuoteResponseLevel> = &FieldDef {
    name: "QuoteResponseLevel",
    tag: 301,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum QuoteResponseLevel {
    #[fefix(variant = "0")]
    NoAcknowledgement,
    #[fefix(variant = "1")]
    AcknowledgeOnlyNegativeOrErroneousQuotes,
    #[fefix(variant = "2")]
    AcknowledgeEachQuoteMessages,
}

pub const QUOTE_SET_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "QuoteSetID",
    tag: 302,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const QUOTE_REQUEST_TYPE: &FieldDef<'static, QuoteRequestType> = &FieldDef {
    name: "QuoteRequestType",
    tag: 303,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum QuoteRequestType {
    #[fefix(variant = "1")]
    Manual,
    #[fefix(variant = "2")]
    Automatic,
}

pub const TOT_QUOTE_ENTRIES: &FieldDef<'static, i64> = &FieldDef {
    name: "TotQuoteEntries",
    tag: 304,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_ID_SOURCE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "UnderlyingIDSource",
    tag: 305,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_ISSUER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "UnderlyingIssuer",
    tag: 306,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_SECURITY_DESC: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "UnderlyingSecurityDesc",
    tag: 307,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_SECURITY_EXCHANGE: &FieldDef<'static, &[u8; 4]> = &FieldDef {
    name: "UnderlyingSecurityExchange",
    tag: 308,
    is_group_leader: false,
    data_type: DataType::Exchange,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_SECURITY_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "UnderlyingSecurityID",
    tag: 309,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_SECURITY_TYPE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "UnderlyingSecurityType",
    tag: 310,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_SYMBOL: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "UnderlyingSymbol",
    tag: 311,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_SYMBOL_SFX: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "UnderlyingSymbolSfx",
    tag: 312,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_MATURITY_MONTH_YEAR: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "UnderlyingMaturityMonthYear",
    tag: 313,
    is_group_leader: false,
    data_type: DataType::MonthYear,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_MATURITY_DAY: &FieldDef<'static, u32> = &FieldDef {
    name: "UnderlyingMaturityDay",
    tag: 314,
    is_group_leader: false,
    data_type: DataType::DayOfMonth,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_PUT_OR_CALL: &FieldDef<'static, i64> = &FieldDef {
    name: "UnderlyingPutOrCall",
    tag: 315,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_STRIKE_PRICE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "UnderlyingStrikePrice",
    tag: 316,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_OPT_ATTRIBUTE: &FieldDef<'static, u8> = &FieldDef {
    name: "UnderlyingOptAttribute",
    tag: 317,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_CURRENCY: &FieldDef<'static, &[u8; 3]> = &FieldDef {
    name: "UnderlyingCurrency",
    tag: 318,
    is_group_leader: false,
    data_type: DataType::Currency,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const RATIO_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "RatioQty",
    tag: 319,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_REQ_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecurityReqID",
    tag: 320,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_REQUEST_TYPE: &FieldDef<'static, SecurityRequestType> = &FieldDef {
    name: "SecurityRequestType",
    tag: 321,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SecurityRequestType {
    #[fefix(variant = "0")]
    RequestSecurityIdentityAndSpecifications,
    #[fefix(variant = "1")]
    RequestSecurityIdentityForTheSpecificationsProvided,
    #[fefix(variant = "2")]
    RequestListSecurityTypes,
    #[fefix(variant = "3")]
    RequestListSecurities,
}

pub const SECURITY_RESPONSE_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecurityResponseID",
    tag: 322,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SECURITY_RESPONSE_TYPE: &FieldDef<'static, SecurityResponseType> = &FieldDef {
    name: "SecurityResponseType",
    tag: 323,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SecurityResponseType {
    #[fefix(variant = "1")]
    AcceptSecurityProposalAsIs,
    #[fefix(variant = "2")]
    AcceptSecurityProposalWithRevisionsAsIndicatedInTheMessage,
    #[fefix(variant = "3")]
    ListOfSecurityTypesReturnedPerRequest,
    #[fefix(variant = "4")]
    ListOfSecuritiesReturnedPerRequest,
    #[fefix(variant = "5")]
    RejectSecurityProposal,
    #[fefix(variant = "6")]
    CanNotMatchSelectionCriteria,
}

pub const SECURITY_STATUS_REQ_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "SecurityStatusReqID",
    tag: 324,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNSOLICITED_INDICATOR: &FieldDef<'static, UnsolicitedIndicator> = &FieldDef {
    name: "UnsolicitedIndicator",
    tag: 325,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum UnsolicitedIndicator {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const SECURITY_TRADING_STATUS: &FieldDef<'static, SecurityTradingStatus> = &FieldDef {
    name: "SecurityTradingStatus",
    tag: 326,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SecurityTradingStatus {
    #[fefix(variant = "1")]
    OpeningDelay,
    #[fefix(variant = "10")]
    MarketOnCloseImbalanceSell,
    #[fefix(variant = "12")]
    NoMarketImbalance,
    #[fefix(variant = "13")]
    NoMarketOnCloseImbalance,
    #[fefix(variant = "14")]
    ItsPreOpening,
    #[fefix(variant = "15")]
    NewPriceIndication,
    #[fefix(variant = "16")]
    TradeDisseminationTime,
    #[fefix(variant = "17")]
    ReadyToTrade,
    #[fefix(variant = "18")]
    NotAvailableForTrading,
    #[fefix(variant = "19")]
    NotTradedOnThisMarket,
    #[fefix(variant = "2")]
    TradingHalt,
    #[fefix(variant = "20")]
    UnknownOrInvalid,
    #[fefix(variant = "3")]
    Resume,
    #[fefix(variant = "4")]
    NoOpenNoResume,
    #[fefix(variant = "5")]
    PriceIndication,
    #[fefix(variant = "6")]
    TradingRangeIndication,
    #[fefix(variant = "7")]
    MarketImbalanceBuy,
    #[fefix(variant = "8")]
    MarketImbalanceSell,
    #[fefix(variant = "9")]
    MarketOnCloseImbalanceBuy,
}

pub const HALT_REASON_CHAR: &FieldDef<'static, HaltReasonChar> = &FieldDef {
    name: "HaltReasonChar",
    tag: 327,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum HaltReasonChar {
    #[fefix(variant = "D")]
    NewsDissemination,
    #[fefix(variant = "E")]
    OrderInflux,
    #[fefix(variant = "I")]
    OrderImbalance,
    #[fefix(variant = "M")]
    AdditionalInformation,
    #[fefix(variant = "P")]
    NewsPending,
    #[fefix(variant = "X")]
    EquipmentChangeover,
}

pub const IN_VIEW_OF_COMMON: &FieldDef<'static, InViewOfCommon> = &FieldDef {
    name: "InViewOfCommon",
    tag: 328,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum InViewOfCommon {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const DUE_TO_RELATED: &FieldDef<'static, DueToRelated> = &FieldDef {
    name: "DueToRelated",
    tag: 329,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum DueToRelated {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const BUY_VOLUME: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "BuyVolume",
    tag: 330,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SELL_VOLUME: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "SellVolume",
    tag: 331,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const HIGH_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "HighPx",
    tag: 332,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LOW_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LowPx",
    tag: 333,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ADJUSTMENT: &FieldDef<'static, Adjustment> = &FieldDef {
    name: "Adjustment",
    tag: 334,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum Adjustment {
    #[fefix(variant = "1")]
    Cancel,
    #[fefix(variant = "2")]
    Error,
    #[fefix(variant = "3")]
    Correction,
}

pub const TRAD_SES_REQ_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "TradSesReqID",
    tag: 335,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRADING_SESSION_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "TradingSessionID",
    tag: 336,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CONTRA_TRADER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ContraTrader",
    tag: 337,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRAD_SES_METHOD: &FieldDef<'static, TradSesMethod> = &FieldDef {
    name: "TradSesMethod",
    tag: 338,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum TradSesMethod {
    #[fefix(variant = "1")]
    Electronic,
    #[fefix(variant = "2")]
    OpenOutcry,
    #[fefix(variant = "3")]
    TwoParty,
}

pub const TRAD_SES_MODE: &FieldDef<'static, TradSesMode> = &FieldDef {
    name: "TradSesMode",
    tag: 339,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum TradSesMode {
    #[fefix(variant = "1")]
    Testing,
    #[fefix(variant = "2")]
    Simulated,
    #[fefix(variant = "3")]
    Production,
}

pub const TRAD_SES_STATUS: &FieldDef<'static, TradSesStatus> = &FieldDef {
    name: "TradSesStatus",
    tag: 340,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum TradSesStatus {
    #[fefix(variant = "1")]
    Halted,
    #[fefix(variant = "2")]
    Open,
    #[fefix(variant = "3")]
    Closed,
    #[fefix(variant = "4")]
    PreOpen,
    #[fefix(variant = "5")]
    PreClose,
}

pub const TRAD_SES_START_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "TradSesStartTime",
    tag: 341,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRAD_SES_OPEN_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "TradSesOpenTime",
    tag: 342,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRAD_SES_PRE_CLOSE_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "TradSesPreCloseTime",
    tag: 343,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRAD_SES_CLOSE_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "TradSesCloseTime",
    tag: 344,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRAD_SES_END_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "TradSesEndTime",
    tag: 345,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NUMBER_OF_ORDERS: &FieldDef<'static, i64> = &FieldDef {
    name: "NumberOfOrders",
    tag: 346,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MESSAGE_ENCODING: &FieldDef<'static, MessageEncoding> = &FieldDef {
    name: "MessageEncoding",
    tag: 347,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MessageEncoding {
    #[fefix(variant = "EUC-JP")]
    EucJp,
    #[fefix(variant = "ISO-2022-JP")]
    Iso2022Jp,
    #[fefix(variant = "SHIFT_JIS")]
    ShiftJis,
    #[fefix(variant = "UTF-8")]
    Utf8,
}

pub const ENCODED_ISSUER_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedIssuerLen",
    tag: 348,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_ISSUER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedIssuer",
    tag: 349,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_SECURITY_DESC_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedSecurityDescLen",
    tag: 350,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_SECURITY_DESC: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedSecurityDesc",
    tag: 351,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_LIST_EXEC_INST_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedListExecInstLen",
    tag: 352,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_LIST_EXEC_INST: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedListExecInst",
    tag: 353,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_TEXT_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedTextLen",
    tag: 354,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_TEXT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedText",
    tag: 355,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_SUBJECT_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedSubjectLen",
    tag: 356,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_SUBJECT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedSubject",
    tag: 357,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_HEADLINE_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedHeadlineLen",
    tag: 358,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_HEADLINE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedHeadline",
    tag: 359,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_ALLOC_TEXT_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedAllocTextLen",
    tag: 360,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_ALLOC_TEXT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedAllocText",
    tag: 361,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_UNDERLYING_ISSUER_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedUnderlyingIssuerLen",
    tag: 362,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_UNDERLYING_ISSUER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedUnderlyingIssuer",
    tag: 363,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_UNDERLYING_SECURITY_DESC_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedUnderlyingSecurityDescLen",
    tag: 364,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_UNDERLYING_SECURITY_DESC: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedUnderlyingSecurityDesc",
    tag: 365,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ALLOC_PRICE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "AllocPrice",
    tag: 366,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const QUOTE_SET_VALID_UNTIL_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "QuoteSetValidUntilTime",
    tag: 367,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const QUOTE_ENTRY_REJECT_REASON: &FieldDef<'static, QuoteEntryRejectReason> = &FieldDef {
    name: "QuoteEntryRejectReason",
    tag: 368,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum QuoteEntryRejectReason {
    #[fefix(variant = "1")]
    UnknownSymbol,
    #[fefix(variant = "2")]
    Exchange,
    #[fefix(variant = "3")]
    QuoteExceedsLimit,
    #[fefix(variant = "4")]
    TooLateToEnter,
    #[fefix(variant = "5")]
    UnknownQuote,
    #[fefix(variant = "6")]
    DuplicateQuote,
    #[fefix(variant = "7")]
    InvalidBidAskSpread,
    #[fefix(variant = "8")]
    InvalidPrice,
    #[fefix(variant = "9")]
    NotAuthorizedToQuoteSecurity,
}

pub const LAST_MSG_SEQ_NUM_PROCESSED: &FieldDef<'static, i64> = &FieldDef {
    name: "LastMsgSeqNumProcessed",
    tag: 369,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ON_BEHALF_OF_SENDING_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "OnBehalfOfSendingTime",
    tag: 370,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const REF_TAG_ID: &FieldDef<'static, i64> = &FieldDef {
    name: "RefTagID",
    tag: 371,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const REF_MSG_TYPE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "RefMsgType",
    tag: 372,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SESSION_REJECT_REASON: &FieldDef<'static, SessionRejectReason> = &FieldDef {
    name: "SessionRejectReason",
    tag: 373,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SessionRejectReason {
    #[fefix(variant = "0")]
    InvalidTagNumber,
    #[fefix(variant = "1")]
    RequiredTagMissing,
    #[fefix(variant = "10")]
    SendingtimeAccuracyProblem,
    #[fefix(variant = "11")]
    InvalidMsgtype,
    #[fefix(variant = "2")]
    TagNotDefinedForThisMessageType,
    #[fefix(variant = "3")]
    UndefinedTag,
    #[fefix(variant = "4")]
    TagSpecifiedWithoutAValue,
    #[fefix(variant = "5")]
    ValueIsIncorrect,
    #[fefix(variant = "6")]
    IncorrectDataFormatForValue,
    #[fefix(variant = "7")]
    DecryptionProblem,
    #[fefix(variant = "8")]
    SignatureProblem,
    #[fefix(variant = "9")]
    CompidProblem,
}

pub const BID_REQUEST_TRANS_TYPE: &FieldDef<'static, BidRequestTransType> = &FieldDef {
    name: "BidRequestTransType",
    tag: 374,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum BidRequestTransType {
    #[fefix(variant = "C")]
    Cancel,
    #[fefix(variant = "N")]
    No,
}

pub const CONTRA_BROKER: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ContraBroker",
    tag: 375,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const COMPLIANCE_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ComplianceID",
    tag: 376,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SOLICITED_FLAG: &FieldDef<'static, SolicitedFlag> = &FieldDef {
    name: "SolicitedFlag",
    tag: 377,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum SolicitedFlag {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const EXEC_RESTATEMENT_REASON: &FieldDef<'static, ExecRestatementReason> = &FieldDef {
    name: "ExecRestatementReason",
    tag: 378,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ExecRestatementReason {
    #[fefix(variant = "0")]
    GtCorporateAction,
    #[fefix(variant = "1")]
    GtRenewal,
    #[fefix(variant = "2")]
    VerbalChange,
    #[fefix(variant = "3")]
    RepricingOfOrder,
    #[fefix(variant = "4")]
    BrokerOption,
    #[fefix(variant = "5")]
    PartialDeclineOfOrderqty,
}

pub const BUSINESS_REJECT_REF_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "BusinessRejectRefID",
    tag: 379,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BUSINESS_REJECT_REASON: &FieldDef<'static, BusinessRejectReason> = &FieldDef {
    name: "BusinessRejectReason",
    tag: 380,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum BusinessRejectReason {
    #[fefix(variant = "0")]
    Other,
    #[fefix(variant = "1")]
    UnkownId,
    #[fefix(variant = "2")]
    UnknownSecurity,
    #[fefix(variant = "3")]
    UnsupportedMessageType,
    #[fefix(variant = "4")]
    ApplicationNotAvailable,
    #[fefix(variant = "5")]
    ConditionallyRequiredFieldMissing,
}

pub const GROSS_TRADE_AMT: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "GrossTradeAmt",
    tag: 381,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_CONTRA_BROKERS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoContraBrokers",
    tag: 382,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MAX_MESSAGE_SIZE: &FieldDef<'static, i64> = &FieldDef {
    name: "MaxMessageSize",
    tag: 383,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_MSG_TYPES: &FieldDef<'static, i64> = &FieldDef {
    name: "NoMsgTypes",
    tag: 384,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MSG_DIRECTION: &FieldDef<'static, MsgDirection> = &FieldDef {
    name: "MsgDirection",
    tag: 385,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MsgDirection {
    #[fefix(variant = "R")]
    Receive,
    #[fefix(variant = "S")]
    Send,
}

pub const NO_TRADING_SESSIONS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoTradingSessions",
    tag: 386,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TOTAL_VOLUME_TRADED: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "TotalVolumeTraded",
    tag: 387,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DISCRETION_INST: &FieldDef<'static, DiscretionInst> = &FieldDef {
    name: "DiscretionInst",
    tag: 388,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum DiscretionInst {
    #[fefix(variant = "0")]
    RelatedToDisplayedPrice,
    #[fefix(variant = "1")]
    RelatedToMarketPrice,
    #[fefix(variant = "2")]
    RelatedToPrimaryPrice,
    #[fefix(variant = "3")]
    RelatedToLocalPrimaryPrice,
    #[fefix(variant = "4")]
    RelatedToMidpointPrice,
    #[fefix(variant = "5")]
    RelatedToLastTradePrice,
}

pub const DISCRETION_OFFSET: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "DiscretionOffset",
    tag: 389,
    is_group_leader: false,
    data_type: DataType::PriceOffset,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BID_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "BidID",
    tag: 390,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CLIENT_BID_ID: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ClientBidID",
    tag: 391,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIST_NAME: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ListName",
    tag: 392,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TOTAL_NUM_SECURITIES: &FieldDef<'static, i64> = &FieldDef {
    name: "TotalNumSecurities",
    tag: 393,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BID_TYPE: &FieldDef<'static, i64> = &FieldDef {
    name: "BidType",
    tag: 394,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NUM_TICKETS: &FieldDef<'static, i64> = &FieldDef {
    name: "NumTickets",
    tag: 395,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SIDE_VALUE1: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "SideValue1",
    tag: 396,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SIDE_VALUE2: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "SideValue2",
    tag: 397,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NO_BID_DESCRIPTORS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoBidDescriptors",
    tag: 398,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BID_DESCRIPTOR_TYPE: &FieldDef<'static, i64> = &FieldDef {
    name: "BidDescriptorType",
    tag: 399,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const BID_DESCRIPTOR: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "BidDescriptor",
    tag: 400,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const SIDE_VALUE_IND: &FieldDef<'static, i64> = &FieldDef {
    name: "SideValueInd",
    tag: 401,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIQUIDITY_PCT_LOW: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LiquidityPctLow",
    tag: 402,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIQUIDITY_PCT_HIGH: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LiquidityPctHigh",
    tag: 403,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIQUIDITY_VALUE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "LiquidityValue",
    tag: 404,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EFP_TRACKING_ERROR: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "EFPTrackingError",
    tag: 405,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const FAIR_VALUE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "FairValue",
    tag: 406,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const OUTSIDE_INDEX_PCT: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "OutsideIndexPct",
    tag: 407,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const VALUE_OF_FUTURES: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "ValueOfFutures",
    tag: 408,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIQUIDITY_IND_TYPE: &FieldDef<'static, LiquidityIndType> = &FieldDef {
    name: "LiquidityIndType",
    tag: 409,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum LiquidityIndType {
    #[fefix(variant = "1")]
    N5DayMovingAverage,
    #[fefix(variant = "2")]
    N20DayMovingAverage,
    #[fefix(variant = "3")]
    NormalMarketSize,
    #[fefix(variant = "4")]
    Other,
}

pub const WT_AVERAGE_LIQUIDITY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "WtAverageLiquidity",
    tag: 410,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EXCHANGE_FOR_PHYSICAL: &FieldDef<'static, ExchangeForPhysical> = &FieldDef {
    name: "ExchangeForPhysical",
    tag: 411,
    is_group_leader: false,
    data_type: DataType::Boolean,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ExchangeForPhysical {
    #[fefix(variant = "N")]
    No,
    #[fefix(variant = "Y")]
    Yes,
}

pub const OUT_MAIN_CNTRY_U_INDEX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "OutMainCntryUIndex",
    tag: 412,
    is_group_leader: false,
    data_type: DataType::Amt,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CROSS_PERCENT: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "CrossPercent",
    tag: 413,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const PROG_RPT_REQS: &FieldDef<'static, ProgRptReqs> = &FieldDef {
    name: "ProgRptReqs",
    tag: 414,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ProgRptReqs {
    #[fefix(variant = "1")]
    BuysideExplicitlyRequestsStatusUsingStatusrequest,
    #[fefix(variant = "2")]
    SellsidePeriodicallySendsStatusUsingListstatusPeriodOptionallySpecifiedInProgressperiod,
    #[fefix(variant = "3")]
    RealTimeExecutionReports,
}

pub const PROG_PERIOD_INTERVAL: &FieldDef<'static, i64> = &FieldDef {
    name: "ProgPeriodInterval",
    tag: 415,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const INC_TAX_IND: &FieldDef<'static, IncTaxInd> = &FieldDef {
    name: "IncTaxInd",
    tag: 416,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum IncTaxInd {
    #[fefix(variant = "1")]
    Net,
    #[fefix(variant = "2")]
    Gross,
}

pub const NUM_BIDDERS: &FieldDef<'static, i64> = &FieldDef {
    name: "NumBidders",
    tag: 417,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TRADE_TYPE: &FieldDef<'static, TradeType> = &FieldDef {
    name: "TradeType",
    tag: 418,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum TradeType {
    #[fefix(variant = "A")]
    Agency,
    #[fefix(variant = "G")]
    VwapGuarantee,
    #[fefix(variant = "J")]
    GuaranteedClose,
    #[fefix(variant = "R")]
    RiskTrade,
}

pub const BASIS_PX_TYPE: &FieldDef<'static, BasisPxType> = &FieldDef {
    name: "BasisPxType",
    tag: 419,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum BasisPxType {
    #[fefix(variant = "2")]
    ClosingPriceAtMorningSession,
    #[fefix(variant = "3")]
    ClosingPrice,
    #[fefix(variant = "4")]
    CurrentPrice,
    #[fefix(variant = "5")]
    Sq,
    #[fefix(variant = "6")]
    VwapThroughADay,
    #[fefix(variant = "7")]
    VwapThroughAMorningSession,
    #[fefix(variant = "8")]
    VwapThroughAnAfternoonSession,
    #[fefix(variant = "9")]
    VwapThroughADayExceptYori,
    #[fefix(variant = "A")]
    VwapThroughAMorningSessionExceptYori,
    #[fefix(variant = "B")]
    VwapThroughAnAfternoonSessionExceptYori,
    #[fefix(variant = "C")]
    Strike,
    #[fefix(variant = "D")]
    Open,
    #[fefix(variant = "Z")]
    Others,
}

pub const NO_BID_COMPONENTS: &FieldDef<'static, i64> = &FieldDef {
    name: "NoBidComponents",
    tag: 420,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const COUNTRY: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "Country",
    tag: 421,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const TOT_NO_STRIKES: &FieldDef<'static, i64> = &FieldDef {
    name: "TotNoStrikes",
    tag: 422,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const PRICE_TYPE: &FieldDef<'static, PriceType> = &FieldDef {
    name: "PriceType",
    tag: 423,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum PriceType {
    #[fefix(variant = "1")]
    Percentage,
    #[fefix(variant = "2")]
    PerShare,
    #[fefix(variant = "3")]
    FixedAmount,
}

pub const DAY_ORDER_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "DayOrderQty",
    tag: 424,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DAY_CUM_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "DayCumQty",
    tag: 425,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const DAY_AVG_PX: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "DayAvgPx",
    tag: 426,
    is_group_leader: false,
    data_type: DataType::Price,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const GT_BOOKING_INST: &FieldDef<'static, GtBookingInst> = &FieldDef {
    name: "GTBookingInst",
    tag: 427,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum GtBookingInst {
    #[fefix(variant = "0")]
    BookOutAllTradesOnDayOfExecution,
    #[fefix(variant = "1")]
    AccumulateExecutionsUntilOrderIsFilledOrExpires,
    #[fefix(variant = "2")]
    AccumulateUntilVerballyNotifiedOtherwise,
}

pub const NO_STRIKES: &FieldDef<'static, i64> = &FieldDef {
    name: "NoStrikes",
    tag: 428,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIST_STATUS_TYPE: &FieldDef<'static, i64> = &FieldDef {
    name: "ListStatusType",
    tag: 429,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const NET_GROSS_IND: &FieldDef<'static, NetGrossInd> = &FieldDef {
    name: "NetGrossInd",
    tag: 430,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum NetGrossInd {
    #[fefix(variant = "1")]
    Net,
    #[fefix(variant = "2")]
    Gross,
}

pub const LIST_ORDER_STATUS: &FieldDef<'static, i64> = &FieldDef {
    name: "ListOrderStatus",
    tag: 431,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const EXPIRE_DATE: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ExpireDate",
    tag: 432,
    is_group_leader: false,
    data_type: DataType::LocalMktDate,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIST_EXEC_INST_TYPE: &FieldDef<'static, ListExecInstType> = &FieldDef {
    name: "ListExecInstType",
    tag: 433,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum ListExecInstType {
    #[fefix(variant = "1")]
    Immediate,
    #[fefix(variant = "2")]
    WaitForExecuteInstruction,
}

pub const CXL_REJ_RESPONSE_TO: &FieldDef<'static, CxlRejResponseTo> = &FieldDef {
    name: "CxlRejResponseTo",
    tag: 434,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum CxlRejResponseTo {
    #[fefix(variant = "1")]
    OrderCancelRequest,
    #[fefix(variant = "2")]
    OrderCancelReplaceRequest,
}

pub const UNDERLYING_COUPON_RATE: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "UnderlyingCouponRate",
    tag: 435,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const UNDERLYING_CONTRACT_MULTIPLIER: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "UnderlyingContractMultiplier",
    tag: 436,
    is_group_leader: false,
    data_type: DataType::Float,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CONTRA_TRADE_QTY: &FieldDef<'static, rust_decimal::Decimal> = &FieldDef {
    name: "ContraTradeQty",
    tag: 437,
    is_group_leader: false,
    data_type: DataType::Qty,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CONTRA_TRADE_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "ContraTradeTime",
    tag: 438,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CLEARING_FIRM: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ClearingFirm",
    tag: 439,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const CLEARING_ACCOUNT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ClearingAccount",
    tag: 440,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIQUIDITY_NUM_SECURITIES: &FieldDef<'static, i64> = &FieldDef {
    name: "LiquidityNumSecurities",
    tag: 441,
    is_group_leader: false,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const MULTI_LEG_REPORTING_TYPE: &FieldDef<'static, MultiLegReportingType> = &FieldDef {
    name: "MultiLegReportingType",
    tag: 442,
    is_group_leader: false,
    data_type: DataType::Char,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataField)]
pub enum MultiLegReportingType {
    #[fefix(variant = "1")]
    SingleSecurity,
    #[fefix(variant = "2")]
    IndividualLegOfAMultiLegSecurity,
    #[fefix(variant = "3")]
    MultiLegSecurity,
}

pub const STRIKE_TIME: &FieldDef<'static, fefix::dtf::Timestamp> = &FieldDef {
    name: "StrikeTime",
    tag: 443,
    is_group_leader: false,
    data_type: DataType::UtcTimestamp,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const LIST_STATUS_TEXT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "ListStatusText",
    tag: 444,
    is_group_leader: false,
    data_type: DataType::String,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_LIST_STATUS_TEXT_LEN: &FieldDef<'static, i64> = &FieldDef {
    name: "EncodedListStatusTextLen",
    tag: 445,
    is_group_leader: true,
    data_type: DataType::Int,
    phantom: PhantomData,
    location: FieldLocation::Body,
};

pub const ENCODED_LIST_STATUS_TEXT: &FieldDef<'static, &[u8]> = &FieldDef {
    name: "EncodedListStatusText",
    tag: 446,
    is_group_leader: false,
    data_type: DataType::Data,
    phantom: PhantomData,
    location: FieldLocation::Body,
};
