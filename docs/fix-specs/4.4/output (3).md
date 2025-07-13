
FINANCIAL INFORMATION
EXCHANGE PROTOCOL
(FIX)
Version 4.4 with Errata 20030618


# VOLUME 4 – FIX APPLICATION MESSAGES: ORDERS AND EXECUTIONS (TRADE)

Includes Errata adjustments as of June 18, 2003

# Errata Purpose:

This document includes a list of minor adjustments to the FIX 4.4 Specification document due to typographical errors or ambiguities. The nature and scope of Errata adjustments do not introduce new functionality, additional fields, new values for existing fields, or new messages. Regretably some functionality was introduced in FIX 4.4 which contained errors that required a new value or field on a specific message in order to make the intended functionality implementable. Any such exceptions to the “do not introduce”, “additional fields”, or “new messages” Errata rules were kept to a minimum using the “required to make the intended functionality implementable” rationale. The list of items has been reviewed and approved by the FIX Technical Committee and Steering Committees. Implementers of FIX version 4.4 should refer to this document to ensure the most consistent implementation and clearest understanding of the FIX protocol.

The specific adjustments made to the original FIX version 4.4 specification as a result of the Errata can be seen and printed via Microsoft Word’s revision feature of this document. A separate document with an itemized list of changes is available via the FIX website.

~~April 30, 2003~~June 18, 2003


~~April 30, 2003~~June 18, 2003              1        FIX 4.4 with Errata 20030618- Volume 4

---

Contents – Volume 4

# FIX APPLICATION MESSAGES: ORDERS AND EXECUTIONS (TRADE)

# CATEGORY: SINGLE/GENERAL ORDER HANDLING

# New Order - Single -

7

# Execution Reports -

13

# Use of the Execution Report for Multileg Instruments:

24

# Don’t Know Trade (DK) -

26

# Order Cancel/Replace Request (a.k.a. Order Modification Request) -

28

# Order Cancel Request -

34

# Order Cancel Reject -

36

# Order Status Request -

38

# Order Mass Cancel Request

40

# Order Mass Cancel Report

42

# Order Mass Status Request

44

# Order State Change Matrices

# A Vanilla

51

# B Cancel

52

# C Cancel/Replace quantity changes

# C.1 Replace to increase quantity

57

# C.2 Replace not for quantity change

60

# C.3 Replace to decrease quantity

61

# D Cancel/Replace sequencing and chaining

# D.1 Sequencing

63

# D.2 Chaining

66

# E Unsolicited/Reinstatement

70

# F Order Reject

72

# G Status

74

# H GT

76

# I TimeInForce

80

# J Execution Cancels/Corrects

81

# K Trading Halt

84

# L Miscellaneous

85

# Order Handling and Instruction Semantics

# London SETS Order Types Matrix

87

# Asia/Pacific Regional Order Handling

87

# Japanese Exchange Price Conditions

88

# Euronext and Similar Exchange Price Conditions

88

# Handling Instructions (HandlInst) field

88

# Pegged Orders

89

# Discretionary Pricing

90

# “Target Strategy” Orders

90

# “Reserve Quantity” Orders

91

# Time In Force (TIF)

91

# Booking Instructions Specified at Time of Order

91

# OrderCapacity and OrderRestrictions (formerly Rule80A) Usage by Market

# Example Usage of PartyRole="Investor ID"

# Format of the Party ID field (PartyRole="Investor ID")

96

# Example Representations of Orders

96

# KOREA

97

# TAIWAN

97

# Additional Notes:

97

# CATEGORY: CROSS ORDERS

# Background

98

~~April 30, 2003~~June 18, 2003                  2             FIX 4.4 with Errata 20030618- Volume 4


---

# Prioritization of a side of a cross order


# Classification of cross trades

# Execution Reporting for cross orders

# Cross Order Handling Rules

# Acknowledgement of a Cross Order

# Message Flow for cross order with CrossType=1 with only one side of the order provided

# Message Flow for cross order with CrossType=1 when both sides of the cross order provided

# Message Flow for cross order with CrossType=2

# Message Flow for cross order with CrossType=3

# Message Flow for cross order with CrossType=4

# Cross Order Messages

# New Order - Cross

# Cross Order Cancel/Replace Request ( a.k.a. Cross Order Modification Request)

# Cross Order Cancel Request

# Cross Order Change Matrices

# Cross Type 1

# Cross Type 2

# Cross Type 3

# Cross Type 4

# CATEGORY: MULTILEG ORDERS (SWAPS, OPTION STRATEGIES, ETC)

# Background

# Predefined Multileg Security Model (FIX 4.2) (Model 1)

# Enhanced Predefined Security Model (Model 2)

# Product Definition Model using New Order - Multileg Message (Model 3)

# Single Message Model (Model 4)

# Messages Used for Multileg Trading

# New Order - Multileg

# Multileg Order Cancel Replace Request (a.k.a MultilegOrder Modification Request)

# CATEGORY: LIST/PROGRAM/BASKET TRADING

# Bid Request -

# Bid Response -

# New Order - List -

# List Strike Price -

# List Status -

# List Execute -

# List Cancel Request -

# List Status Request -

# Fragmentation for List Order Messages

# Program/Basket/List Trading

# Overview

# Message Flow Diagrams

# Scenario 1 Bidding performed by Telephone and List provided via FIX

# Scenario 2 Fully Disclosed Program Trade – with bidding stage through FIX

# Scenario 3 Non-Disclosed Program Trade – with bidding stage through FIX

# Illustration of liquidity indicator fields usage

~~April 30, 2003~~June 18, 2003                         3  FIX 4.4 with Errata 20030618- Volume 4
---
FIX APPLICATION MESSAGES: ORDERS AND EXECUTIONS (TRADE)

“Orders and Executions” (or “Trade”) messaging is characterized as messages which are used to place or amend orders and communicate the results and status of orders.

The specific FIX “Orders and Executions” (or “Trade”) messaging categories are:

1. SINGLE/GENERAL ORDER HANDLING
2. CROSS ORDER HANDLING
3. MULTILEG ORDER HANDLING
4. LIST/PROGRAM/BASKET TRADING

Descriptions and formats of the specific FIX “Orders and Executions” (or “Trade”) application messages follow.

~~April 30, 2003~~ June 18, 2003 4 FIX 4.4 with Errata 20030618- Volume 4


---

CATEGORY: SINGLE/GENERAL ORDER HANDLING


See Volume 7 – PRODUCT: FIXED INCOME for usage guidance in using general order handling messages for Fixed Income trading.

# New Order - Single -

The new order message type is used by institutions wishing to electronically submit securities and forex orders to a broker for execution. The New Order message type may also be used by institutions or retail intermediaries wishing to electronically submit Collective Investment Vehicle (CIV) orders to a broker or fund manager for execution. See VOLUME 7 - "PRODUCT: COLLECTIVE INVESTMENT VEHICLES".

Orders can be submitted with special handling instructions and execution instructions. Handling instructions refer to how the broker should handle the order on its trading floor (see HandlInst field). Execution instructions contain explicit directions as to how the order should be executed (see ExecInst field).

New Order messages received with the PossResend flag set in the header should be validated by ClOrdID. Implementations should also consider checking order parameters (side, symbol, quantity, etc.) to determine if the order had been previously submitted. PossResends previously received should be acknowledged back to the client via an Execution - Status message. PossResends not previously received should be processed as a new order and acknowledged via an Execution - New message.

The value specified in the TransactTime field should allow the receiver of the order to apply business rules to determine if the order is potentially "stale" (e.g. in the event that there have been communication problems). To support forex accommodation trades, two fields, ForexReq and SettlCurrency, are included in the message. To request a broker to execute a forex trade in conjunction with the securities trade, the institution would set the ForexReq = Y and SettlCurrency = “intended settlement currency”. The broker would then execute a forex trade from the execution currency to the settlement currency and report the results via the execution message in the SettlCurrAmt and SettlCurrency fields.

The order message can also be used to request a straight forex trade. Conventions for identifying a forex transaction are as follows:

- The forex Symbol is defined in Electronic Broking Services, Ltd. (see http://www.ebs.com) format: "CCY1/CCY2".
- Rates are expressed as "currency1 in currency2" (or "currency2 per currency1") and are calculated as CCY2 divided by CCY1 (NOT CCY1 divided by CCY2) (e.g. "GBP/USD" represents a rate expressed as USD per GBP, "USD/JPY" represents a rate expressed as JPY per USD, etc.).
- CCY1 and CCY2 are ISO currency codes.
- The value of the Currency field represents the denomination of the quantity fields (e.g. JPY represents quantity of JPY).
- In the case of a Forex - Swap (buying (or selling) a currency at one value date and selling (or buying) the same currency at a different value date), Side should represent the side of the SettlDate2 transaction.
- OrdType = Market, Limit, Forex - Swap, or Previously Quoted. Product = Currency.
- Netting can be specified via the ExecInst field.

See VOLUME 7 - "PRODUCT: FOREIGN EXCHANGE"

Orders involving or requiring Pre-Trade Allocation consist of the following steps:

~~April 30, 2003~~June 18, 2003 5 FIX 4.4 with Errata 20030618- Volume 4



---

# Buyside sends a New Order request message specifying one or more AllocAccount and AllocQty values within the repeating group designated by NoAllocs.

Sellside sends Execution Report messages for the “New” and resulting fills. Post-Trade Allocation messaging takes place.

To “take” an IOI (or Quote) from an ECN or exchange and not display the order on the book, the New Order message should contain the TimeInForce field with ImmediateOrCancel and an OrdType field with Previously Indicated (or Previously Quoted).

See “Order State Change Matrices”.

# The format for the new order message is as follows:

| Tag             | Field Name             | Req'd | Comments                                                                                                                                                          |
| --------------- | ---------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|                 | Standard Header        | Y     | MsgType = D                                                                                                                                                       |
| 11              | ClOrdID                | Y     | Unique identifier of the order as assigned by institution or by the intermediary (CIV term, not a hub/service bureau) with closest association with the investor. |
| 526             | SecondaryClOrdID       | N     |                                                                                                                                                                   |
| 583             | ClOrdLinkID            | N     |                                                                                                                                                                   |
| component block | \<Parties>             | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                              |
| 229             | TradeOriginationDate   | N     |                                                                                                                                                                   |
| 75              | TradeDate              | N     |                                                                                                                                                                   |
| 1               | Account                | N     |                                                                                                                                                                   |
| 660             | AcctIDSource           | N     |                                                                                                                                                                   |
| 581             | AccountType            | N     | Type of account associated with the order (Origin)                                                                                                                |
| 589             | DayBookingInst         | N     |                                                                                                                                                                   |
| 590             | BookingUnit            | N     |                                                                                                                                                                   |
| 591             | PreallocMethod         | N     |                                                                                                                                                                   |
| 70              | AllocID                | N     | Used to assign an overall allocation id to the block of preallocations                                                                                            |
| 78              | NoAllocs               | N     | Number of repeating groups for pre-trade allocation                                                                                                               |
|                 | 79 AllocAccount        | N     | Required if NoAllocs > 0. Must be first field in repeating group.                                                                                                 |
|                 | 661 AllocAcctIDSource  | N     |                                                                                                                                                                   |
|                 | 736 AllocSettlCurrency | N     |                                                                                                                                                                   |
|                 | 467 IndividualAllocID  | N     |                                                                                                                                                                   |
| component block | \<NestedParties>       | N     | Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined                                           |

~~April 30, 2003~~June 18, 2003
# 6 FIX 4.4 with Errata 20030618- Volume 4


---

in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# Used for NestedPartyRole=Clearing Firm

| 80                                                                                                                                                                                   | AllocQty             | N |                                                                                                                                                                 |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------- | - | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 63                                                                                                                                                                                   | SettlType            | N |                                                                                                                                                                 |
| 64                                                                                                                                                                                   | SettlDate            | N | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                         |
| 544                                                                                                                                                                                  | CashMargin           | N |                                                                                                                                                                 |
| 635                                                                                                                                                                                  | ClearingFeeIndicator | N |                                                                                                                                                                 |
| 21                                                                                                                                                                                   | HandlInst            | N |                                                                                                                                                                 |
| 18                                                                                                                                                                                   | ExecInst             | N | Can contain multiple instructions, space delimited. If OrdType=P, exactly one of the following values (ExecInst = L, R, M, P, O, T, W, a, d) must be specified. |
| 110                                                                                                                                                                                  | MinQty               | N |                                                                                                                                                                 |
| 111                                                                                                                                                                                  | MaxFloor             | N |                                                                                                                                                                 |
| 100                                                                                                                                                                                  | ExDestination        | N |                                                                                                                                                                 |
| 386                                                                                                                                                                                  | NoTradingSessions    | N | Specifies the number of repeating TradingSessionIDs                                                                                                             |
| 336                                                                                                                                                                                  | TradingSessionID     | N | Required if NoTradingSessions is > 0.                                                                                                                           |
| 625                                                                                                                                                                                  | TradingSessionSubID  | N |                                                                                                                                                                 |
| 81                                                                                                                                                                                   | ProcessCode          | N | Used to identify soft trades at order entry.                                                                                                                    |
| component block \<Instrument> Y Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                        |                      |   |                                                                                                                                                                 |
| 711                                                                                                                                                                                  | NoUnderlyings        | N | Number of underlyings                                                                                                                                           |
| component block N Must be provided if Number of underlyings > 0 \<UnderlyingInstrument>                                                                                              |                      |   |                                                                                                                                                                 |
| 140                                                                                                                                                                                  | PrevClosePx          | N | Useful for verifying security identification                                                                                                                    |
| 54                                                                                                                                                                                   | Side                 | Y |                                                                                                                                                                 |
| 114                                                                                                                                                                                  | LocateReqd           | N | Required for short sell orders                                                                                                                                  |
| 60                                                                                                                                                                                   | TransactTime         | Y | Time this order request was initiated/released by the trader, trading system, or intermediary.                                                                  |
| component block \<Stipulations> N Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |                      |   |                                                                                                                                                                 |
| 854                                                                                                                                                                                  | QtyType              | N |                                                                                                                                                                 |
| component block \<OrderQtyData> Y Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                |                      |   |                                                                                                                                                                 |
| 40                                                                                                                                                                                   | OrdType              | Y |                                                                                                                                                                 |

~~April 30, 2003~~June 18, 2003

7 FIX 4.4 with Errata 20030618- Volume 4


---

# 423 PriceType

N

# 44 Price

N Required for limit OrdTypes. For F/X orders, should be the “all-in” rate (spot rate adjusted for forward points). Can be used to specify a limit price for a pegged order, previously indicated, etc.

# 99 StopPx

N Required for OrdType = “Stop” or OrdType = “Stop limit”.

# component

block N Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# component

block <yielddata> N Insert here the set of "YieldData" (yield-related) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"</yielddata>

# 15 Currency

N

# 376 ComplianceID

N

# 377 SolicitedFlag

N

# 23 IOIid

N Required for Previously Indicated Orders (OrdType=E)

# 117 QuoteID

N Required for Previously Quoted Orders (OrdType=D)

# 59 TimeInForce

N Absence of this field indicates Day order

# 168 EffectiveTime

N Can specify the time at which the order should be considered valid

# 432 ExpireDate

N Conditionally required if TimeInForce = GTD and ExpireTime is not specified.

# 126 ExpireTime

N Conditionally required if TimeInForce = GTD and ExpireDate is not specified.

# 427 GTBookingInst

N States whether executions are booked out or accumulated on a partially filled GT order

# component

block <commissiondata> N Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"</commissiondata>

# 528 OrderCapacity

N

# 529 OrderRestrictions

N

# 582 CustOrderCapacity

N

# 121 ForexReq

N Indicates that broker is requested to execute a Forex accommodation trade in conjunction with the security trade.

# 120 SettlCurrency

N Required if ForexReq = Y.

# 775 BookingType

N Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking.

# 58 Text

N

# 354 EncodedTextLen

N Must be set if EncodedText field is specified and must immediately precede it.

# 355 EncodedText

N Encoded (non-ASCII characters) representation of the Text

~~April 30, 2003~~ June 18, 2003 8 FIX 4.4 with Errata 20030618- Volume 4


---

# FIXML Definition for this message

See http://www.fixprotocol.org for details

| Field                                     | Required | Description                                                                                                                                                                                                           |
| ----------------------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| SettlDate2                                | N        | Can be used with OrdType = “Forex - Swap” to specify the “value date” for the future portion of a F/X swap.                                                                                                           |
| OrderQty2                                 | N        | Can be used with OrdType = “Forex - Swap” to specify the order quantity for the future portion of a F/X swap.                                                                                                         |
| Price2                                    | N        | Can be used with OrdType = “Forex - Swap” to specify the price for the future portion of a F/X swap which is also a limit order. For F/X orders, should be the “all-in” rate (spot rate adjusted for forward points). |
| PositionEffect                            | N        | For use in derivatives omnibus accounting                                                                                                                                                                             |
| CoveredOrUncovered                        | N        | For use with derivatives, such as options                                                                                                                                                                             |
| MaxShow                                   | N        |                                                                                                                                                                                                                       |
| component block \<PegInstructions>        | N        | Insert here the set of "PegInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                                 |
| component block \<DiscretionInstructions> | N        | Insert here the set of "DiscretionInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                          |
| TargetStrategy                            | N        | The target strategy of the order                                                                                                                                                                                      |
| TargetStrategyParameters                  | N        | For further specification of the TargetStrategy                                                                                                                                                                       |
| ParticipationRate                         | N        | Mandatory for a TargetStrategy=Participate order and specifies the target participation rate. For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)  |
| CancellationRights                        | N        | For CIV - Optional                                                                                                                                                                                                    |
| MoneyLaunderingStatus                     | N        |                                                                                                                                                                                                                       |
| RegistID                                  | N        | Reference to Registration Instructions message for this Order.                                                                                                                                                        |
| Designation                               | N        | Supplementary registration information for this Order                                                                                                                                                                 |
| Standard Trailer                          | Y        |                                                                                                                                                                                                                       |

April 30, 2003

June 18, 2003

FIX 4.4 with Errata 20030618 - Volume 4


---

# Money Laundering Status

| RegistID? | Designation? | Accrued Interest Rate? | Accrued Interest Amount? | Net Money? |
| --------- | ------------ | ---------------------- | ------------------------ | ---------- |

Order Custom: %OrderCustom;

&#x3C;!ELEMENT Order (%OrderContent;)>

&#x3C;!ATTLIST Order FIXTag CDATA #FIXED '35'>

DataType CDATA #FIXED 'String'

Value CDATA #FIXED 'D' > Refer to FIXML element NewOrdSingle

Date: April 30, 2003

Revision Date: June 18, 2003

Version: FIX 4.4 with Errata 20030618 - Volume 4


---
Execution Reports -
The execution report message is used to:

1. confirm the receipt of an order
2. confirm changes to an existing order (i.e. accept cancel and replace requests)
3. relay order status information
4. relay fill information on working orders
5. relay fill information on tradeable or restricted tradeable quotes
6. reject orders
7. report post-trade fees calculations associated with a trade

NOTE: Execution reports do not replace the end-of-day confirm. Execution reports are to be regarded only as replacements for the existing fill messages currently communicated via telephone.

NOTE: Individual Execution Reports are sent for each order on a New Order - List.

Each execution report contains two fields which are used to communicate both the current state of the order as understood by the broker (OrdStatus) and the purpose of the message (ExecType).

In an execution report the OrdStatus is used to convey the current state of the order. If an order simultaneously exists in more than one order state, the value with highest precedence is the value that is reported in the OrdStatus field. The order statuses are as follows (in highest to lowest precedence):

| Precedence | OrdStatus        | Description                                                                                                                                                                 |
| ---------- | ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 11         | Pending Cancel   | Order with an Order Cancel Request pending, used to confirm receipt of an Order Cancel Request. DOES NOT INDICATE THAT THE ORDER HAS BEEN CANCELED.                         |
| 10         | Pending Replace  | Order with an Order Cancel/Replace Request pending, used to confirm receipt of an Order Cancel/Replace Request. DOES NOT INDICATE THAT THE ORDER HAS BEEN REPLACED.         |
| 9          | Done for Day     | Order not, or partially, filled; no further executions forthcoming for the trading day                                                                                      |
| 8          | Calculated       | Order has been completed for the day (either filled or done for day). Commission or currency settlement details have been calculated and reported in this execution message |
| 7          | Filled           | Order completely filled, no remaining quantity                                                                                                                              |
| 6          | Stopped          | Order has been stopped at the exchange. Used when guaranteeing or protecting a price and quantity                                                                           |
| 5          | Suspended        | Order has been placed in suspended state at the request of the client.                                                                                                      |
| 4          | Canceled         | Canceled order with or without executions                                                                                                                                   |
| 4          | Expired          | Order has been canceled in broker’s system due to time in force instructions.                                                                                               |
| 3          | Partially Filled | Outstanding order with executions and remaining quantity                                                                                                                    |
| 2          | New              | Outstanding order with no executions                                                                                                                                        |
| 2          | Rejected         | Order has been rejected by sell-side (broker, exchange, ECN). NOTE: An order can be rejected subsequent to                                                                  |

April 30, 2003 June 18, 2003 11 FIX 4.4 with Errata 20030618- Volume 4
---

# Order Acknowledgment

i.e. an order can pass from New to Rejected status.

| 2 | Pending New          | Order has been received by sell-side’s (broker, exchange, ECN) system but not yet accepted for execution. An execution message with this status will only be sent in response to a Status Request message. |
| - | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 | Accepted for bidding | Order has been received and is being evaluated for pricing. It is anticipated that this status will only be used with the “Disclosed” BidType List Order Trading model.                                    |

The ExecType is used to identify the purpose of the execution report message. To transmit a change in OrdStatus for an order, the broker (sell side) should send an Execution Report with the new OrdStatus value in both the ExecType AND the OrdStatus fields to signify this message is changing the state of the order. The only exception to this rule is that when rejecting a cancel or cancel/replace request the CancelReject message is used both to reject the request and to communicate the current OrdStatus. An ExecType of Pending Cancel or Pending Replace is used to indicate that a cancel or cancel/replace request is being processed. An ExecType of Canceled or Replace is used to indicate that the cancel or cancel/replace request has been successfully processed.

Execution information (e.g. new partial fill or complete fill) should not be communicated in the same report as one which communicates other state changes (such as pending cancel, pending replace, canceled, replaced, accepted, done for day etc).

Any fills which occur and need to be communicated to the customer while an order is “pending” and waiting to achieve a new state (i.e. via a Order Cancel Replace (aka Order Modification) Request) must contain the “original” (current order prior to state change request) order parameters (i.e. ClOrdID, OrderQty, Price, etc). These fills will cause the CumQty and AvgPx to be updated. An order cannot be considered replaced until it has been explicitly accepted and confirmed to have reached the replaced status via an execution report with ExecType = ‘Replace’, at which time the effect of the replacement (ClOrdID, new quantity or limit price etc) will be seen.

Requests to cancel or cancel/replace an order are only acted upon when there is an outstanding order quantity. Requests to replace the OrderQty to a level less than the CumQty will be interpreted by the broker as requests to stop executing the order. Requests to change price on a filled order will be rejected (see Order Cancel Reject message type). The OrderQty, CumQty, LeavesQty, and AvgPx fields should be calculated to reflect the cumulative result of all versions of an order. For example, if partially filled order A were replaced by order B, the OrderQty, CumQty, LeavesQty, and AvgPx on order B’s fills should represent the cumulative result of order A plus those on order B.

The general rule is: OrderQty = CumQty + LeavesQty.

There can be exceptions to this rule when ExecType and/or OrdStatus are Canceled, DoneForTheDay (e.g. on a day order), Expired, Calculated, or Rejected in which case the order is no longer active and LeavesQty could be 0.

Communication of information about a new fill is via the Execution report with ExecType = Trade. Execution Reports with ExecType = Trade Cancel or Trade Correct are used to cancel or correct a previously modified execution report as follows:

The ExecType of Trade Cancel applies at the execution level and is used to cancel an execution which has been reported in error. The canceled execution will be identified in the ExecRefID field. Note: ExecType of Trade Cancel should not be used to cancel a previous ExecutionRpt with ExecType of Trade Cancel (e.g. cannot cancel a cancel).

The ExecType of Trade Correct applies at the execution level and is used to modify an incorrectly reported fill. The incorrect execution will be identified in the ExecRefID field. If a single execution is corrected more than once, ExecRefID should refer to the ExecID of the last corrected ExecutionRpt.

~~April 30, 2003~~ June 18, 2003

FIX 4.4 with Errata 20030618- Volume 4


---

(same convention as ClOrdID and OrigClOrdID). To correct an ExecutionRpt which was previously canceled, an ExecutionRpt with ExecType=Trade should be sent (e.g. cannot send ExecType=Trade Cancel). Note: Data reported in the CumQty, LeavesQty, and AvgPx fields represent the status of the order as of the time of the correction, not as of the time of the originally reported execution.

An ExecType of Order Status indicates that the execution messages contains no new information, only summary information regarding order status. It is used, for example, in response to an Order Status request message.

See "Order State Change Matrices" for examples of key state changes, processing of cancel and cancel/replace requests, and for execution cancel/corrects. An ExecutionRpt with ExecType = Restated represents an ExecutionRpt sent by the sellside communicating a change in the order or a restatement of the order’s parameters without an electronic request from the customer. ExecRestatementReason must be set. This is used for GT orders and corporate actions (see below), changes communicated verbally to the sellside either due to normal business practices or as an emergency measure when electronic systems are not available, repricing of orders by the sellside (such as making Sell Short orders compliant with uptick / downtick rules), or other reasons (Broker option). ExecRestatementReason can also be used to communicate unsolicited cancels.

The field ClOrdID is provided for institutions or buy-side brokers or intermediaries to affix an identification number to an order to coincide with internal systems. The OrderID field is populated with the sell-side broker-generated order number (or fund manager-generated order number for CIVs). Unlike ClOrdID/OrigClOrdID which requires a chaining through Cancel/Replaces and Cancels, OrderID and SecondaryOrderID are not required to change through changes to an order.

The underlying business assumption of orders that can trade over multiple days, such as GTC and Good Till Date orders expiring on a future trading date (henceforth referred to as GT orders) is that a GT order that is not fully executed and has not been canceled and has not expired on a given day remains good for the broker to execute the following day. Note that the concept of “day” is determined by the market convention, which will be security specific. At the end of each trading day, once the order is no longer subject to execution, the broker may optionally send an Execution Report with ExecType=Done for Day(3). When the ExpireDate or ExpireTime of a Good Till Date order is reached, or a GTC order reaches a maximum age, the order is considered expired and the broker may optionally send an Execution Report with ExecType and OrdStatus=Expired(C).

In handling GT orders, the OrderQty, CumQty and AvgPx fields will represent the entirety of the order over all days. The fields DayOrderQty, DayCumQty, and DayAvgPx can be used on days following the day of the first trade on a GT order. Prior to the start of business each day, for all GT orders that have partial fills on previous days, DayCumQty and DayAvgPx are set to zero, and DayOrderQty becomes the LeavesQty. The following relationship holds: DayOrderQty = OrderQty – (CumQty – DayCumQty). Since (CumQty – DayCumQty) represents the volume traded on all previous days, DayOrderQty = OrderQty – Volume traded on all previous days. Note that when changing the quantity of an order, both OrderQty and DayOrderQty will change. Requests to change or cancel an order will be made in terms of the total quantity for the order, not the quantity open today. For example, on an order where OrderQty=10000 and 2000 shares trade during the previous days, a request to change OrderQty to 15000 will mean that 13000 shares will be open. See "Order State Change Matrices" for examples of canceling and changing GT orders partially filled on previous days.

A Cancel on an execution (trade bust, ExecType = Trade Cancel) happening the same day of the trade will result in CumQty and DayCumQty each decreasing by the quantity busted, and LeavesQty increasing by the quantity busted. OrderQty and DayOrderQty will remain unchanged. If the business rules allow for a trade bust to be reported on a later date than the trade being busted, the OrderQty and DayCumQty will remain.

~~April 30, 2003~~June 18, 2003 13 FIX 4.4 with Errata 20030618- Volume 4

---

unchanged, the LeavesQty and DayOrderQty will increase by the quantity busted, and the CumQty will decrease by the quantity busted.

If bilaterally agreed between counterparties, a broker may wish to transmit a list of all open GT orders, permitting reconciliation of the open orders. Typically this transmission may occur at the end of the trading day or at the start of the following trading day. There is no expected response to such retransmission; in the event of a reconciliation problem this should be resolved manually or via the DK message. Assuming no corporate actions have occurred, the broker will send an Execution Report with ExecType = Restated (D) and ExecRestatementReason = GT renewal / restatement (no corporate action) (1) for each open GT order. These Execution Reports may have DayCumQty and DayAvgPx restated to zero, and DayOrderQty restated to LeavesQty if the transmission occurs at the start of the following business day. The broker has the option of changing the OrderID and SecondaryOrderID fields, or leaving them unchanged. If they are changed, then the buy-side should use these new ID fields when sending Order Cancel Request, Order Cancel/Replace Request, and Order Status Request messages.

In the case of a corporate action resulting in the adjustment of an open GT order, the broker will send an Execution Report with ExecType = Restated (D) and ExecRestatementReason = GT Corporate action (0) with the order’s state after the corporate action adjustment. In the case of stock splits, OrderQty, CumQty, AvgPx, and LeavesQty will be adjusted to reflect the order’s state in terms of current quantity (e.g. shares), not pre-split quantity (e.g. shares). See "Order State Change Matrices" for examples of GT order restatement with and without a corporate action.

CIV orders to be executed by the fund manager do not use the TimeInForce field and only a subset of OrdStatus values are expected to be used. See VOLUME 7 - "PRODUCT: COLLECTIVE INVESTMENT VEHICLES" for the CIV-specific OrdStatus values.

The Execution Report message is also used for multileg instrument. See “Use of the Execution Report for Multileg Instruments” for multileg-specific details.

# The execution report message format is as follows:

| Tag | Field Name       | Req'd | Comments                                                                                                                                                                                                               |
| --- | ---------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     | Standard Header  | Y     | MsgType = 8                                                                                                                                                                                                            |
| 37  | OrderID          | Y     | OrderID is required to be unique for each chain of orders.                                                                                                                                                             |
| 198 | SecondaryOrderID | N     | Can be used to provide order id used by exchange or executing system.                                                                                                                                                  |
| 526 | SecondaryClOrdID | N     |                                                                                                                                                                                                                        |
| 527 | SecondaryExecID  | N     |                                                                                                                                                                                                                        |
| 11  | ClOrdID          | N     | Required for executions against electronically submitted orders which were assigned an ID by the institution or intermediary. Not required for orders manually entered by the broker or fund manager (for CIV orders). |
| 41  | OrigClOrdID      | N     | Conditionally required for response to an electronic Cancel or Cancel/Replace request (ExecType=PendingCancel, Replace, or Canceled). ClOrdID of the previous accepted order (NOT                                      |


---

# FIX 4.4 with Errata 20030618 - Volume 4

the initial order of the day) when canceling or replacing an order.

| 583                        | ClOrdLinkID          | N |                                                                                                                                                                    |   |   |
| -------------------------- | -------------------- | - | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | - | - |
| 693                        | QuoteRespID          | N | Required if responding to a QuoteResponse message. Echo back the Initiator’s value specified in the message.                                                       |   |   |
| 790                        | OrdStatusReqID       | N | Required if responding to and if provided on the Order Status Request message. Echo back the value provided by the requester.                                      |   |   |
| 584                        | MassStatusReqID      | N | Required if responding to an Order Mass Status Request. Echo back the value provided by the requester.                                                             |   |   |
| 911                        | TotNumReports        | N | Can be used when responding to an Order Mass Status Request to identify the total number of Execution Reports which will be returned.                              |   |   |
| 912                        | LastRptRequested     | N | Can be used when responding to an Order Mass Status Request to indicate that this is the last Execution Reports which will be returned as a result of the request. |   |   |
| component block \<Parties> |                      |   | N                                                                                                                                                                  |   |   |
| 229                        | TradeOriginationDate | N |                                                                                                                                                                    |   |   |
| 382                        | NoContraBrokers      | N | Number of ContraBrokers repeating group instances.                                                                                                                 |   |   |
| 375                        | ContraBroker         | N | First field in repeating group. Required if NoContraBrokers > 0.                                                                                                   |   |   |
| 337                        | ContraTrader         | N |                                                                                                                                                                    |   |   |
| 437                        | ContraTradeQty       | N |                                                                                                                                                                    |   |   |
| 438                        | ContraTradeTime      | N |                                                                                                                                                                    |   |   |
| 655                        | ContraLegRefID       | N |                                                                                                                                                                    |   |   |
| 66                         | ListID               | N | Required for executions against orders which were submitted as part of a list.                                                                                     |   |   |
| 548                        | CrossID              | N | CrossID for the replacement order                                                                                                                                  |   |   |
| 551                        | OrigCrossID          | N | Must match original cross order. Same order chaining mechanism as ClOrdID/OrigClOrdID with single order Cancel/Replace.                                            |   |   |
| 549                        | CrossType            | N |                                                                                                                                                                    |   |   |
| 17                         | ExecID               | Y | Unique identifier of execution message as assigned by sell-side (broker, exchange, ECN) (will be 0 (zero) for ExecType=I (Order Status)).                          |   |   |
| 19                         | ExecRefID            | N | Required for Trade Cancel and Trade Correct ExecType messages                                                                                                      |   |   |
| 150                        | ExecType             | Y | Describes the purpose of the execution report.                                                                                                                     |   |   |
| 39                         | OrdStatus            | Y | Describes the current state of a CHAIN of orders, same scope as OrderQty, CumQty, LeavesQty, and AvgPx                                                             |   |   |
| 636                        | WorkingIndicator     | N | For optional use with OrdStatus = 0 (New)                                                                                                                          |   |   |
| 103                        | OrdRejReason         | N | For optional use with ExecType = 8 (Rejected)                                                                                                                      |   |   |

~~April 30, 2003~~June 18, 2003


---


# 378 ExecRestatementReason

N Required for ExecType = D (Restated).

# 1 Account

N Required for executions against electronically submitted orders which were assigned an account by the institution or intermediary

# 660 AcctIDSource

N

# 581 AccountType

N Specifies type of account

# 589 DayBookingInst

N

# 590 BookingUnit

N

# 591 PreallocMethod

N

# 63 SettlType

N

# 64 SettlDate

N Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.

# 544 CashMargin

N

# 635 ClearingFeeIndicator

N

# component block &#x3C;Instrument>

Y Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# component block &#x3C;FinancingDetails>

N Insert here the set of "FinancingDetails" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# 711 NoUnderlyings

N Number of underlyings

# component block

N Must be provided if Number of underlyings > 0

# &#x3C;UnderlyingInstrument>

# 54 Side

Y

# component block &#x3C;Stipulations>

N Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# 854 QtyType

N

# component block &#x3C;OrderQtyData>

** Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

**Note: OrderQty field is required for Single Instrument Orders unless rejecting or acknowledging an order for a CashOrderQty or PercentOrder.

# 40 OrdType

N

# 423 PriceType

N

# 44 Price

N Required if specified on the order

# 99 StopPx

N Required if specified on the order

# component block &#x3C;PegInstructions>

N Insert here the set of "PegInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# component block

N Insert here the set of "DiscretionInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

~~April 30, 2003~~June 18, 2003

16 FIX 4.4 with Errata 20030618- Volume 4



---

# MESSAGES

| 839 | PeppedPrice               | N | The current price the order is pegged at                                                                                                                                                                                                   |
| --- | ------------------------- | - | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 845 | DiscretionPrice           | N | The current discretionary price of the order                                                                                                                                                                                               |
| 847 | TargetStrategy            | N | The target strategy of the order                                                                                                                                                                                                           |
| 848 | TargetStrategyParameters  | N | For further specification of the TargetStrategy                                                                                                                                                                                            |
| 849 | ParticipationRate         | N | Mandatory for a TargetStrategy=Participate order and specifies the target participation rate. For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)                       |
| 850 | TargetStrategyPerformance | N | For communication of the performance of the order versus the target strategy                                                                                                                                                               |
| 15  | Currency                  | N |                                                                                                                                                                                                                                            |
| 376 | ComplianceID              | N |                                                                                                                                                                                                                                            |
| 377 | SolicitedFlag             | N |                                                                                                                                                                                                                                            |
| 59  | TimeInForce               | N | Absence of this field indicates Day order                                                                                                                                                                                                  |
| 168 | EffectiveTime             | N | Time specified on the order at which the order should be considered valid                                                                                                                                                                  |
| 432 | ExpireDate                | N | Conditionally required if TimeInForce = GTD and ExpireTime is not specified.                                                                                                                                                               |
| 126 | ExpireTime                | N | Conditionally required if TimeInForce = GTD and ExpireDate is not specified.                                                                                                                                                               |
| 18  | ExecInst                  | N | Can contain multiple instructions, space delimited.                                                                                                                                                                                        |
| 528 | OrderCapacity             | N |                                                                                                                                                                                                                                            |
| 529 | OrderRestrictions         | N |                                                                                                                                                                                                                                            |
| 582 | CustOrderCapacity         | N |                                                                                                                                                                                                                                            |
| 32  | LastQty                   | N | Quantity (e.g. shares) bought/sold on this (last) fill. Required if ExecType = Trade or Trade Correct. If ExecType=Stopped, represents the quantity stopped/guaranteed/protected for.                                                      |
| 652 | UnderlyingLastQty         | N |                                                                                                                                                                                                                                            |
| 31  | LastPx                    | N | Price of this (last) fill. Required if ExecType = Trade or Trade Correct. Should represent the “all-in” (LastSpotRate + LastForwardPoints) rate for F/X orders. If ExecType=Stopped, represents the price stopped/guaranteed/protected at. |
| 651 | UnderlyingLastPx          | N |                                                                                                                                                                                                                                            |
| 669 | LastParPx                 | N | Last price expressed in percent-of-par. Conditionally required for Fixed Income trades when LastPx is expressed in Yield, Spread, Discount or any other price type that is not percent-of-par.                                             |
| 194 | LastSpotRate              | N | Applicable for F/X orders                                                                                                                                                                                                                  |

~~April 30, 2003~~June 18, 2003
17 FIX 4.4 with Errata 20030618- Volume 4


---
FIX 4.4 with Errata 20030618- Volume 4

| LastForwardPoints                                                                                                                                                                                                                                                                                                                                            | N | Applicable for F/X orders                                                                                                                                                                                                            |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | - | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| LastMkt                                                                                                                                                                                                                                                                                                                                                      | N | If ExecType = Trade (F), indicates the market where the trade was executed. If ExecType = New (0), indicates the market where the order was routed.                                                                                  |
| TradingSessionID                                                                                                                                                                                                                                                                                                                                             | N |                                                                                                                                                                                                                                      |
| TradingSessionSubID                                                                                                                                                                                                                                                                                                                                          | N |                                                                                                                                                                                                                                      |
| TimeBracket                                                                                                                                                                                                                                                                                                                                                  | N |                                                                                                                                                                                                                                      |
| LastCapacity                                                                                                                                                                                                                                                                                                                                                 | N |                                                                                                                                                                                                                                      |
| LeavesQty                                                                                                                                                                                                                                                                                                                                                    | Y | Quantity open for further execution. If the OrdStatus is Canceled, DoneForTheDay, Expired, Calculated, or Rejected (in which case the order is no longer active) then LeavesQty could be 0, otherwise LeavesQty = OrderQty - CumQty. |
| CumQty                                                                                                                                                                                                                                                                                                                                                       | Y | Currently executed quantity for chain of orders.                                                                                                                                                                                     |
| AvgPx                                                                                                                                                                                                                                                                                                                                                        | Y |                                                                                                                                                                                                                                      |
| DayOrderQty                                                                                                                                                                                                                                                                                                                                                  | N | For GT orders on days following the day of the first trade.                                                                                                                                                                          |
| DayCumQty                                                                                                                                                                                                                                                                                                                                                    | N | For GT orders on days following the day of the first trade.                                                                                                                                                                          |
| DayAvgPx                                                                                                                                                                                                                                                                                                                                                     | N | For GT orders on days following the day of the first trade.                                                                                                                                                                          |
| GTBookingInst                                                                                                                                                                                                                                                                                                                                                | N | States whether executions are booked out or accumulated on a partially filled GT order                                                                                                                                               |
| TradeDate                                                                                                                                                                                                                                                                                                                                                    | N | Used when reporting other than current day trades.                                                                                                                                                                                   |
| TransactTime                                                                                                                                                                                                                                                                                                                                                 | N | Time the transaction represented by this ExecutionReport occurred                                                                                                                                                                    |
| ReportToExch                                                                                                                                                                                                                                                                                                                                                 | N |                                                                                                                                                                                                                                      |
| component block \<CommissionData>                                                                                                                                                                                                                                                                                                                            |   | N                                                                                                                                                                                                                                    |
| Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" Note: On a fill/partial fill messages, it represents value for that fill/partial fill. On ExecType=Calculated, it represents cumulative value for the order. Monetary commission values are expressed in the currency reflected by the Currency field. |   |                                                                                                                                                                                                                                      |
| component block                                                                                                                                                                                                                                                                                                                                              |   | N                                                                                                                                                                                                                                    |
| Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                                                                                                                   |   |                                                                                                                                                                                                                                      |
| component block \<YieldData>                                                                                                                                                                                                                                                                                                                                 |   | N                                                                                                                                                                                                                                    |
| Insert here the set of "YieldData" (yield-related) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                                                                                                                                                             |   |                                                                                                                                                                                                                                      |
| GrossTradeAmt                                                                                                                                                                                                                                                                                                                                                | N |                                                                                                                                                                                                                                      |
| NumDaysInterest                                                                                                                                                                                                                                                                                                                                              | N |                                                                                                                                                                                                                                      |
| ExDate                                                                                                                                                                                                                                                                                                                                                       | N |                                                                                                                                                                                                                                      |
| AccruedInterestRate                                                                                                                                                                                                                                                                                                                                          | N |                                                                                                                                                                                                                                      |
| AccruedInterestAmt                                                                                                                                                                                                                                                                                                                                           | N |                                                                                                                                                                                                                                      |
| InterestAtMaturity                                                                                                                                                                                                                                                                                                                                           | N | For fixed income products which pay lump-sum interest at                                                                                                                                                                             |

~~April 30, 2003~~June 18, 2003


---

# FIX 4.4 with Errata 20030618 - Volume 4

| 920 | EndAccruedInterestAmt | N | For repurchase agreements the accrued interest on termination.                                                                                                                                                                |
| --- | --------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 921 | StartCash             | N | For repurchase agreements the start (dirty) cash consideration                                                                                                                                                                |
| 922 | EndCash               | N | For repurchase agreements the end (dirty) cash consideration                                                                                                                                                                  |
| 258 | TradedFlatSwitch      | N |                                                                                                                                                                                                                               |
| 259 | BasisFeatureDate      | N |                                                                                                                                                                                                                               |
| 260 | BasisFeaturePrice     | N |                                                                                                                                                                                                                               |
| 238 | Concession            | N |                                                                                                                                                                                                                               |
| 237 | TotalTakedown         | N |                                                                                                                                                                                                                               |
| 118 | NetMoney              | N | Note: On a fill/partial fill messages, it represents value for that fill/partial fill, on ExecType=Calculated, it represents cumulative value for the order. Value expressed in the currency reflected by the Currency field. |
| 119 | SettlCurrAmt          | N | Used to report results of forex accommodation trade                                                                                                                                                                           |
| 120 | SettlCurrency         | N | Used to report results of forex accommodation trade                                                                                                                                                                           |
| 155 | SettlCurrFxRate       | N | Foreign exchange rate used to compute SettlCurrAmt from Currency to SettlCurrency                                                                                                                                             |
| 156 | SettlCurrFxRateCalc   | N | Specifies whether the SettlCurrFxRate should be multiplied or divided                                                                                                                                                         |
| 21  | HandlInst             | N |                                                                                                                                                                                                                               |
| 110 | MinQty                | N |                                                                                                                                                                                                                               |
| 111 | MaxFloor              | N |                                                                                                                                                                                                                               |
| 77  | PositionEffect        | N | For use in derivatives omnibus accounting                                                                                                                                                                                     |
| 210 | MaxShow               | N |                                                                                                                                                                                                                               |
| 775 | BookingType           | N | Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking.     |
| 58  | Text                  | N |                                                                                                                                                                                                                               |
| 354 | EncodedTextLen        | N | Must be set if EncodedText field is specified and must immediately precede it.                                                                                                                                                |
| 355 | EncodedText           | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                                                                                |
| 193 | SettlDate2            | N | Can be used with OrdType = “Forex - Swap” to specify the “value date” for the future portion of a F/X swap.                                                                                                                   |
| 192 | OrderQty2             | N | Can be used with OrdType = “Forex - Swap” to specify the order quantity for the future portion of a F/X swap.                                                                                                                 |
| 641 | LastForwardPoints2    | N | Can be used with OrdType = “Forex - Swap” to specify the forward points (added to LastSpotRate) for the future portion of a F/X swap.                                                                                         |
| 442 | MultiLegReportingType | N | Default is a single security if not specified.                                                                                                                                                                                |

~~April 30, 2003~~June 18, 2003

---

# Cancellation Rights

N For CIV - Optional

# Money Laundering Status

N

# Regist ID

N Reference to Registration Instructions message for this Order.

# Designation

N Supplementary registration information for this Order

# Trans Bkd Time

N For CIV - Optional

# Exec Valuation Point

N For CIV - Optional

# Exec Price Type

N For CIV - Optional

# Exec Price Adjustment

N For CIV - Optional

# Priority Indicator

N

# Price Improvement

N

# Last Liquidity Indicator

N Applicable only on OrdStatus of Partial or Filled.

# No Cont Amts

N Number of contract details in this message (number of repeating groups to follow)

| Cont Amt Type  | N Must be first field in the repeating group. |
| -------------- | --------------------------------------------- |
| Cont Amt Value | N                                             |
| Cont Amt Curr  | N                                             |

# No Legs

N Number of legs Identifies a Multi-leg Execution if present and non-zero.

component block N Must be provided if Number of legs > 0

# Instrument Leg

| Leg Qty       | N                                                                                       |
| ------------- | --------------------------------------------------------------------------------------- |
| Leg Swap Type | N Instead of LegQty – requests that the sellside calculate LegQty based on opposite Leg |

component block N

# Leg Stipulations

| Leg Position Effect      | N Provide if the PositionEffect for the leg is different from that specified for the overall multileg security      |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| Leg Covered Or Uncovered | N Provide if the CoveredOrUncovered for the leg is different from that specified for the overall multileg security. |

component block N Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" Used for NestedPartyRole=Leg Clearing Firm/Account, Leg Account/Account Type

| Leg Ref ID     | N Used to identify a specific leg.                                                                                                        |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Leg Price      | N Provide only if a Price is required for a specific leg. Used for anchoring the overall multileg security price to a specific leg Price. |
| Leg Settl Type | N                                                                                                                                         |
| Leg Settl Date | N Takes precedence over LegSettlType value and conditionally required/omitted for specific LegSettlType values.                           |
| Leg Last Px    | N Used to report the execution price assigned to the leg of the multileg instrument                                                       |

# Copy Msg Indicator

N

~~April 30, 2003~~~~June 18, 2003~~
20 FIX 4.4 with Errata 20030618- Volume 4


---

# 136 NoMiscFees

N Required if any miscellaneous fees are reported. Indicates number of repeating entries. Repeating group.

** Nested Repeating Group follows **

# 137 MiscFeeAmt

N Required if NoMiscFees > 0

# 138 MiscFeeCurr

N

# 139 MiscFeeType

N Required if NoMiscFees > 0

# 891 MiscFeeBasis

N

Standard Trailer Y

# FIXML Definition for this message

– see http://www.fixprotocol.org for details

# ExecReportContent

"OrderID,SecondaryOrderID?,SecondaryClOrdID?,SecondaryExecID?,ClOrdID?,OrigClOrdID?,ClOrdLinkID?,PartiesList?,TradeOriginationDate?,ContraBrokerList?,ListID?,CrossID?,OrigCrossID?,CrossType?,ExecID,ExecRefID?,ExecType,OrdStatus,WorkingIndicator?,OrdRejReason?,ExecRestatementReason?,Account?,AccountType?,DayBookingInst?,BookingUnit?,PreallocMethod?,Settlement?,CashMargin?,ClearingFeeIndicator?,Side,StipulationsList?,QtyType?,OrderQtyData,OrdType?,PriceType?,Price?,StopPx?,PegDifference?,DiscretionInst?,DiscretionOffset?,Currency?,ComplianceID?,SolicitedFlag?,OrderDuration?,EffectiveTime?,ExecInstList?,OrderCapacity?,OrderRestrictions?,CustOrderCapacity?,LastQty?,LastPx?,UnderlyingLastPx?,LastSpotRate?,LastForwardPoints?,LastMkt?,TradingSessionID?,TradingSessionSubID?,LastCapacity?,LeavesQty,CumQty,AvgPx,DayOrderQty?,DayCumQty?,DayAvgPx?,GTBookingInst?,TradeDate?,TransactTime?,ReportToExch?,CommissionData?,SpreadOrBenchmarkCurveData?,YieldData?,GrossTradeAmt?,NumDaysInterest?,ExDate?,AccruedInterestRate?,AccruedInterestAmt?,TradedFlatSwitch?,BasisFeatureDate?,BasisFeaturePrice?,Concession?,TotalTakedown?,NetMoney?,SettlCurrAmt?,SettlCurrency?,SettlCurrFxRate?,SettlCurrFxRateCalc?,HandInst?,MinQty?,MaxFloor?,PositionEffect?,MaxShow?,Text?,EncodedTextGroup?,FutSettDate2?,OrderQty2?,LastForwardPoints2?,MultiLegReportingType?,CancellationRights?,MoneyLaunderingStatus?,RegistID?,Designation?,TransBkdTime?,ExecValuationPoint?,ExecPriceType?,ExecPriceAdjustment?,PriorityIndicator?,PriceImprovement?,ContractDetailsList?,LegList? %ExecReportCustom;"

# ExecutionReport

(%ExecReportContent;)

# ATTLIST ExecutionReport

FIXTag CDATA #FIXED '35'

DataType CDATA #FIXED 'String'

Value CDATA #FIXED '8' >

Refer to FIXML element ExctnRpt

April 30, 2003

June 18, 2003

21 FIX 4.4 with Errata 20030618- Volume 4


---

Use of the Execution Report for Multileg Instruments

The Execution Report has been expanded to include an optional repeating group of instruments legs. The instrument leg repeating group will not be used to track order status (LeavesQty, CumQty, etc.). The instrument leg repeating group can be used to report:

- Each leg of a multileg instrument – this provides a method for data enrichment for productized multileg instruments that can be identified on orders using only the Instrument block.
- The user supplied per leg information for Party block, PositionEffect, CoveredUncovered
- To report the price specified by the user on the order.
- Reporting of last sales price per leg, settlement type, and settlement date.

The multileg repeat group cannot be used to report the following:

- fill quantity per leg
- order status per leg

There are three different ways strategies can be traded on markets.

1. As a product identified by an Instrument block in which all legs of a multileg instrument are traded atomically in the ratio quantities specified for leg where contraparties to the trade are also apportioned per the ratio quantities defined per leg. (Note this method applies to strategies that are or will be productized in the securities definition table)
2. As a product identified by an Instrument block in which all legs of a multileg instrument are traded, but they are traded against individual legs - likely resulting in contraparty trading quantities not corresponding to the ratio quantities. (Note this method applies to strategies that are or will be productized in the securities definition table)
3. As individual legs (legging in). (Note this method applies to strategies that are not and will not be productized in the securities definition table)

Multileg Instruments that are traded atomically and contraparties to the trade being assigned by ratio quantity can be reported by strategy by setting the MultilegReportType (442) field to 3. The OrdQty, LeavesQty, CumQty, AvgPx apply to the overall strategy. Quantities of each individual leg are calculated by multiplying the quantity field for the strategy quantity * the LegRatioQty.

Multileg Instruments that are not traded atomically (because they execute against orders and quotes for individual leg securities or they are traded on an open outcry environment by leg) can:

- Report fills by overall strategy and legs in a single Execution report, where instrument identification is in the Instrument Block and the leg instrument identification is in the Instrument Leg Block. The MultilegReportType field is 3. The OrdQty, LeavesQty, CumQty, AvgPx always apply to the strategy. Reporting must be done within the context of the strategy (ie: fills and partial fills are reported within the ratio quantities defined by the legs) even though contraparties have traded against individual legs and perhaps not within the ratio quantities defined by the legs. The LegRefID and ContraLegRefID are used to associate specific contra trade quantities against a leg with a specific contra party; or
- Counterparties can choose to send a summary Execution Report for the overall multileg instrument (MultilegReportType of 3) once the multileg order has been filled or partially filled, and then separately report details of each leg in separate Execution Reports. (MultilegReportType of 2). The OrdQty, LeavesQty, CumQty, AvgPx always apply to the strategy. Reporting must be done within the context of the strategy (ie: fills and partial fills are reported within the ratio quantities defined by the legs) even though contraparties have traded against individual legs and perhaps not within the ratio quantities defined by the legs.

~~April 30, 2003~~ June 18, 2003 22 FIX 4.4 with Errata 20030618- Volume 4



---

# Execution Report Summary


The summary Execution Report is within the context of the strategy. Instrument identification is in the Instrument Block. This summary report does not contain leg information nor contraparty information. For ExecTypes = Pending New and New only the summary execution report should be sent.

The separate Execution Report for each leg is within the context of a single leg of the strategy. Leg instrument identification is in the Instrument Leg Block. These reports contain the contraparty information for each leg. The ExecType of each separate leg report should be the same as the ExecType stated in the summary Execution Report; or Counterparties can choose to report fills by leg (without a summary Execution Report for the overall strategy).

The MultilegReportType field is 2. Reporting should be done within the context of the strategy (ie: fills and partial fills are reported within the ratio quantities defined by the legs) even though counterparties have traded against individual legs and perhaps not within the ratio quantities defined by the legs.

The Execution Report for each leg is within the context of a single leg of the strategy. Leg instrument identification is in the Instrument Leg Block. These reports contain the contraparty information for each leg. The OrdQty, LeavesQty, CumQty, AvgPx always apply to the strategy. Because a summary Execution Report is not being sent, ExecType = Pending New and New will also have to be reported by leg.

If reporting of leg fills is not done within the context of the strategy, leg instrument identification and details should be promoted to the Instrument Block. Also, the OrdQty, LeavesQty, CumQty, AvgPx then apply to the individual leg. The MultilegReportType remains 2. Always refer to the customs and practices of specific marketplaces to determine whether a specific marketplace permits reporting fills that are not within the context of the strategy and under what conditions such reporting may be allowed.

~~April 30, 2003~~ June 18, 2003 23 FIX 4.4 with Errata 20030618- Volume 4
---

# Don’t Know Trade (DK)

The Don’t Know Trade (DK) message notifies a trading partner that an electronically received execution has been rejected. This message can be thought of as an execution reject message. This message has special utility when dealing with one-way execution reporting. If the initial Order Acknowledgment message (LastQty=0 and OrdStatus=New) does not match an existing order this message can be used to notify the broker of a potential problem order. Note that the decision to DK an execution lies with the institution. Some of the mismatches listed in the DKReason field may be acceptable and will not require a DK messages to be generated.

The Don’t Know Trade (DK) format is as follows:

| Tag                                                                                                                                           | Field Name       | Req'd | Comments                                                                                                                       |
| --------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
|                                                                                                                                               | Standard Header  | Y     | MsgType = Q                                                                                                                    |
| 37                                                                                                                                            | OrderID          | Y     | Broker Order ID as identified on problem execution                                                                             |
| 198                                                                                                                                           | SecondaryOrderID | N     |                                                                                                                                |
| 17                                                                                                                                            | ExecID           | Y     | Execution ID of problem execution                                                                                              |
| 127                                                                                                                                           | DKReason         | Y     |                                                                                                                                |
| component block \<Instrument> Y Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |                  |       |                                                                                                                                |
| 711                                                                                                                                           | NoUnderlyings    | N     | Number of underlyings                                                                                                          |
|                                                                                                                                               | component block  | N     | Must be provided if Number of underlyings > 0 \<UnderlyingInstrument>                                                          |
| 555                                                                                                                                           | NoLegs           | N     | Number of Legs                                                                                                                 |
|                                                                                                                                               | component block  | N     | Must be provided if NoLegs > 0 \<InstrumentLeg>                                                                                |
| 54                                                                                                                                            | Side             | Y     |                                                                                                                                |
| component block \<OrderQtyData> Y Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"         |                  |       |                                                                                                                                |
| 32                                                                                                                                            | LastQty          | N     | Required if specified on the ExecutionRpt                                                                                      |
| 31                                                                                                                                            | LastPx           | N     | Required if specified on the ExecutionRpt                                                                                      |
| 58                                                                                                                                            | Text             | N     |                                                                                                                                |
| 354                                                                                                                                           | EncodedTextLen   | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355                                                                                                                                           | EncodedText      | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
|                                                                                                                                               | Standard Trailer | Y     |                                                                                                                                |

FIXML Definition for this message – see http://www.fixprotocol.org for details

~~April 30, 2003~~June 18, 2003 24 FIX 4.4 with Errata 20030618- Volume 4


---

# DK_Trade

# DK_TradeContent

OrderID, ExecID, DK_Reason, Instrument, Side, OrderQtyData, LastQty?, LastPx?, Text?, EncodedTextGroup?

# DK_Trade

Refer to FIXML element DKTrd

Date: April 30, 2003

Revision Date: June 18, 2003

Version: FIX 4.4 with Errata 20030618 - Volume 4


---
Order Cancel/Replace Request (a.k.a. Order Modification Request)
The order cancel/replace request is used to change the parameters of an existing order. Do not use this message to cancel the remaining quantity of an outstanding order, use the Order Cancel Request message for this purpose. Cancel/Replace will be used to change any valid attribute of an open order (i.e. reduce/increase quantity, change limit price, change instructions, etc.), Subject to agreement between counterparties, it can be used to re-open a filled order by increasing OrderQty.

An immediate response to this message is required. It is recommended that an ExecutionRpt with ExecType=Pending Replace be sent unless the Order Cancel/Replace Request can be immediately accepted (ExecutionRpt with ExecType=Replace) or rejected (Order Cancel Reject message).

The Cancel/Replace request will only be accepted if the order can successfully be pulled back from the exchange floor without executing. Requests which cannot be processed will be rejected using the Cancel Reject message. The Cancel Reject message should provide the ClOrdID and OrigClOrdID values which were specified on the Cancel/Replace Request message for identification. Note that while it is necessary for the ClOrdID to change and be unique, the broker’s OrderID field does not necessarily have to change as a result of the Cancel/Replace request.

The protocol supports the chaining of multiple cancel/replace requests, though trading counterparties may not support this functionality. Care should be taken if the order sender wishes to send a cancel/replace request when there is one or more cancel/replaces which have not been accepted or rejected – in general:

- The order sender should chain client order ids on an ‘optimistic’ basis, i.e. set the OrigClOrdID to the last non rejected ClOrdID sent
- The order receiver should chain client order ids on a ‘pessimistic’ basis, i.e. set the OrigClOrdID on execution reports that convey the receipt or successful application of a cancel/replace and Order Cancel Reject messages to be the last ‘accepted’ ClOrdID (See "Order State Change Matrices" for examples of this)

In the event that the order sender wants to chain order cancel/replaces rapidly then they should ensure that each replace request contains the full details of the order as they would now like it to be. For example if an attempt is made to change the limit price and then an immediate request to change the quantity is issued then if the desired behaviour is that both the limit price and quantity should be changed then the second request should include the revised limit price (in case the first replace request is rejected).

All of the application-level fields in the original order should be retransmitted with the original values in the Order Cancel/Replace Request, except the fields that are being changed. Any field may be changed with this message except those in the <instrument> component block and limited changes to the Side field (noted below), however, buy-side firms should note that sell-side firms may further restrict which fields they allow to change; hence bilateral agreement is required. For example, some sell-side firms may not allow fields such as Side, SettlDate, etc. to change. Sell-side firms should validate the Order Cancel/Replace Request to ensure that the client is not requesting a change for a field that the sell-side cannot change; in this case the sell-side should send a Cancel Reject message with CxlRejReason = 2 (Broker/Exchange Option).</instrument>

When modifying ExecInst values in a replacement order, it is necessary to re-declare all ExecInst in the replacement order. ExecInst values will not be carried forward from the original order to the replacement unless re-declared.

The format of the Order Cancel/Replace Request message is:

~~April 30, 2003~~June 18, 2003 26 FIX 4.4 with Errata 20030618- Volume 4


---

Order Cancel/Replace Request (a.k.a. Order Modification Request)

| Tag             | Field Name           | Req'd | Comments                                                                                                                                                                                                                                                          |
| --------------- | -------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|                 | Standard Header      | Y     | MsgType = G                                                                                                                                                                                                                                                       |
| 37              | OrderID              | N     | Unique identifier of most recent order as assigned by sell-side (broker, exchange, ECN).                                                                                                                                                                          |
| component block | \<Parties>           | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                                                              |
| 229             | TradeOriginationDate | N     |                                                                                                                                                                                                                                                                   |
| 75              | TradeDate            | N     |                                                                                                                                                                                                                                                                   |
| 41              | OrigClOrdID          | Y     | ClOrdID of the previous non rejected order (NOT the initial order of the day) when canceling or replacing an order.                                                                                                                                               |
| 11              | ClOrdID              | Y     | Unique identifier of replacement order as assigned by institution or by the intermediary with closest association with the investor. Note that this identifier will be used in ClOrdID field of the Cancel Reject message if the replacement request is rejected. |
| 526             | SecondaryClOrdID     | N     |                                                                                                                                                                                                                                                                   |
| 583             | ClOrdLinkID          | N     |                                                                                                                                                                                                                                                                   |
| 66              | ListID               | N     | Required for List Orders                                                                                                                                                                                                                                          |
| 586             | OrigOrdModTime       | N     | TransactTime of the last state change that occurred to the original order                                                                                                                                                                                         |
| 1               | Account              | N     |                                                                                                                                                                                                                                                                   |
| 660             | AcctIDSource         | N     |                                                                                                                                                                                                                                                                   |
| 581             | AccountType          | N     |                                                                                                                                                                                                                                                                   |
| 589             | DayBookingInst       | N     |                                                                                                                                                                                                                                                                   |
| 590             | BookingUnit          | N     |                                                                                                                                                                                                                                                                   |
| 591             | PreallocMethod       | N     |                                                                                                                                                                                                                                                                   |
| 70              | AllocID              | N     | Used to assign an overall allocation id to the block of preallocations                                                                                                                                                                                            |
| 78              | NoAllocs             | N     | Number of repeating groups for pre-trade allocation                                                                                                                                                                                                               |
| 79              | AllocAccount         | N     | Required if NoAllocs > 0. Must be first field in repeating group.                                                                                                                                                                                                 |
| 661             | AllocAcctIDSource    | N     |                                                                                                                                                                                                                                                                   |
| 736             | AllocSettlCurrency   | N     |                                                                                                                                                                                                                                                                   |
| 467             | IndividualAllocID    | N     |                                                                                                                                                                                                                                                                   |
| component block | \<NestedParties>     | N     | Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" Used for NestedPartyRole=Clearing Firm                                                     |

~~April 30, 2003~~June 18, 2003

27 FIX 4.4 with Errata 20030618- Volume 4



---

# FIX 4.4 with Errata 20030618 - Volume 4

~~April 30, 2003~~ June 18, 2003


# Order Fields

| 80                                                                                                                                  | AllocQty             | N |                                                                                                                                                                                                                               |
| ----------------------------------------------------------------------------------------------------------------------------------- | -------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 63                                                                                                                                  | SettlType            | N |                                                                                                                                                                                                                               |
| 64                                                                                                                                  | SettlDate            | N | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                                                                                       |
| 544                                                                                                                                 | CashMargin           | N |                                                                                                                                                                                                                               |
| 635                                                                                                                                 | ClearingFeeIndicator | N |                                                                                                                                                                                                                               |
| 21                                                                                                                                  | HandlInst            | N |                                                                                                                                                                                                                               |
| 18                                                                                                                                  | ExecInst             | N | Can contain multiple instructions, space delimited. Replacement order must be created with new parameters (i.e. original order values will not be brought forward to replacement order unless redefined within this message). |
| 110                                                                                                                                 | MinQty               | N |                                                                                                                                                                                                                               |
| 111                                                                                                                                 | MaxFloor             | N |                                                                                                                                                                                                                               |
| 100                                                                                                                                 | ExDestination        | N |                                                                                                                                                                                                                               |
| 386                                                                                                                                 | NoTradingSessions    | N | Specifies the number of repeating TradingSessionIDs                                                                                                                                                                           |
| 336                                                                                                                                 | TradingSessionID     | N | Required if NoTradingSessions is > 0.                                                                                                                                                                                         |
| 625                                                                                                                                 | TradingSessionSubID  | N |                                                                                                                                                                                                                               |
| component block \<Instrument>                                                                                                       |                      |   |                                                                                                                                                                                                                               |
|                                                                                                                                     |                      | Y |                                                                                                                                                                                                                               |
| Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                       |                      |   |                                                                                                                                                                                                                               |
|                                                                                                                                     |                      |   | Must match original order                                                                                                                                                                                                     |
| component block \<FinancingDetails>                                                                                                 |                      |   |                                                                                                                                                                                                                               |
|                                                                                                                                     |                      | N |                                                                                                                                                                                                                               |
| Insert here the set of "FinancingDetails" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                 |                      |   |                                                                                                                                                                                                                               |
|                                                                                                                                     |                      |   | Must match original order                                                                                                                                                                                                     |
| 711                                                                                                                                 | NoUnderlyings        | N | Number of underlyings                                                                                                                                                                                                         |
| component block                                                                                                                     |                      |   |                                                                                                                                                                                                                               |
|                                                                                                                                     |                      | N |                                                                                                                                                                                                                               |
| \<UnderlyingInstrument>                                                                                                             |                      |   |                                                                                                                                                                                                                               |
| 54                                                                                                                                  | Side                 | Y | Should match original order's side, however, if bilaterally agreed to the following groups could potentially be interchanged:                                                                                                 |
|                                                                                                                                     |                      |   | Buy and Buy Minus                                                                                                                                                                                                             |
|                                                                                                                                     |                      |   | Sell, Sell Plus, Sell Short, and Sell Short Exempt                                                                                                                                                                            |
|                                                                                                                                     |                      |   | Cross, Cross Short, and Cross Short Exempt                                                                                                                                                                                    |
| 60                                                                                                                                  | TransactTime         | Y | Time this order request was initiated/released by the trader or trading system.                                                                                                                                               |
| 854                                                                                                                                 | QtyType              | N |                                                                                                                                                                                                                               |
| component block \<OrderQtyData>                                                                                                     |                      |   |                                                                                                                                                                                                                               |
|                                                                                                                                     |                      | Y |                                                                                                                                                                                                                               |
| Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                 |                      |   |                                                                                                                                                                                                                               |
| Note: OrderQty value should be the “Total Intended Order Quantity” (including the amount already executed for this chain of orders) |                      |   |                                                                                                                                                                                                                               |



---
FIX 4.4 with Errata 20030618- Volume 4

# Order Fields

| OrdType                       |                          | Y                                                                                                                |                                                                                                                                                                                                                      |
| ----------------------------- | ------------------------ | ---------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| PriceType                     |                          | N                                                                                                                |                                                                                                                                                                                                                      |
| Price                         |                          | N                                                                                                                | Required for limit OrdTypes. For F/X orders, should be the “all-in” rate (spot rate adjusted for forward points). Can be used to specify a limit price for a pegged order, previously indicated, etc.                |
| StopPx                        |                          | N                                                                                                                | Required for OrdType = “Stop” or OrdType = “Stop limit”.                                                                                                                                                             |
| component                     |                          | block                                                                                                            | N                                                                                                                                                                                                                    |
| \<SpreadOrBenchmarkCurveData> |                          | (Fixed Income spread or benchmark curve) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"           |                                                                                                                                                                                                                      |
| component                     |                          | block                                                                                                            | N                                                                                                                                                                                                                    |
| \<YieldData>                  |                          | Insert here the set of "YieldData" (yield-related) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |                                                                                                                                                                                                                      |
| component                     |                          | block                                                                                                            | N                                                                                                                                                                                                                    |
| \<PegInstructions>            |                          | Insert here the set of "PegInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"            |                                                                                                                                                                                                                      |
| component                     |                          | block                                                                                                            | N                                                                                                                                                                                                                    |
| \<DiscretionInstructions>     |                          | Insert here the set of "DiscretionInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"     |                                                                                                                                                                                                                      |
|                               | TargetStrategy           | N                                                                                                                | The target strategy of the order                                                                                                                                                                                     |
|                               | TargetStrategyParameters | N                                                                                                                | For further specification of the TargetStrategy                                                                                                                                                                      |
|                               | ParticipationRate        | N                                                                                                                | Mandatory for a TargetStrategy=Participate order and specifies the target participation rate. For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume) |
|                               | ComplianceID             | N                                                                                                                |                                                                                                                                                                                                                      |
|                               | SolicitedFlag            | N                                                                                                                |                                                                                                                                                                                                                      |
| Currency                      |                          | N                                                                                                                | Must match original order.                                                                                                                                                                                           |
|                               | TimeInForce              | N                                                                                                                | Absence of this field indicates Day order                                                                                                                                                                            |
|                               | EffectiveTime            | N                                                                                                                | Can specify the time at which the order should be considered valid                                                                                                                                                   |
|                               | ExpireDate               | N                                                                                                                | Conditionally required if TimeInForce = GTD and ExpireTime is not specified.                                                                                                                                         |
|                               | ExpireTime               | N                                                                                                                | Conditionally required if TimeInForce = GTD and ExpireDate is not specified.                                                                                                                                         |
|                               | GTBookingInst            | N                                                                                                                | States whether executions are booked out or accumulated on a partially filled GT order                                                                                                                               |
| component                     |                          | block                                                                                                            | N                                                                                                                                                                                                                    |
| \<CommissionData>             |                          | Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"            |                                                                                                                                                                                                                      |
|                               | OrderCapacity            | N                                                                                                                |                                                                                                                                                                                                                      |
|                               | OrderRestrictions        | N                                                                                                                |                                                                                                                                                                                                                      |
|                               | CustOrderCapacity        | N                                                                                                                |                                                                                                                                                                                                                      |

April 30, 2003
---

FIX 4.4 with Errata 20030618 - Volume 4

# Order Modification Request

| 121 | ForexReq              | N | Indicates that broker is requested to execute a Forex accommodation trade in conjunction with the security trade.                                                                                                         |
| --- | --------------------- | - | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 120 | SettlCurrency         | N | Required if ForexReq = Y.                                                                                                                                                                                                 |
| 775 | BookingType           | N | Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking. |
| 58  | Text                  | N |                                                                                                                                                                                                                           |
| 354 | EncodedTextLen        | N | Must be set if EncodedText field is specified and must immediately precede it.                                                                                                                                            |
| 355 | EncodedText           | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                                                                            |
| 193 | SettlDate2            | N | Can be used with OrdType = “Forex - Swap” to specify the “value date” for the future portion of a F/X swap.                                                                                                               |
| 192 | OrderQty2             | N | Can be used with OrdType = “Forex - Swap” to specify the order quantity for the future portion of a F/X swap.                                                                                                             |
| 640 | Price2                | N | Can be used with OrdType = “Forex - Swap” to specify the price for the future portion of a F/X swap.                                                                                                                      |
| 77  | PositionEffect        | N | For use in derivatives omnibus accounting                                                                                                                                                                                 |
| 203 | CoveredOrUncovered    | N | For use with derivatives, such as options                                                                                                                                                                                 |
| 210 | MaxShow               | N |                                                                                                                                                                                                                           |
| 114 | LocateReqd            | N | Required for short sell orders                                                                                                                                                                                            |
| 480 | CancellationRights    | N | For CIV - Optional                                                                                                                                                                                                        |
| 481 | MoneyLaunderingStatus | N |                                                                                                                                                                                                                           |
| 513 | RegistID              | N | Reference to Registration Instructions message for this Order.                                                                                                                                                            |
| 494 | Designation           | N | Supplementary registration information for this Order                                                                                                                                                                     |
|     | Standard Trailer      | Y |                                                                                                                                                                                                                           |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;!ENTITY % OrderModificationRequestCustom "">

&#x3C;!ENTITY % OrderModificationRequestContent

"OrderID?,PartiesList?,TradeOriginationDate?,OrigClOrdID,ClOrdID, SecondaryClOrdID?, ClOrdLinkID?,

ListID?, OrigOrdModTime?, Account?,AccountType?,DayBookingInst?,BookingUnit?, PreallocMethod?,

OrdAllocGroupList?,Settlement?,CashMargin?, ClearingFeeIndicator?,

HandInst,ExecInstList?,MinQty?,MaxFloor?,ExDestination?,TrdSessionList?, Instrument,

Side,TransactTime,

QtyType?,

OrderQtyData,OrdType,PriceType?,Price?,StopPx?,SpreadOrBenchmarkCurveData?,

YieldData?,PegDifference?,DiscretionInst?,DiscretionOffset?,ComplianceID?,

SolicitedFlag?,Currency?,OrderDuration?, EffectiveTime?,

GTBookingInst?,CommissionData?,OrderCapacity?,OrderRestrictions?,

CustOrderCapacity?,

ForexReqOrder?,Text?,EncodedTextGroup?,FutSettDate2?,OrderQty2?, Price2?,

PositionEffect?,CoveredOrUncovered?,MaxShow?,LocateReqd?,

April 30, 2003

June 18, 2003



---

# Cancellation Rights?

# Money Laundering Status?

# Regist ID?

# Designation?

# Accrued Interest Rate?

# Accrued Interest Amount?

# Net Money?

# Order Modification Request Custom

Refer to FIXML element OrdCxlRplcReq

April 30, 2003

June 18, 2003

31 FIX 4.4 with Errata 20030618- Volume 4


---
Order Cancel Request
The order cancel request message requests the cancelation of all of the remaining quantity of an existing order. Note that the Order Cancel/Replace Request should be used to partially cancel (reduce) an order). The request will only be accepted if the order can successfully be pulled back from the exchange floor without executing. A cancel request is assigned a ClOrdID and is treated as a separate entity. If rejected, the ClOrdID of the cancel request will be sent in the Cancel Reject message, as well as the ClOrdID of the actual order in the OrigClOrdID field. The ClOrdID assigned to the cancel request must be unique amongst the ClOrdID assigned to regular orders and replacement orders. An immediate response to this message is required. It is recommended that an ExecutionRpt with ExecType=Pending Cancel be sent unless the Order Cancel Request can be immediately accepted (ExecutionRpt with ExecType=Canceled) or rejected (Order Cancel Reject message).

The format of the cancel request message is:

| Tag                                                                                                                                                                                 | Field Name       | Req'd | Comments                                                                                                            |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ----- | ------------------------------------------------------------------------------------------------------------------- |
|                                                                                                                                                                                     | Standard Header  | Y     | MsgType = F                                                                                                         |
| 41                                                                                                                                                                                  | OrigClOrdID      | Y     | ClOrdID of the previous non-rejected order (NOT the initial order of the day) when canceling or replacing an order. |
| 37                                                                                                                                                                                  | OrderID          | N     | Unique identifier of most recent order as assigned by sell-side (broker, exchange, ECN).                            |
| 11                                                                                                                                                                                  | ClOrdID          | Y     | Unique ID of cancel request as assigned by the institution.                                                         |
| 526                                                                                                                                                                                 | SecondaryClOrdID | N     |                                                                                                                     |
| 583                                                                                                                                                                                 | ClOrdLinkID      | N     |                                                                                                                     |
| 66                                                                                                                                                                                  | ListID           | N     | Required for List Orders                                                                                            |
| 586                                                                                                                                                                                 | OrigOrdModTime   | N     |                                                                                                                     |
| 1                                                                                                                                                                                   | Account          | N     |                                                                                                                     |
| 660                                                                                                                                                                                 | AcctIDSource     | N     |                                                                                                                     |
| 581                                                                                                                                                                                 | AccountType      | N     |                                                                                                                     |
| component block \<Parties> N Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                   |                  |       |                                                                                                                     |
| component block \<Instrument> Y Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                       |                  |       |                                                                                                                     |
| component block \<FinancingDetails> N Insert here the set of "FinancingDetails" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" Must match original order |                  |       |                                                                                                                     |
| 711                                                                                                                                                                                 | NoUnderlyings    | N     | Number of underlyings                                                                                               |
| component block N Must be provided if Number of underlyings > 0 \<UnderlyingInstrument>                                                                                             |                  |       |                                                                                                                     |
| 54                                                                                                                                                                                  | Side             | Y     |                                                                                                                     |

~~April 30, 2003~~ June 18, 2003 32 FIX 4.4 with Errata 20030618- Volume 4


---

FIX 4.4 with Errata 20030618 - Volume 4

# OrderCancelRequest

| 60                              | TransactTime     | Y                                                          | Time this order request was initiated/released by the trader or trading system.                                                |   |
| ------------------------------- | ---------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | - |
| component block \<OrderQtyData> |                  | Y                                                          | Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                            |   |
|                                 |                  | Note: OrderQty = CumQty + LeavesQty (see exceptions above) |                                                                                                                                |   |
| 376                             | ComplianceID     | N                                                          |                                                                                                                                |   |
| 58                              | Text             | N                                                          |                                                                                                                                |   |
| 354                             | EncodedTextLen   | N                                                          | Must be set if EncodedText field is specified and must immediately precede it.                                                 |   |
| 355                             | EncodedText      | N                                                          | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |   |
|                                 | Standard Trailer | Y                                                          |                                                                                                                                |   |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;!ENTITY % OrderCancelRequestCustom "">

&#x3C;!ENTITY % OrderCancelRequestContent "OrigClOrdID,OrderID?,ClOrdID,SecondaryClOrdID?, ClOrdLinkID?, ListID?, OrigOrdModTime?, Account?,AccountType?, PartiesList?,Instrument,Side,TransactTime,OrderQtyData,ComplianceID?,Text?,EncodedTextGroup?">

&#x3C;!ELEMENT OrderCancelRequest (%OrderCancelRequestContent;)>

&#x3C;!ATTLIST OrderCancelRequest FIXTag CDATA #FIXED '35'>

&#x3C;!ATTLIST OrderCancelRequest DataType CDATA #FIXED 'String'>

&#x3C;!ATTLIST OrderCancelRequest Value CDATA #FIXED 'F'>Refer to FIXML element OrdCxlReq

April 30, 2003

June 18, 2003


33

---

# Order Cancel Reject

The order cancel reject message is issued by the broker upon receipt of a cancel request or cancel/replace request message which cannot be honored. Requests to change price or decrease quantity are executed only when an outstanding quantity exists. Filled orders cannot be changed (i.e quantity reduced or price change). However, the broker/sellside may support increasing the order quantity on a currently filled order.

When rejecting a Cancel/Replace Request (or Cancel Request), the Cancel Reject message should provide the ClOrdID which was specified on the Cancel/Replace Request (or Cancel Request) message for identification, and the OrigClOrdId should be that of the last accepted order (except in the case of CxlRejReason = “Unknown Order”).

When rejecting an Order Mass Cancel Request, the ClOrdID should be set to the ClOrdID value of the Order Mass Cancel Request. OrigClOrdID is not specified for a rejected Order Mass Cancel Requests. The execution message responds to accepted cancel request and cancel/replace request messages. The order cancel reject message format is as follows:

| Tag | Field Name           | Req'd | Comments                                                                                                                                                      |
| --- | -------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     | Standard Header      | Y     | MsgType = 9                                                                                                                                                   |
| 37  | OrderID              | Y     | If CxlRejReason=”Unknown order”, specify “NONE”.                                                                                                              |
| 198 | SecondaryOrderID     | N     | Can be used to provide order id used by exchange or executing system.                                                                                         |
| 526 | SecondaryClOrdID     | N     |                                                                                                                                                               |
| 11  | ClOrdID              | Y     | Unique order id assigned by institution or by the intermediary with closest association with the investor. to the cancel request or to the replacement order. |
| 583 | ClOrdLinkID          | N     |                                                                                                                                                               |
| 41  | OrigClOrdID          | Y     | ClOrdID which could not be canceled/replaced. ClOrdID of the previous accepted order (NOT the initial order of the day) when canceling or replacing an order. |
| 39  | OrdStatus            | Y     | OrdStatus value after this cancel reject is applied. If CxlRejReason = “Unknown Order”, specify Rejected.                                                     |
| 636 | WorkingIndicator     | N     | For optional use with OrdStatus = 0 (New)                                                                                                                     |
| 586 | OrigOrdModTime       | N     |                                                                                                                                                               |
| 66  | ListID               | N     | Required for rejects against orders which were submitted as part of a list.                                                                                   |
| 1   | Account              | N     |                                                                                                                                                               |
| 660 | AcctIDSource         | N     |                                                                                                                                                               |
| 581 | AccountType          | N     |                                                                                                                                                               |
| 229 | TradeOriginationDate | N     |                                                                                                                                                               |
| 75  | TradeDate            | N     |                                                                                                                                                               |
| 60  | TransactTime         | N     |                                                                                                                                                               |

~~April 30, 2003~~ June 18, 2003 34 FIX 4.4 with Errata 20030618- Volume 4


---

FIXML Definition for this message – see http://www.fixprotocol.org for details


# OrderCancelReject

| Field                                                                                                                          | Required |
| ------------------------------------------------------------------------------------------------------------------------------ | -------- |
| CxlRejResponseTo                                                                                                               | Y        |
| CxlRejReason                                                                                                                   | N        |
| Text                                                                                                                           | N        |
| EncodedTextLen                                                                                                                 | N        |
| Must be set if EncodedText field is specified and must immediately precede it.                                                 |          |
| EncodedText                                                                                                                    | N        |
| Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |          |
| Standard Trailer                                                                                                               | Y        |

# FIXML Definition

&#x3C;!ENTITY % OrderCancelRejectCustom "">
&#x3C;!ENTITY % OrderCancelRejectContent "OrderID,SecondaryOrderID?,SecondaryClOrdID?,ClOrdID,
ClOrdLinkID?, OrigClOrdID, OrdStatus, WorkingIndicator?, OrigOrdModTime?,
ListID?,Account?,AccountType?, TradeOriginationDate?,
TransactTime?,CxlRejResponseTo,CxlRejReason?,Text?,
EncodedTextGroup? %OrderCancelRejectCustom;" >
&#x3C;!ELEMENT OrderCancelReject (%OrderCancelRejectContent;)>
&#x3C;!ATTLIST OrderCancelReject FIXTag
CDATA #FIXED '35'
DataType CDATA #FIXED 'String'
Value CDATA #FIXED '9' >Refer to FIXML element OrdCxlRej

April 30, 2003   June 18, 2003             35        FIX 4.4 with Errata 20030618- Volume 4



---
Order Status Request

The order status request message is used by the institution to generate an order status message back from the broker.

(See "Order State Change Matrices" for examples of usage of this message, including how to respond to a status request for an unknown order.)

# The format of the order status request message is:

| Tag                                 | Field Name              | Req'd | Comments                                                                                                                                      |
| ----------------------------------- | ----------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------- |
|                                     | Standard Header         | Y     | MsgType = H                                                                                                                                   |
| 37                                  | OrderID                 | N     |                                                                                                                                               |
| 11                                  | ClOrdID                 | Y     | The ClOrdID of the order whose status is being requested.                                                                                     |
| 526                                 | SecondaryClOrdID        | N     |                                                                                                                                               |
| 583                                 | ClOrdLinkID             | N     |                                                                                                                                               |
| component block \<Parties>          |                         | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                          |
| 790                                 | OrdStatusReqID          | N     | Optional, can be used to uniquely identify a specific Order Status Request message. Echoed back on Execution Report if provided.              |
| 1                                   | Account                 | N     |                                                                                                                                               |
| 660                                 | AcctIDSource            | N     |                                                                                                                                               |
| component block \<Instrument>       |                         | Y     | Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                 |
| component block \<FinancingDetails> |                         | N     | Insert here the set of "FinancingDetails" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" Must match original order |
| 711                                 | NoUnderlyings           | N     | Number of underlyings                                                                                                                         |
| component block                     | \<UnderlyingInstrument> | N     | Must be provided if Number of underlyings > 0                                                                                                 |
| 54                                  | Side                    | Y     |                                                                                                                                               |
|                                     | Standard Trailer        | Y     |                                                                                                                                               |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;s>&#x3C;!ENTITY % OrderStatusRequestCustom "">&#x3C;/s>

&#x3C;s>&#x3C;!ENTITY % OrderStatusRequestContent "OrderID?,ClOrdID, SecondaryClOrdID?,&#x3C;/s>

&#x3C;s>ClOrdLinkID?, PartiesList?,Account?,Instrument,Side %OrderStatusRequestCustom;" >&#x3C;/s>

April 30, 2003

June 18, 2003

36 FIX 4.4 with Errata 20030618- Volume 4


---

# Order Status Request



DataType CDATA #FIXED 'String'

Value CDATA #FIXED 'H' >

Refer to FIXML element OrdStatReq

April 30, 2003

June 18, 2003 37 FIX 4.4 with Errata 20030618- Volume 4



---
Order Mass Cancel Request
The order mass cancel request message requests the cancelation of all of the remaining quantity of a group of orders matching criteria specified within the request. NOTE: This message can only be used to cancel order messages (reduce the full quantity).

An order mass cancel request is assigned a ClOrdID and is treated as a separate entity. The order mass cancel request is acknowledged using an Order Mass Cancel Report. The Order Mass Cancel Report will contain the ClOrdID that was specified on the Order Mass Cancel Request. The ClOrdID assigned to the cancel request must be unique amongst the ClOrdID assigned to regular orders, replacement orders, cancel requests, and order mass cancel requests.

An immediate response to this message is required. It is recommended that an ExecutionRpt with ExecType=Pending Cancel be sent unless the Order Mass Cancel Request can be immediately accepted (ExecutionRpt with ExecType=Canceled) or rejected (Order Cancel Reject message).

Specifying order cancellation criteria is specified using the MassCancelRequestType field:

| Field | Description                              | Explanation                                                                                                                                                  |
| ----- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | Cancel orders for a security             | Cancel orders that match the security identification block, all fields required to uniquely qualify the security should be specified.                        |
| 2     | Cancel orders for an Underlying security | Cancel orders that match the underlying security identification block, all fields required to uniquely identify the underlying security should be populated. |
| 3     | Cancel orders for a Product              | Cancel orders for a specific type of Product (high-level security classification), Only Product should be specified.                                         |
| 4     | Cancel orders for a CFICode              | Cancel orders for a specific type of CFICode (security classification), Only CFICode should be specified.                                                    |
| 5     | Cancel orders for a SecurityType         | Cancel orders for a specific type of security, Only SecurityType should be specified.                                                                        |
| 6     | Cancel orders for a trading session      | Cancel orders for a specific trading session, TradingSessionID must be specified.                                                                            |
| 7     | Cancel all orders                        | Cancel all orders for the firm identified using this FIX connection.                                                                                         |

Example uses of MassCancelRequestType with Qualifiers:

- Cancel for a Symbol
- Cancel for an underlying
- Cancel orders on one side of a market for a symbol
- Cancel orders for a specific option series
- Cancel all orders
- Cancel all orders on one side of a market
- Cancel all money market orders
- Cancel all common stock orders
- Cancel all orders for a trading session
- Cancel all orders for a trading session on one side of a market

~~April 30, 2003~~June 18, 2003 38 FIX 4.4 with Errata 20030618- Volume 4


---

# Cancel all orders for a trading session for an underlying on one side of a market

The format of the Order Mass Cancel Request message is:

# Order Mass Cancel Request

| Tag                                                                                                                                                                          | Field Name            | Req'd | Comments                                                                                                                                                                           |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|                                                                                                                                                                              | Standard Header       | Y     | MsgType = q (lowercase Q)                                                                                                                                                          |
| 11                                                                                                                                                                           | ClOrdID               | Y     | Unique ID of Order Mass Cancel Request as assigned by the institution.                                                                                                             |
| 526                                                                                                                                                                          | SecondaryClOrdID      | N     |                                                                                                                                                                                    |
| 530                                                                                                                                                                          | MassCancelRequestType | Y     | Specifies the type of cancellation requested                                                                                                                                       |
| 336                                                                                                                                                                          | TradingSessionID      | N     | Trading Session in which orders are to be canceled                                                                                                                                 |
| 625                                                                                                                                                                          | TradingSessionSubID   | N     |                                                                                                                                                                                    |
| component block \<Instrument> N Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                |                       |       |                                                                                                                                                                                    |
| component block \<UnderlyingInstrument> N Insert here the set of "UnderlyingInstrument" (underlying symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |                       |       |                                                                                                                                                                                    |
| 54                                                                                                                                                                           | Side                  | N     | Optional qualifier used to indicate the side of the market for which orders are to be canceled. Absence of this field indicates that orders are to be canceled regardless of side. |
| 60                                                                                                                                                                           | TransactTime          | Y     | Time this order request was initiated/released by the trader or trading system.                                                                                                    |
| 58                                                                                                                                                                           | Text                  | N     |                                                                                                                                                                                    |
| 354                                                                                                                                                                          | EncodedTextLen        | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                                                                     |
| 355                                                                                                                                                                          | EncodedText           | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                                     |
|                                                                                                                                                                              | Standard Trailer      | Y     |                                                                                                                                                                                    |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;s>&#x3C;!ENTITY % OrderMassCancelReqCustom "">&#x3C;/s>

&#x3C;s>&#x3C;!ENTITY % OrderMassCancelReqContent "ClOrdID,

SecondaryClOrdID?,

MassCancelRequestType,TradingSessionID?,

TradingSessionSubID?,

Instrument,UnderlyingInstrument

,Side?,TransactTime,Text?,EncodedTextGroup?>

&#x3C;s>%OrderMassCancelReqCustom;" >&#x3C;/s>

&#x3C;s>&#x3C;!ELEMENT OrderMassCancelReq (%OrderMassCancelReqContent;)>&#x3C;/s>

&#x3C;s>&#x3C;!ATTLIST OrderMassCancelReq FIXTag

CDATA #FIXED '35'>&#x3C;/s>

&#x3C;s>DataType CDATA #FIXED 'String'>&#x3C;/s>

&#x3C;s>Value&#x3C;/s>  &#x3C;s>CDATA #FIXED 'q' >&#x3C;/s> Refer to FIXML element OrdMassCxlReq

April 30, 2003   June 18, 2003                    39      FIX 4.4 with Errata 20030618- Volume 4


---
Order Mass Cancel Report
The Order Mass Cancel Report is used to acknowledge an Order Mass Cancel Request. Note that each affected order that is canceled is acknowledged with a separate Execution Report or Order Cancel Reject message.

# Order Mass Cancel Report

| Tag                                                                                                                                                                          | Field Name                | Req'd | Comments                                                                                                                                                                  |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|                                                                                                                                                                              | Standard Header           | Y     | MsgType = r (lowercase R)                                                                                                                                                 |
| 11                                                                                                                                                                           | ClOrdID                   | N     | ClOrdID provided on the Order Mass Cancel Request.                                                                                                                        |
| 526                                                                                                                                                                          | SecondaryClOrdID          | N     |                                                                                                                                                                           |
| 37                                                                                                                                                                           | OrderID                   | Y     | Unique Identifier for the Order Mass Cancel Request assigned by the recipient of the Order Mass Cancel Request                                                            |
| 198                                                                                                                                                                          | SecondaryOrderID          | N     | Secondary Order ID assigned by the recipient of the Order Mass Cancel Request                                                                                             |
| 530                                                                                                                                                                          | MassCancelRequestType     | Y     | Order Mass Cancel Request Type accepted by the system                                                                                                                     |
| 531                                                                                                                                                                          | MassCancelResponse        | Y     | Indicates the action taken by the counterparty order handling system as a result of the Cancel Request                                                                    |
|                                                                                                                                                                              |                           |       | 0 – Indicates Order Mass Cancel Request was rejected.                                                                                                                     |
| 532                                                                                                                                                                          | MassCancelRejectReason    | N     | Indicates why Order Mass Cancel Request was rejected Required if MassCancelResponse = 0                                                                                   |
| 533                                                                                                                                                                          | TotalAffectedOrders       | N     | Optional field used to indicate the total number of orders affected by the Order Mass Cancel Request                                                                      |
| 534                                                                                                                                                                          | NoAffectedOrders          | N     | Optional field used to indicate the number of order identifiers for orders affected by the Order Mass Cancel Request. Must be followed with OrigClOrdID as the next field |
| 41                                                                                                                                                                           | OrigClOrdID               | N     | Required if NoAffectedOrders > 0 Indicates the client order id of an order affected by the Order Mass Cancel Request.                                                     |
| 535                                                                                                                                                                          | AffectedOrderID           | N     | Contains the OrderID assigned by the counterparty of an affected order. Not required as part of the repeating group.                                                      |
| 536                                                                                                                                                                          | AffectedSecondaryOrder ID | N     | Contains the SecondaryOrderID assigned by the counterparty of an affected order. Not required as part of the repeating group                                              |
| 336                                                                                                                                                                          | TradingSessionID          | N     | Trading Session in which orders are to be canceled                                                                                                                        |
| 625                                                                                                                                                                          | TradingSessionSubID       | N     |                                                                                                                                                                           |
| component block \<Instrument> N Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                |                           |       |                                                                                                                                                                           |
| component block \<UnderlyingInstrument> N Insert here the set of "UnderlyingInstrument" (underlying symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |                           |       |                                                                                                                                                                           |
| 54                                                                                                                                                                           | Side                      | N     | Side of the market specified on the Order Mass Cancel Request                                                                                                             |
| 60                                                                                                                                                                           | TransactTime              | N     | Time this report was initiated/released by the sells-side (broker, exchange, ECN) or sell-side executing system.                                                          |

~~April 30, 2003~~June 18, 2003

40 FIX 4.4 with Errata 20030618- Volume 4


---

# FIXML Definition for this message

See http://www.fixprotocol.org for details

| 58  | Text             | N |                                                                                                                                |   |
| --- | ---------------- | - | ------------------------------------------------------------------------------------------------------------------------------ | - |
| 354 | EncodedTextLen   | N | Must be set if EncodedText field is specified and must immediately precede it.                                                 |   |
| 355 | EncodedText      | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |   |
|     | Standard Trailer |   |                                                                                                                                |   |

# OrderMassCancelReport

&#x3C;!ENTITY % OrderMassCancelReportCustom "">

&#x3C;!ENTITY % OrderMassCancelReportContent "ClOrdID?, SecondaryClOrdID?, OrderID?, SecondaryOrderID?, MassCancelRequestType, MassCancelResponse, MassCancelRejectReason?, TotalAffectedOrders?, AffectedOrdersList?, TradingSessionID?, TradingSessionSubID?, Instrument, UnderlyingInstrument, Side?, TransactTime, Text?, EncodedTextGroup? %OrderMassCancelReportCustom;">

&#x3C;!ELEMENT OrderMassCancelReport (%OrderMassCancelReportContent;)>

&#x3C;!ATTLIST OrderMassCancelReport FIXTag CDATA #FIXED '35' DataType CDATA #FIXED 'String' Value CDATA #FIXED 'r'> Refer to FIXML element OrdMassCxlRpt

April 30, 2003

June 18, 2003

41 FIX 4.4 with Errata 20030618- Volume 4


---

Order Mass Status Request

The order mass status request message requests the status for orders matching criteria specified within the request.

A mass status request is assigned a ClOrdID and is treated as a separate entity.

ExecutionReports with ExecType="Order Status" are returned for all orders matching the criteria provided on the request.

Specifying order selection criteria is specified using the MassStatusReqType field:

| Field | Description                                      | Explanation                                                                                                                                                            |
| ----- | ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | Status for all orders for a security             | Return status on orders that match the security identification block, all fields required to uniquely qualify the security should be specified.                        |
| 2     | Status for all orders for an Underlying security | Return status on orders that match the underlying security identification block, all fields required to uniquely identify the underlying security should be populated. |
| 3     | Status for all orders for a Product              | Return status on orders for a specific type of Product (high-level security classification), Only Product should be specified.                                         |
| 4     | Status for all orders for a CFICode              | Return status on orders for a specific type of CFICode (security classification), Only CFICode should be specified.                                                    |
| 5     | Status for all orders for a SecurityType         | Return status on orders for a specific type of security, Only SecurityType should be specified.                                                                        |
| 6     | Status for all orders for a trading session      | Return status on orders for a specific trading session, TradingSessionID must be specified.                                                                            |
| 7     | Status for all orders                            | Return status on all orders for the firm identified using this FIX connection.                                                                                         |
| 8     | Status for order belonging to a PartyID          | Status all orders belonging to a PartyID.                                                                                                                              |
| 9     | Status for all orders for an Account             | Status for orders for an account.                                                                                                                                      |

Example uses of MassStatusReqType with Qualifiers:

- Status for a Symbol
- Status for an underlying
- Status orders on one side of a market for a symbol
- Status orders for a specific option series
- Status all orders
- Status all orders on one side of a market
- Status all money market orders
- Status all common stock orders
- Status all orders for a trading session
- Status all orders for a trading session on one side of a market
- Status all orders for a trading session for an underlying on one side of a market
- Status all orders belonging to a PartyID.

~~April 30, 2003~~ June 18, 2003 42 FIX 4.4 with Errata 20030618- Volume 4



---
Status all orders belonging to an Account

# Order Mass Status Request

| Tag | Field Name                              | Req'd | Comments                                                                                                                           |
| --- | --------------------------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------- |
|     | Standard Header                         | Y     | MsgType = AF                                                                                                                       |
| 584 | MassStatusReqID                         | Y     | Unique ID of mass status request as assigned by the institution.                                                                   |
| 585 | MassStatusReqType                       | Y     | Specifies the scope of the mass status request                                                                                     |
|     | component block \<Parties>              | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"               |
| 1   | Account                                 | N     | Account                                                                                                                            |
| 660 | AcctIDSource                            | N     |                                                                                                                                    |
| 336 | TradingSessionID                        | N     | Trading Session                                                                                                                    |
| 625 | TradingSessionSubID                     | N     |                                                                                                                                    |
|     | component block \<Instrument>           | N     | Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                      |
|     | component block \<UnderlyingInstrument> | N     | Insert here the set of "UnderlyingInstrument" (underlying symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |
| 54  | Side                                    | N     | Optional qualifier used to indicate the side of the market for which orders will be returned.                                      |
|     | Standard Trailer                        | Y     |                                                                                                                                    |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;!ENTITY % OrderMassStatusReqCustom "">

&#x3C;!ENTITY % OrderMassStatusReqContent "MassStatusReqID,MassStatusReqType,PartiesList?,Account?,TradingSessionID?,TradingSessionSubID?,Instrument?,UnderlyingInstrument?,Side?, %OrderMassStatusReqCustom;">

&#x3C;!ELEMENT OrderMassStatusReq (%OrderMassStatusReqContent;)>

&#x3C;!ATTLIST OrderMassStatusReq FIXTag CDATA #FIXED '35'>

DataType CDATA #FIXED 'String'

Value CDATA #FIXED 'AF' >Refer to FIXML element OrdMassStatReq

April 30, 2003

June 18, 2003 43 FIX 4.4 with Errata 20030618- Volume 4


---
Order State Change Matrices (formerly known as “Appendix D”)

The following matrices are included to clarify the sequence of messages and the status of orders involved in the submission and processing of new orders, executions, cancel requests, cancel/replace requests and order status requests. The matrices have been arranged in groups as follows:

# A   Vanilla

| Ref   | Description                         | Old Reference(4.3 Errata) |
| ----- | ----------------------------------- | ------------------------- |
| A.1.a | Filled order                        | 1                         |
| A.1.b | Part-filled day order, done for day | 2                         |

# B   Cancel

| Ref   | Description                                                                                                               | Old Reference (4.3 Errata) |
| ----- | ------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| B.1.a | Cancel request issued for a zero-filled order                                                                             | 3                          |
| B.1.b | Cancel request issued for a part-filled order – executions occur whilst cancel request is active                          | 4                          |
| B.1.c | Cancel request issued for an order that becomes filled before cancel request can be accepted                              | 5                          |
| B.1.d | Cancel request issued for an order that has not yet been acknowledged                                                     | 6                          |
| B.1.e | Cancel request issued for an order that has not yet been acknowledged – the acknowledgment and the cancel request ‘cross’ | 7                          |
| B.1.f | Cancel request issued for an unknown order                                                                                | 7a                         |

# C   Cancel/Replace quantity changes

# C.1 Replace to increase quantity

| Ref   | Description                                                                                                                   | Old Reference (4.3 Errata) |
| ----- | ----------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| C.1.a | Zero-filled order, cancel/replace request issued to increase order qty                                                        | 8                          |
| C.1.b | Part-filled order, followed by cancel/replace request to increase order qty, execution occurs whilst order is pending replace | 9                          |
| C.1.c | Filled order, followed by cancel/replace request to increase order quantity                                                   | 10                         |

# C.2 Replace not for quantity change

| Ref                                    | Description | Old Reference (4.3 Errata) |
| -------------------------------------- | ----------- | -------------------------- |
| April 30, 2003                         |             |                            |
| June 18, 2003                          |             |                            |
| 44                                     |             |                            |
| FIX 4.4 with Errata 20030618- Volume 4 |             |                            |


---
FIX 4.4 with Errata 20030618 - Volume 4

# C. Cancel/Replace Request

# C.2.a

Cancel/replace request (not for quantity change) is rejected as a fill has occurred

# C.3 Replace to decrease quantity

| Ref   | Description                                                                                                                                   | Old Reference (4.3 Errata) |
| ----- | --------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| C.3.a | Cancel/replace request sent whilst execution is being reported – the requested order qty exceeds the cum qty. Order is replaced then filled   | 12                         |
| C.3.b | Cancel/replace request sent whilst execution is being reported – the requested order qty equals the cum qty – order qty is amended to cum qty | 13                         |
| C.3.c | Cancel/replace request sent whilst execution is being reported – the requested order qty is below cum qty – order qty is amended to cum qty   | 14                         |

# D. Cancel/Replace Sequencing and Chaining

# D.1 Sequencing

| Ref   | Description                                                                                                                                | Old Reference (4.3 Errata) |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------- |
| D.1.a | One cancel/replace request is issued which is accepted – another one is issued which is also accepted                                      | 15                         |
| D.1.b | One cancel/replace request is issued which is rejected before order becomes pending replace – then another one is issued which is accepted | 16                         |
| D.1.c | One cancel/replace request is issued which is rejected after it is in pending replace – then another one is issued which is accepted       | 17                         |

# D.2 Chaining

| Ref   | Description                                                                                                                  | Old Reference (4.3 Errata) |
| ----- | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| D.2.a | One cancel/replace request is issued followed immediately by another – broker processes sequentially                         | 18                         |
| D.2.b | One cancel/replace request is issued followed immediately by another – broker processes pending replaces before replaces     | 19                         |
| D.2.c | One cancel/replace request is issued followed immediately by another – both are rejected                                     | 20                         |
| D.2.d | One cancel/replace request is issued followed immediately by another – broker rejects the second as order is pending replace | 21                         |

~~April 30, 2003~~ June 18, 2003


---


# E   Unsolicited/Reinstatement

| Ref   | Description                                                                                                       | Old Reference (4.3 Errata) |
| ----- | ----------------------------------------------------------------------------------------------------------------- | -------------------------- |
| E.1.a | Telephoned order                                                                                                  | 22                         |
| E.1.b | Unsolicited cancel of a part-filled order                                                                         | 23                         |
| E.1.c | Unsolicited replacement of a part-filled order                                                                    | 24                         |
| E.1.d | Unsolicited reduction of order quantity by sell side ( e.g. for US ECNs to communicate Nasdaq SelectNet declines) | 25                         |
| E.1.e | Unsolicited cancel of ‘cancel if not best’ order                                                                  |                            |

# F   Order Reject

| Ref   | Description                                                          | Old Reference (4.3 Errata) |
| ----- | -------------------------------------------------------------------- | -------------------------- |
| F.1.a | Order rejected due to duplicate ClOrdID                              | 26                         |
| F.1.b | Poss resend and duplicate ClOrdID                                    | 27                         |
| F.1.c | Order rejected because the order has already been verbally submitted | 28                         |

# G   Status

| Ref   | Description                                                                                                | Old Reference (4.3 Errata) |
| ----- | ---------------------------------------------------------------------------------------------------------- | -------------------------- |
| G.1.a | Order status request rejected for unknown order                                                            | 29                         |
| G.1.b | Transmitting a CMS-style “Nothing Done” in response to a status request                                    | 30                         |
| G.1.c | Order sent, immediately followed by a status request. Subsequent status requests sent during life of order | 31                         |

# H   GT

| Ref   | Description                                                                                    | Old Reference (4.3 Errata) |
| ----- | ---------------------------------------------------------------------------------------------- | -------------------------- |
| H.1.a | GTC order partially filled, restated (renewed) and partially filled the following day          | 32                         |
| H.1.b | GTC order with partial fill, a 2:1 stock split then a partial fill and fill the following day  | 33                         |
| H.1.c | GTC order partially filled, restated(renewed) and canceled the following day                   | 34                         |
| H.1.d | GTC order partially filled, restated(renewed) followed by replace request to increase quantity | 35                         |

~~April 30, 2003~~June 18, 2003                46              FIX 4.4 with Errata 20030618- Volume 4


---

# I     TimeInForce

| Ref   | Description                                              | Old Reference (4.3 Errata) |
| ----- | -------------------------------------------------------- | -------------------------- |
| I.1.a | Fill or Kill order cannot be filled                      | 36                         |
| I.1.b | Immediate or Cancel order that cannot be immediately hit | 37                         |

# J     Execution Cancels/Corrects

| Ref   | Description                                                                                                                            | Old Reference (4.3 Errata) |
| ----- | -------------------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| J.1.a | Filled order, followed by correction and cancellation of executions                                                                    | 38                         |
| J.1.b | A canceled order followed by a busted execution and a new execution                                                                    | 39                         |
| J.1.c | GTC order partially filled, restated (renewed) and partially filled the following day, with corrections of quantity on both executions | 40                         |
| J.1.d | Part-filled order Done for day followed by trade correction and bust                                                                   | 41                         |

# K     Trading Halt

| Ref   | Description              | Old Reference (4.3 Errata) |
| ----- | ------------------------ | -------------------------- |
| K.1.a | Trading Halt – Reinstate | 43                         |
| K.1.b | Trading Halt – Cancel    | 44                         |

# L     Miscellaneous

| Ref   | Description                                                                  | Old Reference (4.3 Errata) |
| ----- | ---------------------------------------------------------------------------- | -------------------------- |
| L.1.a | Transmitting a guarantee of execution prior to execution (Stopped/Guarantee) | 42                         |
| L.1.b | Use of CashOrderQty                                                          |                            |

~~April 30, 2003~~June 18, 2003

47 FIX 4.4 with Errata 20030618- Volume 4


---

# FIX 4.4 with Errata 20030618 - Volume 4

# April 30, 2003

# June 18, 2003



The Table below shows which state transitions have been illustrated by the matrices in this Appendix (marked with an asterisk). The row represents the current value of OrdStatus and the column represents the next value as reported back to the buy-side via an execution report or order cancel reject message. Next to each OrdStatus value is its precedence – this is used when the order exists in a number of states simultaneously to determine the value that should be reported back. Note that absence of a scenario should not necessarily be interpreted as meaning that the state transition is not allowed:

| OrdStatus            | New (2) | Partially Filled (4) | Filled (8) | Done for Day (10) | Pending Cancel (12) | Pending Replace (11) | Canceled (5) | Rejected (2) | Stopped (7) |
| -------------------- | ------- | -------------------- | ---------- | ----------------- | ------------------- | -------------------- | ------------ | ------------ | ----------- |
| Pending New (2)      | \*      |                      |            |                   |                     |                      |              |              |             |
| New (2)              |         | \*                   |            |                   |                     |                      |              |              |             |
| Partially Filled (4) |         |                      | \*         |                   |                     |                      |              |              |             |
| Filled (8)           |         |                      |            | \*                |                     |                      |              |              |             |
| Done for Day (10)    |         |                      |            |                   | \*                  |                      |              |              |             |
| Pending Cancel (12)  |         |                      |            |                   |                     | \*                   |              |              |             |
| Pending Replace (11) |         |                      |            |                   |                     |                      | \*           |              |             |
| Canceled (5)         |         |                      |            |                   |                     |                      |              | \*           |             |
| Rejected (2)         |         |                      |            |                   |                     |                      |              |              | \*          |
| Stopped (7)          |         |                      |            |                   |                     |                      |              |              |             |

# How to read the Order State Change Matrices:

- The ‘Execution Report’ message is referred to simply as ‘Execution’
- The ‘Order Cancel/Replace Request’ and ‘Order Cancel Request’ messages are referred to as ‘Replace Request’ and ‘Cancel Request’ respectively
- The shaded rows represent messages sent from buy-side to the sell-side
- In general where two lines of a matrix share the same time, this means either
- that there are two possible paths (e.g. a request is accepted or rejected) – in this case the first row of the two possible paths is the reject case which is italicized. The non-italicized row is the path that is continued by the remainder of the matrix
- that two messages are being sent at the same time but in different directions such that the messages cross on the connection (e.g. a cancel request is sent at the same time as the sell-side is sending an execution) – in this case both lines have bold text
- For scenarios involving cancel requests or cancel/replace requests ‘X’ refers to the original order, ‘Y’ refers to the cancel/replacing order. A similar convention is used for corrections or cancels to executions



---
A Vanilla

# A.1.a - Filled order

| Time | Message      | Message Sent | Exec     | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                 |
| ---- | ------------ | ------------ | -------- | ---------------- | --------- | ------- | ---------- | -------- | --------------------------------------- |
| 1    | New Order(X) |              |          |                  | 10000     |         |            |          |                                         |
| 2    |              | Execution(X) | Rejected | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sales           |
| 2    |              | Execution(X) | New      | New              | 10000     | 0       | 10000      | 0        |                                         |
| 3    |              | Execution(X) | Rejected | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by trader/exchange |
| 3    |              | Execution(X) | Trade    | Partially Filled | 10000     | 2000    | 8000       | 2000     | Execution of 2000                       |
| 4    |              | Execution(X) | Trade    | Partially Filled | 10000     | 3000    | 7000       | 1000     | Execution of 1000                       |
| 5    |              | Execution(X) | Trade    | Filled           | 10000     | 10000   | 0          | 7000     | Execution of 7000                       |

# A.1.b – Part-filled day order, done for day

| Time | Message      | Message Sent | Exec     | OrdStatus        | Order Qty    | Cum Qty | Leaves Qty | Last Qty | Comment              |                                                              |
| ---- | ------------ | ------------ | -------- | ---------------- | ------------ | ------- | ---------- | -------- | -------------------- | ------------------------------------------------------------ |
| 1    | New Order(X) |              |          |                  | 10000        |         |            |          |                      |                                                              |
| 2    |              | Execution(X) | Rejected | Rejected         | 10000        | 0       | 0          | 0        | If order is rejected |                                                              |
| 2    |              | Execution(X) | New      | New              | 10000        | 0       | 10000      | 0        |                      |                                                              |
| 3    |              | Execution(X) | Trade    | Partially Filled | 10000        | 2000    | 8000       | 2000     | Execution of 2000    |                                                              |
| 4    |              | Execution(X) | Trade    | Partially Filled | 10000        | 3000    | 7000       | 1000     | Execution of 1000    |                                                              |
| 5    |              | Execution(X) |          | Done for Day     | Done for Day | 10000   | 3000       | 0        | 0                    | Assuming day order. See other examples which cover GT orders |

March 25, 2003 – DRAFT 49 FIX 4.4 - Volume 4
---


# B Cancel

# B.1.a – Cancel request issued for a zero-filled order

| Time | Message             | Message Sent  | Exec     | OrdStatus | Order Type | Cum Qty | Leaves Qty | Last Qty | Comment                        |                                      |
| ---- | ------------------- | ------------- | -------- | --------- | ---------- | ------- | ---------- | -------- | ------------------------------ | ------------------------------------ |
| 1    | New Order(X)        |               |          |           | 10000      |         |            |          |                                |                                      |
| 2    |                     | Execution(X)  | Rejected | Rejected  | 10000      | 0       | 0          | 0        | If order is rejected           |                                      |
| 2    |                     | Execution(X)  | New      | New       | 10000      | 0       | 10000      | 0        |                                |                                      |
| 3    | Cancel Request(Y,X) |               |          |           | 10000      |         |            |          |                                |                                      |
| 4    |                     | Cancel Reject |          | New       |            |         |            |          |                                | If rejected by salesperson           |
| 4    |                     | Execution     | Pending  | Pending   | 10000      | 0       | 10000      | 0        | Acknowledge the cancel request |                                      |
| 5    |                     | Cancel Reject |          | New       |            |         |            |          |                                | If rejected by trader/exchange       |
| 5    |                     | Execution     |          | Canceled  | Canceled   | 10000   | 0          | 0        | 0                              | Confirm that order has been canceled |

March 25, 2003 – DRAFT

50

FIX 4.4 - Volume 4


---


# B.1.b – Cancel request issued for a part-filled order – executions occur whilst cancel request is active

| Time | Message             | Message Sent | Exec Type        | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty               | Comment                                                                                                                                  |                                                                               |
| ---- | ------------------- | ------------ | ---------------- | ---------------- | --------- | ------- | ---------- | ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| 1    | New Order(X)        |              |                  |                  | 10000     |         |            |                        |                                                                                                                                          |                                                                               |
| 2    |                     | Execution(X) | Rejected         | Rejected         | 10000     | 0       | 0          | 0                      | If order is rejected                                                                                                                     |                                                                               |
| 2    |                     | Execution(X) | New              | New              | 10000     | 0       | 10000      | 0                      |                                                                                                                                          |                                                                               |
| 3    |                     | Execution(X) | Trade            | Partially Filled | 10000     | 2000    | 8000       | 2000                   | Execution for 2000                                                                                                                       |                                                                               |
| 4    | Cancel Request(Y,X) |              |                  |                  | 10000     |         |            |                        |                                                                                                                                          |                                                                               |
| 4    |                     | Execution(X) | Trade            | Partially Filled | 10000     | 5000    | 5000       | 3000                   | Execution for 3000. This execution passes the cancel request on the connection                                                           |                                                                               |
| 5    | Cancel Reject (Y,X) |              | Partially Filled |                  |           |         |            | If request is rejected |                                                                                                                                          |                                                                               |
| 5    |                     | Execution    | Pending          | Pending          | 10000     | 5000    | 5000       | 0                      | ‘Pending cancel’ order status takes precedence over ‘partially filled’ order status                                                      |                                                                               |
| 6    |                     | Execution(X) | Trade            | Pending Cancel   | 10000     | 6000    | 4000       | 1000                   | Execution for 1000 whilst order is pending cancel – ‘pending cancel’ order status takes precedence over ‘partially filled’ order status. |                                                                               |
| 7    | Cancel Reject (Y,X) |              | Partially Filled |                  |           |         |            | If request is rejected |                                                                                                                                          |                                                                               |
| 7    |                     | Execution    |                  | Canceled         | Canceled  | 10000   | 6000       | 0                      | 0                                                                                                                                        | ‘Canceled’ order status takes precedence over ‘partially filled’ order status |

March 25, 2003 – DRAFT

51

FIX 4.4 - Volume 4


---


# B.1.c – Cancel request issued for an order that becomes filled before cancel request can be accepted

| Time | Message             | Message Sent | Exec    | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                      |
| ---- | ------------------- | ------------ | ------- | ---------------- | --------- | ------- | ---------- | -------- | ---------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)        |              |         | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected                                                                                                         |
| 2    | Execution(X)        |              | New     | New              | 10000     | 0       | 10000      | 0        |                                                                                                                              |
| 3    | Execution(X)        |              | Trade   | Partially Filled | 10000     | 2000    | 8000       | 2000     | Execution for 2000                                                                                                           |
| 4    | Cancel Request(Y,X) |              |         |                  | 10000     |         |            |          |                                                                                                                              |
| 4    | Execution(X)        |              | Trade   | Partially Filled | 10000     | 5000    | 5000       | 3000     | Execution for 3000. This execution passes the cancel request on the connection                                               |
| 5    | Cancel Reject (Y,X) |              |         | Partially Filled |           |         |            |          | If request is rejected                                                                                                       |
| 5    | Execution (Y,X)     |              | Pending | Pending          | 10000     | 5000    | 5000       | 0        | ‘Pending cancel’ order status takes precedence over ‘partially filled’ order status                                          |
| 6    | Execution(X)        |              | Trade   | Pending          | 10000     | 10000   | 0          | 5000     | Execution for 5000 whilst order is pending cancel. ‘Pending cancel’ order status takes precedence over ‘filled’ order status |
| 7    | Cancel Reject (Y,X) |              |         | Filled           |           |         |            |          | Cancel request rejected – CxlRejectReason = 0 (too late to cancel)                                                           |

March 25, 2003 – DRAFT

52

FIX 4.4 - Volume 4


---

# B.1.d – Cancel request issued for an order that has not yet been acknowledged

| Time | Message Received    | Message Sent (ClOrdID, OrigClOrdID) | Exec Type | OrdStatus | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                 |
| ---- | ------------------- | ----------------------------------- | --------- | --------- | --------- | ------- | ---------- | -------- | ------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)        |                                     |           |           | 10000     |         |            |          | Order sender immediately wishes to cancel the order                                                     |
| 2    | Cancel Request(Y,X) |                                     |           |           | 10000     |         |            |          | Order sender immediately wishes to cancel the order                                                     |
| 3    |                     | Execution (Y,X)                     | Pending   | Pending   | 10000     | 0       | 10000      | 0        | OrigClOrd set to X even though X has not yet been ‘accepted’.                                           |
| 4    |                     | Execution (X)                       | New       | New       | 10000     | 0       | 10000      | 0        | Order accepted before cancel request is processed.                                                      |
| 5    |                     | Execution (Y,X)                     | Cancele   | Canceled  | 10000     | 0       | 0          | 0        | Order canceled.                                                                                         |
| 6    | New Order(A)        |                                     |           |           | 5000      |         |            |          | Order sender immediately wishes to cancel the order                                                     |
| 7    | Cancel Request(B,A) |                                     |           |           | 5000      |         |            |          | Order sender immediately wishes to cancel the order                                                     |
| 8    |                     | Execution (B,A)                     | Pending   | Pending   | 5000      | 0       | 5000       | 0        | OrigClOrd set to A even though A has not yet been ‘accepted’.                                           |
| 9    |                     | Execution (B,A)                     | Cancele   | Canceled  | 5000      | 0       | 0          | 0        | Order canceled before it is accepted. Note OrigClOrdID set to A even though A has not yet been accepted |

# B.1.e – Cancel request issued for an order that has not yet been acknowledged – the acknowledgment and the cancel request ‘cross’

| Time | Message Received    | Message Sent (ClOrdID, OrigClOrdID) | Exec Type | OrdStatus | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                             |
| ---- | ------------------- | ----------------------------------- | --------- | --------- | --------- | ------- | ---------- | -------- | --------------------------------------------------- |
| 1    | New Order(X)        |                                     |           |           | 10000     |         |            |          | Order sender immediately wishes to cancel the order |
| 2    | Cancel Request(Y,X) |                                     |           |           | 10000     |         |            |          | Order sender immediately wishes to cancel the order |
| 2    |                     | Execution (X)                       | New       | New       | 10000     | 0       | 10000      | 0        | This message crosses the Cancel request             |
| 3    |                     | Execution (Y,X)                     | Pending   | Pending   | 10000     | 0       | 10000      | 0        |                                                     |
| 4    |                     | Execution (Y,X)                     | Cancele   | Canceled  | 10000     | 0       | 0          | 0        | Order canceled.                                     |


March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---


# B.1.f – Cancel request issued for an unknown order

| Time | Message             | Message Sent | Exec | OrdStatus | Order | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                      |
| ---- | ------------------- | ------------ | ---- | --------- | ----- | ------- | ---------- | -------- | ------------------------------------------------------------------------------------------------------------ |
| 1    | Cancel Request(Y,X) |              |      |           |       | 10000   |            |          |                                                                                                              |
| 2    | Cancel Reject (Y,X) |              |      | Rejected  |       |         |            |          | Cancel request rejected with reject reason of “Unknown Order”, OrdStatus is “Rejected” and OrderID is “NONE” |

NOTE: It is important to note that rejecting a cancel request for an unknown OrigClOrdID does not cause the sell-side to consume the OrigClOrdID used in the Cancel Request.

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---

CURRENT_PAGE_RAW_OCR_TEXT


# C Cancel/Replace quantity changes

# C.1 Replace to increase quantity

# C.1.a – Zero-filled order, cancel/replace request issued to increase order qty

| Time | Message      | Message Sent  | Exec     | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                   |
| ---- | ------------ | ------------- | -------- | ---------------- | --------- | ------- | ---------- | -------- | --------------------------------------------------------- |
| 1    | New Order(X) |               |          |                  | 10000     |         |            |          |                                                           |
| 2    |              | Execution(X)  | Rejected | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN) |
| 2    |              | Execution(X)  | New      | New              | 10000     | 0       | 10000      | 0        |                                                           |
| 3    | Replace      | Request(Y,X)  |          |                  | 11000     |         |            |          | Request to increase order qty to 11000                    |
| 4    |              | Cancel Reject |          | New              |           |         |            |          | If request is rejected by salesperson                     |
| 4    |              | Execution     | Pending  | Pending          | 10000     | 0       | 10000      | 0        | Acknowledge the Replace request                           |
| 5    |              | Cancel Reject |          | New              |           |         |            |          | If rejected by trader/exchange                            |
| 5    |              | Execution     | Replace  | New              | 11000     | 0       | 11000      | 0        | Confirm order has been replaced                           |
| 6    |              | Execution (Y) | Trade    | Partially Filled | 11000     | 1000    | 10000      | 1000     | Execution for 1000. Use Y as the new ClOrdID.             |
| 7    |              | Execution (Y) | Trade    | Partially Filled | 11000     | 3000    | 8000       | 2000     | Execution for 2000                                        |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4



---


# C.1.b – Part-filled order, followed by cancel/replace request to increase order qty, execution occurs whilst order is pending replace

| Time | Message              | Message Sent (ClOrdID, OrigClOrdID) | Exec Type        | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                              |
| ---- | -------------------- | ----------------------------------- | ---------------- | ---------------- | --------- | ------- | ---------- | -------- | ------------------------------------------------------------------------------------ |
| 1    | New Order(X)         |                                     |                  |                  | 10000     |         |            |          |                                                                                      |
| 2    |                      | Execution(X)                        | Rejected         | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                            |
| 2    |                      | Execution(X)                        | New              | New              | 10000     | 0       | 10000      | 0        |                                                                                      |
| 3    |                      | Execution(X)                        | Trade            | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                   |
| 4    | Replace Request(Y,X) |                                     |                  |                  | 12000     |         |            |          | Request increase in order quantity to 12000                                          |
| 5    |                      | Cancel Reject                       | Partially Filled |                  |           |         |            |          | If request is rejected                                                               |
| 5    |                      | Execution                           | Pending          | Pending          | 10000     | 1000    | 9000       | 0        | ‘Pending replace’ order status takes precedence over ‘partially filled’ order status |
| 6    |                      | Execution(X)                        | Trade            | Pending Replace  | 10000     | 1100    | 8900       | 100      | Execution for 100 before cancel/replace request is dealt with                        |
| 7    |                      | Cancel Reject                       | Partially Filled |                  |           |         |            |          | If request is rejected                                                               |
| 7    |                      | Execution                           | Replace          | Partially Filled | 12000     | 1100    | 10900      | 0        | Confirm replace has been accepted                                                    |
| 8    |                      | Execution(Y)                        | Trade            | Filled           | 12000     | 12000   | 0          | 10900    | Execution for 10900                                                                  |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---


# C.1.c – Filled order, followed by cancel/replace request to increase order quantity

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent  | Exec Type | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                              |
| ------------- | ------------------------------ | ------------- | --------- | ---------------- | --------- | ------- | ---------- | -------- | ------------------------------------------------------------------------------------ |
| 1             | New Order(X)                   |               |           |                  | 10000     |         |            |          |                                                                                      |
| 2             |                                | Execution(X)  | Rejected  | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                            |
| 2             |                                | Execution(X)  | New       | New              | 10000     | 0       | 10000      | 0        |                                                                                      |
| 3             |                                | Execution(X)  | Trade     | Filled           | 10000     | 10000   | 0          | 10000    | Execution for 10000                                                                  |
| 4             | Replace Request(Y,X)           |               |           |                  | 12000     |         |            |          | Request increase in order quantity to 12000                                          |
| 5             |                                | Cancel Reject | Filled    |                  |           |         |            |          | If request is rejected                                                               |
| 5             |                                | Execution     | Pending   | Pending          | 10000     | 10000   | 0          | 0        | ‘Pending replace’ order status takes precedence over ‘partially filled’ order status |
| 6             |                                | Cancel Reject | Filled    |                  |           |         |            |          | If request is rejected                                                               |
| 6             |                                | Execution     | Replace   | Partially Filled | 12000     | 10000   | 2000       | 0        | Confirm order has been replaced                                                      |
| 7             |                                | Execution(Y)  | Trade     | Filled           | 12000     | 12000   | 0          | 2000     | Execution for 2000                                                                   |

March 25, 2003 – DRAFT

57

FIX 4.4 - Volume 4


---

# C.2 Replace not for quantity change

# C.2.a – Cancel/replace request (not for quantity change) is rejected as a fill has occurred

| Time | Message              | Message Sent  | Exec     | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                      |
| ---- | -------------------- | ------------- | -------- | ---------------- | --------- | ------- | ---------- | -------- | ------------------------------------------------------------------------------------------------------------ |
| 1    | New Order(X)         |               |          |                  | 10000     |         |            |          |                                                                                                              |
| 2    |                      | Execution(X)  | Rejected | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                                                    |
| 2    |                      | Execution(X)  | New      | New              | 10000     | 0       | 10000      | 0        |                                                                                                              |
| 3    |                      | Execution(X)  | Trade    | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                                           |
| 4    | Replace Request(Y,X) |               |          |                  | 10000     |         |            |          | Assume in this scenario that client does not wish to increase qty (e.g. client wants to amend limit price)   |
| 4    |                      | Execution (X) | Trade    | Filled           | 10000     | 10000   | 0          | 9000     | Execution for 9000 – the replace request message and this execution report pass each other on the connection |
| 5    |                      | Cancel Reject |          | Filled           |           |         |            |          | CxlRejectReason = 0 (too late to cancel)                                                                     |

March 25, 2003 – DRAFT

58

FIX 4.4 - Volume 4


---

# C.3 Replace to decrease quantity

# C.3.a – Cancel/replace request sent whilst execution is being reported – the requested order qty exceeds the cum qty. Order is replaced then filled

| Time | Message              | Message Sent (ClOrdID, OrigClOrdID) | Exec Type        | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                             |
| ---- | -------------------- | ----------------------------------- | ---------------- | ---------------- | --------- | ------- | ---------- | -------- | --------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)         |                                     |                  |                  | 10000     |         |            |          |                                                                                                     |
| 2    |                      | Execution(X)                        | Rejected         | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected                                                                                |
| 2    |                      | Execution(X)                        | New              | New              | 10000     | 0       | 10000      | 0        |                                                                                                     |
| 3    |                      | Execution(X)                        | Trade            | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                                  |
| 4    | Replace Request(Y,X) |                                     |                  |                  | 8000      |         |            |          | Request a decrease order quantity to 8000 (leaving 7000 open)                                       |
| 4    |                      | Execution(X)                        | Trade            | Partially Filled | 10000     | 1500    | 8500       | 500      | Execution for 500 sent. Replace request and this execution report pass each other on the connection |
| 5    | Cancel Reject (Y,X)  |                                     | Partially Filled |                  |           |         |            |          | If request is rejected by salesperson                                                               |
| 5    |                      | Execution                           | Pending          | Pending          | 10000     | 1500    | 8500       | 0        | ‘Pending replace’ order status takes precedence over ‘partially filled’ order status                |
| 6    |                      | Execution(X)                        | Trade            | Pending          | 10000     | 1600    | 8400       | 100      | Execution for 100 occurs before cancel/replace request is accepted                                  |
| 7    | Cancel Reject (Y,X)  |                                     | Partially Filled |                  |           |         |            |          | If request is rejected by trader/exchange                                                           |
| 7    |                      | Execution                           | Replace          | Partially Filled | 8000      | 1600    | 6400       | 0        | Replace is accepted as requested order qty exceeds cum qty                                          |
| 8    |                      | Execution (Y)                       | Trade            | Filled           | 8000      | 8000    | 0          | 6400     | Execution for 6400.                                                                                 |

March 25, 2003 – DRAFT                                    59                                         FIX 4.4 - Volume 4
---

# C.3.b – Cancel/replace request sent whilst execution is being reported – the requested order qty equals the cum qty – order qty is amended to cum qty

| Time | Message Received (ClOrdID, OrigClOrdID) | Message Sent (ClOrdID, OrigClOrdID) | Exec Type | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                                               |
| ---- | --------------------------------------- | ----------------------------------- | --------- | ---------------- | --------- | ------- | ---------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)                            |                                     |           |                  | 10000     |         |            |          |                                                                                                                                                       |
| 2    |                                         | Execution(X)                        | Rejected  | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                                                                                             |
| 2    |                                         | Execution(X)                        | New       | New              | 10000     | 0       | 10000      | 0        |                                                                                                                                                       |
| 3    | Replace Request(Y,X)                    |                                     |           |                  | 7000      |         |            |          | Client wishes to amend order qty to 7000                                                                                                              |
| 3    |                                         | Execution(X)                        | Trade     | Partially Filled | 10000     | 7000    | 3000       | 7000     | Execution for 7000 - the replace message and this execution report pass each other on the connection                                                  |
| 4    |                                         | Execution                           | Replace   | Filled           | 7000      | 7000    | 0          | 0        | The replace request is interpreted as requiring the balance of the order to be canceled – the ‘filled’ order status takes precedence over ‘canceled’. |

# C.3.c – Cancel/replace request sent whilst execution is being reported – the requested order qty is below cum qty – order qty is amended to cum qty

| Time | Message Received (ClOrdID, OrigClOrdID) | Message Sent (ClOrdID, OrigClOrdID) | Exec Type | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                                               |
| ---- | --------------------------------------- | ----------------------------------- | --------- | ---------------- | --------- | ------- | ---------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)                            |                                     |           |                  | 10000     |         |            |          |                                                                                                                                                       |
| 2    |                                         | Execution(X)                        | Rejected  | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                                                                                             |
| 2    |                                         | Execution(X)                        | New       | New              | 10000     | 0       | 10000      | 0        |                                                                                                                                                       |
| 3    | Replace Request(Y,X)                    |                                     |           |                  | 7000      |         |            |          | Client wishes to amend order qty to 7000                                                                                                              |
| 3    |                                         | Execution(X)                        | Trade     | Partially Filled | 10000     | 8000    | 2000       | 8000     | Execution for 8000 - the replace message and this execution report pass each other on the connection                                                  |
| 4    |                                         | Execution                           | Replace   | Filled           | 8000      | 8000    | 0          | 0        | The replace request is interpreted as requiring the balance of the order to be canceled – the ‘filled’ order status takes precedence over ‘canceled’. |


March 25, 2003 – DRAFT

60

FIX 4.4 - Volume 4


---


# D Cancel/Replace sequencing and chaining

# D.1 Sequencing

# D.1.a – One cancel/replace request is issued which is accepted – another one is issued which is also accepted

| Time | Message              | Message Sent  | Exec     | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                              |   |
| ---- | -------------------- | ------------- | -------- | ---------------- | --------- | ------- | ---------- | -------- | ------------------------------------------------------------------------------------ | - |
| 1    | New Order(X)         |               |          |                  | 10000     |         |            |          |                                                                                      |   |
| 2    |                      | Execution(X)  | Rejected | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                            |   |
| 2    |                      | Execution(X)  | New      | New              | 10000     | 0       | 10000      | 0        |                                                                                      |   |
| 3    |                      | Execution(X)  | Trade    | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                   |   |
| 4    | Replace Request(Y,X) |               |          |                  | 8000      |         |            |          | Request decrease in order quantity to 8000, leaving 7000 open                        |   |
| 5    |                      | Execution     | Pending  | Pending          | 10000     | 1000    | 9000       | 0        | ‘Pending replace’ order status takes precedence over ‘partially filled’ order status |   |
| 6    |                      | Execution(X)  | Trade    | Pending          | 10000     | 1500    | 8500       | 500      | Execution for 500                                                                    |   |
| 7    |                      | Execution     | Replace  | Partially Filled | 8000      | 1500    | 6500       | 0        |                                                                                      |   |
| 8    |                      | Execution (Y) | Trade    | Partially Filled | 8000      | 3500    | 4500       | 2000     | Execution for 2000                                                                   |   |
| 9    | Replace Request(Z,Y) |               |          |                  | 6000      |         |            |          | Request decrease in order quantity to 6000, leaving 2500 open                        |   |
| 10   |                      | Execution     | Pending  | Pending          | 8000      | 3500    | 4500       | 0        |                                                                                      |   |
| 11   |                      | Execution(Y)  | Trade    | Pending          | 8000      | 4000    | 4000       | 500      | Execution for 500                                                                    |   |
| 12   |                      | Execution     | Replace  | Partially Filled | 6000      | 4000    | 2000       | 0        |                                                                                      |   |
| 13   |                      | Execution(Z)  | Trade    | Filled           | 6000      | 6000    | 0          | 2000     | Execution for 2000                                                                   |   |

March 25, 2003 – DRAFT

61

FIX 4.4 - Volume 4


---

March 25, 2003 – DRAFT


# D.1.b – One cancel/replace request is issued which is rejected before order becomes pending replace – then another one is issued which is accepted

| Time | Message Received     | Message Sent (ClOrdID, OrigClOrdID) | Exec Type        | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                            |
| ---- | -------------------- | ----------------------------------- | ---------------- | ---------------- | --------- | ------- | ---------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)         |                                     |                  |                  | 10000     |         |            |          |                                                                                                                                    |
| 2    |                      | Execution(X)                        | Rejected         | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                                                                          |
| 2    |                      | Execution(X)                        | New              | New              | 10000     | 0       | 10000      | 0        |                                                                                                                                    |
| 3    |                      | Execution(X)                        | Trade            | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                                                                 |
| 4    | Replace Request(Y,X) |                                     |                  |                  | 8000      |         |            |          | Request decrease in order quantity to 8000, leaving 7000 open                                                                      |
| 5    |                      | Cancel Reject                       | Partially Filled |                  |           |         |            |          | Request is rejected                                                                                                                |
| 6    |                      | Execution(X)                        | Trade            | Partially Filled | 10000     | 1500    | 8500       | 500      | Execution for 500                                                                                                                  |
| 7    |                      | Execution(X)                        | Trade            | Partially Filled | 10000     | 3500    | 6500       | 2000     | Execution for 2000                                                                                                                 |
| 8    | Replace Request(Z,X) |                                     |                  |                  | 6000      |         |            |          | Request decrease in order quantity to 6000, leaving 2500 open. Note that OrigClOrdID = X, as this is the last non rejected ClOrdID |
| 9    |                      | Execution                           | Pending Replace  | Pending          | 10000     | 3500    | 6500       | 0        | Note that OrigClOrdID = X                                                                                                          |
| 10   |                      | Execution                           | Replace          | Partially Filled | 6000      | 3500    | 2500       | 0        | Note that OrigClOrdID = X                                                                                                          |
| 11   |                      | Execution(Z)                        | Trade            | Partially Filled | 6000      | 5000    | 1000       | 1500     | Execution for 1500                                                                                                                 |


62                                         FIX 4.4 - Volume 4

---


# D.1.c - One cancel/replace request is issued which is rejected after it is in pending replace – then another one is issued which is accepted

| Time | Message Received     | Message Sent (ClOrdID, OrigClOrdID) | Exec Type        | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                           |
| ---- | -------------------- | ----------------------------------- | ---------------- | ---------------- | --------- | ------- | ---------- | -------- | --------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)         |                                     |                  |                  | 10000     |         |            |          |                                                                                                                                   |
| 2    |                      | Execution(X)                        | Rejected         | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                                                                         |
| 2    |                      | Execution(X)                        | New              | New              | 10000     | 0       | 10000      | 0        |                                                                                                                                   |
| 3    |                      | Execution(X)                        | Trade            | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                                                                |
| 4    | Replace Request(Y,X) |                                     |                  |                  | 8000      |         |            |          | Request decrease in order quantity to 8000, leaving 7000 open                                                                     |
| 5    |                      | Execution                           | Pending          | Pending          | 10000     | 1000    | 9000       | 0        |                                                                                                                                   |
| 6    |                      | Execution(X)                        | Trade            | Pending          | 10000     | 1500    | 8500       | 500      | Execution for 500. ‘Pending replace’ order status takes precedence over ‘partially filled’ order status                           |
| 7    | Cancel Reject        |                                     | Partially Filled |                  | 10000     | 3500    | 6500       | 2000     | Execution for 2000                                                                                                                |
| 9    | Replace Request(Z,X) |                                     |                  |                  | 6000      |         |            |          | Request decrease in order quantity to 6000, leaving 2500 open. Note that OrigClOrdID = X as this is the last non rejected ClOrdID |
| 10   |                      | Execution                           | Pending          | Pending          | 10000     | 3500    | 6500       | 0        |                                                                                                                                   |
| 11   | Cancel Reject        |                                     | Partially Filled |                  | 10000     | 3500    | 6500       | 0        | If request is rejected (e.g. by trader/exchange)                                                                                  |
| 11   |                      | Execution                           | Replace          | Partially Filled | 6000      | 3500    | 2500       | 0        |                                                                                                                                   |
| 12   |                      | Execution(Z)                        | Trade            | Partially Filled | 6000      | 5000    | 1000       | 1500     | Execution for 1500                                                                                                                |

March 25, 2003 – DRAFT

63

FIX 4.4 - Volume 4


---

D.2 Chaining


# D.2 Chaining

# D.2.a – One cancel/replace request is issued followed immediately by another – broker processes sequentially

| Time | Message Received     | Message Sent (ClOrdID, OrigClOrdID) | Exec Type | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                                            |
| ---- | -------------------- | ----------------------------------- | --------- | ---------------- | --------- | ------- | ---------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)         |                                     |           |                  | 10000     |         |            |          |                                                                                                                                                    |
| 2    |                      | Execution(X)                        | New       | New              | 10000     | 0       | 10000      | 0        |                                                                                                                                                    |
| 3    |                      | Execution(X)                        | Trade     | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                                                                                 |
| 4    | Replace Request(Y,X) |                                     |           |                  | 8000      |         |            |          | Request decrease in order quantity to 8000, leaving 7000 open                                                                                      |
| 5    | Replace Request(Z,Y) |                                     |           |                  | 7000      |         |            |          | Request decrease in order quantity to 7000, leaving 6000 open. Note OrigClOrdID set to last non rejected ClOrdID i.e. Y (on an ‘optimistic’ basis) |
| 6    |                      | Execution (Y,X)                     | Pending   | Pending          | 10000     | 1000    | 9000       | 0        | Broker processes Replace (Y,X) first                                                                                                               |
| 7    |                      | Execution (Y,X)                     | Replace   | Partially Filled | 8000      | 1000    | 7000       | 0        | Broker processes Replace (Y,X) first                                                                                                               |
| 8    |                      | Execution (Z,Y)                     | Pending   | Pending          | 8000      | 1000    | 7000       | 0        | Broker then processes Replace (Z,Y). Note OrigClOrdID set to last accepted ClOrdID i.e. Y                                                          |
| 9    |                      | Execution (Z,Y)                     | Replace   | Partially Filled | 7000      | 1000    | 6000       | 0        | Broker then processes Replace (Z,Y)                                                                                                                |
| 10   |                      | Execution(Z)                        | Trade     | Filled           | 7000      | 7000    | 0          | 6000     | Execution for 6000                                                                                                                                 |

March 25, 2003 – DRAFT

64

FIX 4.4 - Volume 4



---


# D.2.b – One cancel/replace request is issued followed immediately by another – broker processes pending replaces before replaces

| Time | Message Received     | Message Sent (ClOrdID, OrigClOrdID) | Exec    | OrdStatus        | Order Type | Cum Qty | Leaves Qty | Last Qty | Comment            |                                                                                                                         |      |      |      |   |   |                                                                                                                                                                |
| ---- | -------------------- | ----------------------------------- | ------- | ---------------- | ---------- | ------- | ---------- | -------- | ------------------ | ----------------------------------------------------------------------------------------------------------------------- | ---- | ---- | ---- | - | - | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)         |                                     |         |                  | 10000      |         |            |          |                    |                                                                                                                         |      |      |      |   |   |                                                                                                                                                                |
| 2    |                      | Execution(X)                        | New     | New              | 10000      | 0       | 10000      | 0        |                    |                                                                                                                         |      |      |      |   |   |                                                                                                                                                                |
| 3    |                      | Execution(X)                        | Trade   | Partially Filled | 10000      | 1000    | 9000       | 1000     | Execution for 1000 |                                                                                                                         |      |      |      |   |   |                                                                                                                                                                |
| 4    | Replace Request(Y,X) |                                     |         |                  | 8000       |         |            |          |                    | Request decrease in order quantity to 8000, leaving 7000 open                                                           |      |      |      |   |   |                                                                                                                                                                |
| 5    | Replace Request(Z,Y) |                                     |         |                  | 7000       |         |            |          |                    | Request decrease in order quantity to 7000, leaving 6000 open. Note OrigClOrdID set to last non rejected ClOrdID i.e. Y |      |      |      |   |   |                                                                                                                                                                |
| 6    | Execution (Y,X)      |                                     | Pending | Pending          | 10000      | 1000    | 9000       | 0        |                    | Broker processes Replace (Y,X) first                                                                                    |      |      |      |   |   |                                                                                                                                                                |
| 7    | Execution (Z,X)      |                                     | Pending | Pending          | 8000       | 1000    | 7000       | 0        |                    | Broker then processes Replace (Z,Y). Note OrigClOrdID set to last accepted ClOrdID i.e. X                               |      |      |      |   |   |                                                                                                                                                                |
| 8    |                      | Execution                           |         |                  |            |         |            |          | Replace (Y,X)      | Pending                                                                                                                 | 8000 | 1000 | 7000 | 0 |   | Broker processes Replace (Y,X) first Note OrigClOrdID set to last accepted ClOrdID i.e. X. OrdStatus of Pending Replace takes precedence over Partially Filled |
| 9    |                      | Execution                           |         |                  |            |         |            |          | Replace (Z,Y)      | Partially Filled                                                                                                        | 7000 | 1000 | 6000 | 0 |   | Broker then processes Replace (Z,Y) Note OrigClOrdID set to last accepted ClOrdID i.e. Y                                                                       |
| 10   |                      | Execution(Z)                        | Trade   | Filled           | 7000       | 7000    | 0          | 6000     | Execution for 6000 |                                                                                                                         |      |      |      |   |   |                                                                                                                                                                |

March 25, 2003 – DRAFT

65

FIX 4.4 - Volume 4


---

March 25, 2003 – DRAFT


# D.2.c – One cancel/replace request is issued followed immediately by another – both are rejected

| Time | Message              | Message Sent (ClOrdID, OrigClOrdID) | Exec Type        | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty           | Comment                                                                                                                |
| ---- | -------------------- | ----------------------------------- | ---------------- | ---------------- | --------- | ------- | ---------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)         |                                     |                  |                  | 10000     |         |            |                    |                                                                                                                        |
| 2    | Execution(X)         | New                                 | New              | 10000            | 0         | 10000   | 0          |                    |                                                                                                                        |
| 3    | Execution(X)         | Trade                               | Partially Filled | 10000            | 1000      | 9000    | 1000       | Execution for 1000 |                                                                                                                        |
| 4    | Replace Request(Y,X) |                                     |                  |                  | 8000      |         |            |                    | Request decrease in order quantity to 8000, leaving 7000 open                                                          |
| 5    | Replace Request(Z,Y) |                                     |                  |                  | 7000      |         |            |                    | Request decrease in order quantity to 7000, leaving 6000 open. Note OrigCOrdID set to last non rejected ClOrdID i.e. Y |
| 6    | Execution            | (Y,X)                               | Pending          | Pending          | 10000     | 1000    | 9000       | 0                  | Broker processes Replace (Y,X) first                                                                                   |
| 7    | Cancel               | (Y,X)                               | Reject           | Partially Filled |           |         |            |                    | Broker rejects first replace request Note OrigClOrdID set to last accepted ClOrdID i.e. X                              |
| 8    | Execution            | (Z,X)                               | Pending          | Pending          | 10000     | 1000    | 9000       | 0                  | Broker then processes Replace (Z,Y). Note OrigClOrdID set to last accepted ClOrdID i.e. X                              |
| 9    | Cancel               | (Z,X)                               | Reject           | Partially Filled |           |         |            |                    | Broker then rejects second replace request Note OrigClOrdID set to last accepted ClOrdID i.e. X                        |
| 10   | Execution(X)         | Trade                               | Partially Filled | 10000            | 7000      | 3000    | 6000       | Execution for 6000 |                                                                                                                        |


66                                         FIX 4.4 - Volume 4

---

FIX 4.4 - Volume 4


# D.2.d – One cancel/replace request is issued followed immediately by another – broker rejects the second as order is pending replace

| Time | Message Received (ClOrdID, OrigClOrdID) | Message Sent (ClOrdID, OrigClOrdID) | Exec Type       | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                                                                                                                                        |
| ---- | --------------------------------------- | ----------------------------------- | --------------- | ---------------- | --------- | ------- | ---------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)                            |                                     |                 |                  | 10000     |         | 10000      | 0        |                                                                                                                                                                                                                                                |
| 2    |                                         | Execution(X)                        | New             | New              | 10000     | 0       | 10000      | 0        |                                                                                                                                                                                                                                                |
| 3    |                                         | Execution(X)                        | Trade           | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                                                                                                                                                                             |
| 4    | Replace Request(Y,X)                    |                                     |                 |                  | 8000      |         |            |          | Request decrease in order quantity to 8000, leaving 7000 open                                                                                                                                                                                  |
| 5    | Replace Request(Z,Y)                    |                                     |                 |                  | 7000      |         |            |          | Request decrease in order quantity to 7000, leaving 6000 open Note OrigCOrdID set to last non rejected ClOrdID i.e. Y                                                                                                                          |
| 6    |                                         | Execution (Y,X)                     | Pending Replace | Pending          | 10000     | 1000    | 9000       | 0        |                                                                                                                                                                                                                                                |
| 7    |                                         | Cancel Reject (Z,X)                 |                 | Pending          |           |         |            |          | Rejected because broker does not support processing of order cancel replace request whilst order is pending cancel. CxlRejReason = ‘Order already in pending cancel or pending replace status’ OrigClOrdID set to last accepted ClOrdID i.e. X |
| 8    |                                         | Execution (Y,X)                     | Replace         | Partially Filled | 8000      | 1000    | 7000       | 0        |                                                                                                                                                                                                                                                |
| 9    |                                         | Execution (Y)                       | Trade           | Partially Filled | 8000      | 3000    | 5000       | 2000     | Execution for 2000                                                                                                                                                                                                                             |

This matrix illustrates the case where the broker/order receiver does not support multiple outstanding order cancel or order cancel/replace requests

March 25, 2003 – DRAFT


67

---

# E      Unsolicited/Reinstatement

# E.1.a – Telephoned order

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent | Exec Type | OrdStatus | Order Qty        | Cum Qty | Leaves Qty | Last Qty | Comment                          |
| ------------- | ------------------------------ | ------------ | --------- | --------- | ---------------- | ------- | ---------- | -------- | -------------------------------- |
| 1             |                                |              | Execution | New       | 10000            | 0       | 0          | 0        | Order for 10000 phoned to broker |
| 2             |                                |              | Execution | Trade     | Partially Filled | 10000   | 2000       | 8000     | Execution of 2000                |
| 3             |                                |              | Execution | Trade     | Partially Filled | 10000   | 3000       | 7000     | Execution of 1000                |
| 4             |                                |              | Execution | Trade     | Filled           | 10000   | 10000      | 0        | Execution of 7000                |

# E.1.b – Unsolicited cancel of a part-filled order

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent | Exec Type | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                               |
| ------------- | ------------------------------ | ------------ | --------- | ---------------- | --------- | ------- | ---------- | -------- | ------------------------------------------------------------------------------------- |
| 1             | New Order(X)                   |              |           |                  | 10000     |         |            |          |                                                                                       |
| 2             |                                | Execution(X) | Rejected  | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                             |
| 2             |                                | Execution(X) | New       | New              | 10000     | 0       | 10000      | 0        |                                                                                       |
| 3             |                                | Execution(X) | Trade     | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                    |
| 4             |                                |              |           |                  |           |         |            |          | Broker verbally agrees to cancel order                                                |
| 5             |                                | Execution(X) | Canceled  | Canceled         | 10000     | 1000    | 0          | 0        | Broker signifies that order has been canceled - ExecRestatementReason = Verbal change |

This scenario might occur if the buy-side has not implemented order cancel requests or alternatively there is an electronic communication problem at the point that the buy-side wishes to send a cancel request.

March 25, 2003 – DRAFT

68

FIX 4.4 - Volume 4


---


# E.1.c – Unsolicited replacement of a part-filled order

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent | Exec Type | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                             |
| ------------- | ------------------------------ | ------------ | --------- | ---------------- | --------- | ------- | ---------- | -------- | ----------------------------------------------------------------------------------- |
| 1             | New Order(X)                   |              |           |                  | 10000     |         |            |          |                                                                                     |
| 2             |                                | Execution(X) | Rejected  | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                           |
| 2             |                                | Execution(X) | New       | New              | 10000     | 0       | 10000      | 0        |                                                                                     |
| 3             |                                |              |           |                  |           |         |            |          | Broker verbally agrees to increase order quantity to 11000                          |
| 4             |                                | Execution(X) | Restated  | New              | 11000     | 0       | 11000      | 0        | Broker signifies that order has been replaced ExecRestatementReason = Verbal        |
| 5             |                                | Execution(X) | Trade     | Partially Filled | 11000     | 1000    | 10000      | 1000     | Execution for 1000                                                                  |
| 6             |                                |              |           |                  |           |         |            |          | Broker verbally agrees to increase order quantity to 12000                          |
| 7             |                                | Execution(X) | Restated  | Partially Filled | 12000     | 1000    | 11000      | 0        | Broker signifies that order has been replaced ExecRestatementReason = Verbal change |

This scenario might occur if the buy-side has not implemented order cancel/replace requests or alternatively there is an electronic communication problem at the point that the buy-side wishes to send a cancel replace request.

# E.1.d - Unsolicited reduction of order quantity by sell side (e.g. for US ECNs to communicate Nasdaq SelectNet declines)

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent | Exec Type | OrdStatus | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                   |
| ------------- | ------------------------------ | ------------ | --------- | --------- | --------- | ------- | ---------- | -------- | --------------------------------------------------------- |
| 1             | New Order(X)                   |              |           |           | 10000     |         |            |          |                                                           |
| 2             |                                | Execution(X) | Rejected  | Rejected  | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN) |
| 2             |                                | Execution(X) | New       | New       | 10000     | 0       | 10000      | 0        |                                                           |
| 3             |                                | Execution(X) | Restated  | New       | 9000      | 0       | 9000       | 0        | ExecRestatementReason="Partial Decline of OrderQty"       |
| 4             |                                | Execution(X) | Trade     | Filled    | 9000      | 9000    | 0          | 9000     |                                                           |

March 25, 2003 – DRAFT

69

FIX 4.4 - Volume 4


---

# E.1.e - Unsolicited cancel of a ‘cancel if not best’ order

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent | Exec Type | OrdStatus | Order Qty | Price | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                          |
| ------------- | ------------------------------ | ------------ | --------- | --------- | --------- | ----- | ------- | ---------- | -------- | -------------------------------------------------------------------------------------------------------------------------------- |
| 1             | New Order(X)                   |              |           |           | 10000     | 56    |         |            |          | ExecInst = Z ( Cancel if Not Best)                                                                                               |
| 2             |                                | Execution(X) | Rejected  | Rejected  | 10000     | 56    | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN) (e.g. if the order book is at 56.1-57.1 prior to this order)           |
| 2             |                                | Execution(X) | New       | New       | 10000     | 56    | 0       | 10000      | 0        | Order accepted as order book was 55.9-56.9 prior to this order. Order book is now 56.0-56.9                                      |
| 3             |                                | Execution(X) | Canceled  | Canceled  | 10000     | 56    | 0       | 0          | 0        | Order book moves to 56.1-57.0. Order is no longer best bid/offer so is canceled with ExecRestatementReason =”Canceled, Not Best” |

# F Order Reject

# F.1.a– Order rejected due to duplicate ClOrdID

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent | Exec Type | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                                               |                                         |
| ------------- | ------------------------------ | ------------ | --------- | ---------------- | --------- | ------- | ---------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------- |
| 1             | New Order(X)                   |              |           |                  | 10000     |         |            |          |                                                                                                                                                       |                                         |
| 2             |                                | Execution(X) | New       | New              | 10000     | 0       | 10000      | 0        |                                                                                                                                                       |                                         |
| 3             |                                | Execution(X) | Trade     | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                                                                                                    |                                         |
| 4             | New Order(X)                   |              |           |                  | 15000     |         |            |          |                                                                                                                                                       | Order submitted with the same order id. |
| 5             |                                | Execution(X) | Rejected  | Partially Filled | 10000     | 1000    | 9000       | 0        | OrdRejReason = duplicate order. Note combining a reject of the order for 15000 with a status on the first order for 10000 (which is partially filled) |                                         |


---

FIX 4.4 - Volume 4


# F.1.b - Poss resend and duplicate ClOrdID

| Time | Message Received | Message Sent (ClOrdID, OrigClOrdID) | Exec Type    | OrdStatus | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                                    |
| ---- | ---------------- | ----------------------------------- | ------------ | --------- | --------- | ------- | ---------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| 1    | New Order(X)     |                                     |              |           | 10000     |         |            |          |                                                                                                                                            |
| 2    |                  | Execution(X)                        | New          | New       | 10000     | 0       | 10000      |          |                                                                                                                                            |
| 3    | New Order(X)     |                                     |              |           | 10000     |         |            |          | PossResend=Y                                                                                                                               |
| 4    |                  | Execution(X)                        | Order Status | New       | 10000     | 0       | 10000      |          | Because order X has already been received, confirm back the current state of the order. Last Qty not required when ExecType = Order Status |
| 5    | New Order(X)     |                                     |              |           | 20000     |         |            |          | PossResend=N or not set                                                                                                                    |
| 6    |                  | Execution(X)                        | Rejected     | New       | 10000     | 0       | 10000      |          | OrdRejReason = duplicate order. Note combining a reject of the second order for 20000 with a status on the first order for 10000.          |
| 7    | New Order(Y)     |                                     |              |           | 15000     |         |            |          | PossResend=Y                                                                                                                               |
| 8    |                  | Execution(Y)                        | New          | New       | 15000     | 0       | 15000      | 0        | Because order Y has not been received before, confirm back as a new order.                                                                 |

# F.1.c - Order rejected because the order has already been verbally submitted

| Time | Message Received | Message Sent (ClOrdID, OrigClOrdID) | Exec Type | OrdStatus | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                                      |
| ---- | ---------------- | ----------------------------------- | --------- | --------- | --------- | ------- | ---------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)     |                                     |           |           | 10000     |         |            |          | Order for 10000 sent electronically                                                                                                          |
| 2    |                  |                                     |           |           |           |         |            |          | Order passed verbally as there is communication problem and order does not arrive. The verbally passed order starts getting executed         |
| 3    |                  | Execution(X)                        | Rejected  | Rejected  | 10000     | 0       | 0          | 0        | Order finally arrives and is detected as a duplicate of a verbal order and is therefore rejected. OrdRejReason = duplicate of a verbal order |

Note that the sell-side may employ a number of mechanisms to detect that the electronic order is potentially a duplicate of a verbally passed order, e.g. :

- Checking the possdup flag on the order message header
- Checking the incoming order details against other orders from the same client (e.g. side, quantity)
- Looking at the transact time on the order as a guide to ‘staleness’

March 25, 2003 – DRAFT


71

---

# G Status

# G.1.a - Order status request rejected for unknown order

| Time | Message            | Message Sent | Exec  | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                      |
| ---- | ------------------ | ------------ | ----- | ---------------- | --------- | ------- | ---------- | -------- | ---------------------------------------------------------------------------- |
| 1    | New Order(X)       |              |       |                  | 10000     |         |            |          |                                                                              |
| 2    |                    | Execution(X) | New   | New              | 10000     | 0       | 10000      | 0        |                                                                              |
| 3    |                    | Execution(X) | Trade | Partially Filled | 10000     | 1000    | 9000       | 1000     | Execution for 1000                                                           |
| 4    | Status Request (Y) |              |       |                  |           |         |            |          |                                                                              |
| 5    |                    | Execution(Y) | Order | Rejected         | 0         | 0       | 0          |          | OrdRejReason = unknown order LastQty not required when ExecType=Order Status |

# G.1.b – Transmitting a CMS-style “Nothing Done” in response to a status request

| Time | Message           | Message Sent | Exec     | OrdStatus | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                   |
| ---- | ----------------- | ------------ | -------- | --------- | --------- | ------- | ---------- | -------- | --------------------------------------------------------- |
| 1    | New Order(X)      |              |          |           | 10000     |         |            |          |                                                           |
| 2    |                   | Execution(X) | Rejected | Rejected  | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN) |
| 2    |                   | Execution(X) | New      | New       | 10000     | 0       | 10000      | 0        |                                                           |
| 3    | Status Request(X) |              |          |           |           |         |            |          |                                                           |
| 4    |                   | Execution(X) | Order    | New       | 10000     | 0       | 10000      | 0        | Text=”Nothing Done”                                       |


---


# G.1.c - Order sent, immediately followed by a status request. Subsequent status requests sent during life of order

| Time | Message              | Message Sent    | Exec             | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                         |
| ---- | -------------------- | --------------- | ---------------- | ---------------- | --------- | ------- | ---------- | -------- | --------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X)         |                 |                  |                  | 10000     |         |            |          |                                                                                                                 |
| 2    | Status Request (X)   |                 |                  |                  |           |         |            |          |                                                                                                                 |
| 3    |                      | Execution(X)    | Order Status     | Pending New      | 10000     | 0       | 10000      |          | Sent in response to status request. LastQty not required when ExecType=Order Status                             |
| 4    |                      | Execution(X)    | Rejected         | Rejected         | 10000     | 0       | 0          | 0        | If order is rejected                                                                                            |
| 4    |                      | Execution(X)    | New              | New              | 10000     | 0       | 10000      | 0        |                                                                                                                 |
| 5    | Status Request (X)   |                 |                  |                  |           |         |            |          |                                                                                                                 |
| 6    |                      | Execution(X)    | Order Status     | New              | 10000     | 0       | 10000      |          | Sent in response to status request.                                                                             |
| 7    |                      | Execution(X)    | Trade            | Partially Filled | 10000     | 2000    | 8000       | 2000     | Execution for 2000                                                                                              |
| 8    | Status Request (X)   |                 |                  |                  |           |         |            |          |                                                                                                                 |
| 9    |                      | Execution(X)    | Order Status     | Partially Filled | 10000     | 2000    | 8000       |          | Sent in response to status request                                                                              |
| 10   |                      | Execution(X)    | Trade            | Filled           | 10000     | 10000   | 0          | 8000     | Execution for 8000                                                                                              |
| 11   | Status Request (X)   |                 |                  |                  |           |         |            |          |                                                                                                                 |
| 12   |                      | Execution(X)    | Order Status     | Filled           | 10000     | 10000   | 0          |          | Sent in response to status request                                                                              |
| 13   | Replace Request(Y,X) |                 |                  |                  | 12000     |         |            |          | Request to increase order qty                                                                                   |
| 14   | Execution (Y,X)      | Pending Replace | Pending          |                  | 10000     | 10000   | 0          | 0        |                                                                                                                 |
| 15   | Execution (Y,X)      |                 | Replace          | Partially Filled | 12000     | 10000   | 2000       | 0        |                                                                                                                 |
| 16   | Status Request (X)   |                 |                  |                  |           |         |            |          |                                                                                                                 |
| 17   | Execution (Y,X)      | Order Status    | Partially Filled |                  | 12000     | 10000   | 2000       |          | Sent in response to status request. Note reference to X to allow tie back of execution report to status request |
| 18   | Status Request       |                 |                  |                  |           |         |            |          |                                                                                                                 |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---


# Execution Order

# Partially Filled

| 12000 | 10000 | 2000 | Sent in response to status request |
| ----- | ----- | ---- | ---------------------------------- |

# H.1.a - GTC order partially filled, restated (renewed) and partially filled the following day

| Time Received | Message (ClOrdID, OrigClOrdID) | Exec Type | Status       | Order Qty | Cum Qty | Leaves Qty | Last Order Qty | Day  | Comment                                                                                             |
| ------------- | ------------------------------ | --------- | ------------ | --------- | ------- | ---------- | -------------- | ---- | --------------------------------------------------------------------------------------------------- |
| Day 1,1       | New Order(X)                   |           | New          | 10000     | 0       | 10000      | 0              |      |                                                                                                     |
| Day 1,2       | Execution(X)                   |           | Trade        | 10000     | 2000    | 8000       | 2000           |      | Execution for 2000                                                                                  |
| Day 1,3       | Execution(X)                   |           | Done for Day | 10000     | 2000    | 8000       | 0              |      | Optional at end of trading day                                                                      |
| Day 1,4       | Execution(X)                   |           | Restated     | 10000     | 2000    | 8000       | 0              | 8000 | ExecRestatementReason = GTC renewal/restatement (no change) – optionally sent the following morning |
| Day 2,1       | Execution(X)                   |           | Trade        | 10000     | 3000    | 7000       | 1000           | 8000 | Execution for 1000                                                                                  |
| Day 2,2       | Execution(X)                   |           |              |           |         |            |                |      | Filled                                                                                              |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---


# H.1.b - GTC order with partial fill, a 2:1 stock split then a partial fill and fill the following day

| Time    | Message      | Message Sent | Exec Type | Ord Status       | Order Qty | Cum Qty | Leaves Qty | Last Qty | Day Cum Qty | Day Leaves Qty | Comment                                    |
| ------- | ------------ | ------------ | --------- | ---------------- | --------- | ------- | ---------- | -------- | ----------- | -------------- | ------------------------------------------ |
| Day 1,1 | New Order(X) |              |           | New              | 10000     | 0       | 10000      | 0        |             |                |                                            |
| Day 1,2 | Execution(X) |              | Trade     | Partially Filled | 10000     | 2000    | 8000       | 2000     |             |                | Execution for 2000 @ 50                    |
| Day 1,3 | Execution(X) |              | Done for  | Done for         | 10000     | 2000    | 8000       | 0        |             |                | Optional at end of trading day             |
| Day 1,4 | Execution(X) |              | Restated  | Partially Filled | 20000     | 4000    | 16000      | 0        | 16000       | 0              | Sent the following morning after the split |
| Day 2,1 | Execution(X) |              | Trade     | Partially Filled | 20000     | 9000    | 11000      | 5000     | 16000       | 5000           | Execution for 5000                         |
| Day 2,2 | Execution(X) |              | Trade     | Filled           | 20000     | 20000   | 0          | 11000    | 16000       | 16000          | Execution for 11000                        |

March 25, 2003 – DRAFT

75

FIX 4.4 - Volume 4


---


# H.1.c - GTC order partially filled, restated(renewed) and canceled the following day

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent | Exec Type | Order Status     | Ord Qty | Cum Qty | Leaves Qty | Last Qty | Day Ord Qty | Day Cum Qty | Comment                                                                                             |
| ------------- | ------------------------------ | ------------ | --------- | ---------------- | ------- | ------- | ---------- | -------- | ----------- | ----------- | --------------------------------------------------------------------------------------------------- |
| Day 1,1       | New Order(X)                   |              | New       | New              | 10000   | 0       | 10000      | 0        |             |             |                                                                                                     |
| Day 1,2       | Execution(X)                   |              | Trade     | Partially Filled | 10000   | 2000    | 8000       | 2000     |             |             | Execution for 2000                                                                                  |
| Day 1,3       | Execution(X)                   |              | Done for  | Done for         | 10000   | 2000    | 8000       | 0        |             |             | Optional at end of trading day                                                                      |
| Day 1,4       | Execution(X)                   |              | Restated  | Partially Filled | 10000   | 2000    | 8000       | 0        | 8000        | 0           | ExecRestatementReason = GTC renewal/restatement (no change) – optionally sent the following morning |
| Day 2,1       | Cancel Request (Y,X)           |              |           |                  | 10000   |         |            |          |             |             |                                                                                                     |
| Day 2,2       | Cancel                         | Reject       |           | Partially Filled |         |         |            |          |             |             | If rejected by salesperson                                                                          |
| Day 2,3       | Execution (Y,X)                |              | Pending   | Pending          | 10000   | 2000    | 8000       | 0        | 8000        | 0           |                                                                                                     |
| Day 2,4       | Cancel                         | Reject       |           | Partially Filled |         |         |            |          |             |             | If rejected by trader/exchange                                                                      |
| Day 2,4       | Execution (Y,X)                |              | Canceled  | Canceled         | 10000   | 2000    | 0          | 0        | 8000        | 0           |                                                                                                     |

March 25, 2003 – DRAFT

76

FIX 4.4 - Volume 4


---

March 25, 2003 – DRAFT


# H.1.d - GTC order partially filled, restated(renewed) followed by replace request to increase quantity

| Time    | Message              | Message Sent | Exec Type | Ord Status       | Order Qty | Cum Qty | Leaves Qty | Last Qty | Day Ord Qty | Day Cum Qty | Comment                                                                                             |
| ------- | -------------------- | ------------ | --------- | ---------------- | --------- | ------- | ---------- | -------- | ----------- | ----------- | --------------------------------------------------------------------------------------------------- |
| Day 1,1 | New Order(X)         |              |           | New              | 10000     | 0       | 10000      | 0        |             |             |                                                                                                     |
| Day 1,2 | Execution(X)         |              | Trade     | Partially Filled | 10000     | 2000    | 8000       | 2000     |             |             | Execution for 2000                                                                                  |
| Day 1,3 | Execution(X)         |              | Done for  | Done for         | 10000     | 2000    | 8000       | 0        |             |             | Optional at end of trading day                                                                      |
| Day 1,4 | Execution(X)         |              | Restated  | Partially Filled | 10000     | 2000    | 8000       | 0        | 8000        | 0           | ExecRestatementReason = GTC renewal/restatement (no change) – optionally sent the following morning |
| Day 2,1 | Replace Request(Y,X) |              |           |                  | 15000     |         |            |          |             |             | Increasing qty                                                                                      |
| Day 2,2 | Cancel               | Reject       |           | Partially Filled |           |         |            |          |             |             | If rejected by salesperson                                                                          |
| Day 2,3 | Execution            | (Y,X)        | Pending   | Pending          | 10000     | 2000    | 8000       | 0        | 8000        | 0           |                                                                                                     |
| Day 2,4 | Execution (X)        |              | Trade     | Pending          | 10000     | 3000    | 7000       | 1000     | 8000        | 1000        | Execution for 1000 Replace                                                                          |
| Day 2,5 | Cancel               | Reject       |           | Partially Filled |           |         |            |          |             |             | If rejected by trader/exchange                                                                      |
| Day 2,5 | Execution            | (Y,X)        | Replace   | Partially Filled | 15000     | 3000    | 12000      | 0        | 13000       | 1000        |                                                                                                     |


FIX 4.4 - Volume 4

---

# TimeInForce

# I.1.a – Fill or Kill order cannot be filled

| Time | Message      | Message Sent | Exec     | OrdStatus | Order Type | Cum Qty | Leaves Qty | Last Qty | Comment                               |                                                           |
| ---- | ------------ | ------------ | -------- | --------- | ---------- | ------- | ---------- | -------- | ------------------------------------- | --------------------------------------------------------- |
| 1    | New Order(X) |              |          |           | 10000      |         |            |          | Order is FOK                          |                                                           |
| 2    |              | Execution(X) | Rejected | Rejected  | 10000      | 0       | 0          | 0        |                                       | If order is rejected by sell-side (broker, exchange, ECN) |
| 2    |              | Execution(X) | New      | New       | 10000      | 0       | 10000      | 0        |                                       |                                                           |
| 3    |              | Execution(X) | Canceled | Canceled  | 10000      | 0       | 0          | 0        | If order cannot be immediately filled |                                                           |

# I.1.b – Immediate or Cancel order that cannot be immediately hit

| Time | Message      | Message Sent | Exec     | OrdStatus        | Order Type | Cum Qty | Leaves Qty | Last Qty | Comment                                                   |   |
| ---- | ------------ | ------------ | -------- | ---------------- | ---------- | ------- | ---------- | -------- | --------------------------------------------------------- | - |
| 1    | New Order(X) |              |          |                  | 10000      |         |            |          | Order is IOC                                              |   |
| 2    |              | Execution(X) | Rejected | Rejected         | 10000      | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN) |   |
| 2    |              | Execution(X) | New      | New              | 10000      | 0       | 10000      | 0        |                                                           |   |
| 3    |              | Execution(X) | Trade    | Partially Filled | 10000      | 1000    | 9000       | 1000     | Execution for 1000                                        |   |
| 4    |              | Execution(X) | Canceled | Canceled         | 10000      | 1000    | 0          | 0        | If order cannot be immediately hit                        |   |

March 25, 2003 – DRAFT

78

FIX 4.4 - Volume 4


---


# Execution Cancels/Corrects

# J.1.a – Filled order, followed by correction and cancellation of executions

| Time | Message               | Message Sent | Exec     | OrdStatus        | Order Qty | Cum Qty | Leaves Qty | AvgPx | Last Qty | Last Px | ExecId | Comment                                       |                                                           |
| ---- | --------------------- | ------------ | -------- | ---------------- | --------- | ------- | ---------- | ----- | -------- | ------- | ------ | --------------------------------------------- | --------------------------------------------------------- |
| 1    | New Order(X)          |              |          |                  | 10000     |         |            |       |          |         |        |                                               |                                                           |
| 2    |                       | Execution(X) | Rejected | Rejected         | 10000     | 0       | 0          |       |          |         | A      |                                               | If order is rejected by sell-side (broker, exchange, ECN) |
| 2    |                       | Execution(X) | New      | New              | 10000     | 0       | 10000      | 0     |          |         | B      |                                               |                                                           |
| 3    |                       | Execution(X) | Trade    | Partially Filled | 10000     | 1000    | 9000       | 100   | 1000     | 100     | C      | Execution for 1000 @ 100                      |                                                           |
| 4    |                       | Execution(X) | Trade    | Filled           | 10000     | 10000   | 0          | 109   | 9000     | 110     | D      | Execution for 9000 @ 110                      |                                                           |
| 5    |                       | Execution(X) | Trade    | Partially Filled | 10000     | 9000    | 1000       | 110   | 0        | 0       | E      | Cancel execution for 1000.                    |                                                           |
| 6    |                       | Execution(X) | Trade    | Partially Filled | 10000     | 9000    | 1000       | 100   | 9000     | 100     | F (D)  | Correct price on execution for 9000 to 100.   |                                                           |
| 7    |                       | Execution(X) | Trade    | Filled           | 10000     | 10000   | 0          | 102   | 1000     | 120     | G      | Execution for 1000 @ 120                      |                                                           |
| 8    |                       | Execution(X) | Trade    | Filled           | 10000     | 10000   | 0          | 120   | 9000     | 120     | H(F)   | Correct price on execution for 9000 to 120    |                                                           |
| 9    | Replace Request (Y,X) |              |          |                  | 12000     |         |            |       |          |         |        | Request to increase order qty                 |                                                           |
| 10   | Execution (Y,X)       |              | Pending  | Pending          | 10000     | 10000   | 0          | 120   | 0        | 0       | I      |                                               |                                                           |
| 11   | Execution (Y,X)       |              | Replace  | Partially Filled | 12000     | 10000   | 2000       | 120   | 0        | 0       | J      |                                               |                                                           |
| 12   |                       | Execution(Y) | Trade    | Partially Filled | 12000     | 10500   | 1500       | 120   | 9500     | 120     | K(H)   | Correct execution of 9000 @ 120 to 9500 @ 120 |                                                           |

March 25, 2003 – DRAFT

79

FIX 4.4 - Volume 4


---


# J.1.b - A canceled order followed by a busted execution and a new execution

| Time | Message             | Message Sent (ClOrdID, OrigClOrdID) | Exec Type        | Ord Status | Qty   | Cum Qty | Leaves Qty | Last Qty | ExecID (ExecRefID)                                                           | Comment |
| ---- | ------------------- | ----------------------------------- | ---------------- | ---------- | ----- | ------- | ---------- | -------- | ---------------------------------------------------------------------------- | ------- |
| 1    | New Order(X)        |                                     |                  |            | 10000 |         |            |          |                                                                              |         |
| 2    | Execution(X)        | New                                 | New              | 10000      | 0     | 10000   | 0          | A        |                                                                              |         |
| 3    | Execution(X)        | Trade                               | Partially Filled | 10000      | 5000  | 5000    | 5000       | B        | LastPx=50                                                                    |         |
| 4    | Cancel Request(Y,X) |                                     |                  |            | 10000 |         |            |          |                                                                              |         |
| 5    | Execution           | Pending Cancel                      | Pending          | 10000      | 5000  | 5000    | 0          | C        |                                                                              |         |
| 6    | Execution           | Canceled                            | Canceled         | 10000      | 5000  | 0       | 0          | D        |                                                                              |         |
| 7    | Execution(X)        | Trade                               | Canceled         | 10000      | 0     | 0       | 0          | E(B)     | Cancel of the execution. ‘Canceled’ order status takes precedence over ‘New’ |         |
| 8    | Execution(X)        | Trade                               | Canceled         | 10000      | 4000  | 0       | 4000       | F        | Fill for 4000. LastPx=51                                                     |         |

March 25, 2003 – DRAFT

80

FIX 4.4 - Volume 4


---


# J.1.c - GTC order partially filled, restated (renewed) and partially filled the following day, with corrections of quantity on both executions

| Time Received | Message (ClOrdID, OrigClOrdID) | Message Sent | Exec Type | Order Status     | Qty   | Cum Qty | Leaves Qty | Last Exec Qty | Day Ord Qty | Day Cum Qty | ExecID | Comment                                                                                             |
| ------------- | ------------------------------ | ------------ | --------- | ---------------- | ----- | ------- | ---------- | ------------- | ----------- | ----------- | ------ | --------------------------------------------------------------------------------------------------- |
| Day 1,1       | New Order(X)                   |              | New       | New              | 10000 | 0       | 10000      | 0             |             |             | A      |                                                                                                     |
| Day 1,2       | Execution(X)                   |              | Trade     | Partially Filled | 10000 | 2000    | 8000       | 2000          |             |             | B      | Execution for 2000                                                                                  |
| Day 1,3       | Execution(X)                   |              | Done for  | Done for         | 10000 | 2000    | 8000       | 0             |             |             | C      | Optional at end of trading day                                                                      |
| Day 1,4       | Execution(X)                   |              | Restated  | Partially Filled | 10000 | 2000    | 8000       | 0             | 8000        | 0           | D      | ExecRestatementReason = GTC renewal/restatement (no change) – optionally sent the following morning |
| Day 2,1       | Execution(X)                   |              | Trade     | Partially Filled | 10000 | 3000    | 7000       | 1000          | 8000        | 1000        | E      | Execution for 1000                                                                                  |
| Day 2,2       | Execution(X)                   |              | Trade     | Partially Filled | 10000 | 2500    | 7500       | 1500          | 8500        | 1000        | F (B)  | Correct quantity on previous day’s execution from 2000 to 1500                                      |
| Day 2,3       | Execution(X)                   |              | Trade     | Partially Filled | 10000 | 2000    | 8000       | 500           | 8500        | 500         | G (E)  | Correct quantity on today’s execution from 1000 to 500                                              |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---

March 25, 2003 – DRAFT


# J.1.d – Part-filled order Done for day followed by trade correction and bust

| Time | Message      | Message Sent | Exec      | Ord    | Order Status | Qty   | Cum Qty | Leaves Qty | Last Qty | ExecID | Comment                                            |
| ---- | ------------ | ------------ | --------- | ------ | ------------ | ----- | ------- | ---------- | -------- | ------ | -------------------------------------------------- |
| 1    | New Order(X) |              |           | New    |              | 10000 | 0       | 10000      | 0        | A      |                                                    |
| 2    | Execution(X) | Trade        | Partially | Filled |              | 10000 | 5000    | 5000       | 5000     | B      | LastPx=50                                          |
| 3    | Execution(X) | Done for     | Done      | for    | 10000        | 5000  | 0       | 0          | C        |        | Done for day message sent                          |
| 4    | Execution(X) | Trade        | Done      | for    | 10000        | 4000  | 0       | 4000       | D (B)    |        | Correct quantity on execution to 4000. LastPx = 50 |
| 5    | Execution(X) | Trade        | Done      | for    | 10000        | 0     | 0       | 0          | E (D)    |        | Done for Day OrdStatus takes precedence            |

# K Trading Halt

# K.1.a – Trading Halt – Reinstate

| Time | Message      | Message Sent | Exec     | OrdStatus | Order | Cum Qty | Leaves Qty | Last Qty |   |   |   | Comment             |                                                           |
| ---- | ------------ | ------------ | -------- | --------- | ----- | ------- | ---------- | -------- | - | - | - | ------------------- | --------------------------------------------------------- |
| 1    | New Order(X) |              |          |           | 10000 |         |            |          |   |   |   |                     | ExecInst set to reinstate on trading halt                 |
| 2    |              | Execution(X) | Rejected | Rejected  | 10000 | 0       | 0          | 0        |   |   |   |                     | If order is rejected by sell-side (broker, exchange, ECN) |
| 2    |              | Execution(X) | New      | New       | 10000 | 0       | 10000      | 0        |   |   |   |                     |                                                           |
| 3    |              |              |          |           |       |         |            |          |   |   |   |                     | Trading halt established                                  |
| 4    |              |              |          |           |       |         |            |          |   |   |   | Trading halt lifted |                                                           |
| 5    |              | Execution(X) | Trade    | Filled    | 10000 | 10000   | 0          | 10000    |   |   |   |                     |                                                           |


82
FIX 4.4 - Volume 4

---


# K.1.b – Trading Halt – Cancel

| Time | Message      | Message Sent | Exec     | OrdStatus | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                  |
| ---- | ------------ | ------------ | -------- | --------- | --------- | ------- | ---------- | -------- | ---------------------------------------------------------------------------------------- |
| 1    | New Order(X) |              |          |           | 10000     |         |            |          | ExecInst set to cancel on trading halt                                                   |
| 2    |              | Execution(X) | Rejected | Rejected  | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                                |
| 2    |              | Execution(X) | New      | New       | 10000     | 0       | 10000      | 0        |                                                                                          |
| 3    |              |              |          |           |           |         |            |          | Trading halt established                                                                 |
| 4    |              | Execution    | Canceled | Canceled  | 10000     | 0       | 0          | 0        | Order canceled due to trading halt. ExecRestatementReason = Canceled due to trading halt |

# L.1.a– Transmitting a guarantee of execution prior to execution

| Time | Message      | Message Sent | Exec     | OrdStatus | Order Qty | Cum Qty | Leaves Qty | Last Qty | Comment                                                                                                                                                                    |
| ---- | ------------ | ------------ | -------- | --------- | --------- | ------- | ---------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1    | New Order(X) |              |          |           | 10000     |         |            |          |                                                                                                                                                                            |
| 2    |              | Execution(X) | Rejected | Rejected  | 10000     | 0       | 0          | 0        | If order is rejected by sell-side (broker, exchange, ECN)                                                                                                                  |
| 2    |              | Execution(X) | New      | New       | 10000     | 0       | 10000      | 0        |                                                                                                                                                                            |
| 3    |              | Execution(X) | Stopped  | Stopped   | 10000     | 0       | 10000      | 1000     | Text=”You are guaranteed to buy 1000 at 50.10”; LastPx=50.10. This is similar to the concept of a ‘protected’ trade. Not actually reporting a trade, so Exectype = Stopped |
| 4    |              | Execution(X) | Trade    | Stopped   | 10000     | 1000    | 9000       | 1000     | LastPx=50                                                                                                                                                                  |


---

# L.1.b- Use of CashOrderQty

| Time | Message      | Message Sent | Exec             | OrdStatus | Order Type | Cash Qty | Cum Qty | Leaves Qty | Last Qty | Last Px | Last Comment                                                                       |
| ---- | ------------ | ------------ | ---------------- | --------- | ---------- | -------- | ------- | ---------- | -------- | ------- | ---------------------------------------------------------------------------------- |
| 1    | New Order(X) |              |                  |           | 10000      |          |         |            |          |         | Currency=EUR. A buy order to invest 10,000 EUR.                                    |
| 2    | Execution(X) | Rejected     | Rejected         |           | 10000      | 0        | 0       |            | 0        |         | If order is rejected                                                               |
| 2    | Execution(X) | New          | New              |           | 500        | 10000    | 0       | 500        |          | 0       | Assuming product has a unit price of 20 EUR at time of order receipt               |
| 3    | Execution(X) | Trade        | Partially Filled |           | 500        | 10000    | 200     | 300        | 200      | 20.1    | Execution of 200 @20.1 (i.e. does not have to be at the ‘conversion price’ of 20\_ |
| 4    | Execution(X) | Trade        | Filled           |           | 500        | 10000    | 500     | 0          | 300      | 20.2    | Execution of 300 @20.2 (i.e. does not have to be at the ‘conversion price’ of 20\_ |

March 25, 2003 – DRAFT

84

FIX 4.4 - Volume 4


---
Order Handling and Instruction Semantics

# London SETS Order Types Matrix

The table below presents the representation of the London Stock Exchange Trading System (SETS) order types in the FIX protocol:

| LSE Order Type                | OrdType |
| ----------------------------- | ------- |
| At Best                       | 1       |
| Fill or Kill - no limit price | 1       |
| Fill or Kill - limit price    | 2       |
| Limit - day                   | 2       |
| Limit - good until            | 2       |
| Execute and Eliminate         | 2       |
| Market Orders - day           | 1       |
| Market Orders - good until    | 1       |

| TimeInForce | ExpireTime     | Price | Comment               |
| ----------- | -------------- | ----- | --------------------- |
| 3           | n/a            | No    |                       |
| 4           | n/a            | No    |                       |
| 4           | n/a            | Yes   |                       |
| n/a, 0      | n/a            | Yes   | SETS Release 3.1 Only |
| 6           | Good Till Date | Yes   |                       |
| 3           | n/a            | Yes   |                       |
| n/a, 0      | N/a            | No    | SETS Release 3.1 Only |
| 6           | Good Till Date | No    | SETS Release 3.1 Only |

# Asia/Pacific Regional Order Handling

The following table identifies how to represent via FIX the commonly used and understood order handling instructions within the Asia/Pacific region.

| Asia/Pacific Dealer Instruction    | OrdType     | ExecInst                                | Other Fields                                                                                                     |
| ---------------------------------- | ----------- | --------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Careful Discretion                 | 1 (Market)  | 4 (Over the Day)                        |                                                                                                                  |
| Market                             | 1 (Market)  | 5 (Held)                                |                                                                                                                  |
| Trader Discretion                  | 1 (Market)  | 1 (Not Held)                            |                                                                                                                  |
| Hong Kong SE – Regular Limit Order | 2 (Limit)   | b = Strict Limit (No Price Improvement) | Price = xxx                                                                                                      |
| Hong Kong SE – Special Limit       | 2 (Limit)   |                                         | TimeInForce = Immediate Or Cancel Price = xxx                                                                    |
| HongKongSE – Enhanced Limit        | 2 (Limit)\* | d = Peg to Limit Price                  | Price = xxx PegType = fixed PegOffsetType = tick PegOffsetValue = -1 (buy) 1(sell) PegOffsetLimitType = or worse |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---

# Japanese Exchange Price Conditions

The following table identifies how to represent via FIX the price conditions implemented by Japanese exchanges.

| Japanese Exchange Price Condition | OrdType                           | ExecInst       | PegOffsetVal | TimeInForce                   |
| --------------------------------- | --------------------------------- | -------------- | ------------ | ----------------------------- |
| Current price limit               | P (Pegged)                        | P (Market Peg) |              |                               |
| Preferred price limit             | P (Pegged)                        | P (Market Peg) | +1 (or –1)   |                               |
| Market with Leftover Limit        | K (Market with Leftover As Limit) |                |              |                               |
| Market Fill with Leftover Kill    | 1 (Market)                        |                |              | 3 (Immediate or Cancel (IOC)) |

# Euronext and Similar Exchange Price Conditions

The following table identifies how to represent via FIX the price conditions implemented by the Euronext and Similar exchanges.

| Euronext and Similar Exchange Price Condition | OrdType                           |
| --------------------------------------------- | --------------------------------- |
| A tout prix (At All Price)                    | 1 (Market)                        |
| Au prix du marche ()                          | J (Market with Leftover As Limit) |

# Handling Instructions (HandlInst) field

The following identifies the meaning and expected usage of the HandlInst (Handling Instructions) field. This field has been required on the New Order messages since the inception of FIX. Usage of this field may vary by market and by broker. Buy side and sell side firms should confirm their mutual understanding of the usage and implementation of HandlInst.

- 1 = Automated execution
- Order is systematically routed to the market place, usually to an exchange or ECN or market maker, for execution. It is expected that no broker intervention is required to accept or forward the order into the market.

Notes:

- Private does not mean broker cannot see buy side order flow. In many markets, the Broker has the legal requirement to monitor customer order flow and be responsible for those orders.


---

# 1. Buy Side Firm Expectations

Buy side firm may be expected to supply the symbology required by the market. Broker may require certain optional fields, such as ExDestination and/or Currency. Implies an immediate reject will be sent if order cannot be forwarded immediately into the market.

# 2. Automated Execution

Broker may stop order from flowing immediately into the market place. This would typically be done, if the broker can cross this order against another order to provide price improvement and/or liquidity.

If Broker does not choose to stop this order, it will automatically flow into the market for execution.

# 3. Manual Order, Best Execution

Order is routed to appropriate sell side broker who then accepts responsibility for the order. This should operate as though the buy side firm called the order into their broker.

Notes:

- Different than “not held”.
- Does not imply “call first” (an ExecInst value).

# Pegged Orders

The following are all pegging ExecInst values used when OrdType=P to specify the type of pegged order represented. Note that these fields cannot be combined; only one may be specified on a pegged order.

| L | Last peg (last sale)                                    |
| - | ------------------------------------------------------- |
| M | Mid-price peg (midprice of inside quote)                |
| O | Opening peg                                             |
| P | Market peg                                              |
| R | Primary peg (primary market - buy at bid/sell at offer) |
| W | Peg to VWAP                                             |
| a | Trailing Stop Peg                                       |
| d | Peg to Limit Price                                      |

A pegged order acts like a limit order, except that the limit price is set relative to another price, such as the last sale price, midpoint price, opening price, bid, offer, or VWAP (Volume Weighted Average Price). A primary peg order is priced relative to the bid if buying, the offer if selling. A market peg order is priced relative to the offer if buying, the bid if selling.

Pegs can be fixed (that is they are calculated when the order is received) or floating, in which case they fluctuate according to movements in the reference price (using the PegMoveType field). The PegOffsetType field can be used to specify whether the desired offset is being expressed as a price, in basis points, in ticks or in price tiers/levels. For example a primary pegged buy order with PegOffsetValue = -0.01, PegMoveType = Fixed (1), and PegOffsetType = Price (0) will have a fixed price equal to the bid less 0.01. The same order with a PegOffsetType = Ticks (2) and a PegOffsetValue = -1 will have a fixed price equal to the bid less one tick. To specify that a buy order is to float on the third best price level set the PegOffsetType = Price Tier/Level (3), ExecInst = Primary Peg.


---

(R), PegMoveType = Floating (0) and PegOffsetValue = -2 (i.e. 2 below the best bid). PegRoundingDirection can be used to specify, in the event that the calculated price is not a valid tick size, whether the price should be rounded aggressively or passively.

When calculating the peg price, the reference price can be obtained from more than one liquidity pool as specified by the PegScope field. For example a PegScope = national excluding local will use a reference price based on all liquidity pools except the one in which the order resides.

Prior FIX specifications defined ExecInst = Fixed Peg to Local best bid or offer at time of order (T). This must now be expressed as a pegged order with ExecInst = Primary Peg (R), PegMoveType = Fixed (1), and PegScope = Local (1).

In the absence of the PegOffsetValue field, or when PegOffsetValue = 0, the price of the pegged order follows the referenced quantity exactly. Note that the PegOffsetValue is always ‘added’ to the reference value. PegMoveType will default to float if not specified.

Some systems allow pegged orders to be specified with a Price field. In such cases the OrdType should be specified as ‘limit’. In this case, the Price field serves to put a limit on how far the pegged value can move. For instance, if the bid for a stock is 50, the offer is 50.10, the order is a primary peg to sell, PegOffsetValue = -0.02, and Price = 45, the order will be priced to sell at the offer + (-0.02) or 50.08. If the offer falls, the order's price will fall such that it is always 0.02 less than the offer. However, once the order's price hits 45 (the limit specified in the Price field) it can fall no further.

A pegged order with ExecInst = a (lower case A), a trailing stop peg, behaves differently. It requires PegOffsetValue, which must be positive when buying and negative when selling. A trailing stop peg represents a stop order whose price can fluctuate relative to the last sale price. Initially, the stop is placed at the last sale price + PegOffsetValue. The stop price will move like a last peg so that the stop price is the last sale price + PegOffsetValue, with one exception: if buying, the fluctuating stop price cannot increase, and if selling, the stop price cannot decrease. For example, a security trades at $10.00, and a trailing stop peg order to sell with PegOffsetValue = -0.10 is placed. The pegged stop price will rest at $9.90. The security rises in price to $10.20, and the stop similarly rises to $10.10. The security price falls to $10.15, but the trailing stop holds its price at $10.10. The security's price keeps falling, and when it reaches $10.10, the stop order is triggered and the security is sold. Trailing stop pegs are incompatible with PegMoveType = Fixed (1).

# Discretionary Pricing

The presence of DiscretionInst on an order indicates that the trader wishes to display one price but will accept trades at another price. For example, a sell order with OrdType = Limit (2), Price=50.00, DiscretionInst = Related to displayed price (0) and DiscretionOffsetValue = -0.25 means that the order should be displayed as an offer for 50.00, but will match any bid >= 49.75. Discretionary pricing can also be used when pegging an order - for example to indicate that a buy order is to be displayed as pegged to the bid minus 0.25, but can be matched by anything &#x3C;= the offer, set OrdType = Pegged (P), ExecInst = Primary Peg (R), PegOffsetValue = -0.25, DiscretionInst = Related to market price (1) and DiscretionOffsetValue = 0. Discretionary prices can be pegged to reference values in the same way as displayed prices (see above).

# “Target Strategy” Orders

The presence of an ExecInst=e (lower case E), work to target strategy, indicates that the order is to be worked to try to achieve the specified target in the TargetStrategy field, typically by slicing the order into the market, either manually or via an algorithm. The start and end times during which the order is to be worked can be communicated using the EffectiveTime and ExpireTime fields respectively or through using TradingSessionIDs.

For example, to indicate that the receiver of the order should try to work the order to achieve the volume weighted average price, set TargetStrategy = VWAP. A Participate order is one where the sender of the order.


---

wants the order to be worked such that the execution profile of the order is the specified percentage of the volume profile in the market. The target participation rate is communicated via the ParticipationRate field. Where appropriate the performance versus the target can be communicated back to the originator of the order by use of the TargetStrategyPerformance field on the Execution Report. The use of this field will depend on the strategy. For a VWAP order this would be the VWAP price for the appropriate time period (taking into account any limit price on the order and excluding/including off order book trades as per the market convention). For Participate orders this field can be used to convey the actual % of volume in the appropriate time period that this executed volume represents. For Minimise Market Impact orders this may be utilised to give an estimate of the order’s market impact in basis points, etc.

More complex parameters can be specified in the TargetStrategyParameters field.

# “Reserve Quantity” Orders

| MaxFloor: | Traditionally used to indicate reserve quantity. To indicate a single level of reserve quantity, MaxFloor should be used.                                                                                  |
| --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| MaxShow:  | Used when two levels of reserve quantities are needed, e.g. one level displayed to the world (MaxFloor) and another displayed to subscribers of their ECN (MaxShow.) I.e. MaxFloor <= MaxShow <= OrderQty. |

One may place an order for 100,000 shares (OrderQty), only want 1000 shares shown to NASDAQ at any one time (MaxFloor), but will allow other subscribers of that ECN to see 5000 shares (MaxShow).

# Time In Force (TIF)

When TIF=0 (DAY) is used in conjunction with TradingSessionID, the Time In Force of DAY means the order is good for the duration of the specified session. This will accommodate trading platforms where the specified trading session may span more than a calendar day (i.e. specified session starts at 8 p.m. and ends next day at 2 p.m.).

# Booking Instructions Specified at Time of Order

| DayBookingInst | BookingUnit    | Effect (end-result)                                                               |
| -------------- | -------------- | --------------------------------------------------------------------------------- |
| "auto"         | "each partial" | Each partial can be booked as soon as the partial is reported to the client       |
| "auto"         | "whole order"  | The order can be booked as soon as it is filled (or part-filled and Done For Day) |
| "auto"         | "aggregation"  | The order can be booked as soon as it is filled (or part-filled and Done For Day) |
| "speak first"  | "each partial" | Do not book after reporting a fill without discussion                             |
| "speak first"  | "whole order"  | Do not book order when filled (or part-filled when Done For Day) but discuss      |
| "speak first"  | "aggregation"  | Do not book the aggregate until verbally agreed                                   |

March 25, 2003 – DRAFT

89

FIX 4.4 - Volume 4


---


# Order Capacity and Order Restrictions (formerly Rule 80A) Usage by Market

The Rule 80A field was deprecated in FIX 4.3 and replaced by the combination of a new Order Capacity field and Order Restrictions field. See Volume 6: "Appendix 6-E - Deprecated Features and Supported Approach".

The term Rule 80A is a very US market-specific term. Other world markets need to convey similar information, however, often a subset of the US values. In addition, the deprecated Rule 80A field's values both "overloaded" the field with various combinations of order capacity and associated order restrictions, and the Rule 80A field as structured (modeled after CMS and SEC Rule 11Ac1-1/4) made it both difficult to understand and difficult to convey the various order capacities. This section documents the market-specific usage of the Order Capacity and Order Restrictions fields.

# United States Listed Equity Markets:

Rule 80A’s values and usage details are documented in SEC Rule 11Ac1-1/4. Note the purpose behind the rule is to restrict prices from rising or falling too fast providing more stability in the market. See Investments by Sharpe, 6ᵗʰ edition p. 50. Indicates the order type upon which exchange Rule 80A is applied.

The following values are valid and applicable when using FIX to communicate with the New York Stock Exchange (NYSE) or other US listed equity exchanges per the SuperDOT Notification document. The values and usage details when used for US trading are documented in SEC Rule 11Ac1-1/4.

With regards to Order Capacity and Order Restrictions field usage in the United States Listed Equity Markets, the following table provides a cross-reference of former Rule 80A values to FIX supported values:

| Rule 80A Value | Order Capacity (528) | Order Restrictions (529) | Side (54)                               |
| -------------- | -------------------- | ------------------------ | --------------------------------------- |
| A              | Agency               |                          | Sell short exempt or Cross short exempt |
| B              | Agency               |                          | 6 or A                                  |
| C              | P                    | Principal                | 1 3 Program Trade Non-Index Arbitrage   |
| D              | P                    | Principal                | 1 2 Program Trade Index Arbitrage       |
| E              | P                    | Principal                | 6 or A                                  |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---

# Short exempt transaction types

| Type | Description                                                                                                                                                               | Account Type           | Code | Notes                                           |
| ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- | ---- | ----------------------------------------------- |
| F    | Short exempt transaction (refer to W type)                                                                                                                                | Agent for Other Member | 6 or | Sell short A exempt or Cross short exempt       |
| H    | Short exempt transaction (refer to I type)                                                                                                                                | Individual             | 6 or | Sell short A exempt or Cross short exempt       |
| I    | Individual Investor, single order                                                                                                                                         | Individual             |      |                                                 |
| J    | Program Order, index arb, for individual customer                                                                                                                         | Individual             | 1 2  | Program Trade Index Arbitrage                   |
| K    | Program Order, non-index arb, for individual customer                                                                                                                     | Individual             | 1 3  | Program Trade Non-Index Arbitrage               |
| L    | Short exempt transaction for member competing market-maker affiliated with the firm clearing the trade (refer to P and O types)                                           | Principal              | 4    | Competing Market A exempt or Cross short exempt |
| M    | Program Order, index arb, for other member                                                                                                                                | Agent for Other Member | 1 2  | Program Trade Index Arbitrage                   |
| N    | Program Order, non-index arb, for other member                                                                                                                            | Agent for Other Member | 1 3  | Program Trade Non-Index Arbitrage               |
| O    | Proprietary transactions for competing market-maker that is affiliated with the clearing member (was incorrectly identified in the FIX spec as “Competing dealer trades”) | Principal              | 4    | Competing Market Maker                          |
| P    | Principal                                                                                                                                                                 | Principal              |      |                                                 |
| R    | Transactions for the account of a non-member competing market maker (was incorrectly identified in the FIX spec as “Competing dealer trades”)                             | Agency                 | 4    | Competing Market Maker                          |
| S    | Specialist trades                                                                                                                                                         | Principal              | 5    | Acting as Market Maker or                       |


---

# Specialist in the security

| T | Transactions for the account of an unaffiliated member’s competing market maker (was incorrectly identified in the FIX spec as “Competing dealer trades”) | W | Agent for | 5 | Acting as Market Maker or Specialist in the security              |
| - | --------------------------------------------------------------------------------------------------------------------------------------------------------- | - | --------- | - | ----------------------------------------------------------------- |
| U | Program Order, index arb, for other agency                                                                                                                | A | Agency    | 1 | Program Trade Index Arbitrage                                     |
| W | All other orders as agent for other member                                                                                                                | W | Agent for |   | Other Member                                                      |
| X | Short exempt transaction for member competing market-maker not affiliated with the firm clearing the trade (refer to W and T types)                       | W | Agent for | 4 | Competing Market Maker or Sell short exempt or Cross short exempt |
| Y | Program Order, non-index arb, for other agency                                                                                                            | A | Agency    | 1 | Program Trade Non-Index Arbitrage                                 |
| Z | Short exempt transaction for non-member competing market-maker (refer to A and R types)                                                                   | A | Agency    | 4 | Competing Market Maker or Sell short exempt or Cross short exempt |

# Japanese Equity Markets:

OrderCapacity is used to specify whether order is Agency or Principal.

Valid values:

- A = Agency single order
- P = Principal

# Other Markets:

All or a subset of the OrderCapacity and OrderRestrictions field values defined in the field reference may be applicable for other markets. Future markets will be included in this section as they are defined and brought forward to the FIX Technical Committee.

March 25, 2003 – DRAFT 93 FIX 4.4 - Volume 4

---

# Example Usage of PartyRole="Investor ID"

Two fields, PartyID and PartyIDSource, facilitate the passing of exchange required ID's when PartyRole="Investor ID". At present, regulatory requirements require the exchange’s Investor ID be provided when orders are placed in Taiwan, China (Shenzhen and Shanghai), and Korea. At present, India, Malaysia and Poland have regulatory requirements requiring the exchange’s Investor ID be provided post-trade.

When placing an order on behalf of multiple distinct Investor ID values, then multiple orders will be sent at the same time with each representing a single Investor ID. For example, three funds/sub-accounts in Taiwan would result in three Investor ID values and thus would also result in three orders.

Note that the Investor ID value is not the same as the customer’s AllocAccount field nor is there necessarily a one-to-one mapping between AllocAccount and Investor ID. In addition, in Korea one Investor ID value may represent multiple accounts. Thus, account pre-trade allocation (See Volume 5: “Example Usage of Allocations”) can still take place in addition to PartyRole="Investor ID", PartyID, and PartyIDSource usage.

# Format of the Party ID field (PartyRole="Investor ID")

| PartyIDSource                                      | Format of PartyID (Investor ID) value |
| -------------------------------------------------- | ------------------------------------- |
| Korean Investor ID                                 | Single digit to six digits            |
| Taiwanese Qualified Foreign Investor ID QFII / FID | Eight digits                          |
| Taiwanese Trading Account                          | Seven digits                          |
| Malaysian Central Depository (MCD) number          | Fifteen digits                        |
| Chinese B Share (Shenzhen and Shanghai)            | Nine digits                           |

Note: All Investor ID values above should be provided in PartyID as numeric only (e.g. exclude alpha-numeric characters such as dashes).

# Example Representations of Orders

| Symbol   | Quantity | Side | OrdType | PartyIDSource | PartyID (Investor ID) | Comments                                                                                      |
| -------- | -------- | ---- | ------- | ------------- | --------------------- | --------------------------------------------------------------------------------------------- |
| 00660.KS | 1000     | Buy  | Market  | 1             | 3452                  | Korean ID provided                                                                            |
| 00660.KS | 3000     | Buy  | Market  | 1             | 232                   | Different Korean ID provided                                                                  |
| 2330.TW  | 3000     | Sell | Market  | 2             | 90567878              | QFII/FID given and Sell-side derives Trading Account based on QFII/FID and Buy-side Client ID |
| 2330.TW  | 2000     | Sell | Limit   | 3             | 9901234               | Trading Account given                                                                         |
| STAR.KL  | 1000     | Sell | Market  | 4             | 456789562467          | MCD given for T+0, pre- or post-trade                                                         |

March 25, 2003 – DRAFT 94 FIX 4.4 - Volume 4


---

KOREA

# Buy 10,000 Hyundai Elec for 3 funds/sub-accounts sharing same ID

| Symbol   | Quantity | Side | OrdType | PartyIDSourcePartyID (Investor ID) |      |
| -------- | -------- | ---- | ------- | ---------------------------------- | ---- |
| 00660.KS | 10,000   | Buy  | Market  | 1                                  | 3452 |

# Buy 10,000 Hyundai Elec for 3 funds/sub-accounts sharing 2 ID’s (sent as 2 orders)

| Symbol   | Quantity | Side | OrdType | PartyIDSourcePartyID (Investor ID) |      | AllocAcct | AllocQty |
| -------- | -------- | ---- | ------- | ---------------------------------- | ---- | --------- | -------- |
| 00660.KS | 4000     | Buy  | Market  | 1                                  | 3452 | B56-78    | 4000     |
| 00660.KS | 6000     | Buy  | Market  | 1                                  | 56   | B56-48    | 2000     |
|          |          |      |         |                                    |      | C24-67    | 4000     |

Note: AllocAccount and AllocQty are optional and are not a substitute for PartyID (Investor ID) value (nor used to lookup PartyID (Investor ID)).

<page_header>
TAIWAN
</page_header>
# Buy 12,000 TSMC for 3 funds/sub-accounts for 4000 each (sent as 3 orders)

| Symbol  | Quantity | Side | OrdType | PartyIDSourcePartyID (Investor ID) |         |   |
| ------- | -------- | ---- | ------- | ---------------------------------- | ------- | - |
| 2330.TW | 4000     | Buy  | Market  | 3                                  | 9903327 |   |
| 2330.TW | 4000     | Buy  | Market  | 3                                  | 9925562 |   |
| 2330.TW | 4000     | Buy  | Market  | 3                                  | 9903562 |   |

# Additional Notes:

- Any change to the PartyID (Investor ID) post submission must be made through the allocation message – you cannot amend PartyID (Investor ID).
- If PartyIDSource and PartyID (Investor ID) provided are not valid for PartyRole="Investor ID", the sell-side will send an Execution Report with OrdRejReason of “Invalid Investor ID”.
- These fields are not to be used to determine the routing of an order to an Exchange (value of PartyID is not a substitute for ExDestination).


March 25, 2003 – DRAFT FIX 4.4 - Volume 4

---
CATEGORY: CROSS ORDERS

# Background

FIX provides support for a cross order using Side[54]=Cross on a New Order Single Message. For many markets the New Order – Single does not provide enough information about the counterparties of a trade to meet regulatory and post-trade requirements. Markets that find the use of a New Order – Single Message with Side[54]=Cross adequate for cross trading – can continue to use this implementation. When additional information regarding the counterparty to the cross trade is required – the Cross Order message should be used. The Japanese Exchange Working Group proposed the creation of a Cross Order message that would elaborate both counterparties involved in the cross for a security. Companion Cross Order Cancel Replace Requests and Cross Order Cancel Requests were also proposed.

# Prioritization of a side of a cross order

Some markets permit one side or the other of the cross order to be prioritized for execution. A new field, CrossPrioritization[550] indicates which side of the cross order will be prioritized for execution. The definition of prioritization is left to the market. In some markets the prioritized side will be guaranteed execution. In other markets, prioritization means that the prioritized side will be applied to the market first.

# Classification of cross trades

Four types of cross trades have been identified.

1. Cross Trade that is executed completely or not. Both sides are treated in the same manner. This is equivalent to Fill or Kill type behavior, where the cross order meets the crossing criteria – within the market and is executed or it is rejected.
2. Cross Trade that is executed partially and the rest is canceled. One side is fully executed, the other side is partially executed with the remainder being canceled. This is equivalent to an Immediate or Cancel on the other side. Note: The CrossPrioritization[550] field is used to indicate which side should fully execute in this scenario.
3. Cross trade that is partially executed with the unfilled portions remaining active. One side of the cross is fully executed as denoted with the CrossPrioritization[550] field, but the unfilled portion remains active.
4. Cross trade is executed with existing orders with the same price. In the case other orders exist with the same price, the quantity of the Cross is executed against the existing orders and quotes, the remainder of the cross is executed against the other side of the cross. The two sides potentially have different quantities. The use of CrossPrioritization[550] field is used to indicate which side of the cross will be executed against the existing market.

# Execution Reporting for cross orders

Execution reporting for cross orders is done by side. Execution Reports are sent using the ClOrdID[11], OrigClOrdID[41], OrderID[37] of the side of the cross order being reported. The CrossID[548] and the CrossType[549] fields are provided as optional fields on the Execution Report. The CrossID[548] must be provided on the Execution Reports for sides of Cross Orders.

# Cross Order Handling Rules

If one side of the cross order is invalid then the entire cross order is rejected. Markets should not accept one side of the cross order without accepting the other side. The CrossType[549] field defines the proper processing of cross orders once the cross order has been accepted.

March 25, 2003 – DRAFT                              96                              FIX 4.4 - Volume 4
---

# Acknowledgement of a Cross Order

The order state changes for each leg are reported independently using separate Execution Reports for each side.

The following shows typical message flows for the acknowledgement of cross orders.

| Broker | Market                                                                                                                                                                         |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1      | Sends Cross Order                                                                                                                                                              |
|        | Receives Cross Order and processes                                                                                                                                             |
|        | OPTIONAL: Market conditionally accepts or fully rejects the order (This optional state change is assumed for all remaining examples and will not be elaborated to save space). |
|        | Sends Execution Report Side 1 ClOrdID(1)                                                                                                                                       |
|        | OrdStatus=Pending New or OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.                                                             |
|        | If one side is rejected then the entire cross order is rejected.                                                                                                               |
|        | If the Cross Order contains two sides:                                                                                                                                         |
|        | Sends Execution Report Side 2 ClOrdID(2)                                                                                                                                       |
|        | OrdStatus=Pending New or OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.                                                             |
| 3      | Order is accepted or rejected by market                                                                                                                                        |
|        | Sends Execution Report Side 1 ClOrdID(1)                                                                                                                                       |
|        | OrdStatus=New or OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.                                                                     |
|        | If one side is rejected then the entire cross order is rejected.                                                                                                               |
|        | If the Cross Order contains two sides:                                                                                                                                         |
|        | Sends Execution Report Side 2 ClOrdID(2)                                                                                                                                       |
|        | OrdStatus=New or OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.                                                                     |


March 25, 2003 – DRAFT

97

FIX 4.4 - Volume 4


---

Message Flow for cross order with CrossType=1 with only one side of the order provided


In the case where the broker is crossing the order and no further identification is required as part of the order – the cross order can contain one leg. The cross order is executed fully or canceled.

| Broker                                 | Market                                                                                                                                           |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1 Sends Cross Order with one side only | Receives Cross Order and processes                                                                                                               |
|                                        | Order Accepted or Rejected by Market                                                                                                             |
|                                        | Sends Execution Report for ClOrdID(1) OrdStatus=New or OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field. |
|                                        | Order fully executed or is canceled by market                                                                                                    |
|                                        | Sends Execution Report for ClOrdID(1) OrdStatus=FILL or OrdStatus=CANCEL. The reason for the cancel should be specified in the Text\[58] field.  |

March 25, 2003 – DRAFT

98

FIX 4.4 - Volume 4



---

# Message Flow for cross order with CrossType=1 when both sides of the cross order provided

In this example two sides of the cross are explicitly provided on the cross order. The cross order is executed fully or canceled.

| Broker                                       | Market                                                                                                                                               |
| -------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 Sends Cross Order with both sides provided | Receives Cross Order and processes                                                                                                                   |
| 2a                                           | Order Accepted by Market                                                                                                                             |
|                                              | Sends Execution Report for Side 1 ClOrdID(1) OrdStatus=New                                                                                           |
|                                              | Sends Execution Report for Side 2 ClOrdID(2) OrdStatus=New                                                                                           |
| 2b                                           | Order Rejected by Market                                                                                                                             |
|                                              | Sends Execution Report for Side 1 ClOrdID(1) OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.               |
|                                              | Sends Execution Report for Side 2 ClOrdID(2) OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.               |
| 3a                                           | Order is fully executed by market                                                                                                                    |
|                                              | Sends Execution Report for ClOrdID(1) OrdStatus=FILLED                                                                                               |
|                                              | Sends Execution Report for ClOrdID(2) OrdStatus=FILLED                                                                                               |
| 3b                                           | Order is canceled by market                                                                                                                          |
|                                              | Sends Execution Report for ClOrdID(1) OrdStatus=FILLED or OrdStatus=CANCELED. The reason for the cancel should be specified in the Text\[58] field.  |
|                                              | Sends Execution Report for ClOrdID(2) OrdStatus=FILLLED or OrdStatus=CANCELED. The reason for the cancel should be specified in the Text\[58] field. |

# Message Flow for cross order with CrossType=2

In the following example the cross order contains a buy and sell order. The buy side is prioritized and in the case of CrossType=2 will be fully executed. In the following example – the sell side is not fully executed – the balance being canceled.

March 25, 2003 – DRAFT

99

FIX 4.4 - Volume 4


---

# Broker Market

| 1  | Sends Cross Order with CrossType=2 and CrossPrioritization = Buy Side                                                                       |
| -- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| 2a | Order Accepted by Market                                                                                                                    |
|    | Sends Execution Report for Buy Side 1 ClOrdID(1) OrdStatus=New                                                                              |
|    | Sends Execution Report for Sell Side 2 ClOrdID(2) OrdStatus=New                                                                             |
| 2b | Order Rejected by Market                                                                                                                    |
|    | Sends Execution Report for Buy Side 1 ClOrdID(1) OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.  |
|    | Sends Execution Report for Sell Side 2 ClOrdID(2) OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field. |
| 3  | Buy Side of Cross Order is partially filled                                                                                                 |
|    | Sends Execution Report for Buy Side ClOrdID(1) OrdStatus=PARTIALLY FILLED                                                                   |
| 4  | Cross Order is partially crossed with FILLED status of Buy Side                                                                             |
|    | Sends Execution Report for Buy Side ClOrdID(1) OrdStatus=FILLED                                                                             |
|    | Sends Execution Report for Sell Side ClOrdID(2) OrdStatus=PARTIALLY FILLED                                                                  |
| 5  | Remaining quantity of Sell Side is canceled by the market automatically                                                                     |
|    | Sends Execution Report for Sell Side ClOrdID(2) OrdStatus=CANCELED. The reason for the cancel should be specified in the Text\[58] field.   |

March 25, 2003 – DRAFT

100

FIX 4.4 - Volume 4


---
# Message Flow for cross order with CrossType=3

In this scenario – the cross order is executed with the buy side prioritized. The buy side is fully executed. The remaining part of the Sell side remains active and is eventually filled or canceled.

| Broker | Market                                                                                                                                      |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| 1      | Sends Cross Order with CrossType=3 and CrossPrioritization = Buy Side                                                                       |
| 2a     | Order Accepted by Market                                                                                                                    |
|        | Sends Execution Report for Buy Side 1 ClOrdID(1) OrdStatus=New                                                                              |
|        | Sends Execution Report for Sell Side 2 ClOrdID(2) OrdStatus=New                                                                             |
| 2b     | Order Rejected by Market                                                                                                                    |
|        | Sends Execution Report for Buy Side 1 ClOrdID(1) OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.  |
|        | Sends Execution Report for Sell Side 2 ClOrdID(2) OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field. |
| 3      | Buy Side of Cross Order is partially filled                                                                                                 |
|        | Sends Execution Report for Buy Side ClOrdID(1) OrdStatus=PARTIALLY FILLED                                                                   |
| 4      | Cross Order is partially crossed with FILLED status of Buy Side                                                                             |
|        | Sends Execution Report for Buy Side ClOrdID(1) OrdStatus=FILLED                                                                             |
|        | Sends Execution Report for Sell Side ClOrdID(2) OrdStatus=PARTIALLY FILLED                                                                  |
| 5a     | Remaining quantity of Sell Side remains active and is later filled                                                                          |
|        | Sends Execution Report for Sell Side ClOrdID(2) OrdStatus=FILLED                                                                            |
| 5b     | Remaining quantity of Sell Side that remains active is canceled by request                                                                  |
|        | Order Cancel Request submitted after cross order completes to cancel remaining unfilled portion of sell side                                |
|        | Sends Execution Report for Sell Side ClOrdID(2) OrdStatus= Pending Cancel                                                                   |
|        | Sends Execution Report for Sell Side ClOrdID(2) OrdStatus=Canceled                                                                          |

March 25, 2003 – DRAFT

101

FIX 4.4 - Volume 4
---

Message Flow for cross order with CrossType=4

In the following example the buy order is prioritized. The buy side will trade against orders in the book at the same price. The sell side of the cross will trade with the remaining quantity of the buy side. The sell side will be filled at a lower quantity than the buy side that executed against existing orders. NOTE: It is possible for the sell side to be filled with no quantity – if sufficient sell orders exist in the book.

| Broker                                                                  | Market                                                                                                                                           |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1 Sends Cross Order with CrossType=4 and CrossPrioritization = Buy Side | Receives Cross Order and processes                                                                                                               |
| 2a                                                                      | Order Accepted by Market                                                                                                                         |
|                                                                         | Sends Execution Report for Buy Side 1 ClOrdID(1) OrdStatus=New                                                                                   |
|                                                                         | Sends Execution Report for Sell Side 2 ClOrdID(2) OrdStatus=New                                                                                  |
| 2b                                                                      | Order Rejected by Market                                                                                                                         |
|                                                                         | Sends Execution Report for Buy Side 1 ClOrdID(1) OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.       |
|                                                                         | Sends Execution Report for Sell Side 2 ClOrdID(2) OrdStatus=Rejected. The reason for the reject should be specified in the Text\[58] field.      |
| 3                                                                       | Buy side of the Cross Order is partially crossed with sell orders in the book                                                                    |
|                                                                         | Sends Execution Report for Buy Side ClOrdID(1) OrdStatus=PARTIALLY FILLED                                                                        |
| 4                                                                       | Cross Order is completed when remaining Buy Side quantity is filled against the Sell Side of the cross                                           |
|                                                                         | Sends Execution Report for Buy Side ClOrdID(1) OrdStatus=FILLED                                                                                  |
|                                                                         | Sends Execution Report for Sell Side ClOrdID(2) OrdStatus=FILLED even though the filled quantity of the sell side < filled quantity on buy side. |

March 25, 2003 – DRAFT

102

FIX 4.4 - Volume 4


---
Cross Order Messages
# New Order - Cross

Used to submit a cross order into a market. The cross order contains two order sides (a buy and a sell). The cross order is identified by its CrossID.

# New Order - Cross

| Tag | Field Name                       | Req'd | Comments                                                                                                                                                     |
| --- | -------------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|     | Standard Header                  | Y     | MsgType = s (lowercase S)                                                                                                                                    |
| 548 | CrossID                          | Y     |                                                                                                                                                              |
| 549 | CrossType                        | Y     |                                                                                                                                                              |
| 550 | CrossPrioritization              | Y     |                                                                                                                                                              |
| 552 | NoSides                          | Y     | Must be 1 or 2 1 or 2 if CrossType=1 2 otherwise                                                                                                             |
|     | Side                             | Y     |                                                                                                                                                              |
| 11  | ClOrdID                          | Y     | Unique identifier of the order as assigned by institution or by the intermediary with closest association with the investor.                                 |
| 526 | SecondaryClOrdID                 | N     |                                                                                                                                                              |
| 583 | ClOrdLinkID                      | N     |                                                                                                                                                              |
|     | component block \<Parties>       | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                         |
| 229 | TradeOriginationDate             | N     |                                                                                                                                                              |
| 75  | TradeDate                        | N     |                                                                                                                                                              |
| 1   | Account                          | N     |                                                                                                                                                              |
| 660 | AcctIDSource                     | N     |                                                                                                                                                              |
| 581 | AccountType                      | N     |                                                                                                                                                              |
| 589 | DayBookingInst                   | N     |                                                                                                                                                              |
| 590 | BookingUnit                      | N     |                                                                                                                                                              |
| 591 | PreallocMethod                   | N     |                                                                                                                                                              |
| 70  | AllocID                          | N     | Use to assign an identifier to the block of preallocations                                                                                                   |
| 78  | NoAllocs                         | N     | Number of repeating groups for pre-trade allocation                                                                                                          |
|     | 79 AllocAccount                  | N     | Required if NoAllocs > 0. Must be first field in repeating group.                                                                                            |
| 661 | AllocAcctIDSource                | N     |                                                                                                                                                              |
| 736 | AllocSettlCurrency               | N     |                                                                                                                                                              |
| 467 | IndividualAllocID                | N     |                                                                                                                                                              |
|     | component block \<NestedParties> | N     | Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION |

March 25, 2003 – DRAFT 103 FIX 4.4 - Volume 4
---

# MESSAGES

Used for NestedPartyRole=Clearing Firm

| 80                                                                                                                                                                                                                        | AllocQty             | N |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- | - |
| 854                                                                                                                                                                                                                       | QtyType              | N |
| component                                                                                                                                                                                                                 |                      |   |
| block                                                                                                                                                                                                                     |                      |   |
| Y                                                                                                                                                                                                                         |                      |   |
| \<OrderQtyData>                                                                                                                                                                                                           |                      |   |
| Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                                       |                      |   |
| component                                                                                                                                                                                                                 |                      |   |
| block                                                                                                                                                                                                                     |                      |   |
| N                                                                                                                                                                                                                         |                      |   |
| \<CommissionData>                                                                                                                                                                                                         |                      |   |
| Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                                     |                      |   |
| 528                                                                                                                                                                                                                       | OrderCapacity        | N |
| 529                                                                                                                                                                                                                       | OrderRestrictions    | N |
| 582                                                                                                                                                                                                                       | CustOrderCapacity    | N |
| 121                                                                                                                                                                                                                       | ForexReq             | N |
| Indicates that broker is requested to execute a Forex accommodation trade in conjunction with the security trade.                                                                                                         |                      |   |
| 120                                                                                                                                                                                                                       | SettlCurrency        | N |
| Required if ForexReq = Y.                                                                                                                                                                                                 |                      |   |
| 775                                                                                                                                                                                                                       | BookingType          | N |
| Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking. |                      |   |
| 58                                                                                                                                                                                                                        | Text                 | N |
| 354                                                                                                                                                                                                                       | EncodedTextLen       | N |
| Must be set if EncodedText field is specified and must immediately precede it.                                                                                                                                            |                      |   |
| 355                                                                                                                                                                                                                       | EncodedText          | N |
| Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                                                                            |                      |   |
| 77                                                                                                                                                                                                                        | PositionEffect       | N |
| 203                                                                                                                                                                                                                       | CoveredOrUncovered   | N |
| 544                                                                                                                                                                                                                       | CashMargin           | N |
| 635                                                                                                                                                                                                                       | ClearingFeeIndicator | N |
| 377                                                                                                                                                                                                                       | SolicitedFlag        | N |
| 659                                                                                                                                                                                                                       | SideComplianceID     | N |
| component                                                                                                                                                                                                                 | block                | Y |
| \<Instrument>                                                                                                                                                                                                             |                      |   |
| 711                                                                                                                                                                                                                       | NoUnderlyings        | N |
| Number of underlyings                                                                                                                                                                                                     |                      |   |
| component                                                                                                                                                                                                                 | block                | N |
| \<UnderlyingInstrument>                                                                                                                                                                                                   |                      |   |
| Insert here the set of "UnderlyingInstrument" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES. Required if NoUnderlyings > 0                                                                                 |                      |   |
| 555                                                                                                                                                                                                                       | NoLegs               | N |
| component                                                                                                                                                                                                                 | block                | N |
| \<InstrumentLeg>                                                                                                                                                                                                          |                      |   |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---

COMMON COMPONENTS OF APPLICATION MESSAGES

Required if NoLegs > 0

| 63                                                                                                                                                         | SettlType           | N |                                                                                                                                                                                                       |   |   |   |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | - | - | - |
| 64                                                                                                                                                         | SettlDate           | N | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                                                               |   |   |   |
| 21                                                                                                                                                         | HandlInst           | N |                                                                                                                                                                                                       |   |   |   |
| 18                                                                                                                                                         | ExecInst            | N | Can contain multiple instructions, space delimited. If OrdType=P, exactly one of the following values (ExecInst = L, R, M, P, O, T, or W) must be specified.                                          |   |   |   |
| 110                                                                                                                                                        | MinQty              | N |                                                                                                                                                                                                       |   |   |   |
| 111                                                                                                                                                        | MaxFloor            | N |                                                                                                                                                                                                       |   |   |   |
| 100                                                                                                                                                        | ExDestination       | N |                                                                                                                                                                                                       |   |   |   |
| 386                                                                                                                                                        | NoTradingSessions   | N | Specifies the number of repeating TradingSessionIDs                                                                                                                                                   |   |   |   |
| 336                                                                                                                                                        | TradingSessionID    | N | Required if NoTradingSessions is > 0.                                                                                                                                                                 |   |   |   |
| 625                                                                                                                                                        | TradingSessionSubID | N |                                                                                                                                                                                                       |   |   |   |
| 81                                                                                                                                                         | ProcessCode         | N | Used to identify soft trades at order entry.                                                                                                                                                          |   |   |   |
| 140                                                                                                                                                        | PrevClosePx         | N | Useful for verifying security identification                                                                                                                                                          |   |   |   |
| 114                                                                                                                                                        | LocateReqd          | N | Required for short sell orders                                                                                                                                                                        |   |   |   |
| 60                                                                                                                                                         | TransactTime        | Y | Time this order request was initiated/released by the trader, trading system, or intermediary.                                                                                                        |   |   |   |
| component block \<Stipulations>                                                                                                                            |                     |   | N                                                                                                                                                                                                     |   |   |   |
| Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"         |                     |   |                                                                                                                                                                                                       |   |   |   |
| 40                                                                                                                                                         | OrdType             | Y |                                                                                                                                                                                                       |   |   |   |
| 423                                                                                                                                                        | PriceType           | N |                                                                                                                                                                                                       |   |   |   |
| 44                                                                                                                                                         | Price               | N | Required for limit OrdTypes. For F/X orders, should be the “all-in” rate (spot rate adjusted for forward points). Can be used to specify a limit price for a pegged order, previously indicated, etc. |   |   |   |
| 99                                                                                                                                                         | StopPx              | N | Required for OrdType = “Stop” or OrdType = “Stop limit”.                                                                                                                                              |   |   |   |
| component block                                                                                                                                            |                     |   | N                                                                                                                                                                                                     |   |   |   |
| Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |                     |   |                                                                                                                                                                                                       |   |   |   |
| component block \<YieldData>                                                                                                                               |                     |   | N                                                                                                                                                                                                     |   |   |   |
| Insert here the set of "YieldData" (yield-related) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                           |                     |   |                                                                                                                                                                                                       |   |   |   |
| 15                                                                                                                                                         | Currency            | N |                                                                                                                                                                                                       |   |   |   |
| 376                                                                                                                                                        | ComplianceID        | N |                                                                                                                                                                                                       |   |   |   |
| 23                                                                                                                                                         | IOIid               | N | Required for Previously Indicated Orders (OrdType=E)                                                                                                                                                  |   |   |   |
| 117                                                                                                                                                        | QuoteID             | N | Required for Previously Quoted Orders (OrdType=D)                                                                                                                                                     |   |   |   |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4



---

FIX 4.4 - Volume 4

# 59 TimeInForce

N Absence of this field indicates Day order

# 168 EffectiveTime

N Can specify the time at which the order should be considered valid

# 432 ExpireDate

N Conditionally required if TimeInForce = GTD and ExpireTime is not specified.

# 126 ExpireTime

N Conditionally required if TimeInForce = GTD and ExpireDate is not specified.

# 427 GTBookingInst

N States whether executions are booked out or accumulated on a partially filled GT order

# 210 MaxShow

N

# component block &#x3C;PegInstructions>

N Insert here the set of "PegInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# component block &#x3C;DiscretionInstructions>

N Insert here the set of "DiscretionInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# 847 TargetStrategy

N The target strategy of the order

# 848 TargetStrategyParameters

N For further specification of the TargetStrategy

# 849 ParticipationRate

N Mandatory for a TargetStrategy=Participate order and specifies the target participation rate. For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)

# 480 CancellationRights

N For CIV - Optional

# 481 MoneyLaunderingStatus

N

# 513 RegistID

N Reference to Registration Instructions message for this Order.

# 494 Designation

N Supplementary registration information for this Order

# Standard Trailer

Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;s>&#x3C;!ENTITY % CrossOrderCustom "">&#x3C;/s>

&#x3C;s>&#x3C;!ENTITY % CrossOrderContent "CrossID,CrossType,CrossPrioritization,SideList,Instrument,Settlement?,

CashMargin?,HandInst,ExecInst?,MinQty?,MaxFloor?,ExDestination?,TrdSessionList?,

ProcessCode?,PrevClosePx?,LocateReqd?,TransactTime,StipulationsList?,OrdType,PriceType?,Price?,

StopPx?,SpreadOrBenchmarkCurveData?,YieldData?,Currency?,ComplianceID?,SolicitedFlag?,

IOI_ID?,QuoteID?,OrderDuration?,EffectiveTime?,GTBookingInst?,FutSettDate2?,

OrderQty2?,MaxShow?,PegDifference?,DiscretionInst?,DiscretionOffset?,CancellationRights?,

MoneyLaunderingStatus?,RegistID?,Designation?,AccruedInterestRate?,AccruedInterestAmt?,

NetMoney? %CrossOrderCustom;" >&#x3C;/s>

&#x3C;s>&#x3C;!ELEMENT CrossOrder (%CrossOrderContent;)>&#x3C;/s>

&#x3C;s>&#x3C;!ATTLIST CrossOrder FIXTag

&#x3C;s>CDATA #FIXED '35'&#x3C;/s>

&#x3C;s>DataType CDATA #FIXED 'String'&#x3C;/s>

March 25, 2003 – DRAFT


106

---
# FIX 4.4 - Volume 4

March 25, 2003 – DRAFT

| Value                             | CDATA #FIXED 's' |
| --------------------------------- | ---------------- |
| Refer to FIXML element NewOrdCrss |                  |

---
Cross Order Cancel/Replace Request ( a.k.a. Cross Order Modification Request)
Used to modify a cross order previously submitted using the New Order - Cross message. See Order Cancel Replace Request for details concerning message usage.

Refer to the Order Cancel Replace Request (a.k.a. Order Modification Request) message for restrictions on what fields can be changed during a cancel replace.

The Cross Order-specific fields, CrossType (tag 549) and CrossPrioritization (tag 550), can not be modified using the Cross Order Cancel Replace Request.

# Cross Order Cancel / Replace Request (a.k.a. Cross Order Modification Request)

| Tag | Field Name                 | Req'd | Comments                                                                                                                                   |
| --- | -------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------ |
|     | Standard Header            | Y     | MsgType = t (lowercase T)                                                                                                                  |
| 37  | OrderID                    | N     | Unique identifier of most recent order as assigned by sell-side (broker, exchange, ECN).                                                   |
| 548 | CrossID                    | Y     | CrossID for the replacement order                                                                                                          |
| 551 | OrigCrossID                | Y     | Must match the CrossID of the previous cross order. Same order chaining mechanism as ClOrdID/OrigClOrdID with single order Cancel/Replace. |
| 549 | CrossType                  | Y     |                                                                                                                                            |
| 550 | CrossPrioritization        | Y     |                                                                                                                                            |
| 552 | NoSides                    | Y     | Must be 1 or 2                                                                                                                             |
|     | 54 Side                    | Y     |                                                                                                                                            |
|     | 41 OrigClOrdID             | Y     |                                                                                                                                            |
|     | 11 ClOrdID                 | Y     | Unique identifier of the order as assigned by institution or by the intermediary with closest association with the investor.               |
| 526 | SecondaryClOrdID           | N     |                                                                                                                                            |
| 583 | ClOrdLinkID                | N     |                                                                                                                                            |
| 586 | OrigOrdModTime             | N     |                                                                                                                                            |
|     | component block \<Parties> | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                       |
| 229 | TradeOriginationDate       | N     |                                                                                                                                            |
| 75  | TradeDate                  | N     |                                                                                                                                            |
| 1   | Account                    | N     |                                                                                                                                            |
| 660 | AcctIDSource               | N     |                                                                                                                                            |
| 581 | AccountType                | N     |                                                                                                                                            |
| 589 | DayBookingInst             | N     |                                                                                                                                            |
| 590 | BookingUnit                | N     |                                                                                                                                            |
| 591 | PreallocMethod             | N     |                                                                                                                                            |
| 70  | AllocID                    | N     | Used to assign an identifier to the block of individual preallocations                                                                     |
| 78  | NoAllocs                   | N     | Number of repeating groups for pre-trade allocation                                                                                        |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---


# FIX 4.4 - Volume 4

# 79 AllocAccount

N Required if NoAllocs > 0. Must be first field in repeating group.

# 661 AllocAcctIDSource

N

# 736 AllocSettlCurrency

N

# 467 IndividualAllocID

N

# component block

N Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" Used for NestedPartyRole=Clearing Firm

# &#x3C;NestedParties>

# 80 AllocQty

N

# 854 QtyType

N

# component block

Y Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# &#x3C;OrderQtyData>

# component block

N Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# &#x3C;CommissionData>

# 528 OrderCapacity

N

# 529 OrderRestrictions

N

# 582 CustOrderCapacity

N

# 121 ForexReq

N Indicates that broker is requested to execute a Forex accommodation trade in conjunction with the security trade.

# 120 SettlCurrency

N Required if ForexReq = Y.

# 775 BookingType

N Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking.

# 58 Text

N

# 354 EncodedTextLen

N Must be set if EncodedText field is specified and must immediately precede it.

# 355 EncodedText

N Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.

# 77 PositionEffect

N For use in derivatives omnibus accounting

# 203 CoveredOrUncovered

N For use with derivatives, such as options

# 544 CashMargin

N

# 635 ClearingFeeIndicator

N

# 377 SolicitedFlag

N

# 659 SideComplianceID

N

# component block

Y Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# &#x3C;Instrument>

March 25, 2003 – DRAFT


---


# FIX 4.4 - Volume 4

# Field Definitions

| 711             | NoUnderlyings                 | N | Number of underlyings                                                                                                                                                                                 |
| --------------- | ----------------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|                 | component block               | N | Insert here the set of "UnderlyingInstrument" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES. Required if NoUnderlyings > 0                                                             |
| 555             | NoLegs                        | N | Number of Legs                                                                                                                                                                                        |
|                 | component block               | N | Insert here the set of "InstrumentLeg" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES. Required if NoLegs > 0                                                                           |
| 63              | SettlType                     | N |                                                                                                                                                                                                       |
| 64              | SettlDate                     | N | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                                                               |
| 21              | HandlInst                     | N |                                                                                                                                                                                                       |
| 18              | ExecInst                      | N | Can contain multiple instructions, space delimited. If OrdType=P, exactly one of the following values (ExecInst = L, R, M, P, O, T, or W) must be specified.                                          |
| 110             | MinQty                        | N |                                                                                                                                                                                                       |
| 111             | MaxFloor                      | N |                                                                                                                                                                                                       |
| 100             | ExDestination                 | N |                                                                                                                                                                                                       |
| 386             | NoTradingSessions             | N | Specifies the number of repeating TradingSessionIDs                                                                                                                                                   |
| 336             | TradingSessionID              | N | Required if NoTradingSessions is > 0.                                                                                                                                                                 |
| 625             | TradingSessionSubID           | N |                                                                                                                                                                                                       |
| 81              | ProcessCode                   | N | Used to identify soft trades at order entry.                                                                                                                                                          |
| 140             | PrevClosePx                   | N | Useful for verifying security identification                                                                                                                                                          |
| 114             | LocateReqd                    | N | Required for short sell orders                                                                                                                                                                        |
| 60              | TransactTime                  | Y | Time this order request was initiated/released by the trader, trading system, or intermediary.                                                                                                        |
| component block | \<Stipulations>               | N | Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                    |
| 40              | OrdType                       | Y |                                                                                                                                                                                                       |
| 423             | PriceType                     | N |                                                                                                                                                                                                       |
| 44              | Price                         | N | Required for limit OrdTypes. For F/X orders, should be the “all-in” rate (spot rate adjusted for forward points). Can be used to specify a limit price for a pegged order, previously indicated, etc. |
| 99              | StopPx                        | N | Required for OrdType = “Stop” or OrdType = “Stop limit”.                                                                                                                                              |
| component block | \<SpreadOrBenchmarkCurveData> | N | Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                            |
| component block | \<YieldData>                  | N | Insert here the set of "YieldData" (yield-related) fields                                                                                                                                             |

March 25, 2003 – DRAFT


---

FIX 4.4 - Volume 4

# Common Components of Application Messages

| Field                                     | Required | Description                                                                                                                                                                                                          |
| ----------------------------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Currency                                  | N        |                                                                                                                                                                                                                      |
| ComplianceID                              | N        |                                                                                                                                                                                                                      |
| IOIid                                     | N        | Required for Previously Indicated Orders (OrdType=E)                                                                                                                                                                 |
| QuoteID                                   | N        | Required for Previously Quoted Orders (OrdType=D)                                                                                                                                                                    |
| TimeInForce                               | N        | Absence of this field indicates Day order                                                                                                                                                                            |
| EffectiveTime                             | N        | Can specify the time at which the order should be considered valid                                                                                                                                                   |
| ExpireDate                                | N        | Conditionally required if TimeInForce = GTD and ExpireTime is not specified.                                                                                                                                         |
| ExpireTime                                | N        | Conditionally required if TimeInForce = GTD and ExpireDate is not specified.                                                                                                                                         |
| GTBookingInst                             | N        | States whether executions are booked out or accumulated on a partially filled GT order                                                                                                                               |
| MaxShow                                   | N        |                                                                                                                                                                                                                      |
| component block \<PegInstructions>        | N        | Insert here the set of "PegInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                                |
| component block \<DiscretionInstructions> | N        | Insert here the set of "DiscretionInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                         |
| TargetStrategy                            | N        | The target strategy of the order                                                                                                                                                                                     |
| TargetStrategyParameters                  | N        | For further specification of the TargetStrategy                                                                                                                                                                      |
| ParticipationRate                         | N        | Mandatory for a TargetStrategy=Participate order and specifies the target participation rate. For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume) |
| CancellationRights                        | N        | For CIV - Optional                                                                                                                                                                                                   |
| MoneyLaunderingStatus                     | N        |                                                                                                                                                                                                                      |
| RegistID                                  | N        | Reference to Registration Instructions message for this Order.                                                                                                                                                       |
| Designation                               | N        | Supplementary registration information for this Order                                                                                                                                                                |
| Standard Trailer                          | Y        |                                                                                                                                                                                                                      |

FIXML Definition for this message – see http://www.fixprotocol.org for details

~~&#x3C;!ENTITY % CrossOrderModificationReqCustom "">~~
~~&#x3C;!ENTITY % CrossOrderModificationReqContent "CrossID,OrigCrossID,CrossType,CrossPrioritization,SideList,Instrument,Settlement,CashMargin?,HandInst,ExecInst?,MinQty?,MaxFloor?,ExDestination?,TrdSessionList?,ProcessCode?,PrevClosePx?,LocateReqd?,TransactTime,StipulationsList?,OrdType,PriceType?,Price?,StopPx?,">~~

March 25, 2003 – DRAFT



---
March 25, 2003 – DRAFT
# FIX 4.4 - Volume 4

| SpreadOrBenchmarkCurveData? | YieldData?             | Currency?     | ComplianceID? | SolicitedFlag?       | IOI\_ID?            | QuoteID?        | OrderDuration?                    |
| --------------------------- | ---------------------- | ------------- | ------------- | -------------------- | ------------------- | --------------- | --------------------------------- |
| EffectiveTime?              | GTBookingInst?         | FutSettDate2? | OrderQty2?    | MaxShow?             | PegDifference?      | DiscretionInst? | DiscretionOffset?                 |
| CancellationRights?         | MoneyLaunderingStatus? | RegistID?     | Designation?  | AccruedInterestRate? | AccruedInterestAmt? | NetMoney?       | %CrossOrderModificationReqCustom; |

Refer to FIXML element CrssOrdCxlRplcReq

---
Cross Order Cancel Request

# Cross Order Cancel Request

Used to fully cancel the remaining open quantity of a cross order.

| Tag                                     | Field Name           | Req'd | Comments                                                                                                                               |
| --------------------------------------- | -------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------- |
|                                         | Standard Header      | Y     | MsgType = u (lowercase U)                                                                                                              |
| 37                                      | OrderID              | N     | Unique identifier of most recent order as assigned by sell-side (broker, exchange, ECN).                                               |
| 548                                     | CrossID              | Y     | CrossID for the replacement order                                                                                                      |
| 551                                     | OrigCrossID          | Y     | Must match the CrossID of previous cross order. Same order chaining mechanism as ClOrdID/OrigClOrdID with single order Cancel/Replace. |
| 549                                     | CrossType            | Y     |                                                                                                                                        |
| 550                                     | CrossPrioritization  | Y     |                                                                                                                                        |
| 552                                     | NoSides              | Y     | Must be 1 or 2                                                                                                                         |
| 54                                      | Side                 | Y     |                                                                                                                                        |
| 41                                      | OrigClOrdID          | Y     |                                                                                                                                        |
| 11                                      | ClOrdID              | Y     | Unique identifier of the order as assigned by institution or by the intermediary with closest association with the investor.           |
| 526                                     | SecondaryClOrdID     | N     |                                                                                                                                        |
| 583                                     | ClOrdLinkID          | N     |                                                                                                                                        |
| 586                                     | OrigOrdModTime       | N     |                                                                                                                                        |
| component block \<Parties>              |                      |       |                                                                                                                                        |
|                                         |                      | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                   |
| 229                                     | TradeOriginationDate | N     |                                                                                                                                        |
| 75                                      | TradeDate            | N     |                                                                                                                                        |
| component block \<OrderQtyData>         |                      |       |                                                                                                                                        |
|                                         |                      | Y     | Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                    |
| 376                                     | ComplianceID         | N     |                                                                                                                                        |
| 58                                      | Text                 | N     |                                                                                                                                        |
| 354                                     | EncodedTextLen       | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                         |
| 355                                     | EncodedText          | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.         |
| component block \<Instrument>           |                      |       |                                                                                                                                        |
|                                         |                      | Y     | Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                          |
| 711                                     | NoUnderlyings        | N     | Number of underlyings                                                                                                                  |
| component block \<UnderlyingInstrument> |                      |       |                                                                                                                                        |
|                                         |                      | N     | Insert here the set of "UnderlyingInstrument" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                            |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---

MESSAGES.

# 1. Required if NoUnderlyings > 0

| 555                    | NoLegs       | N | Number of Leg component block                                                                  |
| ---------------------- | ------------ | - | ---------------------------------------------------------------------------------------------- |
| \<InstrumentLeg>       |              |   |                                                                                                |
| Required if NoLegs > 0 |              |   |                                                                                                |
| 60                     | TransactTime | Y | Time this order request was initiated/released by the trader, trading system, or intermediary. |
| Standard Trailer       |              |   | Y                                                                                              |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;s>&#x3C;!ENTITY % CrossOrderCancelReqCustom "">&#x3C;/s>

&#x3C;s>&#x3C;!ENTITY % CrossOrderCancelReqContent

&#x3C;s>"CrossID,OrigCrossID,CrossType,CrossPrioritization,CrossCxlSideList,Instrument,

&#x3C;s>TransactTime %CrossOrderCancelReqCustom;" >&#x3C;/s>

&#x3C;s>&#x3C;!ELEMENT CrossOrderCancelReq (%CrossOrderCancelReqContent;)>&#x3C;/s>

&#x3C;s>&#x3C;!ATTLIST CrossOrderCancelReq FIXTag

&#x3C;s>CDATA #FIXED '35'&#x3C;/s>

&#x3C;s>DataType CDATA #FIXED 'String'&#x3C;/s>

&#x3C;s>Value&#x3C;/s>  &#x3C;s>CDATA #FIXED 'u' >&#x3C;/s>Refer to FIXML element CrssOrdCxlReq

March 25, 2003 – DRAFT

114 FIX 4.4 - Volume 4


---
Cross Order Change Matrices

# Cross Type 1

# Scenario-1: Cancel (Before Exchange Crossing Session Time)

| Time | Message           | Message Sent | No | Cross Sides | OrjgCr ID | Buy ClOrdID | Buy OrjgCl | Buy Qty | Sell ClOrdID | Sell OrjgCl | Sell Qty | OrdStatus      | Comment |
| ---- | ----------------- | ------------ | -- | ----------- | --------- | ----------- | ---------- | ------- | ------------ | ----------- | -------- | -------------- | ------- |
| 1    | New Order - Cross | Received     | 2  | X           |           | 10          | 9000       | 20      |              | 9000        |          |                |         |
| 2    | Execution(Buy)    | Received     | X  |             |           | 10          |            |         |              |             |          | New            |         |
|      | Execution(Sell)   | Received     | X  |             |           | 20          |            |         |              |             |          | New            |         |
| 3    | Cross Cancel      | Received     | 2  | Y           | X         | 11          | 10         | 9000    | 21           | 20          | 9000     |                |         |
| 4    | Execution(Buy)    | Received     | Y  | X           | 11        | 10          |            |         |              |             |          | Pending Cancel |         |
|      | Execution(Sell)   | Received     | Y  | X           |           | 21          | 20         |         |              |             |          | Pending Cancel |         |
| 5    | Execution(Buy)    | Received     | Y  | X           | 11        | 10          |            |         |              |             |          | Canceled       |         |
|      | Execution(Sell)   | Received     | Y  | X           |           | 21          | 20         |         |              |             |          | Canceled       |         |

# Scenario-2: Replace (Before Exchange Crossing Session Time)

| Time | Message              | Message Sent | No | Cross Sides | OrjgCr ID | Buy ClOrdID | Buy OrjgCl | Buy Qty | Sell ClOrdID | Sell OrjgCl | Sell Qty | OrdStatus        | Comment |
| ---- | -------------------- | ------------ | -- | ----------- | --------- | ----------- | ---------- | ------- | ------------ | ----------- | -------- | ---------------- | ------- |
| 1    | New Order - Cross    | Received     | 2  | X           |           | 10          | 9000       | 20      |              | 9000        |          |                  |         |
| 2    | Execution(Buy)       | Received     | X  |             |           | 10          |            |         |              |             |          | New              |         |
|      | Execution(Sell)      | Received     | X  |             |           | 20          |            |         |              |             |          | New              |         |
| 3    | Cross Cancel/Replace | Received     | 2  | Y           | X         | 11          | 10         | 9000    | 21           | 20          | 9000     |                  |         |
| 4    | Execution(Buy)       | Received     | Y  | X           | 11        | 10          |            |         |              |             |          | Pending Replaced |         |
|      | Execution(Sell)      | Received     | Y  | X           |           | 21          | 20         |         |              |             |          | Pending Replaced |         |
| 5    | Execution(Buy)       | Received     | Y  | X           | 11        | 10          |            |         |              |             |          | Replaced         |         |
|      | Execution(Sell)      | Received     | Y  | X           |           | 21          | 20         |         |              |             |          | Replaced         |         |
| 6    | Cross                | Received     | 1  | Z           | Y         | 12          | 11         | 9000    |              |             |          |                  | 9000    |

March 25, 2003 – DRAFT 115 FIX 4.4 - Volume 4
---



# Execution Details

| Cancel/Replace                 | Execution(Buy) | Z | Y  | 12 | 11     | Pending  | Replaced                  |
| ------------------------------ | -------------- | - | -- | -- | ------ | -------- | ------------------------- |
|                                | Execution(Buy) | Z | Y  | 12 | 11     | Replaced |                           |
| Exchange Crossing Session Time | Execution(Buy) | Z |    | 12 | 11     | Filled   | Cross trade is performed. |
| Execution(Sell)                | Z              |   | 21 | 20 | Filled |          |                           |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4



---

# Scenario-3: In case that there are no orders in the book of the market, a New Order - Cross is submitted.

(During market hours)

| Time | Message           | Message Sent | No | Cross Sides | OrjgCr ID | Buy ClOrdI | Sell OrjgCl | Qty | ClOr OrdID | OrjgCl dID | OrdID                          |
| ---- | ----------------- | ------------ | -- | ----------- | --------- | ---------- | ----------- | --- | ---------- | ---------- | ------------------------------ |
|      | New Order - Cross | Received     | 2  | X           |           | 10         | 9000        | 20  | 9000       |            |                                |
|      | Execution(Buy)    | X            | 10 |             |           |            | New         |     |            |            | There is no order in the book. |
|      | Execution(Sell)   | X            | 20 |             |           |            | New         |     |            |            |                                |
|      | Execution(Buy)    | X            | 10 |             |           |            | Filled      |     |            |            |                                |
|      | Execution(Sell)   | X            | 20 |             |           |            | Filled      |     |            |            |                                |

# Scenario-4: In case that there are no orders in the book, a New Order - Cross is submitted.

(During market hours)

| Time | Message           | Message Sent | No | Cross Sides | OrjgCr ID | Buy ClOrdI | Sell OrjgCl | Qty | ClOr OrdID | OrjgCl dID | OrdID                           |
| ---- | ----------------- | ------------ | -- | ----------- | --------- | ---------- | ----------- | --- | ---------- | ---------- | ------------------------------- |
|      | New Order - Cross | Received     | 2  | X           |           | 10         | 9000        | 20  | 9000       |            |                                 |
|      | Execution(Buy)    | X            | 10 |             |           |            | Rejected    |     |            |            | There was an order in the book. |
|      | Execution(Sell)   | X            | 20 |             |           |            | Rejected    |     |            |            |                                 |

March 25, 2003 – DRAFT

117

FIX 4.4 - Volume 4


---

# Cross Type 2

# Scenario-1: There are no orders in the book

| Time | Message           | Message Sent | No | Cross Sides | OrjgCr ID | Buy OrdID | Buy Qty | Sell OrdID | Sell Qty | OrdStatus | Comment |
| ---- | ----------------- | ------------ | -- | ----------- | --------- | --------- | ------- | ---------- | -------- | --------- | ------- |
| 1    | New Order - Cross | X            | 2  |             |           | 10        | 9000    |            | 20       | 9000      |         |
| 2    | Execution(Buy)    | X            | 10 |             |           |           |         |            |          | New       |         |
|      | Execution(Sell)   | X            | 10 |             |           |           | 20      |            |          | New       |         |
| 3    | Execution(Buy)    | X            | 10 |             |           |           |         |            |          | Filled    |         |
|      | Execution(Sell)   | X            | 10 |             |           |           | 20      |            |          | Filled    |         |

# Scenario-2: There is an order in the book

| Time | Message           | Message Sent | No | Cross Sides | OrjgCr ID | Buy OrdID | Buy Qty | Sell OrdID | Sell Qty | OrdStatus      | Comment                                       |
| ---- | ----------------- | ------------ | -- | ----------- | --------- | --------- | ------- | ---------- | -------- | -------------- | --------------------------------------------- |
| 1    | New Order - Cross | X            | 2  |             |           | 10        | 9000    |            | 20       | 9000           |                                               |
| 2    | Execution(Buy)    | X            | 10 |             |           |           |         |            |          | New            |                                               |
|      | Execution(Sell)   | X            | 10 |             |           |           | 20      |            |          | New            |                                               |
| 3    | Execution(Buy)    | X            | 10 |             |           |           | 5000    |            |          | Partial Filled | There was a sell order(Qty=5000) in the book. |
| 4    | Execution(Buy)    | X            | 10 |             |           |           | 4000    |            |          | Filled         |                                               |
|      | Execution(Sell)   | X            | 10 |             |           |           | 20      | 4000       |          | Partial Filled | (LastShares)                                  |
| 5    | Execution(Sell)   | X            | 10 |             |           |           | 20      |            |          | Canceled       | Remaining order is canceled.                  |

March 25, 2003 – DRAFT                                  118                                       FIX 4.4 - Volume 4
---
Cross Type 3

| Time | Message              |          | Message Sent    | No | Cross Sides | OrjgCr ID | Buy Qty      | Sell OrdStatus                               |      |    |      |                  | Comment                     |
| ---- | -------------------- | -------- | --------------- | -- | ----------- | --------- | ------------ | -------------------------------------------- | ---- | -- | ---- | ---------------- | --------------------------- |
| 1    | New Order - Cross    | Received |                 | 2  | X           |           | 10           | 9000                                         | 20   |    |      |                  |                             |
| 2    |                      |          | Execution(Buy)  |    | X           |           |              | 10                                           |      |    |      | New              |                             |
|      |                      |          | Execution(Sell) |    | X           |           |              |                                              | 20   |    |      | New              |                             |
| 3    |                      |          | Execution(Buy)  |    | X           |           |              | 10                                           | 5000 |    |      | Partial Filled   |                             |
|      |                      |          |                 |    |             |           | (LastShares) | There is a sell order(Qty=5000) in the book. |      |    |      |                  |                             |
| 4    |                      |          | Execution(Buy)  |    | X           |           |              | 10                                           | 4000 |    |      | Filled           |                             |
|      |                      |          | Execution(Sell) |    | X           |           |              |                                              | 20   |    | 4000 | Partial Filled   |                             |
|      |                      |          |                 |    |             |           | (LastShares) |                                              |      |    |      |                  |                             |
| 5    | Cross Cancel/Replace |          |                 | 1  | Y           | X         |              |                                              | 21   | 20 | 8000 |                  |                             |
| 6    |                      |          | Execution(Sell) |    | Y           | X         |              |                                              | 21   | 20 |      | Pending Replaced |                             |
| 7    |                      |          | Execution(Sell) |    | Y           | X         |              |                                              | 21   | 20 |      | Replaced         |                             |
| 8    | Cross Cancel/Replace |          |                 | 1  | Z           | Y         |              |                                              | 22   | 21 | 4000 |                  |                             |
| 9    |                      |          | Cancel Reject   |    | Z           | Y         |              |                                              | 22   | 21 |      |                  | Replace request is rejected |
| 10   | Cross Cancel Request |          | 1               |    | W           | Y         |              |                                              | 23   | 21 | 8000 |                  |                             |
| 11   |                      |          | Execution(Sell) |    | W           | Y         |              |                                              | 23   | 21 |      | Pending Canceled |                             |
| 12   |                      |          | Execution(Sell) |    | W           | Y         |              |                                              | 23   | 21 |      | Canceled         |                             |

March 25, 2003 – DRAFT                                      119                                       FIX 4.4 - Volume 4
---


# Cross Type 4

| Time | Message           | Message Sent | No | Cross Sides ID | Cross ID | Orjg ClOrdID | Buy Qty | Sell OrdID | Sell Qty | OrdStatus      | Comment                                      |
| ---- | ----------------- | ------------ | -- | -------------- | -------- | ------------ | ------- | ---------- | -------- | -------------- | -------------------------------------------- |
| 1    | New Order - Cross |              | 2  | X              | 10       |              | 14000   | 20         | 9000     |                | There is a sell order(Qty=5000) in the book. |
| 2    | Execution(Buy)    |              | X  | 10             |          |              |         |            |          | New            |                                              |
|      | Execution(Sell)   |              | X  |                |          | 20           |         |            |          | New            |                                              |
| 3    | Execution(Buy)    |              | X  | 10             |          | 5000         |         |            |          | Partial Filled |                                              |
| 4    | Execution(Buy)    |              | X  | 10             |          | 9000         |         |            |          | Filled         |                                              |
|      | Execution(Sell)   |              | X  |                |          | 20           |         |            | 9000     | Filled         |                                              |

March 25, 2003 – DRAFT

120

FIX 4.4 - Volume 4


---

CATEGORY: MULTILEG ORDERS (SWAPS, OPTION STRATEGIES, ETC)


# Background

A multileg security is made up of multiple securities that are traded atomically. Swaps, option strategies, futures spreads, are a few examples of multileg securities. This requirement that all legs be traded in the quantities that they make up the multileg security is the important distinction between a multileg order and a list order.

Two generalized approaches to trading multileg securities are supported by FIX. The first approach involves a market maintaining multileg securities as separate products for which markets can be created. This “product approach” is often used in electronic trading systems. The second approach is to trade the multileg security as a group of separate securities – as is commonly done today in open outcry markets.

The multileg order can be traded using one of the following trading models using FIX. The first three models are variations on the multileg security as a separate tradeable product. The last model permits trading of multileg securities in environments where the multileg securities are not productized.

# Predefined Multileg Security Model (FIX 4.2) (Model 1)

In this model a Security Definition Request for the security is sent to the counterparty that defines the multileg security and the legs. The counterparty accepts the security definition with an acknowledging Security Definition message. The initiating counterparty can then send a New Order – Single message that specifies just the multileg instrument without the legs.

| Counterparty 1 – Interested in trading a multileg instrument                                                                                        | Counterparty 2 or Market                                                                                                                                                                                                        |
| --------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 Sends Security Definition Request that defined Multileg Security                                                                                  | Receives Security Definition Request – determines if multileg security has already been defined. If so – return identification of the multileg security – otherwise create the multileg security and return its identification. |
| 2a Create the order                                                                                                                                 | Reply with Security Definition for multileg security with identification identical to that of the request                                                                                                                       |
| 2b Create the order                                                                                                                                 | Reply with Security Definition for multileg security with identification different from that of the request                                                                                                                     |
| 2c Exception Handling for failed Security Definition Request                                                                                        | Reply with Security Definition rejecting the security request                                                                                                                                                                   |
| 3 Send New Order - Multileg for security identification provide in Security Definition Request (The Instrument Leg component block is not provided) | Accepts order for processing                                                                                                                                                                                                    |
| 4a                                                                                                                                                  | If MultilegReportTypeRequest =0 or =1 or if market rules require reporting by multileg security: Send Execution Report for the overall multileg security (MultilegReportType=2)                                                 |
| 4b                                                                                                                                                  | If MultilegReportTypeRequest =1 or =2 or if market rules require reporting by multileg security                                                                                                                                 |


March 25, 2003 – DRAFT 121 FIX 4.4 - Volume 4

---

# Send Execution Reports for each instrument leg defined previously for the multileg security (MultilegReportType=3)

# Enhanced Predefined Security Model (Model 2)

In the enhanced model – the multileg security is still defined as a product using the Security Definition message. However, the instrument legs are elaborated on the order to provide clearing information per leg, such as LegPositionEffect, LegCoveredOrUncovered, and within &#x3C;NestedParties> information such as ClearingFirm for the leg, etc.

| Counterparty 1 – Interested in trading a multileg instrument                                                                  | Counterparty 2 or Market                                                                                                                                                                                                        |
| ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 Sends Security Definition Request that defined Multileg Security                                                            | Receives Security Definition Request – determines if multileg security has already been defined. If so – return identification of the multileg security – otherwise create the multileg security and return its identification. |
| 2a Create the order                                                                                                           | Reply with Security Definition for multileg security with identification identical to that of the request                                                                                                                       |
| 2b Create the order                                                                                                           | Reply with Security Definition for multileg security with identification different from that of the request                                                                                                                     |
| 2c Exception Handling for failed Security Definition Request                                                                  | Reply with Security Definition rejecting the security request                                                                                                                                                                   |
| 3 Send New Order - Multileg for security identification provide in Security Definition Request. Includes Leg Instrument Block | Accepts order for processing                                                                                                                                                                                                    |
| 4a If MultilegReportTypeRequest =0 or =1 or if market rules require reporting by multileg security:                           | Send Execution Report for the overall multileg security (MultilegReportType=2)                                                                                                                                                  |
| 4b If MultilegReportTypeRequest =1 or =2 or if market rules require reporting by multileg security                            | Send Execution Reports for each instrument leg defined previously for the multileg security (MultilegReportType=3)                                                                                                              |

# Product Definition Model using New Order - Multileg Message (Model 3)

In this approach the Multileg Security is defined using the New Order - Multileg message. However, the market or counterparty still creates or maintains a product definition for the multileg security upon receipt of the New Order - Multileg.

| Counterparty 1 – | Counterparty 2 or Market |
| ---------------- | ------------------------ |

March 25, 2003 – DRAFT 122 FIX 4.4 - Volume 4
---

# Interested in trading a multileg instrument

# 1 Send New Order - Multileg

Accepts order for processing that includes the multileg security definition in the Leg Instrument Block. A product is defined or identified for the multileg security.

If the multileg security is not a valid product in the market – the order is rejected. The order is rejected using an Execution Report – indicating an invalid product was encountered.

# 2a

If MultilegReportTypeRequest =0 or =1 or if market rules require reporting by multileg security:

Send Execution Report for the overall multileg security (MultilegReportType=2)

# 2b

If MultilegReportTypeRequest =1 or =2 or if market rules require reporting by multileg security:

Send Execution Reports for each instrument leg defined previously for the multileg security (MultilegReportType=3)

# Single Message Model (Model 4)

No product definition is used (Likely will be used by open outcry markets that do not have a product definition service). The message flow is the same as model 3 – the difference being that the counterparty or market receiving the order does not create nor maintain product information for the multileg security – most likely the multileg security is simply distributed to the market.

Counterparty 1 – Interested in trading a multileg instrument

# 1 Send New Order - Multileg

Accepts order for processing that includes the multileg security definition in the Leg Instrument Block. The multileg security information is distributed to the market. No product definition takes place.

If the multileg security is not a valid multileg strategy in the market – the order is rejected. The order is rejected using an Execution Report – indicating an invalid product was encountered.

# 2a

If MultilegReportTypeRequest =0 or =1 or if market rules require reporting by multileg security:

Send Execution Report for the overall multileg security (MultilegReportType=2)


---

# 2b

If MultilegReportTypeRequest = 1 or = 2 or if market rules require reporting by multileg security

Send Execution Reports for each instrument leg defined previously for the multileg security (MultilegReportType=3)

# Messages Used for Multileg Trading

# Order Entry

Use the New Order - Multileg (MsgType=AB) message to submit a multileg order to a market place.

# Execution Reports for Multileg Orders

The Execution Report (MsgType=8) has been modified to report the order status of Multileg Orders.

# Modification of a Multileg Order

Use the Multileg Order Cancel Replace Request (a.k.a Multileg Order Modification Request) (MsgType=AC) to modify a Multileg Order.

# Cancellation of a Multileg Order

Multileg orders are canceled using the Order Cancel Request (MsgType = F). The entire multileg order is cancelled by OrderID (tag #37) or ClOrdID (tag #11). The ability to cancel one leg of a multileg order is not supported in FIX 4.3 and above.

# New Order - Multileg

The New Order - Multileg is provided to submit orders for securities that are made up of multiple securities, known as legs.

The format for the new order message is as follows:

| Tag                        | Field Name           | Req'd | Comments                                                                                                                     |
| -------------------------- | -------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------- |
|                            | Standard Header      | Y     | MsgType = AB                                                                                                                 |
| 11                         | ClOrdID              | Y     | Unique identifier of the order as assigned by institution or by the intermediary with closest association with the investor. |
| 526                        | SecondaryClOrdID     | N     |                                                                                                                              |
| 583                        | ClOrdLinkID          | N     |                                                                                                                              |
| component block \<Parties> |                      | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"         |
| 229                        | TradeOriginationDate | N     |                                                                                                                              |
| 75                         | TradeDate            | N     |                                                                                                                              |
| 1                          | Account              | N     |                                                                                                                              |
| 660                        | AcctIDSource         | N     |                                                                                                                              |
| 581                        | AccountType          | N     |                                                                                                                              |
| 589                        | DayBookingInst       | N     |                                                                                                                              |
| 590                        | BookingUnit          | N     |                                                                                                                              |
| 591                        | PreallocMethod       | N     |                                                                                                                              |

March 25, 2003 – DRAFT

124

FIX 4.4 - Volume 4


---

# FIX 4.4 - Volume 4

March 25, 2003 – DRAFT

| 70              | AllocID                                                                                             | N | Used to assign an identifier to the block of individual preallocations                                                                                                 |
| --------------- | --------------------------------------------------------------------------------------------------- | - | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 78              | NoAllocs                                                                                            | N | Number of repeating groups for pre-trade allocation                                                                                                                    |
| 79              | AllocAccount                                                                                        | N | Required if NoAllocs > 0. Must be first field in repeating group.                                                                                                      |
| 661             | AllocAcctIDSource                                                                                   | N |                                                                                                                                                                        |
| 736             | AllocSettlCurrency                                                                                  | N |                                                                                                                                                                        |
| 467             | IndividualAllocID                                                                                   | N |                                                                                                                                                                        |
| component       | block                                                                                               | N | Insert here the set of "NestedParties3" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |
| 80              | AllocQty                                                                                            | N |                                                                                                                                                                        |
| 63              | SettlType                                                                                           | N |                                                                                                                                                                        |
| 64              | SettlDate                                                                                           | N | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                                |
| 544             | CashMargin                                                                                          | N |                                                                                                                                                                        |
| 635             | ClearingFeeIndicator                                                                                | N |                                                                                                                                                                        |
| 21              | HandlInst                                                                                           | N |                                                                                                                                                                        |
| 18              | ExecInst                                                                                            | N | Can contain multiple instructions, space delimited. If OrdType=P, exactly one of the following values (ExecInst = L, R, M, P, O, T, or W) must be specified.           |
| 110             | MinQty                                                                                              | N |                                                                                                                                                                        |
| 111             | MaxFloor                                                                                            | N |                                                                                                                                                                        |
| 100             | ExDestination                                                                                       | N |                                                                                                                                                                        |
| 386             | NoTradingSessions                                                                                   | N | Specifies the number of repeating TradingSessionIDs                                                                                                                    |
| 336             | TradingSessionID                                                                                    | N | Required if NoTradingSessions is > 0.                                                                                                                                  |
| 625             | TradingSessionSubID                                                                                 | N |                                                                                                                                                                        |
| 81              | ProcessCode                                                                                         | N | Used to identify soft trades at order entry.                                                                                                                           |
| 54              | Side                                                                                                | Y | Additional enumeration that indicates this is an order for a multileg order and that the sides are specified in the Instrument Leg component block.                    |
| component block |                                                                                                     | Y | Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                          |
|                 | SecurityType\[167] = “MLEG”                                                                         |   |                                                                                                                                                                        |
|                 | CFICode should be set to the type of multileg product, such as “O” – options, “F” – Future or Swap. |   |                                                                                                                                                                        |
| 711             | NoUnderlyings                                                                                       | N | Number of underlyings                                                                                                                                                  |
| component block |                                                                                                     | N | Must be provided if Number of underlyings > 0                                                                                                                          |
|                 |                                                                                                     |   |                                                                                                                                                                        |
| 140             | PrevClosePx                                                                                         | N | Useful for verifying security identification                                                                                                                           |


---

March 25, 2003 – DRAFT


# FIX 4.4 - Volume 4

| 555                                                                                                            | NoLegs                | Y               | Number of legs                                                                                                                                                                                |
| -------------------------------------------------------------------------------------------------------------- | --------------------- | --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Can be zero (e.g. standardized multileg instrument such as an Option strategy) – must be provided even if zero |                       |                 |                                                                                                                                                                                               |
| component                                                                                                      | block                 | N               | Must be provided if Number of legs > 0                                                                                                                                                        |
| \<InstrumentLeg>                                                                                               |                       |                 |                                                                                                                                                                                               |
| 687                                                                                                            | LegQty                | N               |                                                                                                                                                                                               |
| 690                                                                                                            | LegSwapType           | N               |                                                                                                                                                                                               |
| component                                                                                                      | block                 | N               |                                                                                                                                                                                               |
| \<LegStipulations>                                                                                             |                       |                 |                                                                                                                                                                                               |
| 670                                                                                                            | NoLegAllocs           | N               |                                                                                                                                                                                               |
| 671                                                                                                            | LegAllocAccount       | N               |                                                                                                                                                                                               |
| 672                                                                                                            | LegIndividualAllocID  | N               |                                                                                                                                                                                               |
| component                                                                                                      | block                 | N               | Insert here the set of "Nested Parties #2" (firm identification "second instance of nesting" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |
| 673                                                                                                            | LegAllocQty           | N               |                                                                                                                                                                                               |
| 674                                                                                                            | LegAllocAcctIDS       | N               |                                                                                                                                                                                               |
| 675                                                                                                            | LegSettlCurrency      | N               |                                                                                                                                                                                               |
| 564                                                                                                            | LegPositionEffect     | N               | Provide if the PositionEffect for the leg is different from that specified for the overall multileg security                                                                                  |
| 565                                                                                                            | LegCoveredOrUncovered | N               | Provide if the CoveredOrUncovered for the leg is different from that specified for the overall multileg security.                                                                             |
| component                                                                                                      | block                 | N               | Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                        |
| 654                                                                                                            | LegRefID              | N               | Used to identify a specific leg.                                                                                                                                                              |
| 566                                                                                                            | LegPrice              | N               | Provide only if a price is required for a specific leg. Used for anchoring the overall multileg security price to a specific leg price.                                                       |
| 587                                                                                                            | LegSettlType          | N               | Refer to values for SettlType (63)                                                                                                                                                            |
| 588                                                                                                            | LegSettlDate          | N               | Refer to values for SettlDate (64)                                                                                                                                                            |
| 114                                                                                                            | LocateReqd            | N               | Required for short sell orders                                                                                                                                                                |
| 60                                                                                                             | TransactTime          | Y               | Time this order request was initiated/released by the trader, trading system, or intermediary.                                                                                                |
| 854                                                                                                            | QtyType               | N               |                                                                                                                                                                                               |
| component                                                                                                      | block                 | \<OrderQtyData> | Y                                                                                                                                                                                             |
| 40                                                                                                             | OrdType               | Y               |                                                                                                                                                                                               |


126

---

# 423 PriceType

N

# 44 Price

N Required for limit OrdTypes. For F/X orders, should be the “all-in” rate (spot rate adjusted for forward points). Can be used to specify a limit price for a pegged order, previously indicated, etc.

# 99 StopPx

N Required for OrdType = “Stop” or OrdType = “Stop limit”.

# 15 Currency

N

# 376 ComplianceID

N

# 377 SolicitedFlag

N

# 23 IOIid

N Required for Previously Indicated Orders (OrdType=E)

# 117 QuoteID

N Required for Previously Quoted Orders (OrdType=D)

# 59 TimeInForce

N Absence of this field indicates Day order

# 168 EffectiveTime

N Can specify the time at which the order should be considered valid

# 432 ExpireDate

N Conditionally required if TimeInForce = GTD and ExpireTime is not specified.

# 126 ExpireTime

N Conditionally required if TimeInForce = GTD and ExpireDate is not specified.

# 427 GTBookingInst

N States whether executions are booked out or accumulated on a partially filled GT order

# component block &#x3C;CommissionData>

N Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# 528 OrderCapacity

N

# 529 OrderRestrictions

N

# 582 CustOrderCapacity

N

# 121 ForexReq

N Indicates that broker is requested to execute a Forex accommodation trade in conjunction with the security trade.

# 120 SettlCurrency

N Required if ForexReq = Y.

# 775 BookingType

N Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking.

# 58 Text

N

# 354 EncodedTextLen

N Must be set if EncodedText field is specified and must immediately precede it.

# 355 EncodedText

N Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.

# 77 PositionEffect

N For use in derivatives omnibus accounting

# 203 CoveredOrUncovered

N For use with derivatives, such as options

# 210 MaxShow

N

# component block &#x3C;PegInstructions>

N Insert here the set of "PegInstruction" fields defined in

March 25, 2003 – DRAFT

127 FIX 4.4 - Volume 4


---

COMMON COMPONENTS OF APPLICATION MESSAGES

# COMMON COMPONENTS OF APPLICATION MESSAGES

| component                | block | N | Description                                                                                                                                                                                                          |
| ------------------------ | ----- | - | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| DiscretionInstructions   |       | N | Insert here the set of "DiscretionInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                         |
| TargetStrategy           |       | N | The target strategy of the order                                                                                                                                                                                     |
| TargetStrategyParameters |       | N | For further specification of the TargetStrategy                                                                                                                                                                      |
| ParticipationRate        |       | N | Mandatory for a TargetStrategy=Participate order and specifies the target participation rate. For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume) |
| CancellationRights       |       | N | For CIV - Optional                                                                                                                                                                                                   |
| MoneyLaunderingStatus    |       | N |                                                                                                                                                                                                                      |
| RegistID                 |       | N | Reference to Registration Instructions message for this Order.                                                                                                                                                       |
| Designation              |       | N | Supplementary registration information for this Order                                                                                                                                                                |
| MultiLegRptTypeReq       |       | N | Indicates the method of execution reporting requested by issuer of the order.                                                                                                                                        |
| Standard Trailer         |       | Y |                                                                                                                                                                                                                      |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;!ENTITY % MultiLegOrderCustom "">

&#x3C;!ENTITY % MultiLegOrderContent "ClOrdID,SecondaryClOrdID?, ClOrdLinkID?, PartiesList?,Account?,AccountType?, DayBookingInst?, BookingUnit?, PreallocMethod?, MLAllocGroupList?, Settlement?,CashMargin?, ClearingFeeIndicator?, HandInst, ExecInstList?,MinQty?,MaxFloor?,ExDestination?,TrdSessionList?,ProcessCode?,Side, Instrument,PrevClosePx?,LegList?,LocateReqd?,TransactTime, QtyType?, OrderQtyData,OrdType,PriceType?, Price?,StopPx?,Currency?,ComplianceID?,IOI_ID?,QuoteID?,OrderDuration?, EffectiveTime?,GTBookingInst?,CommissionData?,OrderCapacity?, OrderRestrictions?, CustOrderCapacity?, ForexReqOrder?,Text?,EncodedTextGroup?, PositionEffect?,CoveredOrUncovered?,MaxShow?, PegDifference?,DiscretionInst?,DiscretionOffset?,CancellationRights?, MoneyLaunderingStatus?,RegistID?,Designation?,MultiLegRptTypeReq?, NetMoney? %MultiLegOrderCustom;">

&#x3C;!ELEMENT MultiLegOrder (%MultiLegOrderContent;)>

&#x3C;!ATTLIST MultiLegOrder FIXTag CDATA #FIXED '35' DataType CDATA #FIXED 'String' Value CDATA #FIXED 'AB'> Refer to FIXML element NewOrdMleg


March 25, 2003 – DRAFT 128 FIX 4.4 - Volume 4

---

Multileg Order Cancel Replace Request (a.k.a Multileg Order Modification Request)

Used to modify a multileg order previously submitted using the New Order - Multileg message. See Order Cancel Replace Request for details concerning message usage.

# The format of the Multileg Order Cancel/Replace Request message is:

# Multileg Order Cancel/Replace Request (a.k.a Multileg Order Modification Request)

| Tag             | Field Name           | Req'd | Comments                                                                                                                                                                                                                                                          |
| --------------- | -------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|                 | Standard Header      | Y     | MsgType = AC                                                                                                                                                                                                                                                      |
| 37              | OrderID              | N     | Unique identifier of most recent order as assigned by sell-side (broker, exchange, ECN).                                                                                                                                                                          |
| 41              | OrigClOrdID          | Y     | ClOrdID of the previous order (NOT the initial order of the day) when canceling or replacing an order.                                                                                                                                                            |
| 11              | ClOrdID              | Y     | Unique identifier of replacement order as assigned by institution or by the intermediary with closest association with the investor. Note that this identifier will be used in ClOrdID field of the Cancel Reject message if the replacement request is rejected. |
| 526             | SecondaryClOrdID     | N     |                                                                                                                                                                                                                                                                   |
| 583             | ClOrdLinkID          | N     |                                                                                                                                                                                                                                                                   |
| 586             | OrigOrdModTime       | N     |                                                                                                                                                                                                                                                                   |
| component block | \<Parties>           | N     | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                                                                              |
| 229             | TradeOriginationDate | N     |                                                                                                                                                                                                                                                                   |
| 75              | TradeDate            | N     |                                                                                                                                                                                                                                                                   |
| 1               | Account              | N     |                                                                                                                                                                                                                                                                   |
| 660             | AcctIDSource         | N     |                                                                                                                                                                                                                                                                   |
| 581             | AccountType          | N     |                                                                                                                                                                                                                                                                   |
| 589             | DayBookingInst       | N     |                                                                                                                                                                                                                                                                   |
| 590             | BookingUnit          | N     |                                                                                                                                                                                                                                                                   |
| 591             | PreallocMethod       | N     |                                                                                                                                                                                                                                                                   |
| 70              | AllocID              | N     | Used to assign an identifier to the block of individual preallocations                                                                                                                                                                                            |
| 78              | NoAllocs             | N     | Number of repeating groups for pre-trade allocation                                                                                                                                                                                                               |
| 79              | AllocAccount         | N     | Required if NoAllocs > 0. Must be first field in repeating group.                                                                                                                                                                                                 |
| 661             | AllocAcctIDSource    | N     |                                                                                                                                                                                                                                                                   |
| 736             | AllocSettlCurrency   | N     |                                                                                                                                                                                                                                                                   |
| 467             | IndividualAllocID    | N     |                                                                                                                                                                                                                                                                   |
| component block |                      | N     | Insert here the set of "NestedParties3" (firm identification)                                                                                                                                                                                                     |

March 25, 2003 – DRAFT

FIX 4.4 - Volume 4


---

March 25, 2003 – DRAFT

# FIX 4.4 - Volume 4

# Nested Parties

"nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

| 80                      | AllocQty                                                                                                       | N |                                                                                                                                                              |
| ----------------------- | -------------------------------------------------------------------------------------------------------------- | - | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 63                      | SettlType                                                                                                      | N |                                                                                                                                                              |
| 64                      | SettlDate                                                                                                      | N | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                      |
| 544                     | CashMargin                                                                                                     | N |                                                                                                                                                              |
| 635                     | ClearingFeeIndicator                                                                                           | N |                                                                                                                                                              |
| 21                      | HandlInst                                                                                                      | N |                                                                                                                                                              |
| 18                      | ExecInst                                                                                                       | N | Can contain multiple instructions, space delimited. If OrdType=P, exactly one of the following values (ExecInst = L, R, M, P, O, T, or W) must be specified. |
| 110                     | MinQty                                                                                                         | N |                                                                                                                                                              |
| 111                     | MaxFloor                                                                                                       | N |                                                                                                                                                              |
| 100                     | ExDestination                                                                                                  | N |                                                                                                                                                              |
| 386                     | NoTradingSessions                                                                                              | N | Specifies the number of repeating TradingSessionIDs                                                                                                          |
| 336                     | TradingSessionID                                                                                               | N | Required if NoTradingSessions is > 0.                                                                                                                        |
| 625                     | TradingSessionSubID                                                                                            | N |                                                                                                                                                              |
| 81                      | ProcessCode                                                                                                    | N | Used to identify soft trades at order entry.                                                                                                                 |
| 54                      | Side                                                                                                           | Y | Additional enumeration that indicates this is an order for a multileg order and that the sides are specified in the Instrument Leg component block.          |
| component block         | \<Instrument>                                                                                                  | Y | Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                |
|                         | SecurityType\[167] = “MLEG”                                                                                    |   | CFICode should be set to the type of multileg product, such as “O” – options, “F” – Future or Swap.                                                          |
| 711                     | NoUnderlyings                                                                                                  | N | Number of underlyings                                                                                                                                        |
| component block         |                                                                                                                | N | Must be provided if Number of underlyings > 0                                                                                                                |
| \<UnderlyingInstrument> |                                                                                                                |   |                                                                                                                                                              |
| 140                     | PrevClosePx                                                                                                    | N | Useful for verifying security identification                                                                                                                 |
| 555                     | NoLegs                                                                                                         | Y | Number of legs                                                                                                                                               |
|                         | Can be zero (e.g. standardized multileg instrument such as an Option strategy) – must be provided even if zero |   |                                                                                                                                                              |
| component block         |                                                                                                                | N | Must be provided if Number of legs > 0                                                                                                                       |
| \<InstrumentLeg>        |                                                                                                                |   |                                                                                                                                                              |
| 687                     | LegQty                                                                                                         | N |                                                                                                                                                              |
| 690                     | LegSwapType                                                                                                    | N |                                                                                                                                                              |
| component block         | \<LegStipulations>                                                                                             |   |                                                                                                                                                              |



---

# FIX 4.4 - Volume 4

| 670                                                                                                                                                                                                   | NoLegAllocs           | N |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | - |
| 671                                                                                                                                                                                                   | LegAllocAccount       | N |
| 672                                                                                                                                                                                                   | LegIndividualAllocID  | N |
| component block                                                                                                                                                                                       |                       |   |
| \<NestedParties2>                                                                                                                                                                                     |                       |   |
| Insert here the set of "Nested Parties #2" (firm identification "second instance of nesting" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"         |                       |   |
| 673                                                                                                                                                                                                   | LegAllocQty           | N |
| 674                                                                                                                                                                                                   | LegAllocAcctIDS       | N |
| 675                                                                                                                                                                                                   | LegSettlCurrency      | N |
| 564                                                                                                                                                                                                   | LegPositionEffect     | N |
| Provide if the PositionEffect for the leg is different from that specified for the overall multileg security                                                                                          |                       |   |
| 565                                                                                                                                                                                                   | LegCoveredOrUncovered | N |
| Provide if the CoveredOrUncovered for the leg is different from that specified for the overall multileg security.                                                                                     |                       |   |
| component block                                                                                                                                                                                       |                       |   |
| \<NestedParties>                                                                                                                                                                                      |                       |   |
| Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                |                       |   |
| Used for NestedPartyRole=Leg Clearing Firm/Account, Leg Account/Account Type                                                                                                                          |                       |   |
| 654                                                                                                                                                                                                   | LegRefID              | N |
| Used to identify a specific leg.                                                                                                                                                                      |                       |   |
| 566                                                                                                                                                                                                   | LegPrice              | N |
| Provide only if a price is required for a specific leg. Used for anchoring the overall multileg security price to a specific leg price.                                                               |                       |   |
| 587                                                                                                                                                                                                   | LegSettlType          | N |
| 588                                                                                                                                                                                                   | LegSettlDate          | N |
| 114                                                                                                                                                                                                   | LocateReqd            | N |
| Required for short sell orders                                                                                                                                                                        |                       |   |
| 60                                                                                                                                                                                                    | TransactTime          | Y |
| Time this order request was initiated/released by the trader, trading system, or intermediary.                                                                                                        |                       |   |
| 854                                                                                                                                                                                                   | QtyType               | N |
| component block \<OrderQtyData>                                                                                                                                                                       |                       |   |
| 40                                                                                                                                                                                                    | OrdType               | Y |
| 423                                                                                                                                                                                                   | PriceType             | N |
| 44                                                                                                                                                                                                    | Price                 | N |
| Required for limit OrdTypes. For F/X orders, should be the “all-in” rate (spot rate adjusted for forward points). Can be used to specify a limit price for a pegged order, previously indicated, etc. |                       |   |
| 99                                                                                                                                                                                                    | StopPx                | N |
| 15                                                                                                                                                                                                    | Currency              | N |
| 376                                                                                                                                                                                                   | ComplianceID          | N |
| 377                                                                                                                                                                                                   | SolicitedFlag         | N |


---
March 25, 2003 – DRAFT
# FIX 4.4 - Volume 4

| Field                                                                                                                                                    | Required | Description                                                                                                                                                                                                               |
| -------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IOIid                                                                                                                                                    | N        | Required for Previously Indicated Orders (OrdType=E)                                                                                                                                                                      |
| QuoteID                                                                                                                                                  | N        | Required for Previously Quoted Orders (OrdType=D)                                                                                                                                                                         |
| TimeInForce                                                                                                                                              | N        | Absence of this field indicates Day order                                                                                                                                                                                 |
| EffectiveTime                                                                                                                                            | N        | Can specify the time at which the order should be considered valid                                                                                                                                                        |
| ExpireDate                                                                                                                                               | N        | Conditionally required if TimeInForce = GTD and ExpireTime is not specified.                                                                                                                                              |
| ExpireTime                                                                                                                                               | N        | Conditionally required if TimeInForce = GTD and ExpireDate is not specified.                                                                                                                                              |
| GTBookingInst                                                                                                                                            | N        | States whether executions are booked out or accumulated on a partially filled GT order                                                                                                                                    |
| component block \<CommissionData> N Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                |          |                                                                                                                                                                                                                           |
| OrderCapacity                                                                                                                                            | N        |                                                                                                                                                                                                                           |
| OrderRestrictions                                                                                                                                        | N        |                                                                                                                                                                                                                           |
| CustOrderCapacity                                                                                                                                        | N        |                                                                                                                                                                                                                           |
| ForexReq                                                                                                                                                 | N        | Indicates that broker is requested to execute a Forex accommodation trade in conjunction with the security trade.                                                                                                         |
| SettlCurrency                                                                                                                                            | N        | Required if ForexReq = Y.                                                                                                                                                                                                 |
| BookingType                                                                                                                                              | N        | Method for booking out this order. Used when notifying a broker that an order to be settled by that broker is to be booked out as an OTC derivative (e.g. CFD or similar). Absence of this field implies regular booking. |
| Text                                                                                                                                                     | N        |                                                                                                                                                                                                                           |
| EncodedTextLen                                                                                                                                           | N        | Must be set if EncodedText field is specified and must immediately precede it.                                                                                                                                            |
| EncodedText                                                                                                                                              | N        | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                                                                            |
| PositionEffect                                                                                                                                           | N        | For use in derivatives omnibus accounting                                                                                                                                                                                 |
| CoveredOrUncovered                                                                                                                                       | N        | For use with derivatives, such as options                                                                                                                                                                                 |
| MaxShow                                                                                                                                                  | N        |                                                                                                                                                                                                                           |
| component block \<PegInstructions> N Insert here the set of "PegInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"               |          |                                                                                                                                                                                                                           |
| component block \<DiscretionInstructions> N Insert here the set of "DiscretionInstruction" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |          |                                                                                                                                                                                                                           |
| TargetStrategy                                                                                                                                           | N        | The target strategy of the order                                                                                                                                                                                          |
| TargetStrategyParameters                                                                                                                                 | N        | For further specification of the TargetStrategy                                                                                                                                                                           |
| ParticipationRate                                                                                                                                        | N        | Mandatory for a TargetStrategy=Participate order and specifies the target participation rate.                                                                                                                             |


---

# FIXML Definition for this message

For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)

| CancellationRights    | N | For CIV - Optional                                                            |
| --------------------- | - | ----------------------------------------------------------------------------- |
| MoneyLaunderingStatus | N |                                                                               |
| RegistID              | N | Reference to Registration Instructions message for this Order.                |
| Designation           | N | Supplementary registration information for this Order                         |
| MultiLegRptTypeReq    | N | Indicates the method of execution reporting requested by issuer of the order. |
| Standard Trailer      | Y |                                                                               |

Refer to FIXML element MlegOrdCxlRplc

# CATEGORY: LIST/PROGRAM/BASKET TRADING

# Bid Request -

The BidRequest Message can be used in one of two ways depending on which market conventions are being followed.

In the “Non disclosed” convention (e.g. US/European model) the BidRequest message can be used to request a bid based on the sector, country, index and liquidity information contained within the message itself. In the “Non disclosed” convention the entry repeating group is used to define liquidity of the program. See "Program/Basket/List Trading" for an example.

March 25, 2003 – DRAFT


---

# Bid Request

In the “Disclosed” convention (e.g. Japanese model) the BidRequest message can be used to request bids based on the ListOrderDetail messages sent in advance of BidRequest message. In the “Disclosed” convention the list repeating group is used to define which ListOrderDetail messages a bid is being sort for and the directions of the required bids.

The pair of fields SideValue1 and SideValue2 are used to show the monetary total value in either direction (buy or sell) of the transaction without revealing whether it is the buy-side institution’s intention to buy or sell.

The two repeating groups, NoEntries and NoBidComponents are mutually exclusive and a function of which bidding model is being used. If the “Non Disclosure” method is being used the portfolio of stocks being traded is described by a number of “bid descriptors” entries. If the “Disclosure” Method is being used the portfolio is fully disclosed, except for side, by a number of “list” entries enumerating the lists that list the stocks to be traded. A BidRequest message with BidRequestTransType cancel may be used to indicate to sell side firms that they no longer need to store details of the BidRequest as they have either lost the bid or the List has been canceled.

The format for the Bid Request message is as follows:

| Tag | Field Name             | Req’d | Comments                                                                                                                                     |
| --- | ---------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------- |
|     | Standard Header        | Y     | MsgType = k (lowercase)                                                                                                                      |
| 390 | BidID                  | N     | Required to relate the bid response                                                                                                          |
| 391 | ClientBidID            | Y     |                                                                                                                                              |
| 374 | BidRequestTransType    | Y     | Identifies the Bid Request message transaction type                                                                                          |
| 392 | ListName               | N     |                                                                                                                                              |
| 393 | TotNoRelatedSym        | Y     | ~~TotalNumSecu~~~~rities~~                                                                                                                   |
| 394 | BidType                | Y     | e.g. “Non Disclosed”, “Disclosed”, No Bidding Process                                                                                        |
| 395 | NumTickets             | N     | Total number of tickets/allocations assuming fully executed                                                                                  |
| 15  | Currency               | N     | Used to represent the currency of monetary amounts.                                                                                          |
| 396 | SideValue1             | N     | Expressed in Currency                                                                                                                        |
| 397 | SideValue2             | N     | Expressed in Currency                                                                                                                        |
| 398 | NoBidDescriptors       | N     | Used if BidType=”Non Disclosed”                                                                                                              |
| 399 | BidDescriptorType      | N     | Required if NoBidDescriptors > 0. Must be first field in repeating group.                                                                    |
| 400 | BidDescriptor          | N     |                                                                                                                                              |
| 401 | SideValueInd           | N     | Refers to the SideValue1 or SideValue2. These are used as opposed to Buy or Sell so that the basket can be quoted either way as Buy or Sell. |
| 404 | LiquidityValue         | N     | Value between LiquidityPctLow and LiquidityPctHigh in Currency                                                                               |
| 441 | LiquidityNumSecurities | N     | Number of Securities between LiquidityPctLow and LiquidityPctHigh in Currency                                                                |
| 402 | LiquidityPctLow        | N     | Liquidity indicator or lower limit if LiquidityNumSecurities > 1                                                                             |
| 403 | LiquidityPctHigh       | N     | Upper liquidity indicator if LiquidityNumSecurities > 1                                                                                      |

March 25, 2003 – DRAFT

134

FIX 4.4 - Volume 4


---

# FIX 4.4 - Volume 4

March 25, 2003 – DRAFT


| 405 | EFPTrackingError    | N | Eg Used in EFP (Exchange For Physical) trades                                                                                                                   | 12% |
| --- | ------------------- | - | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | --- |
| 406 | FairValue           | N | Used in EFP trades                                                                                                                                              |     |
| 407 | OutsideIndexPct     | N | Used in EFP trades                                                                                                                                              |     |
| 408 | ValueOfFutures      | N | Used in EFP trades                                                                                                                                              |     |
| 420 | NoBidComponents     | N | Used if BidType=”Disclosed”                                                                                                                                     |     |
| 66  | ListID              | N | Required if NoBidComponents > 0. Must be first field in repeating group.                                                                                        |     |
| 54  | Side                | N | When used in request for a “Disclosed” bid indicates that bid is required on assumption that SideValue1 is Buy or Sell. SideValue2 can be derived by inference. |     |
| 336 | TradingSessionID    | N | Indicates off-exchange type activities for Detail.                                                                                                              |     |
| 625 | TradingSessionSubID | N |                                                                                                                                                                 |     |
| 430 | NetGrossInd         | N | Indicates Net or Gross for selling Detail.                                                                                                                      |     |
| 63  | SettlType           | N |                                                                                                                                                                 |     |
| 64  | SettlDate           | N | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                         |     |
| 1   | Account             | N |                                                                                                                                                                 |     |
| 660 | AcctIDSource        | N |                                                                                                                                                                 |     |
| 409 | LiquidityIndType    | N |                                                                                                                                                                 |     |
| 410 | WtAverageLiquidity  | N | Overall weighted average liquidity expressed as a % of average daily volume                                                                                     |     |
| 411 | ExchangeForPhysical | N |                                                                                                                                                                 |     |
| 412 | OutMainCntryUIndex  | N | % value of stocks outside main country in Currency                                                                                                              |     |
| 413 | CrossPercent        | N | % of program that crosses in Currency                                                                                                                           |     |
| 414 | ProgRptReqs         | N |                                                                                                                                                                 |     |
| 415 | ProgPeriodInterval  | N | Time in minutes between each ListStatus report sent by SellSide. Zero means don’t send status.                                                                  |     |
| 416 | IncTaxInd           | N | Net/Gross                                                                                                                                                       |     |
| 121 | ForexReq            | N | Is foreign exchange required                                                                                                                                    |     |
| 417 | NumBidders          | N | Indicates the total number of bidders on the list                                                                                                               |     |
| 75  | TradeDate           | N |                                                                                                                                                                 |     |
| 418 | BidTradeType        | Y |                                                                                                                                                                 |     |
| 419 | BasisPxType         | Y |                                                                                                                                                                 |     |
| 443 | StrikeTime          | N | Used when BasisPxType = “C”                                                                                                                                     |     |
| 58  | Text                | N |                                                                                                                                                                 |     |
| 354 | EncodedTextLen      | N | Must be set if EncodedText field is specified and must immediately precede it.                                                                                  |     |
| 355 | EncodedText         | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                  |     |



---
Standard Trailer Y

# FIXML Definition for this message

– see http://www.fixprotocol.org for details

|                                                                                    |                                                                                  |
| ---------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
|                                                                                    | "BidID?,ClientBidID,BidRequestTransType,ListName?,TotalNumSecurities,BidType,    |
| NumTickets?,Currency?,SideValue1?,SideValue2?,BidDescriptorList,BidComponentList?, | LiquidityIndType?,WtAverageLiquidity?,ExchangeForPhysical?, OutMainCntryUIndex?, |
| CrossPercent?,ProgRptReqs?,ProgPeriodInterval?,IncTaxInd?,ForexReq?,NumBidders?,   | TradeDate?,TradeType,BasisPxType,StrikeTime?,Text?,EncodedTextGroup?             |
| %BidRequestCustom;" >                                                              |                                                                                  |

DataType CDATA #FIXED 'String'

Value CDATA #FIXED 'k' >

Refer to FIXML element BidReq

March 25, 2003 – DRAFT

136

FIX 4.4 - Volume 4


---
Bid Response -

The Bid Response message can be used in one of two ways depending on which market conventions are being followed.

In the “Non disclosed” convention the Bid Response message can be used to supply a bid based on the sector, country, index and liquidity information contained within the corresponding bid request message. See "Program/Basket/List Trading" for an example.

In the “Disclosed” convention the Bid Response message can be used to supply bids based on the List Order Detail messages sent in advance of the corresponding Bid Request message.

# The format for the Bid Response message is as follows:

| Tag | Field Name          | Req’d | Comments                                                                                                                                                                                       |
| --- | ------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     | Standard Header     | Y     | MsgType = l (lowercase L)                                                                                                                                                                      |
| 390 | BidID               | N     |                                                                                                                                                                                                |
| 391 | ClientBidID         | N     |                                                                                                                                                                                                |
| 420 | NoBidComponents     | Y     | Number of bid repeating groups                                                                                                                                                                 |
|     | component           | block | Y                                                                                                                                                                                              |
|     | \<CommissionData>   |       | Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                                                          |
|     |                     |       | First element of price. Required if NoBidComponents > 0.                                                                                                                                       |
| 66  | ListID              | N     |                                                                                                                                                                                                |
| 421 | Country             | N     | ISO Country Code                                                                                                                                                                               |
| 54  | Side                | N     | When used in response to a “Disclosed” request indicates whether SideValue1 is Buy or Sell. SideValue2 can be derived by inference.                                                            |
| 44  | Price               | N     | Second element of price                                                                                                                                                                        |
| 423 | PriceType           | N     |                                                                                                                                                                                                |
| 406 | FairValue           | N     | The difference between the value of a future and the value of the underlying equities after allowing for the discounted cash flows associated with the underlying stocks (E.g. Dividends etc). |
| 430 | NetGrossInd         | N     | Net/Gross                                                                                                                                                                                      |
| 63  | SettlType           | N     |                                                                                                                                                                                                |
| 64  | SettlDate           | N     | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                                                        |
| 336 | TradingSessionID    | N     |                                                                                                                                                                                                |
| 625 | TradingSessionSubID | N     |                                                                                                                                                                                                |
| 58  | Text                | N     |                                                                                                                                                                                                |

March 25, 2003 – DRAFT

137

FIX 4.4 - Volume 4


---

# FIX 4.4 - Volume 4

# 354 EncodedTextLen

N Must be set if EncodedText field is specified and must immediately precede it.

# 355 EncodedText

N Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.

# Standard Trailer

Y

# FIXML Definition for this message

– see http://www.fixprotocol.org for details

&#x3C;!ENTITY % BidResponseCustom "">
&#x3C;!ENTITY % BidResponseContent "BidID?,ClientBidID?,BidRespComponentList %BidResponseCustom;" >
&#x3C;!ELEMENT BidResponse (%BidResponseContent;)>
&#x3C;!ATTLIST BidResponse FIXTag CDATA #FIXED '35'>
DataType CDATA #FIXED 'String'>
Value CDATA #FIXED 'l' >Refer to FIXML element BidRsp

March 25, 2003 – DRAFT


---
New Order - List
The New Order List Message can be used in one of two ways depending on which market conventions are being followed.

In the “Non disclosed” convention the New Order - List message is sent after the bidding process has been completed, by telephone or electronically. The New Order - List message enumerates the stocks, quantities, direction for the trade and may contain pre-allocation information.

This message may also be used as the first message for the transmission of a program trade where the bidding process has been done by means other than FIX. In this scenario the messages may either be used as a staging process, in which case the broker will start execution once either a ListExecute is received or for immediate execution, in which case the orders will be executed on receipt.

In the “Disclosed” convention the New Order - List message is sent before the bidding process is started, by telephone or electronically. The New Order - List message enumerates the stocks and quantities from the bidding process, and may contain pre-allocation information. The direction of the trade is disclosed after the bidding process is completed.

Where multiple waves of a program trade are submitted by an institution or retail intermediaries, as a series of separate lists, to a broker ClOrdLinkID may be used to link the orders together. See "Program/Basket/List Trading" for examples.

The New Order – List message type may also be used by institutions or retail intermediaries wishing to electronically submit multiple Collective Investment Vehicle orders to a broker or fund manager for execution. See VOLUME 7 - "PRODUCT: COLLECTIVE INVESTMENT VEHICLES"

The format for the New Order - List message is as follows:

| Tag | Field Name            |
| --- | --------------------- |
|     | Standard Header       |
| 66  | ListID                |
| 390 | BidID                 |
| 391 | ClientBidID           |
| 414 | ProgRptReqs           |
| 394 | BidType               |
| 415 | ProgPeriodInterval    |
| 480 | CancellationRights    |
| 481 | MoneyLaunderingStatus |
| 513 | RegistID              |
| 433 | ListExecInstType      |

March 25, 2003 – DRAFT

| Req'd | Comments                                                                              |
| ----- | ------------------------------------------------------------------------------------- |
| Y     | MsgType = E                                                                           |
| Y     | Must be unique, by customer, for the day                                              |
| N     | Should refer to an earlier program if bidding took place.                             |
| N     |                                                                                       |
| N     | e.g. Non Disclosed Model, Disclosed Model, No Bidding Process                         |
| N     |                                                                                       |
| N     | For CIV - Optional                                                                    |
| N     |                                                                                       |
| N     | Reference to Registration Instructions message applicable to all Orders in this List. |
| N     | Controls when execution should begin For CIV Orders                                   |

139 FIX 4.4 - Volume 4


---

March 25, 2003 – DRAFT

# FIX 4.4 - Volume 4

| 69                         | ListExecInst               | N | Free-form text.                                                                                                                        |
| -------------------------- | -------------------------- | - | -------------------------------------------------------------------------------------------------------------------------------------- |
| 352                        | EncodedListExecInstLen     | N | Must be set if EncodedListExecInst field is specified and must immediately precede it.                                                 |
| 353                        | EncodedListExecInst        | N | Encoded (non-ASCII characters) representation of the ListExecInst field in the encoded format specified via the MessageEncoding field. |
| 765                        | AllowableOneSidednessPct   | N | The maximum percentage that execution of one side of a program trade can exceed execution of the other.                                |
| 766                        | AllowableOneSidednessValue | N | The maximum amount that execution of one side of a program trade can exceed execution of the other.                                    |
| 767                        | AllowableOneSidednessCurr  | N | The currency that AllowableOneSidedness is expressed in if AllowableOneSidednessValue is used.                                         |
| 68                         | TotNoOrders                | Y | Used to support fragmentation. Sum of NoOrders across all messages with the same ListID.                                               |
| 893                        | LastFragment               | N | Indicates if this message is the last of a fragmented set of messages.                                                                 |
| 73                         | NoOrders                   | Y | Number of orders in this message (number of repeating groups to follow).                                                               |
| 11                         | ClOrdID                    | Y | Must be the first field in the repeating group.                                                                                        |
| 526                        | SecondaryClOrdID           | N |                                                                                                                                        |
| 67                         | ListSeqNo                  | Y | Order number within the list.                                                                                                          |
| 583                        | ClOrdLinkID                | N |                                                                                                                                        |
| 160                        | SettlInstMode              | N |                                                                                                                                        |
| component block \<Parties> |                            | N | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES".                  |
| 229                        | TradeOriginationDate       | N |                                                                                                                                        |
| 75                         | TradeDate                  | N |                                                                                                                                        |
| 1                          | Account                    | N |                                                                                                                                        |
| 660                        | AcctIDSource               | N |                                                                                                                                        |
| 581                        | AccountType                | N |                                                                                                                                        |
| 589                        | DayBookingInst             | N |                                                                                                                                        |
| 590                        | BookingUnit                | N |                                                                                                                                        |
| 70                         | AllocID                    | N | Use to assign an ID to the block of individual preallocations.                                                                         |
| 591                        | PreallocMethod             | N |                                                                                                                                        |
| 78                         | NoAllocs                   | N | Indicates number of pre-trade allocation accounts to follow.                                                                           |
| 79                         | AllocAccount               | N | Required if NoAllocs > 0. Must be the first field in the repeating group.                                                              |
| 661                        | AllocAcctIDSource          | N |                                                                                                                                        |


140

---

# FIX 4.4 - Volume 4

# Allocations

|                         | 736                  | AllocSettlCurrency | N                                                                                                                                                                      |
| ----------------------- | -------------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|                         | 467                  | IndividualAllocID  | N                                                                                                                                                                      |
| component               | block                | N                  | Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |
| 80                      | AllocQty             | N                  |                                                                                                                                                                        |
| 63                      | SettlType            | N                  |                                                                                                                                                                        |
| 64                      | SettlDate            | N                  | Takes precedence over SettlType value and conditionally required/omitted for specific SettlType values.                                                                |
| 544                     | CashMargin           | N                  |                                                                                                                                                                        |
| 635                     | ClearingFeeIndicator | N                  |                                                                                                                                                                        |
| 21                      | HandlInst            | N                  |                                                                                                                                                                        |
| 18                      | ExecInst             | N                  | Can contain multiple instructions, space delimited. If OrdType=P, exactly one of the following values (ExecInst = L, R, M, P, O, T, or W) must be specified.           |
| 110                     | MinQty               | N                  |                                                                                                                                                                        |
| 111                     | MaxFloor             | N                  |                                                                                                                                                                        |
| 100                     | ExDestination        | N                  |                                                                                                                                                                        |
| 386                     | NoTradingSessions    | N                  |                                                                                                                                                                        |
| 336                     | TradingSessionID     | N                  | First field in repeating group. Required if NoTradingSessions > 0.                                                                                                     |
| 625                     | TradingSessionSubID  | N                  |                                                                                                                                                                        |
| 81                      | ProcessCode          | N                  |                                                                                                                                                                        |
| component               | block                | Y                  | Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                          |
| 711                     | NoUnderlyings        | N                  | Number of Underlyings                                                                                                                                                  |
| component               | block                | N                  | Must be provided if Number of underlyings > 0                                                                                                                          |
| \<UnderlyingInstrument> |                      |                    |                                                                                                                                                                        |
| 140                     | PrevClosePx          | N                  | Useful for verifying security identification                                                                                                                           |
| 54                      | Side                 | Y                  | Note: to indicate the side of SideValue1 or SideValue2, specify Side=Undisclosed and SideValueInd=either the SideValue1 or SideValue2 indicator.                       |
| 401                     | SideValueInd         | N                  | Refers to the SideValue1 or SideValue2. These are used as opposed to Buy or Sell so that the basket can be quoted either way as Buy or Sell.                           |
| 114                     | LocateReqd           | N                  | Required for short sell orders                                                                                                                                         |
| 60                      | TransactTime         | N                  |                                                                                                                                                                        |
| component               | block                | N                  | Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                     |
| 854                     | QtyType              | N                  |                                                                                                                                                                        |

# March 25, 2003 – DRAFT


---

# OrderQtyData

Insert here the set of "OrderQtyData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

| 40  | OrdType   | N |
| --- | --------- | - |
| 423 | PriceType | N |
| 44  | Price     | N |
| 99  | StopPx    | N |

# SpreadOrBenchmarkCurveData

Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

# YieldData

Insert here the set of "YieldData" (yield-related) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

| 15  | Currency      | N |
| --- | ------------- | - |
| 376 | ComplianceID  | N |
| 377 | SolicitedFlag | N |
| 23  | IOIid         | N |
| 117 | QuoteID       | N |
| 59  | TimeInForce   | N |
| 168 | EffectiveTime | N |
| 432 | ExpireDate    | N |
| 126 | ExpireTime    | N |
| 427 | GTBookingInst | N |

# CommissionData

Insert here the set of "CommissionData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"

| 528 | OrderCapacity     | N |
| --- | ----------------- | - |
| 529 | OrderRestrictions | N |
| 582 | CustOrderCapacity | N |
| 121 | ForexReq          | N |
| 120 | SettlCurrency     | N |
| 775 | BookingType       | N |
| 58  | Text              | N |
| 354 | EncodedTextLen    | N |

March 25, 2003 – DRAFT

142

FIX 4.4 - Volume 4


---

FIX 4.4 - Volume 4

# 355 EncodedText

N Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.

# 193 SettlDate2

N Can be used with OrdType = “Forex - Swap” to specify the “value date” for the future portion of a F/X swap.

# 192 OrderQty2

N Can be used with OrdType = “Forex - Swap” to specify the order quantity for the future portion of a F/X swap.

# 640 Price2

N Can be used with OrdType = “Forex - Swap” to specify the price for the future portion of a F/X swap which is also a limit order. For F/X orders, should be the “all-in” rate (spot rate adjusted for forward points).

# 77 PositionEffect

N

# 203 CoveredOrUncovered

N

# 210 MaxShow

N

# component

block N Insert here the set of "PegInstruction" fields defined in  "COMMON COMPONENTS OF APPLICATION MESSAGES"</peginstructions>

# component

block N Insert here the set of "DiscretionInstruction" fields <discretioninstructions> defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"</discretioninstructions>

# 847 TargetStrategy

N The target strategy of the order

# 848 TargetStrategyParameters

N For further specification of the TargetStrategy

# 849 ParticipationRate

N Mandatory for a TargetStrategy=Participate order and specifies the target participation rate. For other order types optionally specifies a volume limit (i.e. do not be more than this percent of the market volume)

# 494 Designation

N Supplementary registration information for this Order within the List

# Standard Trailer

Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;s>&#x3C;!ENTITY % NewOrderListCustom "">&#x3C;/s>

&#x3C;s>&#x3C;!ENTITY % NewOrderListContent

&#x3C;s>"ListID,BidID?,ClientBidID?,ProgRptReqs?,BidType,ProgPeriodInterval?,CancellationRights?,

MoneyLaunderingStatus?,RegistID?,ListExecInstType?,ListExecInst?,EncodedListExecInstGroup?,

TotNoOrders,OrderList %NewOrderListCustom;" >&#x3C;/s>

&#x3C;s>&#x3C;!ELEMENT NewOrderList (%NewOrderListContent;)>&#x3C;/s>

&#x3C;s>&#x3C;!ATTLIST NewOrderList FIXTag

CDATA #FIXED '35'>&#x3C;/s>

&#x3C;s>DataType CDATA #FIXED 'String'>&#x3C;/s>

&#x3C;s>Value&#x3C;/s> &#x3C;s>CDATA #FIXED 'E' >&#x3C;/s> Refer to FIXML element NewOrdList

March 25, 2003 – DRAFT


143

---

# List Strike Price


The strike price message is used to exchange strike price information for principal trades. It can also be used to exchange reference prices for agency trades.

The format for the List Strike Price message is as follows:

| Tag | Field Name                              | Req’d | Comments                                                                                                                                                                          |
| --- | --------------------------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     | Standard Header                         | Y     | MsgType = m (lowercase)                                                                                                                                                           |
| 66  | ListID                                  | Y     |                                                                                                                                                                                   |
| 422 | TotNoStrikes                            | Y     | Used to support fragmentation. Sum of NoStrikes across all messages with the same ListID.                                                                                         |
| 893 | LastFragment                            | N     | Used to indicate if message is the last of a fragmented set of messages                                                                                                           |
| 428 | NoStrikes                               | Y     | Number of strike price entries                                                                                                                                                    |
|     | component block \<Instrument>           | Y     | Insert here the set of "Instrument" (symbology) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES". Required if NoStrikes > 0. Must be first field in repeating group. |
| 711 | NoUnderlyings                           | N     | Number of underlyings                                                                                                                                                             |
|     | component block \<UnderlyingInstrument> | N     | Must be provided if Number of underlyings > 0                                                                                                                                     |
| 140 | PrevClosePx                             | N     | Useful for verifying security identification                                                                                                                                      |
| 11  | ClOrdID                                 | N     | Can use client order identifier or the symbol and side to uniquely identify the stock in the list.                                                                                |
| 526 | SecondaryClOrdID                        | N     |                                                                                                                                                                                   |
| 54  | Side                                    | N     |                                                                                                                                                                                   |
| 44  | Price                                   | Y     |                                                                                                                                                                                   |
| 15  | Currency                                | N     |                                                                                                                                                                                   |
| 58  | Text                                    | N     |                                                                                                                                                                                   |
| 354 | EncodedTextLen                          | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                                                                    |
| 355 | EncodedText                             | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                                    |
|     | Standard Trailer                        | Y     |                                                                                                                                                                                   |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;s>&#x3C;!ENTITY % ListStrikePriceCustom "">&#x3C;/s>

&#x3C;s>&#x3C;!ENTITY % ListStrikePriceContent "ListID,TotNoStrikes,StrikeList %ListStrikePriceCustom;" >&#x3C;/s>

&#x3C;s>&#x3C;!ELEMENT ListStrikePrice (%ListStrikePriceContent;)>&#x3C;/s>

March 25, 2003 – DRAFT 144 FIX 4.4 - Volume 4



---

March 25, 2003 – DRAFT

# FIX 4.4 - Volume 4

~~&#x3C;!ATTLIST ListStrikePrice FIXTag~~
~~CDATA #FIXED '35'~~
~~DataType CDATA #FIXED 'String'~~
~~Value~~
~~CDATA #FIXED 'm' >~~

Refer to FIXML element ListStrkPx


145

---

# List Status

The list status message is issued as the response to a List Status Request message sent in an unsolicited fashion by the sell-side. It indicates the current state of the orders within the list as they exist at the broker's site. This message may also be used to respond to the List Cancel Request. Orders within the list are statused at the summary level. Individual executions are not reported, rather, the current state of the order is reported. The message contains repeating fields for each. The relative position of the repeating fields is important in this message, i.e. each instance of ClOrdID, CumQty, LeavesQty, CxlQty and AvgPx must be in the order shown below.

# Description of ListOrderStatus field values:

- “InBiddingProcess”: indicates that a list has been received and is being evaluated for pricing. It is envisaged that this status will only be used with the "Disclosed" List Order Trading model.
- “ReceivedForExecution”: indicates that a list has been received and the sell side is awaiting the instruction to start working the trade. It is envisaged that this status will be used under both models.
- “Executing”: indicates that a list has been received and the sell side is working it.
- “Canceling”: indicates that a List Cancel Message has been received and the sell side is in the process of pulling any orders that were being worked. The status of individual order can be found out from the detail repeating group.
- “AllDone”: indicates that a list has been executed as far as possible for the day. This would also apply if a list has been previously cancelled. The status of individual order can be determined from the detail repeating group.
- “Alert”: used whenever any of the individual orders have a status that requires something to be done. For instance, an alert would be used when a buy-side firm has submitted a list that has individual stock rejects that have not been addressed.
- “Rejected”: used when a response cannot be generated. For example, when the ListID is not recognized. The text field should include an explanation of why the Request has been rejected.

# The list status message format is as follows:

| Tag | Field Name               | Req'd | Comments                                                                                                          |
| --- | ------------------------ | ----- | ----------------------------------------------------------------------------------------------------------------- |
|     | Standard Header          | Y     | MsgType = N                                                                                                       |
| 66  | ListID                   | Y     |                                                                                                                   |
| 429 | ListStatusType           | Y     |                                                                                                                   |
| 82  | NoRpts                   | Y     | Total number of messages required to status complete list.                                                        |
| 431 | ListOrderStatus          | Y     |                                                                                                                   |
| 83  | RptSeq                   | Y     | Sequence number of this report message.                                                                           |
| 444 | ListStatusText           | N     |                                                                                                                   |
| 445 | EncodedListStatusTextLen | N     | Must be set if EncodedListStatusText field is specified and must immediately precede it.                          |
| 446 | EncodedListStatusText    | N     | Encoded (non-ASCII characters) representation of the ListStatusText field in the encoded format specified via the |


---

# FIX 4.4 - Volume 4

# MessageEncoding field.

| 60               | TransactTime     | N |                                                                                                                                |   |   |
| ---------------- | ---------------- | - | ------------------------------------------------------------------------------------------------------------------------------ | - | - |
| 68               | TotNoOrders      | Y | Used to support fragmentation. Sum of NoOrders across all messages with the same ListID.                                       |   |   |
| 893              | LastFragment     | N | Indicates if the message is the last of a fragmented set of messages                                                           |   |   |
| 73               | NoOrders         | Y | Number of orders statused in this message, i.e. number of repeating groups to follow.                                          |   |   |
| 11               | ClOrdID          | Y |                                                                                                                                |   |   |
| 526              | SecondaryClOrdID | N |                                                                                                                                |   |   |
| 14               | CumQty           | Y |                                                                                                                                |   |   |
| 39               | OrdStatus        | Y |                                                                                                                                |   |   |
| 636              | WorkingIndicator | N | For optional use with OrdStatus = 0 (New)                                                                                      |   |   |
| 151              | LeavesQty        | Y | Quantity open for further execution. LeavesQty = OrderQty - CumQty.                                                            |   |   |
| 84               | CxlQty           | Y |                                                                                                                                |   |   |
| 6                | AvgPx            | Y |                                                                                                                                |   |   |
| 103              | OrdRejReason     | N | Used if the order is rejected                                                                                                  |   |   |
| 58               | Text             | N |                                                                                                                                |   |   |
| 354              | EncodedTextLen   | N | Must be set if EncodedText field is specified and must immediately precede it.                                                 |   |   |
| 355              | EncodedText      | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |   |   |
| Standard Trailer |                  |   | Y                                                                                                                              |   |   |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;s>&#x3C;!ENTITY % ListStatusCustom "">&#x3C;/s>

&#x3C;s>&#x3C;!ENTITY % ListStatusContent "ListID,ListStatusType,NoRpts,ListOrderStatus,RptSeq,ListStatusText?,&#x3C;/s>

&#x3C;s>EncodedListStatusTextGroup?,TransactTime?,&#x3C;/s>

&#x3C;s>TotNoOrders,StatusOrderList %ListStatusCustom;" >&#x3C;/s>

&#x3C;s>&#x3C;ELEMENT ListStatus (%ListStatusContent;)>&#x3C;/s>

&#x3C;s>&#x3C;ATTLIST ListStatus FIXTag&#x3C;/s> &#x3C;!ENTITY % ListStatusContent "ListID,ListStatusType,NoRpts,ListOrderStatus,RptSeq,ListStatusText?,&#x3C;/s>

&#x3C;s>DataType CDATA #FIXED 'String'>&#x3C;/s>

&#x3C;s>Value&#x3C;/s> &#x3C;!ENTITY % ListStatusContent "ListID,ListStatusType,NoRpts,ListOrderStatus,RptSeq,ListStatusText?,&#x3C;/s>

&#x3C;s>CDATA #FIXED 'N' >Refer to ListStat


---

# List Execute

The List Execute message type is used by institutions to instruct the broker to begin execution of a previously submitted list. This message may or may not be used, as it may be mirroring a phone conversation. The format for the list execute message is as follows:

| Tag | Field Name       | Req'd | Comments                                                                                                                       |
| --- | ---------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
|     | Standard Header  | Y     | MsgType = L                                                                                                                    |
| 66  | ListID           | Y     | Must be unique, by customer, for the day                                                                                       |
| 391 | ClientBidID      | N     | Used with BidType=Disclosed to provide the sell side the ability to determine the direction of the trade to execute.           |
| 390 | BidID            | N     |                                                                                                                                |
| 60  | TransactTime     | Y     | Time this order request was initiated/released by the trader or trading system.                                                |
| 58  | Text             | N     |                                                                                                                                |
| 354 | EncodedTextLen   | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355 | EncodedText      | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
|     | Standard Trailer | Y     |                                                                                                                                |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;!ENTITY % ListExecuteCustom "">

&#x3C;!ENTITY % ListExecuteContent "ListID,ClientBidID?,BidID?,TransactTime,Text?,EncodedTextGroup? %ListExecuteCustom;">

&#x3C;!ELEMENT ListExecute (%ListExecuteContent;)>

&#x3C;!ATTLIST ListExecute FIXTag CDATA #FIXED '35'>

DataType CDATA #FIXED 'String'>

Value CDATA #FIXED 'L' >Refer to FIXML element ListExct

March 25, 2003 – DRAFT

148 FIX 4.4 - Volume 4


---

List Cancel Request


The List Cancel Request message type is used by institutions wishing to cancel previously submitted lists either before or during execution. After the list has been staged with the broker, it can be canceled via the submission of the List Cancel message. If the list has not yet been submitted for execution, the List Cancel message will instruct the broker not to execute it. If the list is being executed, the List Cancel message should trigger the broker's system to generate cancel requests for the remaining quantities of each order within the list. Individual orders within the list can be canceled via the Order Cancel Request message. The List Status message type is used by the recipient of the List Cancel Request to communicate the status of the List Cancel Request.

The format for the list - cancel request message is as follows:

| Tag | Field Name           | Req'd | Comments                                                                                                                       |
| --- | -------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
|     | Standard Header      | Y     | MsgType = K                                                                                                                    |
| 66  | ListID               | Y     |                                                                                                                                |
| 60  | TransactTime         | Y     | Time this order request was initiated/released by the trader or trading system.                                                |
| 229 | TradeOriginationDate | N     |                                                                                                                                |
| 75  | TradeDate            | N     |                                                                                                                                |
| 58  | Text                 | N     |                                                                                                                                |
| 354 | EncodedTextLen       | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355 | EncodedText          | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
|     | Standard Trailer     | Y     |                                                                                                                                |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;!ENTITY % ListCancelRequestCustom "">
&#x3C;!ENTITY % ListCancelRequestContent "ListID,TransactTime,Text?,EncodedTextGroup?>
%ListCancelRequestCustom;" >
&#x3C;!ELEMENT ListCancelRequest (%ListCancelRequestContent;)>
&#x3C;!ATTLIST ListCancelRequest FIXTag CDATA #FIXED '35'>
DataType CDATA #FIXED 'String'>
Value CDATA #FIXED 'K' >ListCxlReq

March 25, 2003 – DRAFT


149 FIX 4.4 - Volume 4

---

CURRENT_PAGE_RAW_OCR_TEXT


# List Status Request

The list status request message type is used by institutions to instruct the broker to generate status messages for a list.

The format for the list - status request message is as follows:

| Tag | Field Name       | Req'd | Comments                                                                                                                       |
| --- | ---------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
|     | Standard Header  | Y     | MsgType = M                                                                                                                    |
| 66  | ListID           | Y     |                                                                                                                                |
| 58  | Text             | N     |                                                                                                                                |
| 354 | EncodedTextLen   | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355 | EncodedText      | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
|     | Standard Trailer | Y     |                                                                                                                                |

FIXML Definition for this message – see http://www.fixprotocol.org for details

&#x3C;!ENTITY % ListStatusRequestCustom "">
&#x3C;!ENTITY % ListStatusRequestContent "ListID,Text?,EncodedTextGroup? %ListStatusRequestCustom;" >
&#x3C;!ELEMENT ListStatusRequest (%ListStatusRequestContent;)>
&#x3C;!ATTLIST ListStatusRequest FIXTag CDATA #FIXED '35'>
&#x3C;!ATTLIST ListStatusRequest DataType CDATA #FIXED 'String'>
&#x3C;!ATTLIST ListStatusRequest Value CDATA #FIXED 'M'>

Refer to FIXML element ListStatReq

March 25, 2003 – DRAFT

150

FIX 4.4 - Volume 4



---
# Fragmentation for List Order Messages

The messages used in program trading support fragmentation for the same reason and in the same way as some other FIX messages (e.g. Mass Quote). If there are too many entries within a repeating group to fit into one physical message, then the entries can be continued in another message by repeating all of the top level information and then specifying the number of entries in the continued message. A “Total Entries” field is provided to specify the total number of entries in a repeating group which is split over multiple messages. This permits, but does not require, a receiving application to react in a stateful manner where it can determine if it has received all entries in a repeating group before carrying out some action. However, the overall approach to fragmentation is to permit each message to be processed in a stateless manner as it is received. Each message should contain enough information to have the entries applied to a system without requiring the next message if fragmentation has occurred. Also, a continued message should not require any information from the previous message. The messages that support fragmentation and the repeating groups supporting it are listed in the table below.

| Message           | “Total Entries” field | Repeating group that may be fragmented                                                        |
| ----------------- | --------------------- | --------------------------------------------------------------------------------------------- |
| New Order - List  | TotNoOrders           | Orders repeating group following the NoOrders field in the message definition table           |
| List Strike Price | TotNoStrikes          | Strike price repeating group following the NoStrikes field in the message definition table    |
| List Status       | TotNoOrders           | Status per order repeating group following the NoOrders field in the message definition table |

Maximum message size for fragmentation purposes can be determined by using the optional MaxMessageSize field in the Logon message or by mutual agreement between counterparties.

Note: The TotNoOrders field has been added to the List Status message to support fragmentation in same way as other FIX messages. The NoRpts and RptSeq fields are preserved for backwards compatibility with previous versions of FIX which supported a stateful form of fragmentation.

March 25, 2003 – DRAFT

151

FIX 4.4 - Volume 4
---
Program/Basket/List Trading

# Overview

A set of messages allow for the automation of program trading. While it is hoped that the message set is comprehensive enough, to automate the complete cycle, it is expected that not all messages will be used in all transactions. Although the message set may appear to be quite complex at first glance, most of the complexity arises from developing one message set that can be used to support two different business models for list trading. The two models, the “Disclosed” and “Non Disclosed” models, are described in the next two sections. The “Disclosed” model is commonly used in Japan while the “Non Disclosed” model is commonly used in Europe and America.

# “Non Disclosed” model (e.g. US/European)

The buy-side details to the sell-side information about the sector, country and potential market impact of the stocks to be bought or sold. Using this information the sell-side firms bid for the trade. If successful the buy-side firm gives the sell-side firm a detailed list of the stocks to be traded and the sell-side firm executes the trades. The important point in the “Non Disclosed” model is that the stocks in the list are not disclosed until a particular sell-side firm has won the portfolio.

# “Disclosed” model (e.g. Japanese)

The buy-side details the exact stocks and sizes to be traded and the sell-side firm offers the buy-side firm a two-way price, to buy or to sell the indicated stocks. The buy-side firm then tells the successful sell-side firm to buy or sell on its behalf. The important point in the “Disclosed” model is that all sell-side firms see all of the stocks and quantities in the portfolio during the bidding phase regardless of whether or not they win the business.

The New Order - List message can be used, with the side omitted as part of the bidding process, as is the practice in “Disclosed” model or once the bidding has been completed to exchange the list of stocks that make up the program to be traded. Pre-trade allocation is handled via a repeating group within the repeating order block.

Order modification and cancelation of a portfolio is a major change to the agreement between the buy-side and sell-side firms and as such this change should be conducted by telephone. If an automated route for dealing with amendment/cancelation is required then the existing messages can be used – List Cancel, Order Cancel/Replace Request.

The New Order - List message is based on the single order message and message flows for canceling a single stock line within a program trade should be those used to support order cancelation in the single order model (e.g. Order Cancel/Replace Request, etc). The ListID in those systems should be used to assist in identifying the order as part of a list trade. Similarly, the Order Status Request message can be used to request the status of a single order in a portfolio trade.

The List Strike Price Message details the prices that a principal trade is being executed at. In some transactions this appears to be generated by the sell side and checked by the buy-side, in others the reverse is true, and in other cases this information is not passed until the final execution reports.

Pre-trade allocation is much more common place in the program trading community than it is in block trading. For the purposes of pre-trade allocation, a repeating group to specify AllocAccount and AllocQty has been added to the order message. It is assumed that participants will use either the FIX allocation messages and message flows for post trade allocation or their existing allocation systems. (e.g. in the event of a pre-allocation basket trade).

March 25, 2003 – DRAFT                              152                                FIX 4.4 - Volume 4
---

At any stage in the processing of a list message the buy-side may request the status of the list from the sell-side using the List Status Request message. The sell-side responds with a List Status Response message. The sell-side can also send the List Status Response message in an unsolicited fashion according to the requirements passed in the bidding phase or in the List message. The List Status Response message provides summary detail about each of the orders in the List. The sell-side should acknowledge any list request from the buy-side with a List Status Response message providing the current state.

Once the portfolio has been executed by the sell-side and a List Status Response message has been sent to the buy-side indicating “DONE” for each of the orders in the List, the list can be allocated. If pre-allocation information was provided with the original orders and the orders were fully executed then the allocation information is already known to the sell-side. If the pre-trade allocations are no longer appropriate post trade allocation may be performed either using FIX Allocation messages or existing allocation systems.

# Message Flow Diagrams

# Overview of logical stages

| 1) Buy Side  | 2) Buy-Side          | 3) Selected | 4) Sell-Side     |
| ------------ | -------------------- | ----------- | ---------------- |
| Selects Sell | has chosen Sell-Side | has list    | begins execution |

The diagram above shows the logical stages involved in the execution of a program trade.

| Transition | Description                                                                                                                                                                                                                                                                                                    |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1->2       | This transition can occur in the following ways: Buy-side has preferred list which means the business will be directed to a specific Sell-Side. Buy-Side provides details of the program to a number of sell-sides. This can be achieved using a mixture of telephone, fax, modem links, and FIX Bid messages. |
| 2->3       | Details of the program are transmitted to the chosen Sell-side using telephone, fax, modem links, or FIX New Order - List message.                                                                                                                                                                             |
| 3->4       | This transition can occur in the following ways: Buy-Side and Sell-Side communicate by telephone to confirm content of the program and the Buy-Side instructs the Sell-Side to begin execution. Buy-Side sends a List Execute FIX message to instruct Sell-Side to begin execution.                            |
| 4          | Once the list is being executed the FIX List status messages may be used to notify/request status of the list.                                                                                                                                                                                                 |

# Order Details sent via FIX (Bidding Outside of FIX)

| Buy Side    | New Order - List Message (1 broker) | Sell Side     |
| ----------- | ----------------------------------- | ------------- |
| Institution | 2) List Order Status (ACK)          | Broker Dealer |

March 25, 2003 – DRAFT 153 FIX 4.4 - Volume 4


---

March 25, 2003 – DRAFT


# “Disclosed” Bid and Program Trade

| Buy Side                                  | Sell Side     |
| ----------------------------------------- | ------------- |
| 1) New Order - List Message (N brokers)   | Broker Dealer |
| 2) List Order Status (ACK) (1 per broker) |               |
| 3) Bid Request Message (N brokers)        |               |
| 4) Bid Response (1 per broker)            |               |
| 5) Cancel bid sent to (N – 1) brokers     |               |

# “Non Disclosed” Bid and Program Trade

| Buy Side                                                            | Sell Side     |
| ------------------------------------------------------------------- | ------------- |
| 1) Bid Request Message (N brokers)                                  | Broker Dealer |
| 2) Bid Response (1 per broker)                                      |               |
| Buy-side selects one Sell-side and sends order detail for execution |               |
| 3a) Bid Request Cancel (N – 1 brokers)                              |               |
| 3b) New Order - List Message (1 sell-side)                          |               |
| 4) List Status Response (ACK)                                       |               |


154                                FIX 4.4 - Volume 4

---

# Message Flows

once a buy-side has chosen a single sell-side and transmitted New Order - List messages

The following message flows can occur in any order relative to each other and some may occur many times.

# Optional notification to begin execution (may occur zero or one times)

| Buy Side    | 1) List Execute message (1 sell-side) | Sell Side     |
| ----------- | ------------------------------------- | ------------- |
| Institution | 2) List Order Status (Ack) Response   | Broker Dealer |

# Optional transfer of Principal Portfolio Trade prices from buy-side to sell-side (may occur zero or one times)

| Buy Side    | 1) List Strike Price message | Sell Side     |
| ----------- | ---------------------------- | ------------- |
| Institution | 2) List Status message       | Broker Dealer |

# Optional transfer of Principal Portfolio Trade prices from sell-side to buy-side (may occur zero or one times)

| Buy Side    | 1) List Strike Price message | Sell Side     |
| ----------- | ---------------------------- | ------------- |
| Institution | 2) List Status message       | Broker Dealer |

# Optional Execution Report status update (may occur zero or N times)

| Buy Side    | 1) Execution Reports (if requested) | Sell Side     |
| ----------- | ----------------------------------- | ------------- |
| Institution |                                     | Broker Dealer |

# Optional List Status Request (may occur zero or N times)

| Buy Side    | 1) List Status Request message | Sell Side     |
| ----------- | ------------------------------ | ------------- |
| Institution | 2) List Status message         | Broker Dealer |

# Optional Sell-Side unsolicited Status update (may occur zero or N times)

| Buy Side    | 2) List Status message | Sell Side     |
| ----------- | ---------------------- | ------------- |
| Institution |                        | Broker Dealer |

# Scenario 1

# Bidding performed by Telephone and List provided via FIX

| Message              | Description                                                                                                                                                                    | Purpose                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------- |
| New Order - List     | Details the list of stocks that an institution wishes to trade.                                                                                                                | Normally side is omitted and an indicator is set to show that this message is part of a bid |
| List Status Response | List status response indicates that the sell-side has received the New Order - List message. The status of each order in the list should indicate a status of bid or rejected. | The former if the stock is recognised and the latter if the stock is not recognised.        |

March 25, 2003 – DRAFT

155

FIX 4.4 - Volume 4


---

# Scenario 2

# Fully Disclosed Program Trade – with bidding stage through FIX

| Message                                | Description                                                                                  | Purpose                                                                                                                                                                |
| -------------------------------------- | -------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| New Order - List                       | Details the list of stocks that an institution wishes to trade.                              | Normally side is omitted and an indicator is set to show that this message is part of a bid.                                                                           |
| List Status Response (Acknowledgement) | List status response indicates that the sell-side has received the New Order - List message. | The status of each order in the list should indicate a status of bid or rejected. The former if the stock is recognised and the latter if the stock is not recognised. |
| Bid Request Message from B/S to S/S    | Details the types of bids required, eg Side, Execution Type etc.                             |                                                                                                                                                                        |
| Bid Response Message                   | Details the bid response for a program.                                                      |                                                                                                                                                                        |
| List Execute Message                   | Details the specific bid that has been accepted.                                             | The specific bid indicates the direction of the list to be executed.                                                                                                   |
| List Status Response                   | Details the status of each order in the list.                                                | The status should be executing for each order.                                                                                                                         |

Status updates may optionally follow.

# Scenario 3

# Non-Disclosed Program Trade – with bidding stage through FIX

| Message                              | Description                                                                             | Purpose                                         |
| ------------------------------------ | --------------------------------------------------------------------------------------- | ----------------------------------------------- |
| Bid Request from B/S to S/S          | Details the liquidity information about the stocks that an institution wishes to trade. | It does not identify the stocks in the program. |
| Bid Response Message from S/S to B/S | Details the bid response for a program.                                                 |                                                 |

done by phone

done by phone

March 25, 2003 – DRAFT

156

FIX 4.4 - Volume 4


---

# List Message Detail

Details the list of stocks that an institution wishes to trade from B/S to S/S including the stock, quantity, and direction for each order.

# Required List Status Response

Details the status of each order in the list. The status should be awaiting execution, executing or rejected for each order.

# List Execute Message

Details the bid for a program from B/S to S/S done by phone required if previous provided may be omitted.

# List Status Response

Details the status of each order in the list. The status should be executing for each order.

# Illustration of liquidity indicator fields usage

Normally details, by country and by sector, as number at &#x3C;5%, no in 5-10%, no in 10-30% and number at > 30% eg 1@ 70%, 1 @ 600% For example

# Country

| Country | <5%            | 5 – 10%        | 10 - 30%       | > 30%                                      |
| ------- | -------------- | -------------- | -------------- | ------------------------------------------ |
| DEM     | 1 Sec $1000000 | 4 Sec $2000000 | 7 Sec $1500000 | 1 Sec @60%, $3000000 1 Sec @300%, $8000000 |
| ESP     | 4 Sec $3000000 | 5 Sec $3000000 | 3 Sec $3500000 |                                            |
| UK      | 3 Sec $4500000 | 6 Sec $3600000 | 2 Sec $5000000 | 1 Sec @450%, $9000000                      |

# Sector

| Sector          | <5%            | 5 – 10% | 10 - 30% | > 30%                 |
| --------------- | -------------- | ------- | -------- | --------------------- |
| Industrials     | 2 Sec $1500000 | 5       | 4        | 1 Sec @300%, $8000000 |
| Pharmaceuticals | 4 Sec          | 3       | 3        | 1 Sec @450%, $9000000 |
| Hotels          | 2              | 7       | 5        | 1 Sec @60%, $3000000  |

# Illustration of liquidity indicator fields usage

The liquidity indicator fields are used to describe the shape of a basket trade in terms of the liquidity and classification of the stocks contained within the list. Thus a list that may be described by the following two tables.

# List liquidity information by country.

March 25, 2003 – DRAFT 157 FIX 4.4 - Volume 4


---

March 25, 2003 – DRAFT


# Liquidity Information by Country

| Country | <5%                        | 5 – 10%                   | 10 - 30%                  | > 30%                       |
| ------- | -------------------------- | ------------------------- | ------------------------- | --------------------------- |
| DEM     | 1 Security Value $1000000  | 4 Security Value $2000000 | 7 Security Value $1500000 | 1 Security Value $3M @ 60%  |
|         | 1 Security Value $8M @300% |                           |                           |                             |
| ESP     | 4 Security Value $3000000  | 5 Security Value $3000000 | 3 Security Value $3500000 |                             |
| UK      | 3 Security Value $4500000  | 6 Security Value $3600000 | 2 Security Value $5000000 | 1 Security Value $9M @ 450% |

# Liquidity Information by Security Sector

| Sector         | <5%                       | 5 – 10%                   | 10 - 30%                  | > 30%                      |
| -------------- | ------------------------- | ------------------------- | ------------------------- | -------------------------- |
| Industrials    | 2 Security Value $1500000 | 5 Security Value $2600000 | 4 Security Value $3000000 | 1 Security Value $8M @300% |
| Pharmaceutical | 4 Security Value $3000000 | 3 Security Value $3000000 | 3 Security Value $1500000 | 1 Security Value $9M @450% |
| Hotels         | 2 Security Value $4000000 | 7 Security Value $3000000 | 5 Security Value $2500000 | 1 Security Value $3M @60%  |


158                                         FIX 4.4 - Volume 4

---
BidRequest Message (Non Disclosed bid, basket of securites, not an exchange for physical trade)

# Client Bid Request

| Bid ID | Bid Type | Total Num Securities | Bid Value | Side     | Repeating fields                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| ------ | -------- | -------------------- | --------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1001   | N        | 38                   | 1         | 37100000 | Bid Descriptor Type	Bid Descriptor	Liquidity Value	Liquidity Num Securities	Liquidity Pct Low	Liquidity Pct High&#xA;2	DEM	1	1000000	1	0.00	0.05&#xA;					2	DEM	1	2000000	4	0.05	0.10&#xA;					2	DEM	1	1500000	7	0.10	0.30&#xA;					2	DEM	1	3000000	1	0.60	\*- NP&#xA;					2	DEM	1	8000000	1	3.00	\*- NP&#xA;					2	ESP	1	3000000	4	0.00	0.05&#xA;					2	ESP	1	3000000	5	0.05	0.10&#xA;					2	ESP	1	3500000	3	0.10	0.30&#xA;					2	UK	1	4500000	3	0.00	0.05&#xA;					2	UK	1	3600000	6	0.05	0.10&#xA;					2	UK	1	2000000	2	0.10	0.30&#xA;					2	UK	1	9000000	1	4.50	\*- NP&#xA;					1	Ind	1	1500000	2	0.00	0.05&#xA;					1	Ind	1	2600000	5	0.05	0.10&#xA;					1	Ind	1	3000000	4	0.10	0.30&#xA;					1	Ind	1	8000000	1	3.00	\*- NP&#xA;					1	Pharm	1	3000000	4	0.00	0.05&#xA;					1	Pharm	1	3000000	3	0.05	0.10&#xA;					1	Pharm	1	1500000	3	0.10	0.30&#xA;					1	Parm	1	9000000	1	4.50	\*- NP&#xA;					1	Hotels	1	4000000	2	0.00	0.05&#xA;					1	Hotels	1	3000000	7	0.05	0.10&#xA;					1	Hotels	1	2500000	5	0.10	0.30&#xA;					1	Hotels	1	3000000	1	0.60	\*- NP |

# Notes

*- NP field not present in repeating group as entry is describing a single stock at a specific liquidity.

Where the BidDescriptorType set to 1 the entry in the BidDescriptor field is free text, the sector names Pharmaceuticals and Industrials have been shortened to make everything fit.

March 25, 2003 – DRAFT 159 FIX 4.4 - Volume 4