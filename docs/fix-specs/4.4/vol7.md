
# FINANCIAL INFORMATION

# EXCHANGE PROTOCOL

# (FIX)

# Version 4.4 with Errata 20030618

# VOLUME 7 – FIX USAGE BY PRODUCT

Includes Errata adjustments as of June 18, 2003



# Errata Purpose:

This document includes a list of minor adjustments to the FIX 4.4 Specification document due to typographical errors or ambiguities. The nature and scope of Errata adjustments do not introduce new functionality, additional fields, new values for existing fields, or new messages. Regretably some functionality was introduced in FIX 4.4 which contained errors that required a new value or field on a specific message in order to make the intended functionality implementable. Any such exceptions to the “do not introduce”, “additional fields”, or “new messages” Errata rules were kept to a minimum using the “required to make the intended functionality implementable” rationale. The list of items has been reviewed and approved by the FIX Technical Committee and Steering Committees. Implementers of FIX version 4.4 should refer to this document to ensure the most consistent implementation and clearest understanding of the FIX protocol.

The specific adjustments made to the original FIX version 4.4 specification as a result of the Errata can be seen and printed via Microsoft Word’s revision feature of this document. A separate document with an itemized list of changes is available via the FIX website.

~~April 30, 2003~~June 18, 2003

~~April 30, 2003~~June 18, 2003 1 FIX 4.4 with Errata 20030618- Volume 7
---
Contents – Volume 7

# PRODUCT: COLLECTIVE INVESTMENT VEHICLES (CIV)

# Overview and Scope

# Market environment

# CIV Security Type Identification

# Types of CIV FIX Messages

# Order Quantities

# Intermediary identification

# Investor details

# Investor identification

# New Investor -> New Order -> Registration instruction

# Fund &#x26; Unit Identification

# Order details - single

# Order details - list

# Commission Instructions

# Compliance

# Settlement instructions

# Distribution instructions

# Unit Prices

# Valuation point

# Single pricing

# Dual pricing

# Execution Reports

# CIV-specific use of OrdStatus:

# CIV EXAMPLES

# CIV Example 1. Single order for a CIV fund for a known investor/nominee, to be dealt on a "historic" basis

# CIV Example 2. Single order for a CIV fund for a known investor/nominee, to be dealt on a "forward" basis

# CIV Example 3. Single order for a CIV fund for an investor/nominee not known to the fund manager - registration and settlement instructions after trade

# CIV Example 4. Single order for a CIV fund for an investor/nominee not known to the fund manager - registration and settlement instructions required before trade

# CIV Example 5. Single order for a CIV fund for a known investor/nominee – order modified before execution

# CIV Example 6. Single order for a CIV fund for a new investor/nominee to the fund manager - registration and settlement instructions rejected, then modified &#x26; accepted

# CIV Example 7. Exchange/switch order between several CIV funds from a single fund manager or via a funds supermarket

# CIV Example 8. Order for CIV fund by new or existing investor, routed via a client money/asset holding broker or funds supermarket to fund manager

# CIV Example 9. Order for CIV fund by an institutional investor, routed via a broker to a fund manager – possibly via a hub/exchange

# CIV Example 10. Order for CIV fund by new investor via non-client money/asset holding intermediary to fund manager

# CIV Example 11. Order for CIV fund by new investor, routed via non-client money/asset holding intermediary via a non-aggregating hub/exchange to fund manager

# CIV Example 12. Order for CIV fund by new investor routed via intermediary to a funds supermarket – which places bulk/net orders to the fund manager

# CIV Example 13. Exchange/switch order quantities – OrderPercent, Rounding, Sell Driven

# CIV Example 14. CIV Bulk order – purchase of funds for multiple investors into a designated nominee

# CIV Example 15. Registration Instruction – Joint Investors

# CIV Example 16 Registration Instruction – Tenants in Common

PRODUCT: DERIVATIVES (FUTURES &#x26; OPTIONS)
<page_footer>April 30, 2003 June 18, 2003 2 FIX 4.4 with Errata 20030618- Volume 7</page_footer>
---
Use of CFICode to identify derivatives security
# 26

# Single Leg Instruments

# 26

# Multileg Instrument Specification

# 26

# US Listed Options Order Capacity Values

# 27

# Proposed option order capacity codes and their FIX 4.3 equivalents

# 28

# CustomerOrderCapacity(tag 582) Mappings for Futures CTICode

# 30

# Negative Prices permitted for futures and options strategies

# 31

# Derivatives Markets Order State Transition

# 31

# Party Roles used for Derivatives Markets

# 31

# MAPPING FIX 4.2 to FIX 4.3 Usage for Options Markets

# 33

# General Usage Information – US Futures Markets

# 36

# Execution Time Bracket reporting for US Futures Markets

# 36

# Example New Order – Single for Listed Futures Market

# 37

# Example New Order – Single for Listed Options Market

# 41

# Example New Order - Multileg for Listed Futures Market

# 50

# (Butterfly Strategy)

# 50

# Example Multileg Execution Report for Listed Futures Market

# 57

# Multileg Execution Report Example for Futures Markets

# 57

# PRODUCT: EQUITIES

# 65

# Step-Outs and Give-Ups

# 65

# CFDs

# 66

# CFD with cash equity hedge executed by same broker as writing the CFD

# 66

# CFD with cash equity hedge executed by different broker from that writing the CFD

# 66

# Commission Sharing Arrangements

# 68

# Soft Dollars

# 68

# Directed Brokerage

# 68

# Multi-Day Average Pricing

# 69

# Introduction

# 69

# Flow Summary

# 69

# Example Warehouse Flows

# 70

# Decision Flows

# 74

# PRODUCT: FIXED INCOME (FI)

# 76

# Introduction

# 76

# Message Dialog

# 76

# Indication of Interest (Offerings)

# 77

# Negotiated Trade /Inquiry/Bid or Offer Request

# 78

# Out-of-Band Negotiated Order

# 80

# Allocation Instructions

# 82

# Post Trade Reporting to a 3ʳᵈ Party or Virtual Matching Utility

# 86

# Message Usage Details

# 88

# General Usage Rules

# 88

# Indication Of Interest

# 88

# Quote Request

# 88

# Quote Response

# 89

# Quote

# 89

# New Order - Single

# 89

# New Order - Multileg

# 89

# Execution Report

# 89

# Allocation Instruction

# 90

# Allocation Report

# 90

# Trade Capture Report

# 90

# Instrument component block

# 91

# OrderQtyData component block

# 91

# Repurchase Agreements (Repo) and Collateral Management

# 91

# Repo Terminology

# 91

# Collateral Management

# 97

# Identifying Euro Issuers

# 103

April 30, 2003 June 18, 2003 3 FIX 4.4 with Errata 20030618- Volume 7
---

# Euro CountryOfIssue Codes:

# Euro Issuer Values:


# Example usage of FI specific component blocks

# Example usage of BenchmarkCurve fields

# Example usage of Stipulation fields

# PRODUCT: FOREIGN EXCHANGE

# Foreign Exchange (F/X) Trading

~~April 30, 2003~~ June 18, 2003 4 FIX 4.4 with Errata 20030618- Volume 7



---
PRODUCT: COLLECTIVE INVESTMENT VEHICLES (CIV)

# Overview and Scope

This Appendix summarises how FIX messages can be used to support order initiation / confirmation and to issue settlement / Registration Instructions for open-ended Collective Investment Vehicles (“CIVs”) – known variously as Mutual Funds, Unit Trusts, Managed Investments, Open Ended Investment Companies (OEICs), Undertaking for Collective Investment in Transferable Securities (UCITs) etc. Note that the FIX messages for CIV do not address Exchange Traded Funds, closed funds such as Investment Trusts or other scenarios where CIVs are traded as if they were equities.

# Market environment

Units in funds are typically sold to Retail Investors on the recommendation of an Intermediary advisor (whose firm may not be authorised to hold client assets or settle transactions), or purchased at the Investors’ initiative via a broker or funds supermarket (which may outsource settlement to a third party) or purchased by the Investor directly from the fund manager (who again may outsource fund administration to a third party).

Retail intermediaries (eg. Intermediary advisors) who don’t hold client funds or settle transactions are rewarded by commission from the fund manager out of charges collected from the Investor. Commission and charges may be paid at the time of investment (“front-end load funds”) and/or during the life of the investment (“no-load funds”). The latter may be called “renewal” or “trail” commission, and is typically paid directly to the intermediary at the end of each period.

Intermediaries such as brokers and funds supermarkets may charge their own commission etc. directly to the Investor and instruct the fund manager not to deduct commission from the sum invested. Institutional Investors typically purchase funds directly from the fund manager and no commission is payable.

In some regulatory environments the fund manager is responsible for making compliance and money laundering checks before a CIV order is executed, hence for new investors full details must be supplied with the order. In some markets Hubs, Exchanges or Funds Supermarkets provide messaging, order matching/crossing, clearing and settlement services between Intermediaries/brokers, Fund managers etc.

FIX messages may be used between any of the participants. The fund manager may also use FIX messages to buy and sell fund assets with other participants in the relevant market(s) (eg. Equities):

| Investor, e.g:      | Fund Manager, | Market;       |
| ------------------- | ------------- | ------------- |
| Institution, Retail | ie. Product   | via           |
| Intermediary        | Hub           | Institutional |
| Stockbroker         | or            | Stockbroker   |
| Custodian           | Custodian     |               |
| Third Party         | Custodian     | Third Party   |
| Administrator       | Administrator |               |

Note that in a CIV scenario brokers, intermediaries etc. may be on the “buy side” and institutions may be on the “sell” side, i.e. a reversal of the situation in equity/fixed interest/FX transactions.

# CIV Security Type Identification

A Collective Investment Vehicle security type is designated by a CFICode field (ISO 10962 standards-based) value which starts with “EU”. Note that if the Product field is specified, the value should be set to “Equity” to

~~April 30, 2003~~June 18, 2003

5 FIX 4.4 with Errata 20030618- Volume 7


---

# Types of CIV FIX Messages

The FIX messages specifically supporting CIV trades are:

- “New Order – Single” – used to specify the buy or sell of a CIV fund. The message includes the ability to specify percentage of a holding to be sold, whether or not the order can be crossed or matched, compliance/money laundering status, commission instructions, etc. The New Order – Single comprises the major details:
- Intermediary &#x26; Client Identification Information
- Commission
- Order Quantity
- Registration and Reconciliation details
- “New Order – List” – used for an Investor to initiate exchanges or switches between CIV funds, or by a broker or Hub to place a bulk buy or sell order for several funds. New order List comprises one or more “New Order – Singles”
- Order Cancel Request – used for an Investor, Broker or Hub to request cancellation of an outstanding order
- Order Cancel Reject – used for a fund manager to reject Cancellation of an order
- Order Status Request – used for an Investor, Broker or Hub to request the status of an order
- “Settlement” – used to transmit Investors’ payment details to the fund manager where the Intermediary does not settle trades
- “Registration Instructions” and “Registration response” – used to transmit Investors’ registration details to the Fund manager, allow compliance checks and opening of the correct type of account. This may be sent before or after corresponding New Order messages. The Registration Instructions message type comprises the major details:
- RegistrationID
- OrderLink Fields
- Registration Classification
- Member Registration
- Distribution Details
- “Execution Report” – used to transmit details of Unit price basis, charges, commission etc. to the Investor and Intermediary

Allocation messages are not required for CIV trading with Fund managers, but other FIX messages are unchanged and can be used as required, e.g. Market Data, Security Status Request, Quote, Order Status, Order Cancel / Replace, Don’t Know, Business Reject etc.

(See CIV Examples 1 – 7 below for examples of the use of these message types.)

# Order Quantities

Income on units may be credited as additional units on the Investor’s account with the Fund manager, leading to uncertainty about the exact number of units when a holding is to be sold. Similarly when an exchange or switch is requested the cash value of investments realised and to be re-invested is not known. Hence it can be more

~~April 30, 2003~~June 18, 2003
6 FIX 4.4 with Errata 20030618- Volume 7


---

# Investor details

If an Intermediary places a CIV Order for a new Investor (to the Fund manager) then the Registration instructions message can be used to transmit the details as required by the Fund manager:

- RegistAcctType – identifying which of the fund manager’s account types should be opened
- TaxExemptType – identifying which of the (nationally defined) tax-exempt accounts or “plans” is required
- OwnershipType – indicates relationship between owners where there is more than one, e.g. tenants in common (i.e. equal interests), joint tenants with rights of survivorship.
- RegistDtls &#x26; RegistEmail – name and address into which purchases for this Investor should be registered, plus e-mail address where applicable.
- InvestorCountryOfResidence – identifying the country of residence of the investor, e.g. for compliance and/or tax purposes
- OwnerType – identifying whether the registered investor is an individual, corporation, nominee/street name, trustee etc. (This information may be required for regulatory purposes and/or to indicate which format of Registration name and address information is required)
- InvestorID and InvestorIDSource – containing identifiers issued by official organisations such as tax authorities, company registrar, regulators or national numbering agencies, together with an identifier for the source of the identifier
- MailingDtls – the name and address to which general correspondence should be sent (if different from the Registration address), semi-annual reports, marketing literature.
- MailingInst – e.g. instructions indicating what the mailing address is to be used for, whether marketing literature is acceptable etc.

(See CIV Examples 15-16 below for examples of the use of registration instruction for new investors, accounts etc.)

Having received this information the Fund manager responds with a Registration Instructions Response– which in addition to the RegistID of the Registration request should also contain the Account and/or ClientIDs allocated to the Investor.

(See CIV Examples 3, 4 &#x26; 6 below for examples of the use of Registration instruction response message.)

# Investor identification

A Fund manager may allocate an Account id and/or Client id to each Investor – depending on the architecture of his account database. These can be returned on the Registration status or Execution report message or by some other means (e.g. printed confirm or contract note), and should be quoted on subsequent New Order etc. messages.

~~April 30, 2003~~June 18, 2003

---


(See CIV Examples 8-10 below for examples of the use of identification fields for new and existing investors, accounts etc.)

# New Investor -> New Order -> Registration instruction

Registration instruction messages can be sent before, after or both before and after a related New Order:

- before the New Order, e.g. to give details of a new investor / account (with name &#x26; address etc.). The RegistID specified on this Registration message must also be quoted on the subsequent New Order.
- after the New Order e.g. to give distribution payment details or to override previous Registration instructions for that specific New Order. This message should quote ClOrdID from the New Order (and Account and ClientID if available), but not the RegistID.

The Fund manager will respond to each Registration instruction with one or more Registration status messages, indicating whether the details are:

- Accepted – where possible including the Account and ClientID if these have been allocated by the Fund manager
- Rejected – in which case the RegistRejReasonCode and RegistRejReasonText fields should be populated to indicate the reason for rejection
- Held – e.g. pending receipt of the New Order or for later batch or manual processing, following which an “accepted” or “rejected” Registration status message will be sent

Note that the Designation field is available on the New Order message to provide supplementary registration information.

(See CIV Examples 6 &#x26; 14-16 for examples of registration instructions and the designation field.)

# Fund &#x26; Unit Identification

Many Funds offer several classes of units, e.g. front-end, back-end or no-load; income or accumulation units etc. In some tax regimes Fund managers are required to differentiate between units purchased before and after the most recent distribution. In markets where ticker symbols are allocated to unit types these are entered in the Symbol field; where tickers are not available an alternative identification such as ISIN is entered in the (mandatory) Symbol field and also the (optional) SecurityID field, with the code type in the SecurityIDSource field.

The Issuer and SecurityDesc fields may also be used to further confirm the Fund and Unit type required. Note that the Fund managers or regulators may impose restrictions on the Funds in an order, e.g. they must be available to the type of Investor, Account or Tax Exemption, or (for an exchange/switch) they may all have to be issued by the same Fund manager.

# Order details - single

Order details for a CIV Order typically include:

- Side – “buy” (sometimes known as create, although creation may not actually be involved) or “sell” (sometimes known as a cancel, although cancellation may not actually be involved) - where “buy” or “sell” order can be matched or crossed by an intermediary, funds supermarket, broker/dealer etc. or forwarded to the fund manager. On the other hand a “subscribe” or “redeem” order must be forwarded to the fund manager, e.g. where the originator requires specific tax treatment and/or dealing charges.
- OrdType – Previous Fund Valuation Point (Historic pricing) or Next Fund Valuation Point –(Forward pricing)
- Order quantity expressed as one of:

~~April 30, 2003~~June 18, 2003                  8        FIX 4.4 with Errata 20030618- Volume 7


---

# Order details - list

A CIV “New Order – List” would typically be issued:

- by a retail intermediary to initiate an “exchange” or “switch” between funds on behalf of a single Investor
- by a broker, funds supermarket or hub/exchange to initiate “bulk buy” or “bulk sell” order of funds held for the account of several investors

For an exchange/switch:

- the ListNoOrds and ListSeqNo fields determine the order in which the deals are to be executed
- the ListExecInstType determines how the Order quantities and Settlement amount are to be calculated (i.e. sell-driven, buy-driven with additional cash available, buy-driven without additional cash)

For a bulk buy / bulk sell the Designation field can be used to supply supplementary registration information for each order line, to maintain segregation between the holdings for individual clients.

(See CIV Examples 13 &#x26; 14 below for an example of the use of New Order – List.)

# Commission Instructions

The Intermediary can indicate specific commission requirements using:

- Commission &#x26; CommType – e.g. a specific commission rate or a waiver of the standard commission rate for the fund, the saving on standard commission being credited as for additional units or as a cash discount
- CommCurrency – to specify that commission on an overseas or offshore fund should be paid in domestic currency
- FundBasedRenewalWaived – to indicate whether or not the Intermediary accepts renewal/trail commission

# Compliance

Depending on terms of business and the regulatory environment either or both of the Intermediary and Fund manager may be required to support money laundering status checking and/or right-to-cancel. The New Order message supports these with:

- MoneyLaundering – indicating whether or not checks are required and have already been carried out by the Intermediary
- CancellationRights - indicating whether or not a “right-to-cancel” applies

# Settlement instructions

For CIV Orders retail settlement instructions may be transmitted using Settlement instruction features including:

~~April 30, 2003~~ June 18, 2003 9 FIX 4.4 with Errata 20030618- Volume 7


---


SettlInstMode – indicating that settlement instructions relate to a specific (retail) Order

SettlInstSource – indicating the Investor as the source of settlement instructions

PaymentMethod &#x26; SettCurrency – indicating cheque, bank transfer, payment card, cash account at depository etc.

CardHolderName, CardNumber, CardStartDate, CardExpDate, CardIssNo, PaymentDate and PaymentRemitterID – details required for cash settlement by payment card

SettlBrkrCode, SettlDepositoryCode – for cash settlement via central depositories

CashSettlAgentName, CashSettlAgentCode, CashSettlAgentAcctNum, CashSettlAgentAcctName - for cash settlement by bank transfer

PaymentRef – cross-reference or narrative information for bank transfers etc. to appear on bank statements, SWIFT MT950’s etc. to assist reconciliation

# Distribution instructions

The Registration instruction message can also carry Distribution instructions, including:

NoDistribDetls &#x26; DistribSeqNo – the number of beneficiaries

DistribPercent – the split of each distribution (by value) between several beneficiaries

DistribPaymentMethod &#x26; CashDistribCurr – payment method and currency for a specific beneficiary

CashDistribAgentName, AgentCode, AgentAcctName and AgentAcctNum – bank and account details for a specific beneficiary

CashDistribPayRef - cross-reference or narrative information for bank statements

(See CIV Examples 15 &#x26; 16 below for examples of the use of distribution instructions.)

# Unit Prices

Fund managers calculate a net asset value for each fund – typically at a fixed time each day, the “valuation point”. They then quote either a single Unit price (“single pricing”) or separate buying and selling prices (“dual pricing”) – depending on the fund’s constitution and regulatory environment.

# Valuation point

The unit price applicable to a CIV trade depends on when the Order was received by the fund manager relative to a Valuation point, whether the Fund is normally dealt on a Historic or Forward basis, and possibly also on recent volatility on underlying fund assets and any specific instructions from the Investor.

Some of this information is indicated by fields on the New Order:

TransactTime – the time at which the Investor placed the CIV Order directly, or at which Intermediary placed the Order on behalf of the Investor

OrdType – whether Investor requires a Forward or (where available) a Historic price

Other times establishing the relevant valuation point are shown on the Execution Report:

OrderBookedTime – the time at which the Fund manager provisionally accepted the order for execution (having completed any preliminaries, e.g. setting up an account, money laundering checks)

ExecValuationPoint - the fund valuation point with respect to which a order was priced by the fund manager (may be before or after the OrderBookedTime).

# Single pricing

~~April 30, 2003~~June 18, 2003 10 FIX 4.4 with Errata 20030618- Volume 7


---

The Unit price for single-priced funds is determined from the net asset value, based on the mid-price of the underlying assets of the fund, divided by the applicable number of units. For these funds ExecPriceType on the Execution Report should be set to “S” = Single.

The manager’s Initial charge (if any) is then charged out separately. In addition a Dilution levy may be charged on large buy or sell transactions, e.g. to compensate for the difference between the mid- and buy/sell- price of the underlying investments. These charges can be notified on the Execution Report in the Contract amounts repeat group.

# Dual pricing

For dual priced funds the manager calculates:

- Creation price – based on the “buy” price of the underlying assets (net of transaction taxes etc.)
- Cancellation price – based on the “sell” price of the underlying assets (net of transaction taxes etc.)

If the net cash flow is into the fund new units will be created:

- Offer or Buy price – will be no higher than the Creation price plus the manager’s Initial charge
- Bid, Sell or Redemption price – will be the Offer price minus the manager’s Dealing spread

If the net cash flow is out of the fund existing units will be cancelled:

- Bid, Sell or Redemption price – will be no lower than the Cancellation price
- Offer or Buy price – will be the Bid price plus the manager’s Dealing spread, up to a limit of the Creation price plus the manager’s Initial charge

The manager may sell to buyers units he has re-purchased from sellers (rather than cancelling and re-creating units), thus profiting from the Dealing spread.

The Initial charge covers any commission paid to Intermediaries as well as advertising, administration, dealing costs etc. It can be a money amount or percentage and may be waived on large investments, e.g. by institutional investors. Where the Initial charge is waived for a private investor an Exit charge (money amount or percentage) may be levied if an investment is sold within the first few years. (This is sometimes known as a Deferred contingent sales charge.) These charges can be notified on the Execution Report in the Contract amounts repeat group.

The manager may offer an improved buying price by discounting the initial charge or reducing his dealing spread – the improved price is expressed as “Creation price plus” an amount or percentage, or “Offer price minus” an amount or percentage.

ExecPriceType and (where applicable) ExecPriceAdjustment on the Execution Report indicate how the actual buying or selling price was calculated from the fund valuation price(s).

# Execution Reports

The Fund manager should send Execution Report messages to confirm receipt (OrdStatus=“New”) and execution (OrdStatus= “Filled” and/or “Calculated”) of CIV Orders, plus other Order Status from the list below as agreed between the parties – individual Execution Reports being sent for each line of an New Order – List.

In markets where tax treatment and/or dealing charges depend on whether execution was by crossing / matching by an intermediary, or by subscription / redemption at the fund manager the LastMkt field should be used to indicate either the Exchange or 11 for an OTC trade, or omitted if execution was by the fund manager.

# CIV-specific use of OrdStatus:

CIV orders to be executed by the fund manager do not use the TimeInForce field and only the following OrdStatus values are expected to be used:

| Precedence | OrdStatus      | Description                                                                                     |
| ---------- | -------------- | ----------------------------------------------------------------------------------------------- |
| 11         | Pending Cancel | Order with an Order Cancel Request pending, used to confirm receipt of an Order Cancel Request. |

DOES

~~April 30, 2003~~~~June 18, 2003~~
11 FIX 4.4 with Errata 20030618- Volume 7


---

# Order Status Definitions

NOT INDICATE THAT THE ORDER HAS BEEN CANCELED. (Where supported by the receiving broker, intermediary, fund manager etc.)

| Code | Status          | Description                                                                                                                                                                                                                                                                                |
| ---- | --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 10   | Pending Replace | Order with an Order Cancel/Replace Request pending, used to confirm receipt of an Order Cancel/Replace Request. DOES NOT INDICATE THAT THE ORDER HAS BEEN REPLACED. (Where supported by receiving broker, intermediary, fund manager etc.)                                                 |
| 8    | Calculated      | Order has been filled, settlement details, currency, commission, contract amounts etc. have been calculated and reported in this execution message.                                                                                                                                        |
| 7    | Filled          | Order has been filled, execution valuation point, shares/unit quantity and price have been calculated and reported in this execution message.                                                                                                                                              |
| 4    | Canceled        | Canceled order without executions (where supported by receiving broker, intermediary, fund manager etc.).                                                                                                                                                                                  |
| 2    | New             | Outstanding order which has not been executed. The OrderBookedTime field will be completed. For Forward priced orders or funds the order will be executed at the next Valuation Point. (This status may not be sent if the order can be executed immediately on a Historic pricing basis.) |
| 2    | Rejected        | Order has been rejected by broker, intermediary or fund manager (for CIV orders). NOTE: An order can be rejected subsequent to order acknowledgment, i.e. an order can pass from New to Rejected status.                                                                                   |
| 2    | Pending New     | Order has been received by broker’s system but not yet accepted for execution. An execution message with this status will only be sent in response to a Status Request message. (Where supported by receiving broker, intermediary or fund manager etc.)                                   |

# CIV Fields included for each value of OrdStatus in Execution Report

| OrdStatus       | CIV Fields included on Execution Report                                                                                                                                                               |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Rejected        |                                                                                                                                                                                                       |
| Pending Cancel  | ClOrdID, ListID & TransactTime – Intermediary’s Order (and List) references and time of submission                                                                                                    |
| Canceled        | Other fields may be populated if available                                                                                                                                                            |
| Pending Replace |                                                                                                                                                                                                       |
| Pending New     | ClOrdID, ListID & TransactTime – Intermediary’s Order (and List) references and time of submission. All fields populated on the CIV Order (apart from Order fields not available in Execution Report) |
| New             | Same as for “Pending New” plus:                                                                                                                                                                       |

~~April 30, 2003~~ June 18, 2003

12 FIX 4.4 with Errata 20030618- Volume 7


---

# CIV EXAMPLES

The following examples illustrate how FIX messages can be used to process CIV fund orders and provide settlement and registration instructions to the fund manager.

NOTE that in the examples:

- “Buyside” refers to an institution or private investor investing in a CIV fund via broker, intermediary – or a hub and/or exchange transmitting messages to/from other buyside parties
- “Sellside” refers to a CIV fund manager or intermediary – or a hub and/or exchange transmitting messages to/from other sellside parties

# CIV Example 1. Single order for a CIV fund for a known investor/nominee, to be dealt on a "historic" basis

A typical flow for an order for a CIV fund dealt on Historic price for an investor or nominee known to the fund manager – is as follows:

| BUYSIDE                                                                                      | SELLSIDE             |
| -------------------------------------------------------------------------------------------- | -------------------- |
| New Order-Single (IntroBroker, ClOrdID, Account & ClientID specified)                        | Fund Valuation Point |
| Execution Report (ExecType = “F”) \[Trade] (IntroBroker, ClOrdID, Account & ClientID echoed) | Commission/ Fee Calc |

~~April 30, 2003~~June 18, 2003 13 FIX 4.4 with Errata 20030618- Volume 7

# Definitions

TranBkdTime – time at which the Fund manager accepted the CIV Order onto his books

OrderId – order reference assigned by Fund manager (to each line in a New Order - List)

# Filled

Same as for “New” plus:

- ExecID &#x26; DealTime – Fund manager’s reference &#x26; Valuation point at which the Fund manager priced the CIV Order
- LastQty, LastPx &#x26; ExecPriceType – Unit quantity, price &#x26; basis of calculation of the price (e.g. Bid, Offer / Offer minus, Creation / Creation plus etc.)

# Calculated

As for “Filled” plus:

- ContAmt, Type &#x26; Curr – type, currency and value of various contract amounts (Initial, Commission, Discount Exit, Dilution etc.)

(See CIV Examples 1 – 7 below for examples of the use of Execution Report messages.)


---

# CIV Example 2. Single order for a CIV fund for a known investor/nominee, to be dealt on a "forward" basis

A typical flow for an order for a CIV fund for an investor/nominee known to the fund manager that wishes to deal on a Forward price basis – is as follows:

| BUYSIDE                                                                                        | SELLSIDE                                                                                             |
| ---------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| New Order-Single (IntroBroker, ClOrdID, Account & ClientID specified) (OrdType="M") \[Forward] | Execution Report (ExecType = “0” \[New] (IntroBroker, ClOrdID, Account & ClientID echoed)            |
|                                                                                                | Fund Valuation Point                                                                                 |
|                                                                                                | Execution Report (ExecType = “F”) \[Trade] (IntroBroker, ClOrdID, Account & ClientID echoed)         |
|                                                                                                | Commission/ Fee Calc                                                                                 |
|                                                                                                | Execution Report (ExecType = “B”) \[Calculated] (IntroBroker, ClOrdID, Account & ClientID specified) |

# CIV Example 3. Single order for a CIV fund for an investor/nominee not known to the fund manager - registration and settlement instructions after trade

A typical flow for an order for a CIV fund for an investor/nominee not known to the fund manager where the fund manager does not require settlement or registration instructions in advance – is as follows:

| BUYSIDE                                                                                          | SELLSIDE                                                                  |
| ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| New Order – Single (IntroBroker & ClOrdID specified, Account, ClientID & RegistID not specified) | Execution Report (ExecType = “0” \[New] (IntroBroker & ClOrdID echoed)    |
|                                                                                                  | Fund Valuation Point                                                      |
|                                                                                                  | Execution Report (ExecType = “F”) \[Trade] (IntroBroker & ClOrdID echoed) |
|                                                                                                  | Commission/ Fee Calc                                                      |

~~April 30, 2003~~June 18, 2003 14 FIX 4.4 with Errata 20030618- Volume 7


---

# Execution Report (ExecType = “B”) [Calculated]

(IntroBroker &#x26; ClOrdID echoed)

# Registration Instruction Response (RegistStatus = “N”)

(IntroBroker &#x26; ClOrdID echoed)

# Settlement Instruction (SettInstTransType = “N”) [New]

(SettlInstMode=”4”) [Specific Order]

(IntroBroker &#x26; ClOrdID specified)

# Registration Instruction (RegistTransType = “0” ) [New]

(IntroBroker, ClOrdID &#x26; RegistID specified)

# Validate Registration Instruction

# Registration Instruction Response (RegistStatus = “A”)

[Accepted]

(IntroBroker, ClOrdID &#x26; RegistID echoed, Account and/or ClientID returned)

# CIV Example 4. Single order for a CIV fund for an investor/nominee not known to the fund manager - registration and settlement instructions required before trade

A typical flow for an order for a CIV fund for an investor/nominee not known to the fund manager where the fund manager requires settlement and registration instructions in advance – is as follows:

# Registration Instruction (RegistTransType = “0” ) [New]

(RegistID, IntroBroker &#x26; ClOrdID specified)

# Registration Instruction Response (RegistStatus = “H”) [Held]

(IntroBroker, ClOrdID &#x26; RegistID echoed, Account and/or ClientID not returned)

# Validate Registration Instruction

# Registration Instruction Response (RegistStatus = “A”)

[Accepted]

# New Order – Single

(IntroBroker &#x26; ClOrdID specified, Account, ClientID &#x26; RegistID not specified)

# Execution Report (ExecType = “A” [Pending New])

# Settlement Instruction (SettInstTransType = “A”) [New]

(SettlInstMode=”4”) [Specific Order]

(IntroBroker &#x26; ClOrdID specified)

# Validate Settlement Instruction

# Execution Report (ExecType = “0”) [New]

~~April 30, 2003~~ June 18, 2003 15 FIX 4.4 with Errata 20030618- Volume 7


---


# CIV Example 5. Single order for a CIV fund for a known investor/nominee – order modified before execution

A possible flow for an order for a CIV fund for an investor/nominee known to the fund manager, on which the CashOrdQty is modified before execution – is as follows:

| BUYSIDE                                                                                                | SELLSIDE             |
| ------------------------------------------------------------------------------------------------------ | -------------------- |
| New Order-Single (IntroBroker, ClOrdID, Account & ClientID specified) CashOrdQty = “6,000”             |                      |
| Execution Report (ExecType = “0” \[New] (IntroBroker, ClOrdID, Account & ClientID echoed)              |                      |
| Order Cancel/Replace Request (IntroBroker, ClOrdID, Account & ClientID specified) CashOrdQty = “7,000” |                      |
| Execution Report (ExecType = “5” \[Replaced] (IntroBroker, ClOrdID, Account & ClientID echoed)         |                      |
| Execution Report (ExecType = “F”) \[Trade] (IntroBroker, ClOrdID, Account & ClientID echoed)           |                      |
|                                                                                                        | Fund Valuation Point |
|                                                                                                        | Commission/ Fee Calc |
| Execution Report (ExecType = “B”) \[Calculated] (IntroBroker, ClOrdID, Account & ClientID specified)   |                      |

# CIV Example 6. Single order for a CIV fund for a new investor/nominee to the fund manager - registration and settlement instructions rejected, then modified &#x26; accepted

A possible flow for an order for a CIV fund for an investor/nominee not already known to the fund manager where settlement and registration instructions are supplied, rejected and then corrected after the trade – is as follows:

~~April 30, 2003~~June 18, 2003 16 FIX 4.4 with Errata 20030618- Volume 7


---


# BUYSIDE

# New Order – Single

(IntroBroker &#x26; ClOrdID specified, Account, ClientID &#x26; RegistID not specified)

# Execution Report

(ExecType = “B”) [Calculated] (IntroBroker &#x26; ClOrdID echoed)

# Settlement Instruction

(SettInstTransType = “N”) [New] (SettlInstMode=”4”) [Specific Order] (IntroBroker &#x26; ClOrdID specified)

# Registration Instruction

(RegistTransType = “0” ) [New] (IntroBroker, ClOrdID &#x26; RegistID specified)

# Validate Registration Instruction

# Registration Instruction Response

(RegistStatus = “H”) [Held] (IntroBroker, ClOrdID &#x26; RegistID echoed, Account and/or ClientID not returned)

# Registration Instruction Response

(RegistStatus = “R”) [Rejected] (IntroBroker, ClOrdID &#x26; RegistID echoed, Account and/or ClientID not returned)

# Registration Instruction

(RegistTransType = “2” ) [Replace] (IntroBroker, ClOrdID &#x26; RegistID specified)

# Validate Registration Instruction

# Registration Instruction Response

(RegistStatus = “A”) [Accepted] (IntroBroker, ClOrdID echoed, Account and/or ClientID returned)

# Settlement Instruction

(SettInstTransType = “R”) [Replace] (SettlInstMode=”4”) [Specific Order] (IntroBroker &#x26; ClOrdID specified)

# CIV Example 7

Exchange/switch order between several CIV funds from a single fund manager or via a funds supermarket

A typical flow for an order for a CIV fund for an investor wishing to switch funds between funds from a single fund manager or via a funds supermarket that covers all funds – is as follows:

# BUYSIDE

# New Order-List

(ListId &#x26; ListExecInstType specified, e.g. ListExecInstType=”3” [Exch/switch - Sell Driven])

~~April 30, 2003~~ June 18, 2003 17 FIX 4.4 with Errata 20030618- Volume 7


---

# BUYSIDE

# SELLSIDE

For each component of exchange/switch:

- IntroBroker
- ClOrdID
- ClientID
- Account
- Symbol/SecurityId
- OrderPercent
- Side

For each component of exchange/switch:

Execution Report (ExecType = “0” [New])

- IntroBroker
- ClOrdID
- Account &#x26; ClientID echoed

Fund Valuation Point

For each component of exchange/switch:

Execution Report (ExecType = “F” [Trade])

- IntroBroker
- ClOrdID
- Account &#x26; ClientID echoed

Commission/Fee Calc

For each component of exchange/switch:

Execution Report (ExecType = “B” [Calculated])

- IntroBroker
- ClOrdID
- Account &#x26; ClientID echoed

# Identifier examples – existing investor &#x26; account

CIV Example 8. Order for CIV fund by new or existing investor, routed via a client money/asset holding broker or funds supermarket to fund manager

# Typical usage of fields on Order and/or Post-Trade messages would be as follows:

| Tag | Field Name    | Contents                                                                                                                           |
| --- | ------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| 453 | NoPartyIDs    | 2                                                                                                                                  |
| 448 | PartyID       | An identifier for the broker or funds supermarket which is recognized by the fund manager                                          |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field                                                                       |
| 452 | PartyRole     | 6 \[“Introducing Firm”]                                                                                                            |
| 448 | PartyID       | An identifier for the broker or funds supermarket’s nominee/custodian company which is recognized by the fund manager              |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field, e.g. the Fund manager                                                |
| 452 | PartyRole     | 9 \[“Fund manager Client ID”]                                                                                                      |
| 523 | PartySubID    | An optional sub-identifier for the broker or funds supermarket’s nominee/custodian company which is recognized by the fund manager |

~~April 30, 2003~~ June 18, 2003

18 FIX 4.4 with Errata 20030618- Volume 7


---

# CIV Example 9. Order for CIV fund by an institutional investor, routed via a broker to a fund manager – possibly via a hub/exchange

Typical usage of fields on Order and/or Post-Trade messages would be as follows:

| Tag | Field Name    | Contents                                                                                                  |
| --- | ------------- | --------------------------------------------------------------------------------------------------------- |
| 453 | NoPartyIDs    | 3                                                                                                         |
| 448 | PartyID       | An identifier for the broker closest to the investing institution which is recognized by the fund manager |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field                                              |
| 452 | PartyRole     | 6 \[“Introducing Firm”]                                                                                   |
| 448 | PartyID       | An identifier for hub/exchange (where used) which is recognized by the fund manager                       |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field                                              |
| 452 | PartyRole     | 1 \[“Executing Firm”]                                                                                     |
| 448 | PartyID       | An identifier for the investing institution which is recognized by the fund manager                       |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field, e.g. the Fund manager                       |
| 452 | PartyRole     | 9 \[“Fund manager Client ID”]                                                                             |
| 523 | PartySubID    | An optional sub-identifier for the investor which is recognized by the fund manager                       |
| 11  | ClOrdID       | Assigned by investing institution                                                                         |

# Identifier examples – new investor and/or account

# CIV Example 10. Order for CIV fund by new investor via non-client money/asset holding intermediary to fund manager

Typical usage of fields on Order and/or Post-Trade messages would be as follows:

| Tag | Field Name    |
| --- | ------------- |
| 453 | NoPartyIDs    |
| 448 | PartyID       |
| 447 | PartyIDSource |
| 452 | PartyRole     |
| 448 | PartyID       |

~~April 30, 2003~~ June 18, 2003

Contents

2

An identifier for the broker closest to the investor which is recognized by the fund manager

Indicates source of Party identifier used in preceding field

6 [“Introducing Firm”]

Not present on the “New Order” message, only on Execution

19 FIX 4.4 with Errata 20030618- Volume 7


---

# CIV Example 11. Order for CIV fund by new investor, routed via non-client money/asset holding intermediary via a non-aggregating hub/exchange to fund manager

| Tag | Field Name       | Contents                                                                                                                                                                                                  |
| --- | ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 447 | PartyIDSource    | Indicates source of Party identifier used in preceding field, e.g. the Fund manager                                                                                                                       |
| 452 | PartyRole        | 9 \[“Fund manager Client ID”]                                                                                                                                                                             |
| 523 | PartySubID       | Not present on the “New Order” message, only on Execution Report(s). An optional sub-identifier for the investor which is assigned by the fund manager, e.g. after processing a Registration Instruction. |
| 11  | ClOrdID          | Assigned by intermediary                                                                                                                                                                                  |
| 493 | RegistAcctType   | An identifier for the type of account required which is recognised by the fund manager                                                                                                                    |
| 495 | TaxAdvantageType | An identifier for the type of tax advantaged account required                                                                                                                                             |
| 492 | PaymentMethod    | Entered by intermediary (together with Investor’s bank/card details) to show how investor will settle cash with the fund manager                                                                          |

# Typical usage of fields on Order and/or Post-Trade messages would be as follows:

| Tag | Field Name    | Contents                                                                                                                                                                                                  |
| --- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 453 | NoPartyIDs    | 3                                                                                                                                                                                                         |
| 448 | PartyID       | An identifier for the broker closest to the investor which is recognized by the fund manager                                                                                                              |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field                                                                                                                                              |
| 452 | PartyRole     | 6 \[“Introducing Firm”]                                                                                                                                                                                   |
| 448 | PartyID       | An identifier for hub/exchange (where used) which is recognized by the fund manager                                                                                                                       |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field                                                                                                                                              |
| 452 | PartyRole     | 1 \[“Executing Firm”]                                                                                                                                                                                     |
| 448 | PartyID       | Not present on the “New Order” message, only on Execution Report(s). An identifier for the investor which is assigned by the fund manager, e.g. after processing a Registration Instruction.              |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field, e.g. the Fund manager                                                                                                                       |
| 452 | PartyRole     | 9 \[“Fund manager Client ID”]                                                                                                                                                                             |
| 523 | PartySubID    | Not present on the “New Order” message, only on Execution Report(s). An optional sub-identifier for the investor which is assigned by the fund manager, e.g. after processing a Registration Instruction. |

~~April 30, 2003~~June 18, 2003
# 20 FIX 4.4 with Errata 20030618- Volume 7


---

FIX 4.4 with Errata 20030618 - Volume 7

# Registration Instruction

# CIV Example 12. Order for CIV fund by new investor routed via intermediary to a funds supermarket – which places bulk/net orders to the fund manager

Typical usage of fields on Order and/or Post-Trade messages between intermediary and funds supermarket would be as follows:

| Tag | Field Name | Contents                                                                                                                                                                                                       |                                                                                                                                                                                                   |
| --- | ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 11  | ClOrdID    | Assigned by intermediary                                                                                                                                                                                       |                                                                                                                                                                                                   |
| 453 | NoPartyIDs | 2                                                                                                                                                                                                              |                                                                                                                                                                                                   |
| 448 | PartyID    | An identifier for the intermediary closest to the investor which is recognized by the fund manager                                                                                                             |                                                                                                                                                                                                   |
|     | 447        | PartyIDSource                                                                                                                                                                                                  | Indicates source of Party identifier used in preceding field                                                                                                                                      |
|     | 452        | PartyRole                                                                                                                                                                                                      | 6 \[“Introducing Firm”]                                                                                                                                                                           |
|     | 448        | PartyID                                                                                                                                                                                                        | Not present on the “New Order” message, only on Execution Report(s). An identifier for the investor which is assigned by the funds supermarket, e.g. after processing a Registration Instruction. |
|     | 447        | PartyIDSource                                                                                                                                                                                                  | Indicates source of Party identifier used in preceding field, e.g. the Fund manager                                                                                                               |
| 452 | PartyRole  | 9 \[“Fund manager Client ID”]                                                                                                                                                                                  |                                                                                                                                                                                                   |
| 523 | PartySubID | Not present on the “New Order” message, only on Execution Report(s). An optional sub-identifier for the investor which is assigned by the funds supermarket, e.g. after processing a Registration Instruction. |                                                                                                                                                                                                   |

Typical usage of fields on Order and/or Post-Trade messages between funds supermarket and fund manager for bulk/net orders would be as follows:

| Tag | Field Name    | Contents                                                                            |                                                                                                              |
| --- | ------------- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| 11  | ClOrdID       | Assigned by fund supermarket                                                        |                                                                                                              |
| 453 | NoPartyIDs    | 2                                                                                   |                                                                                                              |
| 448 | PartyID       | An identifier for funds supermarket which is recognized by the fund manager         |                                                                                                              |
|     | 447           | PartyIDSource                                                                       | Indicates source of Party identifier used in preceding field                                                 |
|     | 452           | PartyRole                                                                           | 1 \[“Executing Firm”]                                                                                        |
|     | 448           | PartyID                                                                             | An identifier for the funds supermarket’s nominee/custodian company which is recognized by the fund manager. |
| 447 | PartyIDSource | Indicates source of Party identifier used in preceding field, e.g. the Fund manager |                                                                                                              |


~~April 30, 2003~~ June 18, 2003

---


# 9. PartyRole

“Fund manager Client ID”

# 523. PartySubID

An optional sub-identifier for the funds supermarket’s nominee/custodian company which is recognized by the fund manager.

# Quantity example

# CIV Example 13. Exchange/switch order quantities – OrderPercent, Rounding, Sell Driven

Typical use of OrderPercent and Rounding fields on Order and Execution Report messages to and from fund manager or funds supermarket would be as follows:

# Investor’s holdings before exchange/switch New Order – List are:

| Symbol/ | SecurityId | Quantity held |
| ------- | ---------- | ------------- |
| Fund A  | 5281       |               |
| Fund B  | 2296       |               |
| Fund C  | 1833       |               |

# Exchange/switch order details on the New Order – List are:

| Symbol/ | SecurityId | Sid  | OrderQt | CashOrderQt | OrderPercent |
| ------- | ---------- | ---- | ------- | ----------- | ------------ |
| Fund A  | Sell       | 1281 |         |             |              |
| Fund B  | Sell       |      | £2,000  |             |              |
| Fund C  | Sell       |      |         | 100%        |              |
| Fund X  | Buy        |      |         | 20%         |              |
| Fund Y  | Buy        |      |         | 30%         |              |
| Fund Z  | Buy        |      |         | 50%         |              |

with : RoundingDirection = 1 [Down]

RoundingModulus = 1

# After the Fund Valuation Point the quantities and cash amounts (assuming no commissions, initial or exit charges) are reported on “calculated” Execution Reports are as follows:

| Symbol/ | SecurityId | Sid   | Price (AvePx) | CumQty | Cash value |
| ------- | ---------- | ----- | ------------- | ------ | ---------- |
| Fund A  | Sell       | £5.21 | 1281          | £6,674 |            |

~~April 30, 2003~~ June 18, 2003

22 FIX 4.4 with Errata 20030618- Volume 7


---

# CIV Example 14. CIV Bulk order – purchase of funds for multiple investors into a designated nominee

Typical use of New Order – List by a broker for purchase of funds for multiple investors into a designated nominee would be to specify ListExecInstType=“1” [Immediate], with other fields as follows:

| Tag | Field Name             | Contents                                                                                                                                       |
| --- | ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| 11  | ClOrdID                | Assigned by broker to identify each component within New Order - List. As required for each component.                                         |
| 448 | PartyID                | An identifier for the funds supermarket’s nominee/custodian company which is recognized by the fund manager. Same for each component of order. |
| 447 | PartyIDSource          | Indicates source of Party identifier used in preceding field, e.g. the Fund manager                                                            |
| 452 | PartyRole              | 9 \[“Fund manager Client ID”]                                                                                                                  |
| 523 | PartySubID             | An optional sub-identifier for the funds supermarket’s nominee/custodian company which is recognized by the fund manager.                      |
| 448 | PartyID                | An identifier for the intermediary closest to the investor which is recognized by the fund manager. Same for each component of order.          |
| 447 | PartyIDSource          | Indicates source of Party identifier used in preceding field                                                                                   |
| 452 | PartyRole              | 6 \[“Introducing Firm”]                                                                                                                        |
| 55/ | Symbol/SecurityId etc. | Identifier(s) for fund. As required for each component.                                                                                        |
| 54/ | Side/OrderQty/         | Buy/sell & quantity.                                                                                                                           |
| 38/ | CashOrderQty           | As required for each component.                                                                                                                |
| 152 | RegistID               | Assigned by broker to identify Registration Instruction for nominee company – if required. Same for each component of order.                   |
| 494 | Designation            | Specific registration (“sub-account”) for each component. As required for each component.                                                      |

# Fund Transactions

| Fund   | Action | Price  | Quantity | Total   |
| ------ | ------ | ------ | -------- | ------- |
| Fund B | Sell   | £7.28  | 274      | £1,995  |
| Fund C | Sell   | £3.27  | 1833     | £5,994  |
| Fund X | Buy    | £8.72  | 336      | -£2,930 |
| Fund Y | Buy    | £15.00 | 293      | -£4,395 |
| Fund Z | Buy    | £1.00  | 7331     | -£7,331 |

Settlement amount (ContAmtValue) = £6.72 (credit, i.e. excess cash will be paid to Investor)

~~April 30, 2003~~ June 18, 2003 23 FIX 4.4 with Errata 20030618- Volume 7

---


# CIV Example 15. Registration Instruction – Joint Investors

Typical use of the Registration instruction Joint Investors, e.g. husband &#x26; wife, with cash distribution split equally between them would be:

| Tag | Field Name          | Value                                                  |
| --- | ------------------- | ------------------------------------------------------ |
| 517 | OwnershipType       | J \[“Joint Investors”]                                 |
| 413 | NoRegistDtls        | 2                                                      |
| 509 | RegistDtls          | John Smith Esq, 1 Acacia Avenue, Newtown, Countyshire  |
| 511 | RegistEmail         | johnsmith99\@isp.com                                   |
| 522 | OwnerType           | 1 \[“Individual Investor”]                             |
| 509 | RegistDtls          | Mrs Naomi Smith, 1 Acacia Avenue, Newtown, Countyshire |
| 511 | RegistEmail         | Naomismith32\@isp.com                                  |
| 522 | OwnerType           | 1 \[“Individual Investor”]                             |
| 510 | NoDistribInsts      | 2                                                      |
| 477 | DistribPaymentMeth  | 8 \[“Direct Credit”]                                   |
|     | od                  |                                                        |
| 512 | DistribPercentage   | 50                                                     |
| 478 | CashDistribCurr     | GBP                                                    |
| 498 | CashDistribAgentNa  | Anytown Bank                                           |
|     | me                  |                                                        |
| 499 | CashDistribAgentCo  | 20-01-00                                               |
|     | de                  |                                                        |
| 500 | CashDistribAgentAcc | 23456789                                               |
|     | tNumber             |                                                        |
| 501 | CashDistribPayRef   | Fund income                                            |
| 502 | CashDistribAgentAcc | Mr J & Mrs N Smith                                     |
| 477 | DistribPaymentMeth  | 5 \[“Cheque”]                                          |
|     | od                  |                                                        |
| 512 | DistribPercentage   | 50                                                     |
| 478 | CashDistribCurr     | GBP                                                    |
| 502 | CashDistribAgentAcc | Mrs Naomi Smith                                        |
|     | tName               |                                                        |

# CIV Example 16 Registration Instruction – Tenants in Common

~~April 30, 2003~~June 18, 2003

24 FIX 4.4 with Errata 20030618- Volume 7


---

# Possible use of the Registration instruction for Tenants in Common

e.g. a club of private investors that reinvest all their income could be:

| Tag | Field Name           |
| --- | -------------------- |
| 517 | OwnershipType        |
| 413 | NoRegistDtls         |
| 509 | RegistDtls           |
| 511 | RegistEmail          |
| 509 | RegistDtls           |
| 511 | RegistEmail          |
| 509 | RegistDtls           |
| 511 | RegistEmail          |
| 509 | RegistDtls           |
| 510 | NoDistribInsts       |
| 477 | DistribPaymentMethod |

# Contents

- T [“Tenants in Common”]
- Frank Jones, 2 South Drive, Anyport, Southshire
- fjones@myisp.net
- Sally Smith, 192 West Road, Anyport, Southshire
- ssmith@hotmail.com
- James Jordan, 88 Lime Tree Avenue, Lower Anyport, Southshire
- jamesj@mymail.co.uk
- Anita Robinson, 2 South Drive, Anyport, Southshire

1

12 [“Reinvest in Fund”]

~~April 30, 2003~~ June 18, 2003 25 FIX 4.4 with Errata 20030618- Volume 7


---

PRODUCT: DERIVATIVES (FUTURES &#x26; OPTIONS)

Use of CFICode to identify derivatives security

The CFICode (tag 461) is used to identify the type of instrument in FIX. The following is the recommended usage of the CFICode for futures and options. The CFICodes (ISO 10962) shall replace of SecurityType (tag 167) enumerations for futures – “FUT” and options – “OPT”. The CFICode for options supports definition of Calls – “C” and Puts – “P” in the second position. The PutOrCall (tag 201) tag is replaced (made obsolete) in FIX 4.3 by the adoption of the CFICode (tag 461).

# Single Leg Instruments

| CFICode\[461] | Description                          |
| ------------- | ------------------------------------ |
| OCXXXS        | Standardized Call Option             |
| OPXXXS        | Standardized Put Option              |
| FXXXS         | Standardized Future                  |
| OCXFXS        | Standardized Call Option on a Future |
| OPXFXS        | Standardized Put Option on a Future  |

# FIX 4.2 Mapping Values

| SecurityType\[167] | PutOrCall\[201] |
| ------------------ | --------------- |
| OPT                | 1               |
| OPT                | 0               |
| FUT                | na              |
| na₁                | 1               |
| na                 | 0               |

FFICN Nonstandard (flex) Financial Future on FUT na an index with cash delivery

FCEPN Nonstandard (flex) Commodity Future FUT na on an extraction resource with physical delivery

FXXXN Nonstandard (flex) future – contract type specified in symbology – not provided in CFICode

OCEFCN Nonstandard (flex) call option on future with european style expiration and cash delivery

OPAFPN Nonstandard (flex) put option on future with american style expiration and physical delivery

OPXSPN Nonstandard (flex) put option on a stock with physical delivery (the expiration style is not specified – so is assumed to default to the market standard for flex options).

OCEICN Nonstandard (flex) call option on an index with european style expiration and cash delivery

# Multileg Instrument Specification

The following use of SecurityType and CFICode are proposed for specifying multileg derivative instruments – such as options strategies or futures spreads.

1 A security type enumeration for an Option on a Future does not currently exist.

~~April 30, 2003~~June 18, 2003

26 FIX 4.4 with Errata 20030618- Volume 7

---


# Security Type

| SecurityType | CFICode | Description                                                                                                                                                              |
| ------------ | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| MLEG         | FMXXS   | Multileg Instrument with futures contract legs                                                                                                                           |
| MLEG         | OMXXXN  | Multileg Instrument with option contract legs                                                                                                                            |
|              |         | CFICode refers to Option – Miscellaneous (This would include multileg instruments that include the underlying security).                                                 |
| MLEG         | M       | Multileg Instrument with legs made up of various types of securities (not primarily a futures or options multileg instrument that contains one or more derivative legs). |
|              |         | CFICode refers to M-Miscellaneous                                                                                                                                        |

# US Listed Options Order Capacity Values

The following are commonly used order capacity codes from the US listed options markets and how they map to FIX 4.3.

| Common Listed Option Market Order Capacity Values | OrderCapacity (tag 528)                                                                                                                                                                                                                                                                                   | OrderRestrictions (tag 529) | Other                                                    |
| ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- | -------------------------------------------------------- |
| “B”                                               | any account of a broker/dealer, or any account in which a broker or dealer registered or required to be registered with the SEC pursuant to Section 15 under the Act has an interest. This represents any account that is not otherwise an account that falls into any of the below mentioned categories. | Principal                   |                                                          |
| “C”                                               | any account in which no member or non-member broker/dealer has an interest.                                                                                                                                                                                                                               | Agency                      |                                                          |
| “D”                                               | any account of a foreign broker/dealer.²                                                                                                                                                                                                                                                                  | Principal                   | 6 - Foreign Entity                                       |
| “F”                                               | any firm proprietary account which clears at the Options Clearing Corporation that is not a JBO account.                                                                                                                                                                                                  | Proprietary                 |                                                          |
| “M”                                               | an account representing a CBOE market-maker.                                                                                                                                                                                                                                                              | Proprietary                 | 5-Acting As a specialist or market maker in the security |
| “N”                                               | Any options account of a market-maker or specialist of another options exchange                                                                                                                                                                                                                           | Proprietary                 | 5-Acting As a specialist or market maker in the security |

² A foreign broker/dealer is defined as any person or entity that is registered, authorized, or licensed by a foreign governmental agency or foreign regulatory organization (or is required to be registered, authorized, or licensed) to perform the function of a broker or dealer in securities, or both. For purposes of this definition, a broker or dealer may also be a bank.

~~April 30, 2003~~  June 18, 2003

27 FIX 4.4 with Errata 20030618- Volume 7


---
Common Listed Option Market Order

Order Capacity (tag 528)

Order Restrictions (tag 529)

Other Capacity Values

Sometimes referred to as an order for a “MM or Specialist Away”.

“Y” any options account of a Commodities Trader, Stock Futures Trader or Stock Specialist registered in the underlying security.

Proposed option order capacity codes and their FIX 4.3 equivalents

| Proposed Listed Option Market Order Capacity Values                                                                                                                                                                                                                                                                                                                                                                                                      | Order Capacity **(tag 528)** | Order Restrictions **(tag 529)**                         | Other                       |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- | -------------------------------------------------------- | --------------------------- |
| “I” Proposed Code used to designate a JBO account which clears Customer at OCC: any joint back office (“JBO”) account of a broker/dealer that has a nominal ownership interest in a clearing broker/dealer and receives good faith margin treatment whereby such trade clears in the customer range at OCC. This ownership position allows the JBO clearing firm to finance securities transactions of the JBO participant on a good faith margin basis. | Agency                       | AccountType **(tag 581)=8**                              | Joint Back Office           |
| “J” Proposed Code used to designate a JBO account which clears Firm at OCC: any joint back office (“JBO”) account of a broker/dealer that has a nominal ownership interest in a clearing broker/dealer and receives good faith margin treatment whereby such trade clears in the firm range at OCC. This ownership position allows the JBO clearing firm to finance securities transactions of the JBO participant on a good faith margin basis.         | Proprietary                  | AccountType **(tag 581)=8**                              | Joint Back Office           |
| “K” Proposed Code used to designate a JBO account which clears MM at OCC: any joint back office (“JBO”) account of a broker/dealer that has a nominal ownership interest in a clearing broker/dealer and receives good faith margin treatment whereby such trade clears in the market-maker range at OCC. This ownership position allows the JBO clearing firm to finance securities transactions of the JBO participant on a good faith margin basis.   | Proprietary                  | 5-Acting As a specialist or market maker in the security | AccountType **(tag 581)=8** |

June 18, 2003

28

FIX 4.4 with Errata 20030618- Volume 7


---

# Proposed Listed Option Market Order

| OrderCapacity            | OrderRestrictions                                                                                                                                                                                                                                                                                         | Other                                                    |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| good faith margin basis. | Linkage - Principal acting as agent order (“P/A”) order routed through Linkage. (i.e. an order for the principal account of an eligible MM that is authorized to represent customer orders reflecting the terms of related unexecuted customer orders for which the MM is acting as agent).               | 5-Acting As a specialist or market maker in the security |
|                          | “P”Linkage – Principal order. (i.e. an order for the principal account of an eligible MM which is entered to trade at the NBBO at another exchange and is not a P/A order).                                                                                                                               | 5-Acting As a specialist or market maker in the security |
|                          | “S”Linkage – Principal satisfaction order (i.e. an order for the principal account of an eligible market maker sent through the Linkage to satisfy the liability arising from a trade through that was initiated by that market-maker).                                                                   | 5-Acting As a specialist or market maker in the security |
|                          | “Z” Proposed Code used to designate orders as defined under Filing SR-CBOE-00-62: any non-CBOE member or non-broker/dealer account which typically clears at OCC as customer, but is prohibited from entering orders on RAES (i.e. futures traders, spouses of members, MM’s away who are non B/Ds, etc). | A – Riskless Arbitrage                                   |

# CustomerOrderCapacity(tag 582) Mappings for Futures CTICode

| CustOrderCapacity (tag 582)     | Description                               |
| ------------------------------- | ----------------------------------------- |
| ~~April 30, 2003~~June 18, 2003 | 29 FIX 4.4 with Errata 20030618- Volume 7 |


---

# FIX 4.4 with Errata 20030618 - Volume 7

April 30, 2003 - June 18, 2003


| 1 | Member trading for their own account              |
| - | ------------------------------------------------- |
| 2 | Clearing Firm trading for its proprietary account |
| 3 | Member trading for another member                 |
| 4 | All other                                         |



---

Negative Prices permitted for futures and options strategies


The AvgPx(tag #6), LastPx(Tag #31), Price(tag #44), StopPx(tag #99), AllocAvgPx(tag #153), DayAvgPx(tag #426), LegLastPx(tag #637), UnderlyingLastPx(tag #651) fields can be negative to support pricing of futures and options strategies, that due to theoretical pricing can result in "buying" a strategy for a negative price (receiving a credit for the strategy) or "selling" a strategy for a price (receiving a debit for the strategy).

# 1

# Derivatives Markets Order State Transition

Derivatives markets are encouraged to adopt the following order state transition and order state reporting practices.

| Accept | 1 |
| ------ | - |
| Filled | 1 |

# NOTES:

The broker is not required to move the order from the incoming deck to the working deck before filling the Order. Therefore, the “Working=Y” might not be received by the client. The Execution Report can be sent by the broker handheld from either the Incoming Deck or the Working Deck. The Order can take one or more Fills before the Order is completed, or the Order might only be partially completed by the end of the day.

# Party Roles used for Derivatives Markets

~~April 30, 2003~~ June 18, 2003 31 FIX 4.4 with Errata 20030618- Volume 7



---

# Futures Options

| Role               | Description                                                                                                                                                                                                                                                                                                              | Order | Execution | Order | Execution |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----- | --------- | ----- | --------- |
| ExecutingFirm      | Firm that is executing the trade. Account\[1] will be associated with this firm if present. Carries resultant positions of trades at the clearing house – unless GiveupClearingFirm is specified.                                                                                                                        | Reqd  | Reqd      | Reqd  | Reqd      |
| InitiatingTrader   | If this role exists then this PartyID is the trader acronym that is reported to clearing. The Initiating Trader is associated with the ExecutingFirm. For market makers (specialists), the PartySubID for the InitiatingTrader is used for optional joint account identification.                                        | Opt   | Cond      | Opt   | Cond      |
| ClientID           | Identification of the customer of the order – also known as the correspondent firm in CMS systems. Replaces ClientID\[109]                                                                                                                                                                                               | n/a   | n/a       | Opt   | Cond      |
| ExecutingTrader    | The trader or broker that actually executes a trade. If no InitiatingTrader role exists on the trade – then the ExecutingTrader is assumed to be associated with the ExecutingFirm. For market makers (specialists), the PartySubID for PartyRole=ExecutingTrader can be used for optional joint account identification. | Opt   | Reqd      | Opt   | Cond      |
| OrderOriginator    | ID of the party entering the trade into the system (data entry, userid, buy side trader, etc.). Replaces TraderID\[466].                                                                                                                                                                                                 | Opt   | Cond      | Opt   | Cond      |
| GiveupClearingFirm | Firm that carries the position that results from a trade against the order. This is the firm to which the trade is given up. The PartySubID will be the account associated with this.                                                                                                                                    | Opt   | Cond      | Opt   | Cond      |

~~April 30, 2003~~ June 18, 2003

32 FIX 4.4 with Errata 20030618- Volume 7


---
Giveup Clearing Firm
Will be used for CMTA for US listed options.

Correspondent Clearing Firm that is going to carry the position on their books at another clearing house (exchanges). The resultant position does not reside with the market where it is traded – but instead is sent to an alternative market.

The PartySubID will be the account associated with the Correspondent Clearing Firm.

Executing System System Identifier where n/a Cond n/a Cond execution took place. For instance in some markets there are multiple execution locations – such as an electronic book or automatic execution system.

Replaces NYSE Execution Information [9433]

# MAPPING FIX 4.2 to FIX 4.3 Usage for Options Markets

| FIX 4.2                                                     | FIX 4.3                                                 | Options Order Execution |
| ----------------------------------------------------------- | ------------------------------------------------------- | ----------------------- |
| Executing Broker \[76]                                      | PartyID                                                 | Reqd                    |
|                                                             | PartyRole=ExecutingFirm                                 |                         |
| Account \[1]                                                | Account \[1]                                            | Opt                     |
| Clearing Firm \[439]                                        | PartyID                                                 | Opt                     |
|                                                             | PartyRole=GiveupClearingFirm                            |                         |
| Clearing Account \[440]                                     | PartySubID of                                           | Opt                     |
|                                                             | PartyRole=GiveupClearingFirm                            |                         |
| Market Maker Sub account information (Market Maker Acronym) | PartySubID of                                           | Opt                     |
|                                                             | PartyRole=ExecutingTrader or PartyRole=InitiatingTrader |                         |
| Optional data reported on clearing report                   | PartySubID of                                           | Opt                     |
|                                                             | PartyRole=ExecutingFirm                                 |                         |

# General Usage Information – US Futures Markets

There are three business scenarios involving give-ups and allocations within a single firm and across multiple firms in the futures industry.

~~April 30, 2003~~June 18, 2003
33 FIX 4.4 with Errata 20030618- Volume 7


---

# Scenario 1-Allocate entire trade to multiple accounts within the clearing firm.

All relevant account and allocation information is carried in the allocation block. The total quantity of the order continues to be denoted in the OrderQtyData block. The account field (tag 1) is left blank as the information is fully denoted in the allocation block as outlined in the New Order Single for Corn example in this section. Both the main party block and nested party block within the allocation block are not used to carry allocation information when allocating trades across multiple accounts within the executing firm.

# Scenario 2-Giveup entire trade to a single account at another firm

All relevant giveup information is contained in the main party block using PartyID to identify clearing firm (PartyRole=4) and PartyID to identify the carrying firm (PartyRole=14). The clearing firm suspense account is carried in account (tag 1). The carrying firm account number is populated in the PartySubID in the party block iteration when PartyRole=14. See the example contained in the Corn Calendar Multileg Order record.

# Scenario 3-Allocate entire trade to multiple accounts across multiple firms

All relevant account and giveup information is carried within the allocation block. The total quantity of the order continues to be denoted in the OrderQtyData block. The quantity to be giveup to the each firm is designated using the AllocQty (tag 80) in the allocation block. The appropriate account at the carrying firm is designated using the AllocAccount (tag 79) in the allocation block. The appropriate carrying firm is designated within the nested party block within the appropriate allocation block using the PartyRole=14.

# Execution Time Bracket reporting for US Futures Markets

The TradingSessionSubID (tag 625) is to be used to report execution time bracket codes for the US listed futures markets on the Execution Report.

~~April 30, 2003~~ June 18, 2003 34 FIX 4.4 with Errata 20030618- Volume 7


---
Example New Order – Single for Listed Futures Market
The following addresses sending a New Order - Single message into a futures market. Tags that are not used in the futures and options industries have been omitted from the record. Tags that may be used based on the Exchange, execution medium and product have been included in the record and noted as not applicable (“n/a”). (Examples of such a tag is TradingSessionSubID which is used for floor based trades to carry the required time bracket designation and therefore is not applicable to screen based trading.) The order created here is to buy 27 December 2001 Wheat at a price of 4.50. The order is being executed and cleared by firm 300. The order is also being allocated to multiple accounts within the executing firm, which is also the clearing firm as reflected in the allocation block. The order is also denoted as part of an average price group by placing a value in ClOrdLinkID field.

| Tag | Example Value | Field Name  | Rqd | Comments                                                                                                       |
| --- | ------------- | ----------- | --- | -------------------------------------------------------------------------------------------------------------- |
| 11  | XXX123        | ClOrdID     | Y   |                                                                                                                |
| 583 | 9876          | ClOrdLinkID | N   | The executions on this order will be average priced with executions on other orders with the same ClOrdLinkID. |

component block &#x3C;Parties>

| Tag | Example Value | Field Name    | Rqd | Comments                                                                                                           |
| --- | ------------- | ------------- | --- | ------------------------------------------------------------------------------------------------------------------ |
| 453 | 2             | NoPartyIDs    | N   |                                                                                                                    |
| 448 | 300           | PartyID       | N   | Firm executing and clearing the trade                                                                              |
| 447 | D             | PartyIDSource | N   |                                                                                                                    |
| 452 | 4             | PartyRole     | N   | Firm executing and clearing the trade                                                                              |
| 523 | n/a           | PartySubID    | N   | Not used when allocating trade across multiple accounts within the firm                                            |
| 448 | Tim1234       | PartyID       | N   |                                                                                                                    |
| 447 | D             | PartyIDSource | N   |                                                                                                                    |
| 452 | 13            | PartyRole     | N   | Order Originator-person who entered the order into a system, if appropriate. Generally, the user id of that person |
| 523 | n/a           | PartySubID    | N   |                                                                                                                    |

End &#x3C;/Parties>

| 1   | 1111 | Account     | N | Not used when allocating trades across multiple account within the firm |
| --- | ---- | ----------- | - | ----------------------------------------------------------------------- |
| 581 | 1    | AccountType | N | AKA Origin. Required for futures                                        |

~~April 30, 2003~~June 18, 2003 35 FIX 4.4 with Errata 20030618- Volume 7


---

# FIX 4.4 with Errata 20030618 - Volume 7

| 591 | 0      | PreallocMethod    | N |
| --- | ------ | ----------------- | - |
| 78  | 3      | NoAllocs          | N |
| 79  | 123456 | AllocAccount      | N |
| 467 | n/a    | IndividualAllocID | N |

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

# End &#x3C;NestedParties>

| 80  | 2    | AllocQty          | N |
| --- | ---- | ----------------- | - |
| 79  | 9876 | AllocAccount      | N |
| 467 | n/a  | IndividualAllocID | N |

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

# End &#x3C;NestedParties>

| 80  | 15     | AllocQty          | N |
| --- | ------ | ----------------- | - |
| 79  | 546789 | AllocAccount      | N |
| 467 | n/a    | IndividualAllocID | N |

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

# End &#x3C;NestedParties>

~~April 30, 2003~~June 18, 2003

---

# Component block &#x3C;Instrument>

| 80                                                       | 2            | AllocQty               | N      |   |   |   |
| -------------------------------------------------------- | ------------ | ---------------------- | ------ | - | - | - |
| 63                                                       |              | SettlmntTyp            | N      |   |   |   |
| 64                                                       |              | FutSettDate            | N      |   |   |   |
| 635                                                      | C            | ClearingFeeIndicator   | N      |   |   |   |
| 21                                                       | 3            | HandlInst              | Y      |   |   |   |
| Floor execution for futures markets should always be a 3 |              |                        |        |   |   |   |
| 18                                                       | n/a          | ExecInst               | N      |   |   |   |
| 110                                                      | n/a          | MinQty                 | N      |   |   |   |
| 111                                                      | n/a          | MaxFloor               | N      |   |   |   |
| 100                                                      | XCBT         | ExDestination          | N      |   |   |   |
| 386                                                      | n/a          | NoTradingSessions      | N      |   |   |   |
| 336                                                      | n/a          | TradingSessionID       | N      |   |   |   |
| 625                                                      | n/a          | TradingSessionSubID    | N      |   |   |   |
| 55                                                       | W            | Symbol                 | \*\*\* |   |   |   |
| 65                                                       |              | SymbolSfx              | N      |   |   |   |
| 48                                                       | n/a          | SecurityID             | N      |   |   |   |
| 22                                                       | n/a          | SecurityIDSource       | N      |   |   |   |
| 454                                                      | n/a          | NoSecurityAltID        | N      |   |   |   |
| 455                                                      | n/a          | SecurityAltID          | N      |   |   |   |
| 456                                                      | n/a          | SecurityAltIDSource    | N      |   |   |   |
| 461                                                      | F            | CFICode                | N      |   |   |   |
| 167                                                      | n/a          | SecurityType           | N      |   |   |   |
| 200                                                      | 200112       | MaturityMonthYear      | N      |   |   |   |
| 541                                                      | n/a          | MaturityDate           | N      |   |   |   |
| 470                                                      | n/a          | CountryOfIssue         | N      |   |   |   |
| 471                                                      | n/a          | StateOrProvinceOfIssue | N      |   |   |   |
| 472                                                      | n/a          | LocaleOfIssue          | N      |   |   |   |
| 202                                                      | n/a          | StrikePrice            | N      |   |   |   |
| 206                                                      | n/a          | OptAttribute           | N      |   |   |   |
| 231                                                      | n/a          | ContractMultiplier     | N      |   |   |   |
| 207                                                      | n/a          | SecurityExchange       | N      |   |   |   |
| 107                                                      | Wheat Future | SecurityDesc           | N      |   |   |   |
| 350                                                      | n/a          | EncodedSecurityDescLen | N      |   |   |   |

April 30, 2003

June 18, 2003

37 FIX 4.4 with Errata 20030618- Volume 7


---


# Example New Order – Single for Listed Options Market

~~April 30, 2003~~ June 18, 2003

38 FIX 4.4 with Errata 20030618- Volume 7

| 351     | n/a               | EncodedSecurityDesc | N     |
| ------- | ----------------- | ------------------- | ----- |
| ~~140~~ |                   | ~~PrevClosePx~~     | ~~N~~ |
| 54      | 1                 | Side                | Y     |
| 60      | 20010806-13:34:29 | TransactTime        | Y     |

| Component block |     |              |   |
| --------------- | --- | ------------ | - |
| 38              | 27  | OrderQty     | N |
| 152             | n/a | CashOrderQty | N |
| End             |     |              |   |

| 40      | 2     | OrdType            | Y     | Limit order.                                                              |   |
| ------- | ----- | ------------------ | ----- | ------------------------------------------------------------------------- | - |
| 44      | 4.500 | Price              | N     | Limit Price of 4.500                                                      |   |
| 99      | n/a   | StopPx             | N     |                                                                           |   |
| ~~15~~  |       | ~~Currency~~       | ~~N~~ |                                                                           |   |
| ~~376~~ |       | ~~ComplianceID~~   | ~~N~~ |                                                                           |   |
| ~~377~~ |       | ~~SolicitedFlag~~  | ~~N~~ |                                                                           |   |
| 117     | n/a   | QuoteID            | N     |                                                                           |   |
| 59      | 0     | TimeInForce        | N     |                                                                           |   |
| 168     | n/a   | EffectiveTime      | N     |                                                                           |   |
| 432     | n/a   | ExpireDate         | N     |                                                                           |   |
| 126     | n/a   | ExpireTime         | N     |                                                                           |   |
| 582     | 4     | CustOrderCapacity  | N     | Also know as Customer Type Indicator (CTI). Required for futures markets. |   |
| ~~120~~ |       | ~~SettlCurrency~~  | ~~N~~ |                                                                           |   |
| 58      | n/a   | Text               | N     |                                                                           |   |
| 354     | n/a   | EncodedTextLen     | N     |                                                                           |   |
| 355     | n/a   | EncodedText        | N     |                                                                           |   |
| 77      | n/a   | PositionEffect     | N     |                                                                           |   |
| 203     | n/a   | CoveredOrUncovered | N     |                                                                           |   |
| 210     | n/a   | MaxShow            | N     |                                                                           |   |
| 388     | n/a   | DiscretionInst     | N     |                                                                           |   |
| 389     | n/a   | DiscretionOffset   | N     |                                                                           |   |
|         |       | Standard Trailer   | Y     |                                                                           |   |


---

# New Order - Single Message in Options Market

The following addresses sending a New Order - Single message into an options market. Tags that are not used in the futures and options industries are not included in the example. Tags with strike-through text are not currently used by the industries but may be used in the future. Tags that have an example value of not applicable (n/a) are used in the industries. Herein, however, the value n/a was assigned for one of two reasons. First, specific futures and options markets may or may not utilize certain tags and, if utilized, their use and valid values would need to be addressed by participants in the particular market. Second, the order created here is to buy 5 IBM September 2001 call options with a strike price of $100.00 at a price of $5.50. This and other assumptions concerning the order, such as it is not being allocated, result in some tag values being n/a.

| Tag | Example Value | Field Name  | Rqd | Comments |
| --- | ------------- | ----------- | --- | -------- |
| 11  | XXX123        | ClOrdID     | Y   |          |
| 583 | n/a           | ClOrdLinkID | N   |          |

# Component Block: Parties

| Tag | Example Value | Field Name    | Rqd | Comments                                                               |
| --- | ------------- | ------------- | --- | ---------------------------------------------------------------------- |
| 453 | 5             | NoPartyIDs    | N   |                                                                        |
| 448 | PLC           | PartyID       | N   | Trader badge                                                           |
| 447 | C             | PartyIDSource | N   | As assigned by exchange or clearing house                              |
| 452 | 11            | PartyRole     | N   | Order Origination Trader (if different from Executing Trader) optional |
| 523 | n/a           | PartySubID    | N   |                                                                        |
| 448 | 0690          | PartyID       | N   | OCC Clearing Firm Number                                               |
| 447 | C             | PartyIDSource | N   | As assigned by exchange or clearing house                              |
| 452 | 13            | PartyRole     | N   | Order Origination Firm (if different from Executing Firm) optional     |
| 523 | n/a           | PartySubID    | N   |                                                                        |
| 448 | SMG           | PartyID       | N   | Trader Badge                                                           |
| 447 | C             | PartyIDSource | N   | As assigned by exchange or clearing house                              |
| 452 | 12            | PartyRole     | N   | Executing Trader (required)                                            |
| 523 | n/a           | PartySubID    | N   |                                                                        |
| 448 | 0427          | PartyID       | N   | OCC Clearing Firm Number                                               |
| 447 | C             | PartyIDSource | N   | As assigned by exchange or clearing house                              |
| 452 | 1             | PartyRole     | N   | Executing Firm (required)                                              |
| 523 | n/a           | PartySubID    | N   |                                                                        |
| 448 | 323           | PartyID       | N   | OCC Clearing Firm Number                                               |
| 447 | C             | PartyIDSource | N   | As assigned by exchange or clearing house                              |

~~April 30, 2003~~ June 18, 2003 39 FIX 4.4 with Errata 20030618- Volume 7


---


| 452 | 14  | PartyRole  | N | Giveup Clearing Firm (CMTA) |
| --- | --- | ---------- | - | --------------------------- |
| 523 | n/a | PartySubID | N |                             |

End &#x3C;/Parties>

# 1

|        |     | AAA  | Account             | N                   |
| ------ | --- | ---- | ------------------- | ------------------- |
| 581    |     | n/a  | AccountType         | N                   |
| 591    |     | n/a  | PreallocMethod      | N                   |
| 78     |     | n/a  | NoAllocs            | N                   |
|        | 79  | n/a  | AllocAccount        | N                   |
|        | 467 | n/a  | IndividualAllocID   | N                   |
|        | 80  | n/a  | AllocQty            | N                   |
| ~~63~~ |     |      | ~~SettlmntTyp~~     | ~~N~~               |
| ~~64~~ |     |      | ~~FutSettDate~~     | ~~N~~               |
| 21     | 2   |      | HandlInst           | Y                   |
| 18     |     | n/a  | ExecInst            | N                   |
| 110    |     | n/a  | MinQty              | N                   |
| 111    |     | n/a  | MaxFloor            | N                   |
| 100    |     | XCBO | ExDestination       | N                   |
| 386    |     | n/a  | NoTradingSessions   | N                   |
|        | 336 | n/a  | TradingSessionID    | N                   |
|        | 625 | n/a  | TradingSessionSubID | N                   |
| 54     | 1   | Side | Y                   | Buying the options. |

# Component block &#x3C;Instrument>

| 55                 | IBM           | Symbol                  | \*\*\*                                 | ExDestination ticker symbol. |
| ------------------ | ------------- | ----------------------- | -------------------------------------- | ---------------------------- |
| ~~65~~             |               | ~~SymbolSfx~~           | ~~N~~                                  |                              |
| 48                 | n/a           | SecurityID              | N                                      |                              |
| 22                 | n/a           | SecurityIDSource        | N                                      |                              |
| ~~454~~            |               | ~~NoSecurityAltID~~     | ~~N~~                                  |                              |
| ~~~~               | ~~455~~       | ~~SecurityAltID~~       | ~~N~~                                  |                              |
| ~~~~               | ~~456~~       | ~~SecurityAltIDSource~~ | ~~N~~                                  |                              |
| 461                | OC            | CFICode                 | N                                      |                              |
| ~~167~~            |               | ~~SecurityType~~        | ~~N~~                                  |                              |
| 200                | 200109        | MaturityMonthYear       | N                                      |                              |
| 541                | n/a           | MaturityDate            | N                                      |                              |
| ~~470~~            |               | ~~CountryOfIssue~~      | ~~N~~                                  |                              |
| ~~April 30, 2003~~ | June 18, 2003 | 40                      | FIX 4.4 with Errata 20030618- Volume 7 |                              |


---

# FIX 4.4 with Errata 20030618- Volume 7

|   |   | StateOrProvinceOfIssue | N                 |   |
| - | - | ---------------------- | ----------------- | - |
|   |   | LocaleOfIssue          | N                 |   |
|   |   | StrikePrice            | N                 |   |
|   |   | OptAttribute           | N                 |   |
|   |   | ContractMultiplier     | N                 |   |
|   |   | SecurityExchange       | N                 |   |
|   |   | SecurityDesc           | N                 |   |
|   |   | EncodedSecurityDescLen | N                 |   |
|   |   | EncodedSecurityDesc    | N                 |   |
|   |   | PrevClosePx            | N                 |   |
|   |   | TransactTime           | 20010806-13:34:29 | Y |

# Component block

# OrderQtyData

| OrderQty     | N |
| ------------ | - |
| CashOrderQty | N |

| OrdType           | 2   |
| ----------------- | --- |
| Price             | 5.5 |
| StopPx            | n/a |
| Currency          | n/a |
| ComplianceID      | n/a |
| SolicitedFlag     | n/a |
| QuoteID           | n/a |
| TimeInForce       | 0   |
| EffectiveTime     | n/a |
| ExpireDate        | n/a |
| ExpireTime        | n/a |
| OrderCapacity     | A   |
| OrderRestrictions | n/a |
| CustOrderCapacity | n/a |
| SettlCurrency     | n/a |
| Text              | n/a |
| EncodedTextLen    | n/a |
| EncodedText       | n/a |

April 30, 2003

June 18, 2003

Y Limit order

N Buy at price of 5.5


---

# Example New Order - Multileg for Listed Futures Market (Spread Order)

The following addresses sending a New Order – Multileg message into a futures market.

Tags that are not used in the futures and options industries are not included in the example.

Tags with strike-through text are not currently used by the industries but may be used in the future.

Tags that have an example value of not applicable (n/a) are used in the futures industry. Herein, however, the value n/a was assigned for one of two reasons. First, specific futures and options markets may or may not utilize certain tags and, if utilized, their use and valid values would need to be addressed by participants in the particular market. (Examples of such tags are MultiLegRptTypeReq [563] and TradingSessionID [336].)

Second, the order created here is to buy 15 May 2002 - July 2002 Corn spreads at a price of –12. Some specifics concerning the order, such as it is not being allocated, result in some tag values being n/a.

The direction of the strategy is indicated by the Side (54) taken. When a strategy is pre-defined by a futures or options market and an inconsistency arises between:

- the strategy indicated and the Side, LegSide (624), and/or LegRatioQty (623), or
- the Side indicated and any LegSide indicated

the sell-side may either reject the order or accept the order. If the sell-side accepts the order it will be based on the strategy and Side indicated with any inconsistencies in LegSide and/or LegRatioQty being ignored.

The example also has any trade resulting from this order being given up to another firm. The firm being given up to will carry the trade on its books.

|     | Tag | Example Value   | Field Name  | Rqd | Comments     |
| --- | --- | --------------- | ----------- | --- | ------------ |
|     |     | Standard Header |             | Y   | MsgType = AB |
| 11  |     | 1234567897      | ClOrdID     | Y   |              |
| 583 |     | n/a             | ClOrdLinkID | N   |              |

~~April 30, 2003~~ June 18, 2003 42 FIX 4.4 with Errata 20030618- Volume 7


---


| 453 | 3       | NoPartyIDs    | N |                                                  |
| --- | ------- | ------------- | - | ------------------------------------------------ |
| 448 | 560     | PartyID       | N | Firm executing and clearing the trade            |
| 447 | D       | PartyIDSource | N |                                                  |
| 452 | 4       | PartyRole     | N |                                                  |
| 523 | n/a     | PartySubID    | N |                                                  |
| 448 | 500     | PartyID       | N | Trade being given up to and carried by this firm |
| 447 | D       | PartyIDSource | N |                                                  |
| 452 | 14      | PartyRole     | N |                                                  |
| 523 | 789567  | PartySubID    | N | Customer account number at carrying firm         |
| 448 | Tim1234 | PartyID       | N |                                                  |
| 447 | D       | PartyIDSource | N |                                                  |
| 452 | 13      | PartyRole     | N |                                                  |
| 523 | n/a     | PartySubID    | N |                                                  |

</parties>

| 1   |     | abcdef | Account           | N | Account mnemonic as known by bookkeeping system. In case of giveup specified in party block, this account is at the executing firm. |
| --- | --- | ------ | ----------------- | - | ----------------------------------------------------------------------------------------------------------------------------------- |
| 581 | 1   |        | AccountType       | N | Also known as Origin. Required for futures markets.                                                                                 |
| 591 |     | n/a    | PreallocMethod    | N |                                                                                                                                     |
| 78  |     | n/a    | NoAllocs          | N |                                                                                                                                     |
|     | 79  | n/a    | AllocAccount      | N |                                                                                                                                     |
|     | 467 | n/a    | IndividualAllocID | N |                                                                                                                                     |

<nestedparties>
| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

</nestedparties>

| 80 | n/a | AllocQty | N |
| -- | --- | -------- | - |

~~63~~
~~SettlmntTyp~~
~~N~~
~~April 30, 2003~~
June 18, 2003
43 FIX 4.4 with Errata 20030618- Volume 7


---


# 64

|   |   | FutSettDate          | N |                                                            |
| - | - | -------------------- | - | ---------------------------------------------------------- |
|   |   | ClearingFeeIndicator | N |                                                            |
|   |   | HandlInst            | Y | Floor executions for futures markets should always be "3". |
|   |   | ExecInst             | N |                                                            |
|   |   | MinQty               | N |                                                            |
|   |   | MaxFloor             | N |                                                            |
|   |   | ExDestination        | N | XCBT                                                       |
|   |   | NoTradingSessions    | N |                                                            |
|   |   | TradingSessionID     | N | 336                                                        |
|   |   | TradingSessionSubID  | N | 625                                                        |
|   |   | Side                 | Y | Buying the strategy.                                       |

# Component block &#x3C;Instrument>

|   |   | Symbol                 | \*\*\* | ExDestination ticker symbol. |
| - | - | ---------------------- | ------ | ---------------------------- |
|   |   | SymbolSfx              | N      |                              |
|   |   | SecurityID             | N      |                              |
|   |   | SecurityIDSource       | N      |                              |
|   |   | NoSecurityAltID        | N      |                              |
|   |   | SecurityAltID          | N      |                              |
|   |   | SecurityAltIDSource    | N      |                              |
|   |   | CFICode                | N      | FM                           |
|   |   | SecurityType           | N      |                              |
|   |   | MaturityMonthYear      | N      |                              |
|   |   | MaturityDate           | N      |                              |
|   |   | CountryOfIssue         | N      |                              |
|   |   | StateOrProvinceOfIssue | N      |                              |
|   |   | LocaleOfIssue          | N      |                              |
|   |   | StrikePrice            | N      |                              |
|   |   | OptAttribute           | N      |                              |
|   |   | ContractMultiplier     | N      |                              |
|   |   | SecurityExchange       | N      |                              |
|   |   | SecurityDesc           | N      |                              |
|   |   | EncodedSecurityDescLen | N      |                              |
|   |   | EncodedSecurityDesc    | N      |                              |

End &#x3C;/Instrument>

April 30, 2003

June 18, 2003

44

FIX 4.4 with Errata 20030618- Volume 7



---

# 140

|     | n/a | PrevClosePx | N |
| --- | --- | ----------- | - |
| 555 | 2   | NoLegs      | Y |

# Component block &#x3C;Instrument Leg>

| 600 | C      | LegSymbol            | \*\*\*                    | ExDestination ticker symbol. |
| --- | ------ | -------------------- | ------------------------- | ---------------------------- |
|     | 601    |                      | LegSymbolSfx              | N                            |
|     | 602    | n/a                  | LegSecurityID             | N                            |
|     | 603    | n/a                  | LegSecurityIDSource       | N                            |
|     | 604    | n/a                  | NoLegSecurityAltID        | N                            |
|     | 605    |                      | LegSecurityAltID          | N                            |
|     | 606    |                      | LegSecurityAltIDSource    | N                            |
| 608 | F      | LegCFICode           | N                         | Commodity Future             |
|     | 609    |                      | LegSecurityType           | N                            |
| 610 | 200205 | LegMaturityMonthYear | N                         | May 2002 maturity.           |
|     | 611    | n/a                  | LegMaturityDate           | N                            |
|     | 596    |                      | LegCountryOfIssue         | N                            |
|     | 597    |                      | LegStateOrProvinceOfIssue | N                            |
|     | 598    |                      | LegLocaleOfIssue          | N                            |
|     | 612    | n/a                  | LegStrikePrice            | N                            |
|     | 613    | n/a                  | LegOptAttribute           | N                            |
|     | 614    |                      | LegContractMultiplier     | N                            |
|     | 616    | n/a                  | LegSecurityExchange       | N                            |
|     | 620    | Corn Future          | LegSecurityDesc           | N                            |
|     | 621    | n/a                  | EncodedLegSecurityDescLen | N                            |
|     | 622    | n/a                  | EncodedLegSecurityDesc    | N                            |
| 623 | 1      | LegRatioQty          | N                         | Equal ratios.                |
| 624 | 1      | LegSide              | N                         | Buy leg.                     |
|     | 564    | n/a                  | LegPositionEffect         | N                            |
|     | 565    | n/a                  | LegCoveredOrUncovered     | N                            |

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

End &#x3C;/NestedParties>

April 30, 2003

June 18, 2003

45

FIX 4.4 with Errata 20030618- Volume 7


---

FIX 4.4 with Errata 20030618 - Volume 7

# Leg Information

| 654 | n/a         | LegRefID                  | N      |
| --- | ----------- | ------------------------- | ------ |
| 566 | n/a         | LegPrice                  | N      |
| 587 | n/a         | LegSettlmntTyp            | N      |
| 588 | n/a         | LegFutSettDate            | N      |
| 600 | C           | LegSymbol                 | \*\*\* |
| 601 |             | LegSymbolSfx              | N      |
| 602 | n/a         | LegSecurityID             | N      |
| 603 | n/a         | LegSecurityIDSource       | N      |
| 604 |             | NoLegSecurityAltID        | N      |
| 605 |             | LegSecurityAltID          | N      |
| 606 |             | LegSecurityAltIDSource    | N      |
| 608 | F           | LegCFICode                | N      |
| 610 | 200207      | LegMaturityMonthYear      | N      |
| 611 | n/a         | LegMaturityDate           | N      |
| 596 |             | LegCountryOfIssue         | N      |
| 597 |             | LegStateOrProvinceOfIssue | N      |
| 598 |             | LegLocaleOfIssue          | N      |
| 612 | n/a         | LegStrikePrice            | N      |
| 613 | n/a         | LegOptAttribute           | N      |
| 614 | n/a         | LegContractMultiplier     | N      |
| 616 | n/a         | LegSecurityExchange       | N      |
| 620 | Corn Future | LegSecurityDesc           | N      |
| 621 | n/a         | EncodedLegSecurityDescLen | N      |
| 622 | n/a         | EncodedLegSecurityDesc    | N      |
| 623 | 1           | LegRatioQty               | N      |
| 624 | 2           | LegSide                   | N      |
| 564 | n/a         | LegPositionEffect         | N      |
| 565 | n/a         | LegCoveredOrUncovered     | N      |

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |


April 30, 2003 - June 18, 2003

---

# Instrument Leg

| 654 | n/a | LegRefID       | N |
| --- | --- | -------------- | - |
| 566 | n/a | LegPrice       | N |
| 587 | n/a | LegSettlmntTyp | N |
| 588 | n/a | LegFutSettDate | N |

End

# TransactTime

| 60 | 20010509-09:20:15 | TransactTime | Y |
| -- | ----------------- | ------------ | - |

# OrderQtyData

| 38  | 15  | OrderQty     | N |
| --- | --- | ------------ | - |
| 152 | n/a | CashOrderQty | N |

End

# Order Details

| 40  | 2   | OrdType            | Y | Limit order.                                                              |
| --- | --- | ------------------ | - | ------------------------------------------------------------------------- |
| 44  | -12 | Price              | N | Buy strategy at negative 12.                                              |
| 99  | n/a | StopPx             | N |                                                                           |
| 15  | n/a | Currency           | N |                                                                           |
| 376 | n/a | ComplianceID       | N |                                                                           |
| 377 | n/a | SolicitedFlag      | N |                                                                           |
| 117 | n/a | QuoteID            | N |                                                                           |
| 59  | 0   | TimeInForce        | N |                                                                           |
| 168 | n/a | EffectiveTime      | N |                                                                           |
| 432 | n/a | ExpireDate         | N |                                                                           |
| 126 | n/a | ExpireTime         | N |                                                                           |
| 528 |     | OrderCapacity      | N | Used for options markets.                                                 |
| 529 |     | OrderRestrictions  | N | Used for options markets.                                                 |
| 582 | 4   | CustOrderCapacity  | N | Also know as Customer Type Indicator (CTI). Required for futures markets. |
| 120 | n/a | SettlCurrency      | N |                                                                           |
| 58  | n/a | Text               | N |                                                                           |
| 354 | n/a | EncodedTextLen     | N |                                                                           |
| 355 | n/a | EncodedText        | N |                                                                           |
| 77  | n/a | PositionEffect     | N |                                                                           |
| 203 | n/a | CoveredOrUncovered | N |                                                                           |
| 210 | n/a | MaxShow            | N |                                                                           |
| 388 | n/a | DiscretionInst     | N |                                                                           |

April 30, 2003

June 18, 2003

47 FIX 4.4 with Errata 20030618- Volume 7


---

# Example New Order - Multileg for Listed Futures Market (Butterfly Strategy)

The following addresses sending a New Order – Multileg message into a futures market.

Tags that are not used in the futures and options industries are not included in the example.

Tags with strike-through text are not currently used by the industries but may be used in the future.

Tags that have an example value of not applicable (n/a) are used in the industries. Herein, however, the value n/a was assigned for one of two reasons. First, specific futures and options markets may or may not utilize certain tags and, if utilized, their use and valid values would need to be addressed by participants in the particular market. (Examples of such tags are MultiLegRptTypeReq [563] and TradingSessionID [336].)

Second, the order created here is to buy 10 EuroDollar butterfly spreads at a price of -3.0, and is assumed that it will be productized by the sell-side on its electronic order matching system (ie: trade engine). This and other assumptions concerning the order, such as it is not being allocated, result in some tag values being n/a. (An example is the SecurityID [48] which the buy-side would not know until the sell-side has productized the butterfly.)

The direction of the strategy is indicated by the Side (54) taken. When a strategy is pre-defined by a futures market and an inconsistency arises between:

- the strategy indicated and the Side, LegSide (624), and/or LegRatioQty (623), or
- the Side indicated and any LegSide indicated

the sell-side may either reject the order or accept the order. If the sell-side accepts the order it will be based on the strategy and Side indicated with any inconsistencies in LegSide and/or LegRatioQty being ignored.

| Tag             | Example Value      | Field Name   | Rqd | Comments |
| --------------- | ------------------ | ------------ | --- | -------- |
| Standard Header |                    | MsgType = AB | Y   |          |
| 11              | 05092001- NY-78955 | ClOrdID      | Y   |          |
| 583             | n/a                | ClOrdLinkID  | N   |          |

component block &#x3C;Parties>

| 453 | 2   | NoPartyIDs | N |   |
| --- | --- | ---------- | - | - |
| 448 | 001 | PartyID    | N |   |

~~April 30, 2003~~June 18, 2003

48 FIX 4.4 with Errata 20030618- Volume 7


---

447     D                PartyIDSource             N
452     4                PartyRole                 N
523     n/a              PartySubID                N
448     4114Z9871272     PartyID                   N
447     D                PartyIDSource             N
452     13               PartyRole                 N
523     n/a              PartySubID                N
End

1                        Z9871272         Account                   N
581                      1                AccountType               N
591                      n/a              PreallocMethod            N
78                       n/a              NoAllocs                  N
79      n/a              AllocAccount              N
467      n/a              IndividualAllocID         N

Component block <nestedparties>
539       n/a              NoNestedPartyIDs          N
524       n/a              NestedPartyID             N
525       n/a              NestedPartyIDSource       N
538       n/a              NestedPartyRole           N
545       n/a              NestedPartySubID          N
End </nestedparties>

80      n/a              AllocQty                  N
~~63~~                                 ~~SettlmntTyp~~        ~~N~~
~~64~~                                 ~~FutSettDate~~        ~~N~~
635                      C                ClearingFeeIndicator      N
21                       1                HandlInst                 Y
18                       n/a              ExecInst                  N
110                      n/a              MinQty                    N
111                      n/a              MaxFloor                  N
100                      XCME             ExDestination             N
386                      n/a              NoTradingSessions         N
336      n/a              TradingSessionID          N
625      n/a              TradingSessionSubID       N
54                       1                Side                      Y
~~April 30, 2003~~    June 18, 2003                          49  FIX 4.4 with Errata 20030618- Volume 7


---

# Component block &#x3C;Instrument>

| 55      | GE:BF   | Symbol                     | \*\*\* |
| ------- | ------- | -------------------------- | ------ |
| ~~65~~  |         | ~~SymbolSfx~~              | ~~N~~  |
| 48      | n/a     | SecurityID                 | N      |
| 22      | n/a     | SecurityIDSource           | N      |
| ~~454~~ |         | ~~NoSecurityAltID~~        | ~~N~~  |
| ~~~~    | ~~455~~ | ~~SecurityAltID~~          | ~~N~~  |
| ~~~~    | ~~456~~ | ~~SecurityAltIDSource~~    | ~~N~~  |
| 461     | FM      | CFICode                    | N      |
| ~~167~~ |         | ~~SecurityType~~           | ~~N~~  |
| 200     | n/a     | MaturityMonthYear          | N      |
| 541     | n/a     | MaturityDate               | N      |
| ~~470~~ |         | ~~CountryOfIssue~~         | ~~N~~  |
| ~~471~~ |         | ~~StateOrProvinceOfIssue~~ | ~~N~~  |
| ~~472~~ |         | ~~LocaleOfIssue~~          | ~~N~~  |
| 202     | n/a     | StrikePrice                | N      |
| 206     | n/a     | OptAttribute               | N      |
| ~~231~~ |         | ~~ContractMultiplier~~     | ~~N~~  |
| 207     | n/a     | SecurityExchange           | N      |
| 107     | n/a     | SecurityDesc               | N      |
| 350     | n/a     | EncodedSecurityDescLen     | N      |
| 351     | n/a     | EncodedSecurityDesc        | N      |

# End &#x3C;/Instrument>

| ~~140~~ | ~~PrevClosePx~~ | ~~N~~  |   |
| ------- | --------------- | ------ | - |
| 555     | 3               | NoLegs | Y |

# Component block &#x3C;Instrument Leg>

| 600  | GE           | LegSymbol                  | \*\*\* |
| ---- | ------------ | -------------------------- | ------ |
| ~~~~ | ~~601~~      | ~~LegSymbolSfx~~           | ~~N~~  |
| 602  | CME005060001 | LegSecurityID              | N      |
| 603  | ISIN         | LegSecurityIDSource        | N      |
| ~~~~ | ~~604~~      | ~~NoLegSecurityAltID~~     | ~~N~~  |
| ~~~~ | ~~605~~      | ~~LegSecurityAltID~~       | ~~N~~  |
| ~~~~ | ~~606~~      | ~~LegSecurityAltIDSource~~ | ~~N~~  |
| 608  | F            | LegCFICode                 | N      |

~~April 30, 2003~~ June 18, 2003 50 FIX 4.4 with Errata 20030618- Volume 7


---

# Leg Security Information

|   | 609 |        | LegSecurityType           | N |
| - | --- | ------ | ------------------------- | - |
|   | 610 | 200109 | LegMaturityMonthYear      | N |
|   | 611 | n/a    | LegMaturityDate           | N |
|   | 596 |        | LegCountryOfIssue         | N |
|   | 597 |        | LegStateOrProvinceOfIssue | N |
|   | 598 |        | LegLocaleOfIssue          | N |
|   | 612 | n/a    | LegStrikePrice            | N |
|   | 613 | n/a    | LegOptAttribute           | N |
|   | 614 |        | LegContractMultiplier     | N |
|   | 616 | n/a    | LegSecurityExchange       | N |
|   | 620 | GEU1   | LegSecurityDesc           | N |
|   | 621 | n/a    | EncodedLegSecurityDescLen | N |
|   | 622 | n/a    | EncodedLegSecurityDesc    | N |
|   | 623 | 1      | LegRatioQty               | N |
|   | 624 | 1      | LegSide                   | N |
|   | 564 | n/a    | LegPositionEffect         | N |
|   | 565 | n/a    | LegCoveredOrUncovered     | N |

# Component block <nestedparties></nestedparties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

# End &#x3C;NestedParties>

| 654 | n/a          | LegRefID               | N      |
| --- | ------------ | ---------------------- | ------ |
| 566 | n/a          | LegPrice               | N      |
| 587 |              | LegSettlmntTyp         | N      |
| 588 |              | LegFutSettDate         | N      |
| 600 | GE           | LegSymbol              | \*\*\* |
| 601 | n/a          | LegSymbolSfx           | N      |
| 602 | CME005060004 | LegSecurityID          | N      |
| 603 | ISIN         | LegSecurityIDSource    | N      |
| 604 | n/a          | NoLegSecurityAltID     | N      |
| 605 |              | LegSecurityAltID       | N      |
| 606 |              | LegSecurityAltIDSource | N      |

April 30, 2003

June 18, 2003

51

FIX 4.4 with Errata 20030618- Volume 7


---

# FIX 4.4 with Errata 20030618- Volume 7

| 608 | F      | LegCFICode                | N |
| --- | ------ | ------------------------- | - |
| 609 |        | LegSecurityType           | N |
| 610 | 200112 | LegMaturityMonthYear      | N |
| 611 | n/a    | LegMaturityDate           | N |
| 596 |        | LegCountryOfIssue         | N |
| 597 |        | LegStateOrProvinceOfIssue | N |
| 598 |        | LegLocaleOfIssue          | N |
| 612 | n/a    | LegStrikePrice            | N |
| 613 | n/a    | LegOptAttribute           | N |
| 614 |        | LegContractMultiplier     | N |
| 616 | n/a    | LegSecurityExchange       | N |
| 620 | GEZ1   | LegSecurityDesc           | N |
| 621 | n/a    | EncodedLegSecurityDescLen | N |
| 622 | n/a    | EncodedLegSecurityDesc    | N |
| 623 | 2      | LegRatioQty               | N |
| 624 | 2      | LegSide                   | N |
| 564 | n/a    | LegPositionEffect         | N |
| 565 | n/a    | LegCoveredOrUncovered     | N |

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

# End &#x3C;/NestedParties>

| 654 | n/a          | LegRefID            |
| --- | ------------ | ------------------- |
| 566 | n/a          | LegPrice            |
| 587 |              | LegSettlmntTyp      |
| 588 |              | LegFutSettDate      |
| 600 | GE           | LegSymbol           |
| 601 |              | LegSymbolSfx        |
| 602 | CME005060007 | LegSecurityID       |
| 603 | ISIN         | LegSecurityIDSource |
| 604 |              | NoLegSecurityAltID  |
| 605 |              | LegSecurityAltID    |

April 30, 2003

June 18, 2003

52


---

# 606 LegSecurityAltIDSource

| 608 | F      | LegCFICode                | N |
| --- | ------ | ------------------------- | - |
| 609 |        | LegSecurityType           | N |
| 610 | 200203 | LegMaturityMonthYear      | N |
| 611 | n/a    | LegMaturityDate           | N |
| 596 |        | LegCountryOfIssue         | N |
| 597 |        | LegStateOrProvinceOfIssue | N |
| 598 |        | LegLocaleOfIssue          | N |
| 612 | n/a    | LegStrikePrice            | N |
| 613 | n/a    | LegOptAttribute           | N |
| 614 |        | LegContractMultiplier     | N |
| 616 | n/a    | LegSecurityExchange       | N |
| 620 | GEH2   | LegSecurityDesc           | N |
| 621 | n/a    | EncodedLegSecurityDescLen | N |
| 622 | n/a    | EncodedLegSecurityDesc    | N |
| 623 | 1      | LegRatioQty               | N |
| 624 | 1      | LegSide                   | N |
| 564 | n/a    | LegPositionEffect         | N |
| 565 | n/a    | LegCoveredOrUncovered     | N |

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

# End &#x3C;/NestedParties>

| 654 | n/a | LegRefID       | N |
| --- | --- | -------------- | - |
| 566 | n/a | LegPrice       | N |
| 587 |     | LegSettlmntTyp | N |
| 588 |     | LegFutSettDate | N |

# 60

| 20010509-09:20:15 | TransactTime | Y |
| ----------------- | ------------ | - |

# Component block &#x3C;OrderQtyData>

| April 30, 2003 | June 18, 2003 | 53 | FIX 4.4 with Errata 20030618- Volume 7 |
| -------------- | ------------- | -- | -------------------------------------- |


---


# 38

|     |     | OrderQty     | N |
| --- | --- | ------------ | - |
| 152 | n/a | CashOrderQty | N |

End &#x3C;/OrderQtyData>

# 40

|         | 2       | OrdType            | Y     |
| ------- | ------- | ------------------ | ----- |
|         | -3.0    | Price              | N     |
| 99      | n/a     | StopPx             | N     |
| ~~15~~  |         | ~~Currency~~       | ~~N~~ |
| ~~376~~ |         | ~~ComplianceID~~   | ~~N~~ |
| ~~377~~ |         | ~~SolicitedFlag~~  | ~~N~~ |
| 117     | n/a     | QuoteID            | N     |
| 59      | 0       | TimeInForce        | N     |
| 168     | n/a     | EffectiveTime      | N     |
| 432     | n/a     | ExpireDate         | N     |
| 126     | n/a     | ExpireTime         | N     |
| 528     |         | OrderCapacity      | N     |
| 529     |         | OrderRestrictions  | N     |
| 582     | 4       | CustOrderCapacity  | N     |
| ~~120~~ | ~~n/a~~ | ~~SettlCurrency~~  | ~~N~~ |
| 58      | n/a     | Text               | N     |
| 354     | n/a     | EncodedTextLen     | N     |
| 355     | n/a     | EncodedText        | N     |
| 77      | n/a     | PositionEffect     | N     |
| 203     | n/a     | CoveredOrUncovered | N     |
| 210     | n/a     | MaxShow            | N     |
| 388     | n/a     | DiscretionInst     | N     |
| 389     | n/a     | DiscretionOffset   | N     |
| 563     | n/a     | MultiLegRptTypeReq | N     |
|         |         | Standard Trailer   | Y     |

# Example Multileg Execution Report for Listed Futures Market

Multileg Execution Report Example for Futures Markets

The following addresses receiving an Execution Report – Multileg message.

Tags that are not used in the futures and options industries are not included in the example.

Tags with strike-through text are not currently used by the industries but may be used in the future.

~~April 30, 2003~~ June 18, 2003

54 FIX 4.4 with Errata 20030618- Volume 7


---
Tags that have an example value of not applicable (n/a) are used in the industries. Herein, however, the value n/a was assigned for one of two reasons. First, individual futures and options markets may or may not utilize certain tags and, if utilized, their use and valid values would need to be addressed by participants in the particular market.

The execution report references an order to buy 15 July 2001/September 2001 Corn Spreads. The order is a give-up trade being executed and cleared by firm 560 and carried on the books of firm 500. This is the first execution of the order and it is for a total of 5 spreads. The order was executed on the trading floor as atomically and is being reported to the customer atomically via this execution report. The order will also be cleared atomically.

| Tag | Example Values | Field Name       | Rqd | Comments |
| --- | -------------- | ---------------- | --- | -------- |
| 37  | 987654         | OrderID          | Y   |          |
| 198 | n/a            | SecondaryOrderID | N   |          |
| 526 | n/a            | SecondaryClOrdID | N   |          |
| 527 | n/a            | SecondaryExecID  | N   |          |
| 11  | 123456789      | ClOrdID          | N   |          |
| 41  | n/a            | OrigClOrdID      | N   |          |
| 583 | n/a            | ClOrdLinkID      | N   |          |

component block &#x3C;Parties>

| Tag | Example Values | Field Name    | Rqd | Comments |
| --- | -------------- | ------------- | --- | -------- |
| 453 | 3              | NoPartyIDs    | N   |          |
| 448 | 560            | PartyID       | N   |          |
| 447 | D              | PartyIDSource | N   |          |
| 452 | 4              | PartyRole     | N   |          |
| 523 | n/a            | PartySubID    | N   |          |
| 448 | 500            | PartyID       | N   |          |
| 447 | D              | PartyIDSource | N   |          |
| 452 | 14             | PartyRole     | N   |          |
| 523 | 789567         | PartySubID    | N   |          |
| 448 | tim1234        | PartyID       | N   |          |
| 447 | D              | PartyIDSource | N   |          |
| 452 | 13             | PartyRole     | N   |          |
| 523 | n/a            | PartySubID    | N   |          |

End &#x3C;/Parties>

| 382 | 1   | NoContraBrokers | N |   |
| --- | --- | --------------- | - | - |
| 375 | 455 | ContraBroker    | N |   |

~~April 30, 2003~~June 18, 2003 55 FIX 4.4 with Errata 20030618- Volume 7
---



| 337    | ABC               | ContraTrader          | N     |
| ------ | ----------------- | --------------------- | ----- |
| 437    | 5                 | ContraTradeQty        | N     |
| 438    | 20010509-09:22:40 | ContraTradeTime       | N     |
| 655    | n/a               | ContraLegRefID        | N     |
| 66     | n/a               | ListID                | N     |
| 548    | n/a               | CrossID               | N     |
| 551    | n/a               | OrigCrossID           | N     |
| 549    | n/a               | CrossType             | N     |
| 17     | X6789             | ExecID                | Y     |
| 19     | n/a               | ExecRefID             | N     |
| 150    | F                 | ExecType              | Y     |
| 39     | 1                 | OrdStatus             | Y     |
| 636    | Y                 | WorkingIndicator      | N     |
| 103    | n/a               | OrdRejReason          | N     |
| 378    | n/a               | ExecRestatementReason | N     |
| 1      | abcdef            | Account               | N     |
| 581    | 1                 | AccountType           | N     |
| 591    | n/a               | PreallocMethod        | N     |
| ~~63~~ |                   | ~~SettlmntTyp~~       | ~~N~~ |
| ~~64~~ |                   | ~~FutSettDate~~       | ~~N~~ |
| 635    | C                 | ClearingFeeIndicator  | N     |

# Component block &#x3C;Instrument>

| 55      | C:CAL   | Symbol                     | \*\*\* |
| ------- | ------- | -------------------------- | ------ |
| ~~65~~  |         | ~~SymbolSfx~~              | ~~N~~  |
| 48      | n/a     | SecurityID                 | N      |
| 22      | n/a     | SecurityIDSource           | N      |
| ~~454~~ |         | ~~NoSecurityAltID~~        | ~~N~~  |
| ~~~~    | ~~455~~ | ~~SecurityAltID~~          | ~~N~~  |
| ~~~~    | ~~456~~ | ~~SecurityAltIDSource~~    | ~~N~~  |
| 461     | FM      | CFICode                    | N      |
| ~~167~~ |         | ~~SecurityType~~           | ~~N~~  |
| 200     | n/a     | MaturityMonthYear          | N      |
| 541     | n/a     | MaturityDate               | N      |
| ~~470~~ |         | ~~CountryOfIssue~~         | ~~N~~  |
| ~~471~~ |         | ~~StateOrProvinceOfIssue~~ | ~~N~~  |
| ~~472~~ |         | ~~LocaleOfIssue~~          | ~~N~~  |

April 30, 2003

June 18, 2003

56 FIX 4.4 with Errata 20030618- Volume 7



---

# Instrument

| 202 | n/a | StrikePrice            | N |
| --- | --- | ---------------------- | - |
| 206 | n/a | OptAttribute           | N |
| 231 |     | ContractMultiplier     | N |
| 207 | n/a | SecurityExchange       | N |
| 107 | n/a | SecurityDesc           | N |
| 350 | n/a | EncodedSecurityDescLen | N |
| 351 | n/a | EncodedSecurityDesc    | N |

End &#x3C;/Instrument>

# Instrument Leg

| 54  | 1 | Side   | Y |
| --- | - | ------ | - |
| 555 | 2 | NoLegs | Y |

Number of legs. Can be zero – must be provided even if zero

| Component block | \<Instrument Leg> |             |                           |        |
| --------------- | ----------------- | ----------- | ------------------------- | ------ |
|                 | 600               | C           | LegSymbol                 | \*\*\* |
|                 | 601               |             | LegSymbolSfx              | N      |
|                 | 602               | n/a         | LegSecurityID             | N      |
|                 | 603               | n/a         | LegSecurityIDSource       | N      |
|                 | 604               |             | NoLegSecurityAltID        | N      |
|                 | 605               |             | LegSecurityAltID          | N      |
|                 | 606               |             | LegSecurityAltIDSource    | N      |
|                 | 608               | F           | LegCFICode                | N      |
|                 | 609               |             | LegSecurityType           | N      |
|                 | 610               | 200105      | LegMaturityMonthYear      | N      |
|                 | 611               | n/a         | LegMaturityDate           | N      |
|                 | 596               |             | LegCountryOfIssue         | N      |
|                 | 597               |             | LegStateOrProvinceOfIssue | N      |
|                 | 598               |             | LegLocaleOfIssue          | N      |
|                 | 612               | n/a         | LegStrikePrice            | N      |
|                 | 613               | n/a         | LegOptAttribute           | N      |
|                 | 614               |             | LegContractMultiplier     | N      |
|                 | 616               | n/a         | LegSecurityExchange       | N      |
|                 | 620               | Corn Future | LegSecurityDesc           | N      |
|                 | 621               | n/a         | EncodedLegSecurityDescLen | N      |
|                 | 622               | n/a         | EncodedLegSecurityDesc    | N      |
|                 | 623               | 1           | LegRatioQty               | N      |
|                 | 624               | 1           | LegSide                   | N      |
|                 | 564               | n/a         | LegPositionEffect         | N      |

Provide if the PositionEffect for the

~~April 30, 2003~~ June 18, 2003 57 FIX 4.4 with Errata 20030618- Volume 7


---

# Leg Information

| 565 | n/a | LegCoveredOrUncovered | N | Provide if the CoveredOrUncovered for the leg is different from that specified for the overall multileg security. |
| --- | --- | --------------------- | - | ----------------------------------------------------------------------------------------------------------------- |

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

# End &#x3C;/NestedParties>

| 654     | n/a    | LegRefID                      | N                                                                                                                                           |   |
| ------- | ------ | ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | - |
| 566     | n/a    | LegPrice                      | Provide only if a price was specified for the specific leg. Used for anchoring the overall multileg security price to a specific leg price. |   |
| 637     | n/a    | LegLastPx                     | Used to report the execution price assigned to the leg of the multileg instrument                                                           |   |
| ~~587~~ |        | ~~LegSettlmntTyp~~            |                                                                                                                                             |   |
| ~~588~~ |        | ~~LegFutSettDate~~            | ~~Required when SettlmntTyp = 6 (Future) or SettlmntTyp = 8 (Sellers Option)~~                                                              |   |
| 600     | C      | LegSymbol                     | \*\*\*                                                                                                                                      |   |
| ~~601~~ |        | ~~LegSymbolSfx~~              | ~~N~~                                                                                                                                       |   |
| 602     | n/a    | LegSecurityID                 | N                                                                                                                                           |   |
| 603     | n/a    | LegSecurityIDSource           | N                                                                                                                                           |   |
| ~~604~~ |        | ~~NoLegSecurityAltID~~        | ~~N~~                                                                                                                                       |   |
| ~~605~~ |        | ~~LegSecurityAltID~~          | ~~N~~                                                                                                                                       |   |
| ~~606~~ |        | ~~LegSecurityAltIDSource~~    | ~~N~~                                                                                                                                       |   |
| 608     | F      | LegCFICode                    | N                                                                                                                                           |   |
| ~~609~~ |        | ~~LegSecurityType~~           | ~~N~~                                                                                                                                       |   |
| 610     | 200107 | LegMaturityMonthYear          | N                                                                                                                                           |   |
| 611     | n/a    | LegMaturityDate               | N                                                                                                                                           |   |
| ~~596~~ |        | ~~LegCountryOfIssue~~         | ~~N~~                                                                                                                                       |   |
| ~~597~~ |        | ~~LegStateOrProvinceOfIssue~~ | ~~N~~                                                                                                                                       |   |
| ~~598~~ |        | ~~LegLocaleOfIssue~~          | ~~N~~                                                                                                                                       |   |
| 612     | n/a    | LegStrikePrice                | N                                                                                                                                           |   |
| 613     | n/a    | LegOptAttribute               | N                                                                                                                                           |   |

April 30, 2003

June 18, 2003

58

FIX 4.4 with Errata 20030618- Volume 7


---

# 614 LegContractMultiplier

| 616 | n/a         | LegSecurityExchange       | N |
| --- | ----------- | ------------------------- | - |
| 620 | Corn Future | LegSecurityDesc           | N |
| 621 | n/a         | EncodedLegSecurityDescLen | N |
| 622 | n/a         | EncodedLegSecurityDesc    | N |
| 623 | 1           | LegRatioQty               | N |
| 624 | 2           | LegSide                   | N |
| 564 | n/a         | LegPositionEffect         | N |

Provide if the PositionEffect for the leg is different from that specified for the overall multileg security

| 565 | n/a | LegCoveredOrUncovered | N |
| --- | --- | --------------------- | - |

Provide if the CoveredOrUncovered for the leg is different from that specified for the overall multileg security.

# Component block &#x3C;NestedParties>

| 539 | n/a | NoNestedPartyIDs    | N |
| --- | --- | ------------------- | - |
| 524 | n/a | NestedPartyID       | N |
| 525 | n/a | NestedPartyIDSource | N |
| 538 | n/a | NestedPartyRole     | N |
| 545 | n/a | NestedPartySubID    | N |

# End &#x3C;/NestedParties>

| 654 | n/a | LegRefID | N |
| --- | --- | -------- | - |
| 566 | n/a | LegPrice |   |

Provide only if a price is required for a specific leg. Used for anchoring the overall multileg security price to a specific leg price.

| 637 | n/a | LegLastPx |   |
| --- | --- | --------- | - |

Used to report the execution price assigned to the leg of the multileg instrument

# 587 LegSettlmntTyp

# 588 LegFutSettDate

Required when SettlmntTyp = 6 (Future) or SettlmntTyp = 8 (Sellers Option)

# End &#x3C;/Instrument Leg>

# Component block &#x3C;OrderQtyData>

| 38  | 15  | OrderQty     | N |
| --- | --- | ------------ | - |
| 152 | n/a | CashOrderQty | N |

# End &#x3C;/OrderQtyData>

| 40 | 2 | OrdType | N |
| -- | - | ------- | - |

April 30, 2003

June 18, 2003

59 FIX 4.4 with Errata 20030618- Volume 7


---

FIX 4.4 with Errata 20030618 - Volume 7

# 44

|         |     | Price               | N     | Required if specified on the order                                                                                                            |
| ------- | --- | ------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 99      | n/a | StopPx              | N     | Required if specified on the order                                                                                                            |
| 388     | n/a | DiscretionInst      | N     | Code to identify the price a DiscretionOffset is related to and should be mathematically added to. Required if DiscretionOffset is specified. |
| 389     | n/a | DiscretionOffset    | N     | Amount (signed) added to the “related to” price specified via DiscretionInst.                                                                 |
| ~~15~~  |     | ~~Currency~~        | ~~N~~ |                                                                                                                                               |
| ~~376~~ |     | ~~ComplianceID~~    | ~~N~~ |                                                                                                                                               |
| ~~377~~ |     | ~~SolicitedFlag~~   | ~~N~~ |                                                                                                                                               |
| 59      | 0   | TimeInForce         | N     | Absence of this field indicates Day order                                                                                                     |
| 168     | n/a | EffectiveTime       | N     | Time specified on the order at which the order should be considered valid                                                                     |
| 432     | n/a | ExpireDate          | N     | Conditionally required if TimeInForce = GTD and ExpireTime is not specified.                                                                  |
| 126     | n/a | ExpireTime          | N     | Conditionally required if TimeInForce = GTD and ExpireDate is not specified.                                                                  |
| 18      | n/a | ExecInst            | N     | Can contain multiple instructions, space delimited.                                                                                           |
| 528     | n/a | OrderCapacity       | N     |                                                                                                                                               |
| 529     | n/a | OrderRestrictions   | N     |                                                                                                                                               |
| 582     | 4   | CustOrderCapacity   | N     |                                                                                                                                               |
| 32      | 5   | LastQty             | N     |                                                                                                                                               |
| 31      | -12 | LastPx              | N     |                                                                                                                                               |
| ~~30~~  |     | ~~LastMkt~~         | ~~N~~ |                                                                                                                                               |
| 336     | n/a | TradingSessionID    | N     |                                                                                                                                               |
| 625     | n/a | TradingSessionSubID | N     | Used for time bracket codes for floor trades in the futures markets.                                                                          |
| 151     | 10  | LeavesQty           | Y     |                                                                                                                                               |
| 14      | 5   | CumQty              | Y     |                                                                                                                                               |
| 6       | n/a | AvgPx               | Y     |                                                                                                                                               |
| 424     | n/a | DayOrderQty         | N     | For GT orders on days following the day of the first trade.                                                                                   |
| 425     | n/a | DayCumQty           | N     | For GT orders on days following the day of the first trade.                                                                                   |
| 426     | n/a | DayAvgPx            | N     | For GT orders on days following the day of the first trade.                                                                                   |


April 30, 2003 June 18, 2003

---

# Trade Information

| 75                 | 20010509          | TradeDate             | N                                                                                                                              | Used when reporting other than current day trades. For futures markets, used to report current trade date as opposed to current calendar date at time of execution. |
| ------------------ | ----------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 60                 | 20010509-09:23:05 | TransactTime          | N                                                                                                                              | Time the transaction represented by this ExecutionReport occurred                                                                                                   |
| ~~118~~            | n/a               | ~~NetMoney~~          | ~~N~~                                                                                                                          |                                                                                                                                                                     |
| 21                 | 3                 | HandlInst             | N                                                                                                                              |                                                                                                                                                                     |
| 110                | n/a               | MinQty                | N                                                                                                                              |                                                                                                                                                                     |
| 111                | n/a               | MaxFloor              | N                                                                                                                              |                                                                                                                                                                     |
| 77                 | n/a               | PositionEffect        | N                                                                                                                              |                                                                                                                                                                     |
| 210                | n/a               | MaxShow               |                                                                                                                                |                                                                                                                                                                     |
| 58                 | n/a               | Text                  |                                                                                                                                |                                                                                                                                                                     |
| 354                | n/a               | EncodedTextLen        | Must be set if EncodedText field is specified and must immediately precede it.                                                 |                                                                                                                                                                     |
| 355                | n/a               | EncodedText           | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |                                                                                                                                                                     |
| 442                | n/a               | MultiLegReportingType | N                                                                                                                              | Default is a single security if not specified.                                                                                                                      |
| Standard Trailer Y |                   |                       |                                                                                                                                |                                                                                                                                                                     |

~~April 30, 2003~~ June 18, 2003

61 FIX 4.4 with Errata 20030618- Volume 7


---

PRODUCT: EQUITIES


# Step-Outs and Give-Ups

The new order messages allow a single clearing broker to be identified through use of the Parties component block with PartyRole = 4, Clearing Firm (in the event that the order is to be stepped out to multiple clearing brokers, the NestedParties2 component block in the NoAllocs group should be used, with each entry in the NoAllocs group denoting the quantity to be given up or stepped out to each broker).

The executing broker can optionally send copies of the order executions through to the clearing broker(s) real time using execution report messages. This flow is clearly not relevant in cases where communication to the clearing broker is managed through a central clearing house or similar organisation (e.g. as in the futures markets).

The investment manager provides booking instructions to both the executing and clearing brokers. Where the executing broker does not need to know the details of the underlying funds, a ‘ready to book’ allocation instruction can be used to tell the executing broker to book the order(s) out and settle against the clearing broker(s). The allocation details themselves are communicated from the investment manager to the clearing broker(s) using an allocation instruction message of type ‘Preliminary’ or ‘Calculated’. This message contains a reference to the Executing Broker in the NestedParties2 field in the NoOrders repeating group (PartyRole = 1, Executing Firm).

| Investment manager                                 | 4 (optional)                                                                                                                                                                      |                                                   |   |   |                  |              |                                               |
| -------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------- | - | - | ---------------- | ------------ | --------------------------------------------- |
| New order message:                                 | 3 (optional)                                                                                                                                                                      | Allocation Instruction message, status 'New' type |   |   |                  |              |                                               |
| Clearing broker                                    | Allocation Instruction message, status 'New' type 'Preliminary' or 'Calculated'. Executing broker(s) identified in NestedParties2 component block in the NoOrders repeating group |                                                   |   |   |                  |              |                                               |
| Execution reports sent back to investment manager. | Ready to book                                                                                                                                                                     |                                                   |   |   |                  |              |                                               |
|                                                    |                                                                                                                                                                                   |                                                   |   |   | Executing broker | 2 (optional) | Dropcopy execution reports to clearing broker |

This flow also supports the scenario where the investment manager has a block order which is then sent out (in parts) to a number of executing brokers, all to be settled by the same clearing broker. In this case, each executing broker receives a 'ready to book' allocation from the investment manager for their order(s) and the clearing broker receives a single allocation message for the entire block. This latter message will reference the client order ids on each order (which can be used to match up to the execution reports from the executing brokers) and the executing broker id.

~~April 30, 2003~~June 18, 2003                     62       FIX 4.4 with Errata 20030618- Volume 7



---
CFDs

# CFD with cash equity hedge executed by same broker as writing the CFD

| Investment manager             | Booking Type                                 |
| ------------------------------ | -------------------------------------------- |
| New order message with =1(CFD) | Execution reports back to investment manager |
| Executing broker               |                                              |

The BookingType field is used on the new order messages to transmit the notification that the order is for a CFD. This information is required at the time of execution as a) the broker may need to invoke separate credit or compliance checks (e.g. different RTL) and b) the broker will need to know to execute a principal cash hedge. Note the example here could be extended to cover any OTC derivative product where one or more of its cashflows is derived from a cash equity position.

# CFD with cash equity hedge executed by different broker from that writing the CFD

Here the clearing broker is writing the CFD and the executing broker is simply executing a cash equity hedge for (and settling with) the clearing broker. The allocation instruction from the investment manager to the clearing broker contains the BookingType field to provide notification that the order is to be booked out as a CFD. BookingType can also optionally be provided on the new order message to the executing broker.

~~April 30, 2003~~June 18, 2003               63        FIX 4.4 with Errata 20030618- Volume 7
---
Investment manager
# 4 (optional)

Allocation Instruction message, status 'New' type 'Preliminary'

| Clearing broker       | 3 (optional)                                       | or Calculated                                                                                     |
| --------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| referenced in Parties | Allocation Instruction message, status 'New' type  | Executing broker(s) identified in Nested Parties2 component block in the NoOrders repeating group |
| BookingType (CFD)     | =1 used to identify this as being settled as a CFD |                                                                                                   |

Executing broker

# 2 (optional)

Dropcopy execution reports to clearing broker

~~April 30, 2003~~ June 18, 2003

64 FIX 4.4 with Errata 20030618- Volume 7


---
Commission Sharing Arrangements

# Soft Dollars

Soft dollar programmes are arrangements whereby a proportion of commission on certain trades is not retained by the broker, but set aside for the payment of certain eligible services for the buy side firm sending the orders. FIX supports the handling of such business in two ways:

- Use of the ProcessCode field on the new order messages (new order single and new order list). Takes value '1=soft dollar' for soft dollar trades.
- Use of the ProcessCode field on the FIX allocation instruction or allocation report message. Takes value '1=soft dollar' for soft dollar trades.

The issue with the first approach is that the ProcessCode flag is applied to an order and therefore must be assumed to be associated with every allocated trade belonging to that order which may not necessarily be the case. For this reason, use of ProcessCode on new order messages is not recommended unless the order is pre-trade allocated to a single sub account.

The second approach is recommended as a) it logically forms part of the post trade allocation process, b) existing alternative allocation mechanisms such as Global OASYS block ETC, OASYS Direct and manual (fax etc.) operate in this way.

# Directed Brokerage

Directed brokerage (commission recapture) programmes are arrangements whereby a proportion of commission on certain trades is not retained by the broker, but set aside to be paid ultimately to the underlying funds on whose behalf the trades were executed; this may or may not involve an intermediary (e.g. Frank Russell, State Street, Lynch Jones Ryan) who collects payments from the brokers and manages the payment to the end funds.

As with soft dollars, the ProcessCode field (value '6=plan sponsor') is used. In addition, the identity of the scheme administrator must also be identified. Use of the post-trade allocation instruction message is recommended over use of ProcessCode on the new order messages for the same reasons as given for soft dollars above. The NestedParties component block in the NoAllocs repeating group in the allocation instruction message (for post-trade allocation) or new order message (for pre-trade allocation) should be used for identifying the scheme administrator.

The confirmation message contains an optional field SharedCommission which can be used to communicate the amount of commission actually being split out to the intermediary.

~~April 30, 2003~~ June 18, 2003 65 FIX 4.4 with Errata 20030618- Volume 7


---

Multi-Day Average Pricing

# Introduction

Multi-day average pricing ("warehousing") involves the sellside working a client equity order over a number of days in a similar way to a “good-till” (GT) order, but with the crucial difference that the entire buyside executed quantity is not booked for settlement until the last day of the warehouse period. Given that the sellside will still have to settle its market-side executions, this will involve the funding of buys (sellside receives from the market), and borrowing stock or failing to deliver on sells (sellside delivers to the market). Note that warehousing is not permitted in certain markets.

The flows outlined below and supported in FIX 4.4 are subject to the following constraints:

- Only equities will be warehoused.
- Multi-leg instruments will not be warehoused.
- No special functionality will be provided to cover corporate actions occurring during a warehouse period.
- Only GT orders will carry warehousing instructions on the order message itself (wrong – this covers day orders as well). If the sellside decide to warehouse a day order they will use the FIX allocation message.
- Sellside firms will be responsible for deciding whether or not to accept a warehouse request.

# Flow Summary

The following four flows are supported:

|                                 | Day orders                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Pre-trade notification                                                                                             | Use 589 DayBookingInst (a new value '2 – accumulate' has been added for this purpose). This is used to signify that the day order should be warehoused in full at the end of the day. |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Post-trade notification         | If the entire order is to be warehoused, use a 'warehouse' allocation instruction message (an Allocation Instruction with AllocTransType = 7 – warehouse) for the portion to be warehoused. If only part of the order is to be warehoused, use a 'warehouse' allocation instruction message for the warehoused portion and book and allocate the rest using a standard allocation instruction message.                                                                                                                                       |                                                                                                                    |                                                                                                                                                                                       |
| End of day warehouse recap      | At the end of every day where all or part of an order or orders has been warehoused, use an Allocation Report to communicate details of the warehoused portion of the order(s). This message has AllocReportType 5 = Warehouse recap and will communicate the quantity and average price of the warehoused portion of the order(s). For other details relating to the order (e.g. quantity executed that day, quantity remaining at the beginning of that day, the running average price), a 'done for day' execution report should be used. |                                                                                                                    |                                                                                                                                                                                       |
| Warehouse rejection (pre-trade) | Reject the warehouse allocation message with an allocation ack with 87 AllocStatus '1 – rejected' and 88 AllocRejCode - '13 Warehouse request rejected'. The order will then remain in an unbooked state until it is either booked out manually or a new allocation message is.                                                                                                                                                                                                                                                              |                                                                                                                    |                                                                                                                                                                                       |
| GT orders                       | Pre-trade notification                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Use 427 GTBookingInst, using value '1 – accumulate until filled/expires' or '2 – accumulate until told otherwise'. |                                                                                                                                                                                       |
| Post-trade notification         |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |                                                                                                                    | As for Day orders.                                                                                                                                                                    |
| End of day warehouse recap      |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |                                                                                                                    | As for Day orders.                                                                                                                                                                    |
| Warehouse rejection (pre-trade) |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |                                                                                                                    | As for Day orders.                                                                                                                                                                    |


~~April 30, 2003~~June 18, 2003                      66     FIX 4.4 with Errata 20030618- Volume 7

---

# Warehouse rejection (post-trade)

For all of these flows, either full or partial warehousing is supported (the latter meaning that only part of an order is warehoused, with the balance booked out as normal).

# Example Warehouse Flows

These diagrams show a simplified version of the FIX warehousing flows.

# Good Till Order – Warehouse Until Filled Using Pre-Trade Booking Instruction

# Day 1 – entire part-filled quantity is warehoused

| BuySide                                  | SellSide                                                                                                                                         |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| New order single GTBookingInst = 1       | 1. Buyside sends new GT order with instruction to warehouse any part-filled quantity until the order fills or expires (i.e. GTBookingInst is 1). |
| Execution reports (new\...partial fills) | 2. Sellside accepts the order, then sends 1 or more partial fill execution reports.                                                              |
| Execution report (done for day)          | 3. Sellside sends a “done-for-the-day” (DFD) execution report when execution completes for the day.                                              |
| Allocation report (AllocReportType = 5)  | 4. Sellside sends a warehouse recap allocation report.                                                                                           |

Note a 'warehouse instruction' allocation instruction message from the buyside is not required at this point due to the use of GTBookingInst when placing the order.

# Day 2 – further executions; entire part-filled quantity is again warehoused

| BuySide                                  | SellSide                                                                                            |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Execution reports (new\...partial fills) | 2. Sellside sends 1 or more partial fill execution reports.                                         |
| Execution report (done for day)          | 3. Sellside sends a “done-for-the-day” (DFD) execution report when execution completes for the day. |
| Allocation report (AllocReportType = 5)  | 4. Sellside sends a warehouse recap allocation report.                                              |

Note a 'warehouse instruction' allocation instruction message from the buyside is not required at this point due to the use of GTBookingInst when placing the order.

# Day 3 – further executions; order is now filled and booked out

| BuySide | SellSide |
| ------- | -------- |
|         |          |

~~April 30, 2003~~June 18, 2003 67 FIX 4.4 with Errata 20030618- Volume 7

---

Execution reports

# 2. Sellside sends 0 or more partial fill execution reports and a final fill.

# Allocation instruction

| AllocTransType | 'new'                                                                                |
| -------------- | ------------------------------------------------------------------------------------ |
| AllocType      | either 'Buyside preliminary' (if without MiscFees) or 'Buyside calculated' (if with) |

# Allocation Instruction ACK

| AllocStatus | 'received'                                               |
| ----------- | -------------------------------------------------------- |
| Details     | Sellside acknowledges receipt of the allocation details. |

# Allocation Instruction ACK

| AllocStatus | 'processed'                                                                                                                       |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Details     | Sellside processes and acknowledges allocation details. Confirmation messaging and processing will then take place for the order. |

# Allocation report

| AllocReportType | 5                                                   |
| --------------- | --------------------------------------------------- |
| Details         | Sellside sends a warehouse recap allocation report. |

~~April 30, 2003~~ June 18, 2003

68 FIX 4.4 with Errata 20030618- Volume 7



---

Good Till Order – Partial Warehousing - Day 1

# Day 1 – part of the part-filled quantity is warehoused

| BuySide                                                                                                              | SellSide                                                                                                                                                                         |
| -------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1. Buyside sends new GT order with instruction to warehouse any part-filled quantity (i.e. GTBookingInst is 1 or 2). | Should clarify that use of GTBookingInst implies warehouse instructions not required. Should add an example of 'normal' GT (i.e. no GTBookingInst), i.e. post trade instructions |
| Execution reports (new\...partial fills)                                                                             | 2. Sellside accepts the order, then sends 1 or more partial fill execution reports.                                                                                              |
| Execution report (done for day)                                                                                      | 3. Sellside sends a “done-for-the-day” (DFD) execution report when execution completes for the day.                                                                              |
| Allocation instruction for non-warehoused portion of the order                                                       | 4. (a) Buyside decides to book out a proportion of the part-filled order                                                                                                         |
| AllocTransType 'new'                                                                                                 | AllocType either 'Buyside preliminary' (if without MiscFees) or 'Buyside calculated' (if with)                                                                                   |
| Allocation Instruction ACK (AllocStatus 'received')                                                                  | 5. (a) Sellside acknowledges receipt of the allocation details.                                                                                                                  |
| Allocation Instruction ACK (AllocStatus 'processed')                                                                 | 6. (a) Sellside processes and acknowledges allocation details. Confirmation messaging and processing will then take place for the order.                                         |
| Allocation instruction for buyside warehouse notification                                                            | 4. (b) Buyside warehouses the rest of the order.                                                                                                                                 |
| AllocTransType 'new'                                                                                                 | AllocType 'warehouse'                                                                                                                                                            |
| Allocation Instruction ACK for warehouse instruction (AllocStatus 'received')                                        | 5. (b) Sellside acknowledges receipt of the warehouse allocation instruction.                                                                                                    |
| Allocation Instruction ACK for warehouse instruction (AllocStatus 'processed')                                       | 6. (b) Sellside processes and acknowledges allocation details.                                                                                                                   |
| Allocation report (AllocReportType = 5)                                                                              | 4. Sellside sends a warehouse recap allocation report.                                                                                                                           |
| Allocation report (AllocReportType = 5)                                                                              | 7. Sellside sends a warehouse recap allocation report.                                                                                                                           |

~~April 30, 2003~~June 18, 2003 69 FIX 4.4 with Errata 20030618- Volume 7



---

June 18, 2003                70        FIX 4.4 with Errata 20030618- Volume 7


Subsequent days' flows are as in 'Warehouse till filled' scenario above.

Note the flow is similar when the entire order is warehoused – in this case, messages 4a, 5a and 6a are missing.

# Day Order – Part- or Fully Warehoused

In this case, on day 1 of the order the buyside decides to warehouse a trade after the DFD message has been sent by the sellside. The entire part-filled quantity may be warehoused or a proportion may be allocated to client accounts. The flow is exactly the same as for GT orders as above, apart from the original new order not having any GTBookingInst or DayBookingInst.


~~April 30, 2003~~

---

Decision Flows


# WAREHOUSE REQUEST RECEIVED ON ORDER

| Nevy order.                          | Send warehouse               | Continue working                     |
| ------------------------------------ | ---------------------------- | ------------------------------------ |
| Request                              | Accept                       | Book pant-filled qy                  |
| Warehouse                            | Order fully filled           | Warehouse                            |
| recan                                | order next day               | time limit reached?                  |
| allocation received?                 | Buyside                      | Sellside                             |
| Intortibuyside via allocation report | Normal allocation processing | Do not prompt for new replaced order |
| Sellside                             | Both parties                 |                                      |

April 30, 2003 - June 18, 2003

71 FIX 4.4 with Errata 20030618 - Volume 7



---
WAREHOUSE REQUEST RECEIVED POST-EXECUTION

| Send post-   | FXECution    | Approved by        | Wvarenouse        | Book part-filled qy | Senc wvarehousp | Continue vvorking | WVarehouse | Order fully filled |
| ------------ | ------------ | ------------------ | ----------------- | ------------------- | --------------- | ----------------- | ---------- | ------------------ |
| wvarehouse   | Ilside?      | ertire part-filled | Yest              | vvarenouse          | recap           | orce FexC         | time limit | alocation          |
| request      | Buyside      | Sellside           | Sellside          | Sellside            |                 |                   |            |                    |
| Send partial | Send confirm | shoving allocated  | Normal allocation | processing          |                 | Sellside          | Sellside   | Both parties       |

April 30, 2003 June 18, 2003

72

FIX 4.4 with Errata 20030618- Volume 7


---

PRODUCT: FIXED INCOME (FI)


# Introduction

This section and the enhancements to the protocol has been the result of the joint effort between the BMA and FPL’s Global Fixed Income Committee (formerly Fixed Income Working Group). This Appendix summarizes how FIX messages can be used to support FI trading activities – offerings, negotiated trade/bid or offer request, my bid/offer order, order initiation and execution, and allocation – for the following fixed income asset classes:

- US Treasury Bonds
- US Corporate Bonds
- Municipal Securities
- Agency Securities
- To-Be-Announced (TBA) Mortgage Backed Securities
- Euro Sovereign Bonds
- Euro Corporate Bonds
- US and European Commercial Paper
- Repurchase Agreements (Repos) and Related Securities Lending Activities

The usage models are described as between two counterparties, an Initiator and a Respondent – see the Glossary in Volume 1 for definitions of these roles.

Note that this documentation should be used as a starting point and serves merely to provide guidance in the reader’s FIX for FI implementation.

# Message Dialog

In FI the trading dialog typically starts in one of two ways: 1) one party sending out offerings to their clients and their clients responding to the offerings, or 2) an interested party initiating an inquiry or a bid/offer request. Once the dialog is initiated a trade could be consummated. The allocation of the trade could be conducted “pre-trade” or “post-trade” directly with the trading counterparty. Third party post-trade reporting using FIX messages is also illustrated.

The diagrams below attempts to illustrate the various dialogs that can happen to facilitate an FI trade and the message flows to use depending on the initiation point of the dialog. Note that the diagrams will also show, via the green colored circles, the next step in the message dialog and do not show error conditions (i.e. one party receiving an unknown CUSIP) that can occur during the dialog.


April 30, 2003June 18, 2003                          73       FIX 4.4 with Errata 20030618- Volume 7

---


# Indication of Interest (Offerings)

Offerings are communicated using the Indication Of Interest (IOI) message type. The recipient of the offerings can elect to ignore the IOI messages or respond to specific IOI messages via the use the Quote Response message type. Offerings can be sent by the Respondent to an Initiator on a continuous basis as long as the Initiator wants to receive them. The Initiator has the option to ignore the messages sent by not responding or to respond to an offering of interest by sending a Quote Response message back to the Respondent to either “hit” or “lift” the offering. Figure 1 below illustrates the message flow. The Respondent will pickup on the message dialog flow at “B” in the Negotiated Trade diagram (see next section).

# Figure 1: Indication of Interest/Offerings

| Initiator              | Respondent                   |
| ---------------------- | ---------------------------- |
| Indication of Interest |                              |
| IOIID \<New>           |                              |
| IOIQualifier           | "Ready to trade"             |
| An IOI may             | Quote Response               |
| be ignored:            | QuoteRespID \<new>           |
|                        | IOIld \<respondent's>        |
|                        | CIOrdID \<new>               |
|                        | QuoteResponseType "Hit/Lift" |
|                        | Negotiation B                |

Click here to go to “B”


April 30, 2003June 18, 2003 74 FIX 4.4 with Errata 20030618- Volume 7


---
Negotiated Trade /Inquiry/Bid or Offer Request

A negotiated trade dialog can be initiated not only via the offerings or IOIs as indicated above, but also via a “my bid or offer”, an inquiry/bid or offer request, both using a Quote Request message type. The difference between a “my bid/offer” message and an inquiry/bid or offer request message is that in a “my bid/offer” the Initiator sends a Quote Request message with a “my bid/offer” price set for the security in question. The Respondent would respond with a Quote message. The rest of the dialog would follow the dialog described below and it is illustrated in the “My bid/offer” diagram below.

An inquiry, bid or offer request/wanted begins with a Quote Request from the Initiator. It is possible for the Respondent to send an unsolicited Quote message to their counterparty to initiate the negotiated trade dialog, however, this arrangement should be bilaterally agreed upon by the counterparties involved.

In the negotiation dialog, the Initiator would send a Quote Request message to the Respondent to inquire about a certain security, inquire for a list of securities that meet certain stipulations and parameters, request a bid or offer or request a quote on a certain security. Should the Respondent choose not to provide a quote a Quote Request Reject can be sent with the appropriate reject reason code set. At this point the current dialog would terminate. Alternatively the Respondent can respond to the Quote Request with a Quote message. The Quote message would provide the pricing levels for the securities requested by the Initiator.

The Initiator will respond to the Quote from the Respondent via the use of the Quote Response message type. The Quote Response message type can be used to end the dialog, “hit/lift” the Quote, or counter the Quote. A “hit/lift” response from the Initiator indicates to the Respondent that the Initiator agrees with the price level and the quantity, and want to complete a trade. On the other hand, if the Initiator responded with a counter offer then the negotiation can continue until one party decides to terminate the dialog or a trade is completed.

To a “hit/lift” or counter message from the Initiator, the Respondent can respond with another “counter” message using the Quote message type, end the negotiation dialog with a Quote Status Report, or give the Initiator an Execution Report message indicating that the trade has been completed. This Execution Report message may or may not include calculations for information such as accrued interest, gross trade amount, etc.

Lastly, if the Initiator deems that there are discrepancies in the Execution Report message received from the Respondent, the Initiator may use the Don’t Know Trade (a.k.a. DK Trade) message type to “reject” the trade information. Resolving the error or discrepancies would be done manually and is currently out of scope for the suggested use of the protocol.

The diagram, Negotiated Trade, on the following page illustrates this flow with some additional details of what values within certain fields can be used.

| Figure 2: My Bid/Offer |            |
| ---------------------- | ---------- |
| Initiator              | Respondent |
| Quote Request          |            |
| QuoteRequestID         | \<new>     |
| CIOrdID                | \<new>     |
| QuoteType              | Tradable   |
| OrderType              | Limit      |
| Negotia-               |            |
| tion B                 |            |

April 30, 2003June 18, 2003 75 FIX 4.4 with Errata 20030618- Volume 7


---

FIX 4.4 with Errata 20030618- Volume 7

# Figure 3: Negotiated Trade/Bid or Offer Request

| Initiator                                             | Respondent                            |
| ----------------------------------------------------- | ------------------------------------- |
| Bid or Offer Request;, Bids Wanted, Reverse Inquiries | Quote Request                         |
| QuoteReqID \<new>                                     | QuoteType                             |
|                                                       | "Indicative" or Tradable              |
|                                                       | Quote Request Reject                  |
| Istopl                                                | QuoteReqID \<initiator's>             |
|                                                       | QuoteReqRejReason                     |
|                                                       | "No match for inquiry"                |
|                                                       | "No market for instrument"            |
|                                                       | "No inventory" or "Pass"              |
|                                                       | {OR}                                  |
| Quote                                                 | QuoteReqID \<initiator's>             |
|                                                       | QuotelD \<new>                        |
|                                                       | QuoteType                             |
| Reject                                                | Quote Response                        |
| A                                                     | QuoteRespID \<new>                    |
|                                                       | QuotelD \<respondent's>               |
|                                                       | QuoteRespType                         |
|                                                       | "Expired", "Cover", "Done Away", Pass |
|                                                       | {OR}                                  |
| Accept                                                | Quote Response                        |
| Bid/Offer                                             | QuoteRespID \<new>                    |
|                                                       | QuotelD \<respondent's>               |
|                                                       | CIOrdID \<new> or \<original>         |
|                                                       | QuoteRespType "Hit/Lift"              |
|                                                       | {OR}                                  |
| Counter                                               | Quote Response                        |
|                                                       | QuoteRespID \<new>                    |
|                                                       | QuotelD \<respondent's>               |
|                                                       | CIOrdID \<new> or \<original>         |
|                                                       | QuoteRespType "Counter"               |
| Quote                                                 | Counter                               |
| A                                                     | QuoteRespID \<initiator's>            |
|                                                       | QuotelD \<new>                        |
|                                                       | QuoteType "Counter"                   |
|                                                       | {OR}                                  |
|                                                       | Quote Status Report                   |
| Reject                                                | QuoteRespID \<initiator's>            |
| IStop                                                 | QuotelD \<respondent's orig>          |
|                                                       | QuoteStatus                           |
|                                                       | "Expired" or "Pass"                   |
|                                                       | {OR}                                  |
| Initiator may respond with                            | Execution Report                      |
| DK giving a DKReason of                               | QuoteRespID \<initiators orig?>       |
| "Calculation Difference"                              | OrderID \<new>                        |
| Discrepancies are                                     | CIOrdID \<initiator's>                |
| resolved out-of-band.                                 | ExecID \<new>                         |
|                                                       | OrdStatus "Filled"                    |
|                                                       | include accrued and net money         |
|                                                       | Allocations                           |

Click here to go to “Allocations”


April 30, 2003June 18, 2003 76

---
# FIX 4.4 with Errata 20030618 - Volume 7

April 30, 2003

June 18, 2003

77
---

# Out-of-Band Negotiated Order

A trade that is negotiated “out-of-band” is a trade negotiated through other means such as verbally on the phone or via an alternate trading system platform. In this dialog it is assumed that the Respondent is able to send the completed trade information electronically using the FIX protocol. The initiation of the order placed by the Initiator could be through the New Order message type or through other means (i.e. verbally or via an alternate trading system platform) agreed upon between the counterparties.

When an order is placed by the Initiator using the New Order message type the Respondent could either accept the order or reject the other using the Execution Report message type. If the order is rejected the dialog ends. If the order is accepted the negotiation can begin out-of-band or “offline”. When the negotiation is completed and the terms of the trade are agreed upon the Respondent would send the Initiator an Execution Report message to confirm that the trade has been completed. The terms of the trade are reiterated in the Execution Report message.

In the event that the Initiator deems that there are discrepancies in the Execution Report message received from the Respondent, the Initiator may use the Don’t Know Trade (a.k.a. DK Trade) message type to “reject” the trade information. Resolving the error or discrepancies would be done manually and is currently out of scope for the suggested use of the protocol.

The diagram on the following page illustrates this dialog.

April 30, 2003 June 18, 2003 78 FIX 4.4 with Errata 20030618- Volume 7


---

# Figure 4: Out-of-Band Negotiated Trade

| Initiator                        | Respondent                     |
| -------------------------------- | ------------------------------ |
| New Order Single or Multileg     | CIOrdID \<Newz                 |
| The order may also               | OrdType                        |
| 7 "Market" \[Auction]            | 2 "Limit" \[My Bid/Offer]      |
| Allocation\[1]                   | Allocation\[n]                 |
| Filled" Execution                | Report.                        |
| Execution Report                 | OrderID \<new>                 |
| CIOrdID \<initiators>            | ExecID \<new>                  |
| Istop                            | OrdStatus 8 "Rejected"         |
| OrdRejReason                     | as appropriate or              |
| Order is dead but may            | "incorrect quantity'           |
| be reissued under a              | "incorrect allocated quantity" |
| differen CIOrdID in              | "unknown account(s)"           |
| another New Order                | {OR}                           |
| message. No linkage is           | Execution Report               |
| OrderID \<Newz                   | CIOrdID \<initiators>          |
| ExecID \<new>                    | OrdStatus 0 "New"              |
| Negotiate Out-of-Band            |                                |
| Initiator may respond with       | DK giving a DKReason of        |
| "calculation difference"         | Execution Report               |
| Discrepancies are resolved       | OrderID \<same>                |
| manually:                        | CIOrdID \<initiators>          |
| ExecID \<new>                    | OrdStatus 2 "Filled"           |
| reporting accrued and net moneyl |                                |
| Alloca-                          | tion                           |

Click here to go to “Allocations”

April 30, 2003June 18, 2003

79 FIX 4.4 with Errata 20030618- Volume 7


---
# Allocation Instructions

Allocation instructions can be communicated by the Initiator via three different options:

1. Pre-allocated Order – in this option the Initiator would communicate the allocation instructions within the New Order message when the order is placed with the Respondent.
2. Pre-trade allocation – in this option the Initiator would communicate the allocation instructions to the Respondent in a separate message using the Allocation message. The Allocation message is sent after the order is placed with the Respondent but before the trade is completed by the Respondent.
3. Post-trade allocation – in this option the Initiator would communicate the allocation instructions to the Respondent in a separate message using the Allocation message after the trade has been completed by the Respondent.

For the Initiator options 2 and 3 represents the same message flow. The main difference is when the Allocation message is sent – in option 2 it is sent prior to the trade being completed and in option 3 it is sent after the trade has been completed.

Once the trade is completed and the Respondent is ready to act on the allocation instructions, assuming no errors in the allocation instructions from the Initiator, the message flow for the Respondent is the same regardless of which option is used by the Initiator to communicate those allocation instructions.

Note that these options work for Fixed Income because of FI’s simple trading practices – there is no concept of “done for day”, one set of allocations is applied to a single order usually filled in a single execution.

In the Pre-allocated Order scenario the Initiator would send a New Order message that includes the allocation information needed by the Respondent to allocate the trade once the trade is completed. Note, however, that if even one account cannot be identified, or the quantity of one allocation instance does not meet minimum quantity/minimum increment rules for the instrument, or the sum of allocated quantities does not equal the block trade quantity, the entire request must be rejected. If erroneous allocations are sent via the New Order message, the entire New Order message is rejected using the Execution Report message with the appropriate reject code.

If the pre-allocated Order is accepted and filled, the Respondent communicates that information to the Initiator using the Execution Report message type, setting all the appropriate status values per standard protocol usage.

At this point in the message flow the Respondent would begin to allocate the trade according to the allocation instructions already provided in the New Order message and communicating that information back to the Respondent according to the message flow shown in Figure 5, starting with the AllocationReport.

April 30, 2003 June 18, 2003 80 FIX 4.4 with Errata 20030618- Volume 7
---

FIX 4.4 with Errata 20030618- Volume 7


# Figure 5: Allocations

| Initiator                                                            | Alloca-                         | Respondent                                                                        |
| -------------------------------------------------------------------- | ------------------------------- | --------------------------------------------------------------------------------- |
| AllocationInstruction                                                | Allocation                      |                                                                                   |
| Alloc ID \<new>                                                      | OrderID \<respondent's>         | ExecID \<respondent's>                                                            |
| CIOrdID \<initiator's original>                                      | AllocType "Buyside Preliminary" | AllocTransType                                                                    |
|                                                                      | "New" or "Replace               | If Allocation details have not been communicated pre-trade via the Order message: |
| Execution                                                            | Allocation\[1]                  | Allocation\[n]                                                                    |
| stands, but remains unallocated                                      | AllocationInstructionACK        |                                                                                   |
| AllocID \<initiator's>                                               | CIOrdID \<initiator's>          | AllocStatus "Rejected"                                                            |
| AllocRejCode                                                         |                                 | "unknown account"                                                                 |
|                                                                      |                                 | "incorrect quantity"                                                              |
|                                                                      |                                 | "unknown OrderID"                                                                 |
|                                                                      |                                 | "incorrect allocated quantity"                                                    |
|                                                                      |                                 | "unknown or stale ExecID"                                                         |
|                                                                      |                                 | "mismatched data value"                                                           |
|                                                                      |                                 | "unknown CIOrdID"                                                                 |
| {OR}                                                                 | AllocationInstructionACK        |                                                                                   |
| Alloc ID \<initiator's>                                              | CIOrdID \<initiator's>          | AllocStatus "Received"                                                            |
| AllocationReport                                                     | Allocation ID \<new>            | RefAllocID \<respondent's>                                                        |
| OrderID \<respondent's original>                                     | ExecID \<respondent's original> | CIOrdID \<initiator's>                                                            |
| AllocType "Sellside Calculated Using Preliminary"                    | AllocTransType                  |                                                                                   |
| "New" or "Replace                                                    | Allocation\[1]                  | Allocation\[n]                                                                    |
| If Order was pre-allocated, once trade is completed it is allocated. | AllocationReportACK             | Resolve discrepancies offline.                                                    |
| Alloc ID \<respondent's>                                             | CIOrdID \<initiator's>          | AllocStatus "Rejected"                                                            |
| AllocRejCode                                                         |                                 | "calculation difference"                                                          |
| {OR}                                                                 | AllocationReportACK             |                                                                                   |
| Alloc ID \<respondent's>                                             | CIOrdID \<initiator's>          | AllocStatus "Accepted"                                                            |

# Confirmation

Click here to go to “Confirmation”

April 30, 2003 June 18, 2003

81



---
# FIX 4.4 with Errata 20030618 - Volume 7

April 30, 2003

June 18, 2003

82
---

# FIX 4.4 with Errata 20030618 - Volume 7

April 30, 2003 - June 18, 2003



In the Pre-trade allocation scenario the Initiator would send the allocation instructions, after placing the order but before the Execution Report message indicated that the trade is completed, to the Respondent using the AllocationInstruction message type. On the other hand, in the Post-trade allocation scenario the Initiator would send the allocation instructions to the Respondent after receiving the Execution Report message indicated that the trade is completed – again using the AllocationInstruction message type.

Before accepting the request the Respondent should determine that all accounts are known, the quantity of each allocation instance meets minimum quantity/minimum increment rules for the instrument and the sum of allocated quantities equals the block trade quantity. If any error is found the Respondent must reject the entire Allocation using the AllocationInstructionAck message with the appropriate reject reason code. In this event, whether the trade that has been completed or is pending completion, the order is still a live order. A rejection of the AllocationInstruction message does not equate to a reject of the order placed in this case. The Initiator can send a new AllocationInstruction message with the correct instructions and information to the Respondent.

If the Respondent accepts the AllocationInstruction, the message flow would continue as shown in Figure 5 with the Respondent sending the AllocationReport message to communicate the sub-account level calculations for net monies and accrued interest if appropriate. At this stage the Initiator still has the option to reject the validated/calculated allocation message due to differences in calculations of net money, gross amounts, etc., for each of the allocated sub-accounts. Alternatively the Initiator can acknowledge back to the Respondent that the validated/calculated message is accepted. Both the Initiator’s response is communicated via the use of the AllocationReportAck message type.



---

Figure 6: Confirmation and Affirmation

# Confirma-

| Initiator                                       | Confirmation                    | Respondent                       |
| ----------------------------------------------- | ------------------------------- | -------------------------------- |
| Resolve discrepancies offline                   | ConfirmationID \<newz           | AllocID \<respondent's>          |
| OrderID \<respondent's original>                | ExecID \<respondent's original> |                                  |
|                                                 | CIOrdID \<initiator's>          | IndividualAIlocID \<initiators>  |
| ConfirmationStatus "Error"                      | ConfirmationRejCode             |                                  |
|                                                 | "unknown account                | "missing settlement instructions |
|                                                 | {OR}                            |                                  |
|                                                 | ConfirmationID \<newz           | AllocID \<respondent's>          |
| OrderID \<respondent's original>                | ExecID \<respondent's original> |                                  |
|                                                 | CIOrdID \<initiators>           | IndividualAIlOcID \<initiators>  |
| ConfirmationStatus "Confirmed"                  |                                 |                                  |
| Resolve discrepancies offline.                  | ConfirmationACK                 | ConfirmationD \<respondent's>    |
| Respondent should issue corrected Confirmation. | CIOrdID \<initiator's>          | ConfirmationStatus "Rejected"    |
|                                                 | ConfirmationRejCode             | "calculation difference"         |
|                                                 | {OR}                            |                                  |
|                                                 | ConfirmationACK                 | ConfirmationID \<respondent's>   |
| CIOrdID \<initiators>                           | ConfirmationStatus Affirmed"    |                                  |

Figure 6 illustrates the message flow of the confirmation process for each of the allocated account instance (the sub-accounts in the AllocationInstruction message) the Respondent would use once the allocation calculations have been confirmed by the Initiator.

The Confirmation message is an optional message that the Respondent can use to report back, confirms or raise an exception of the booking/confirm status of each of the allocation instances in the trade. When the “confirmed” status is reported to the Initiator it indicates that that piece of the allocated trade is ready to settle. Each


April 30, 2003June 18, 2003                       84           FIX 4.4 with Errata 20030618- Volume 7

---

Confirmation message will report the details of a single “ticket”, therefore the account names, fees, net money and settlement information are reported using fields designated for single account trades.

Once the “confirmed” is received from the Respondent the Initiator has the final say by sending the ConfirmationAck message with the “affirmed” status. However, should the Initiator disagree with the Respondent’s “confirm” the Initiator can send a reject using the ConfirmationAck message with a status of “rejected” and provide a reason for rejection.

# Post Trade Reporting to a 3ʳᵈ Party or Virtual Matching Utility

Figure 7 illustrates the messages needed by the Initiator and the Respondent to send trade notices to a 3rd party or VMU for trade matching.

April 30, 2003June 18, 2003 85 FIX 4.4 with Errata 20030618- Volume 7


---

# Figure 7: Post Trade 3ʳᵈ Party or VMU Trade Reporting

| Initiator                                                                                                                                                                                                                                                                                                                                                                                                                           | 3rd Party                                                                                                                                                                       | Respondent                                                                                                                                                                                                                    |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Allocation Instruction OrderID \<respondent's> CIOrdID \<initiators> Allocation Instruction Ack AllocStatus Received Allocation Report OrderID \<respondent's> CIOrdID \<initiators> MatchStatus = Matched Allocation Report Ack AllocStatus = Accepted Confirmation OrderID \<respondent's> CIOrdID \<initiators> ConfirmStatus = Confirmed Confirmation Ack OrderID \<respondent's> CIOrdID \<initiators> AffirmStatus = Affirmed | TradeCaptureReport Or Execution Report OrderID \<respondent's> CIOrdID \<initiators> Allocation Instruction OrderID \<respondent's> CIOrdID \<initiators> MatchStatus = Matched | Allocation Instruction Ack AllocStatus = Accepted Confirmation OrderID \<respondent's> CIOrdID \<initiators> ConfirmStatus = Confirmed Confirmation Ack OrderID \<respondent's> CIOrdID \<initiators> AffirmStatus = Affirmed |

The Allocation Instruction message type is used by the Initiator to report one or more orders and block trades along with associated allocations to a 3ʳᵈ party or VMU for trade matching. The Respondent will use the Trade Capture Report, or an Execution Report depending on the 3ʳᵈ party’s requirements, message type to report trades to a 3ʳᵈ party. This notice of execution will be for block level trades.

April 30, 2003 June 18, 2003 86 FIX 4.4 with Errata 20030618- Volume 7


---

Message Usage Details

This section provides some details to the usage of specific fields within messages. These usage guidelines are a supplement to the usage already described in the main volumes of the specification. The usage guidelines discusses requirements for FI that are required by the baseline protocol or will make clarifications specific to FI usage.

# General Usage Rules

1. PriceType field must be present when the following price fields are used in any message: Price, BidPx, OfferPx, MktBidPx, MktOfferPx, MidPx.
2. AvgPx field is usually expressed as “percent of par”. Where it is not, such as in certain Confirmation scenarios, AvgParPx and LastParPx have been added for communicating the percent-of-par price that will drive settlement calculated from the negotiated price.
3. LegPriceType must be present when LegBidPx or LegOfferPx is used in the NoLegs repeating block of any message that contains this repeating block.
4. In all trade and post-trade messages where price information is being communicated, a limit or execution price is always conveyed in Price or LastPx, respectively, with PriceType set appropriately. Depending on market convention for a particular asset class other fields may be used to supplement the quote or execution price such as YieldData component block and/or SpreadOrBenchmark component block. Yield and Spread should communicate only derived information, never the negotiated price.
5. All FIX messages identified for use in FI trading except New Order Single support both single instrument trades “outrights” and trades of two instruments – one to be sold and the other to be bought as a replacement. In the US the latter are often called “swaps”, in other regions they are “switches”, and two-instrument trades involving the sale and purchase of futures contracts with different contract settlement months are called “rolls”. The NoLegs repeating block is used to identify and link the two sides of the trade. LegSwapType can be used instead of LegQty on one side of the trade to instruct the Respondent to calculate the LegQty based on the opposite leg’s quantity. To submit a new order for a swap or roll use New Order Multileg instead of New Order Single.
6. LastPxPar conditionally required in the Execution Report, Allocation, and TradeCaptureReport messages when LastPx is expressed with a PriceType other than “percent of par” (i.e. when LastPx is expressed as “discount” or “yield” PriceType then LastPxPar must be used to express the price in “percent of par” equivalent.)
7. When SettlType is not “regular” then SettlType must to be specified. SettlType “future” requires a value for SettlDate.

# Indication Of Interest

An IOI must specify price information by using either one of the set of price information fields (see General Usage Rules section). Either the IOIQty or the NoLegs repeating block is required. If the NoLegs repeating block is used, put “0” (zero) in the IOIQty field. IOIQty is required and used for offerings of single instruments. The NoLegs repeating block is used for multilegs (swaps/switches/rolls). In FI’s use there would only be two legs – a buy leg and a sell leg. ValidUntilTime is where the IOI sender can specify the “firm time” of the offering.

# Quote Request

In this message the Initiator can specify what form the quote should be in by using the QuotePriceType field. The ClOrdID field has been added to this message allowing the Initiator to assign a ClOrdID when requesting for quotes that are of QuoteType “Tradable” and OrdType of “Limit”. To submit a “my bid/offer” quote request the Initiator will need to specify QuoteType of “Tradable” and OrdType of “Limit”. Pricing information must be specified using either one of the set of price information fields (see General Usage Rules section).


April 30, 2003June 18, 2003                          87          FIX 4.4 with Errata 20030618- Volume 7

---


# ValidUntilTime

– used by the Initiator to indicate the period of time the resulting Quote must be valid for

# ExpireTime

– used by the Initiator to indicate the period of time when this quote request expires

# OrderQtyData component block

– required when QuoteType is “Tradeable”

# Quote Response

Initiator will use the QuoteRespType field to indicate what type of response this is, i.e. “hit/lift”, “counter”, etc.

IOIid is required if the Quote Response is used to respond to an IOI (offering) message, the field would contain the ID of the IOI message.

Fields required when QuoteRespType is “hit/lift” or “counter quote”: OrderQtyData component block, Side, ValidUntilTime, ClOrdID (see paragraph below), and either one of the set of price information fields (see General Usage Rules section).

In the initial use of the “hit/lift” QuoteRespType, the Initiator is required to assign a ClOrdID. This ClOrdID will be reused throughout the negotiation process, including in the “counter”, until the negotiation ends in either a fill or the negotiation dialog is terminated by either party.

In a “counter quote” to a Quote, only a limited set of data elements can change depending on the security type. Price can be expected to change, but also Instrument being quoted can change in some markets as well as Stipulations and ClearingCode within the Parties component block.

In a “counter quote” with a “my price” set, OrdType must be “Limit” and either one of the set of price information fields (see General Usage Rules section).

# Quote

Fields required when QuoteType is “counter” or “Tradeable”: OrderQtyData component block, Side, ValidUntilTime, and either one of the set of price information fields (see General Usage Rules section).

# New Order - Single

For OrdType only the following enumeration are applicable: 1 (market), 2 (limit), D (previously quoted), E (previously indicated).

For OrdType of “limit” either one of the set of price information fields (see General Usage Rules section) is required.

TradeDate is required and is set by the Initiator.

HandlInst is required by the protocol but is not a required field for FI. However, for the purposes of being compliant to the protocol the counterparties should bilaterally agree on the value to use.

# New Order - Multileg

TradeOriginationDate is used for municipal new issue market. Specifies the date in which agreement in principal between counterparties, prior to actual TradeDate.

TradeDate is required and is specified by Initiator.

For the Multileg Order, if the following fields are not applicable to all legs of the trade then the NestedParties component block associated with each leg within the NoLegs repeating block will be used: Account, AccountType, NoAllocs repeating block, SettlType, and SettlDate.

# Execution Report

This message should always use SettlType “future” with a value for SettlDate.

April 30, 2003June 18, 2003 88 FIX 4.4 with Errata 20030618- Volume 7


---

# Stipulations

Component block information must be reiterated and echo back by the Respondent if Initiator had provided information in the Stipulations component block.

For multilegs only use the NoLegs blocks of the Execution Report message for swaps/switches/rolls when OrdStatus is “new”. The partial fill or fill (OrdStatus) Execution Report for each of the legs will be reported separated and execution price for each leg is conveyed in LastPx, AvgPx and LastPxPar, if applicable.

The following fields are required when OrdStatus is “partial”, “filled” or “calculated”: PriceType, Price.

The following fields are required when ExecType is “trade” or “trade correct”: LastQty, LastPx, AvgPx, LastPxPar (when conditionally applicable).

The following fields are required when OrdStatus is “filled” or “calculated” AND if NumDaysInterest is populated and not zero: AccruedInterestRate, AccruedInterestAmt.

GrossTradeAmt and NetMoney is required when OrdStatus is “filled” or “calculated”.

NumDaysInterest is required where applicable based on security type and when OrdStatus is “filled” or “calculated”.

InterestAtMaturity is required in lieu of AccruedInterestAmt for security types that pay lump-sum at maturity.

# Allocation Instruction

PreviouslyReported, ReversalIndicator and MatchType is conditionally required when Initiator is sending the Allocation Instruction message to a 3ʳᵈ party or VMU.

This message should always use SettlType “future” with a value for SettlDate.

GrossTradeAmt – Initiators are required to send this information when sending Allocation post-trade.

For Financing Trades Use QtyType and ContractMultiplier if necessary to identify how quantities are to be expressed and specify in OrderQty the block cash amount to be allocated and in AllocQty the cash amount to be assigned to each fund.

# Allocation Report

Respondents are required to send this information when reporting the Allocation back with calculations.

NetMoney is required from Respondents when reporting the Allocation back with calculations.

NumDaysInterest, AccruedInterestAmt and AccruedInterestRate is required from Respondents when reporting the Allocation back with calculations for security types where this information can be derived or is available.

InterestAtMaturity is required in lieu of AccruedInterestAmt for security types that pay lump-sum at maturity.

AllocNetMoney is required from Respondents when reporting the Allocation back with calculations.

AllocAccruedInterestAmt is required, if the value is not zero, from Respondents when reporting the Allocation back with calculations. AllocAccruedInterestAmt should be calculated and rounded appropriately for each allocation instance. This means that the sum of AllocAccruedInterestAmt will not always match AccruedInterestAmt.

AllocInterestAtMaturity is required, if value is not zero, from Respondents when reporting the Allocation back with calculations. AllocInterestAtMaturity is required in lieu of AllocAccruedInterestAmt for security types that pay lump-sum at maturity. Similar to AccruedInterestAmt, the sum of AllocInterestAtMaturity will not always match InterestAtMaturity.

For Financing Trades use the same quantity rules as given for the Allocation Instruction above.

# Trade Capture Report

This message should always use SettlType “future” with a value for SettlDate.

Parties component block is required.


April 30, 2003June 18, 2003                        89         FIX 4.4 with Errata 20030618- Volume 7

---

# GrossTradeAmt and NetMoney are required.

NumDaysInterest is required where information is applicable.

AccruedInterestRate is required if NumDaysInterest is used and is not zero.

AccruedInterestAmt is required for security types that trade with accrued interest.

InterestAtMaturity is required in lieu of AccruedInterestAmt for security types that pay lump-sum at maturity.

# Instrument component block

Symbol – use “[N/A]” when there are no applicable symbol. For corporate bonds the symbol or ticker for the company issuing the security can be used in this field.

SecurityID and SecurityIDSource are both required.

SecurityType is required.

Factor is conditionally required when it is not equal to one (1) for MBA, TIPS, ABS.

# OrderQtyData component block

OrderQty is to be expressed as par amount.

# Repurchase Agreements (Repo) and Collateral Management

Specifying rates

# Repo Terminology

The following table maps Repurchase Agreements and Security Lending terminology to FIX data elements with additional usage explanation specific to repos and security lending.

In repurchase agreements (repo) and financial deals the rate is to be specified in the Price and PriceType fields. The rate type of fixed or floating interest rate is mapped to PriceType values of Yield for fixed rate and Spread for floating rate. A benchmark should also be specified when the rate is a floating rate.

| Element           | Description                                                                                         | FIX fields              | Usage                                                  |
| ----------------- | --------------------------------------------------------------------------------------------------- | ----------------------- | ------------------------------------------------------ |
| Accrued interest  | Start accrued interest calculated using the day count method appropriate to the underlying security | AccruedInterestAmt      |                                                        |
| Allocating entity | The party responsible for assigning specific securities and amounts to the trade                    | \<Parties>              | PartyRole 39 = Allocating Entity                       |
| Call or put dates | Dates on which the seller or buyer may liquidate the position                                       | \<Instrument>           | NoEvents (group) EventType EventDate EventPx EventText |
| Cash amount       | Amount of currency                                                                                  | StartCash               |                                                        |
| Cash outstanding  | The current balance of the cash debt                                                                | CashOutstanding         |                                                        |
| Clean price       | Spot price of the security without accrued interest                                                 | \<UnderlyingInstrument> | UnderlyingPx                                           |
| Collateral        | The reason for an initial                                                                           | CollAsgnReason          |                                                        |

April 30, 2003 June 18, 2003 90 FIX 4.4 with Errata 20030618- Volume 7


---


| Element              | Description                                                                                                                                                         | FIX fields                                   | Usage                                                                                                                                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| assignment reason    | assignment or subsequent substitution of collateral for a financing deal                                                                                            |                                              | 0 = Initial 1 = Scheduled 2 = Time Warning 3 = Margin Deficiency 4 = Margin Excess 5 = Forward Collateral Demand 6 = Event of default 7 = Adverse tax event                                               |
| Collateral value     | Repo value times the inverse of haircut, also known as the “all in” price                                                                                           | TotalNetValue                                | At the initial collateral assignment TotalNetValue is the sum of (UnderlyingStartValue \* (1-haircut)). In a collateral substitution TotalNetValue is the sum of (UnderlyingCurrentValue \* (1-haircut)). |
| Contract currency    | The base agreement currency, not necessarily the same as the payment currency                                                                                       | \<FinancingDetails> AgreementCurrency        |                                                                                                                                                                                                           |
| Currency of payments | Currency in which payments are to be made                                                                                                                           | Currency                                     |                                                                                                                                                                                                           |
| Day count            | Method for calculating accrued interest – 30/360, actual/360, actual/actual, actual/365, 30/365.                                                                    |                                              | Not supported directly in the protocol – understood in the context of the underlying security type and master agreement                                                                                   |
| Delivery             | Delivery or custody of underlying securities                                                                                                                        | \<FinancingDetails> DeliveryType             | DeliveryType 0 = “Versus. Payment”: Deliver (if Sell) or Receive (if Buy) vs. (Against) Payment 1 = “Free”: Deliver (if Sell) or Receive (if Buy) Free 2 = Tri-Party 3 = Hold In Custody                  |
| Dirty price          | Spot price of the security including accrued interest                                                                                                               | \<UnderlyingInstrument> UnderlyingDirtyPrice |                                                                                                                                                                                                           |
| End consideration    | Total cash returned at the end of the term                                                                                                                          | EndCash                                      |                                                                                                                                                                                                           |
| End date             | Close date, date of the return of the securities for money, “off” date                                                                                              | \<FinancingDetails> EndDate                  |                                                                                                                                                                                                           |
| Face or cash fill    | In collateral assignment and substitution dictates whether the quantity of the replacement security is to be based on par-for-par (face) or value-for-value (cash). | \<Stipulations> StipulationType=FILL         | StipulationValue=\<face or cash>                                                                                                                                                                          |
| Flex schedule        | Single maturity but moneygiver’s cash may be                                                                                                                        | \<FinancingDetails> StipulationType=PAYFREQ  | TerminationType StipulationValue= \<dates>                                                                                                                                                                |

April 30, 2003June 18, 2003

91 FIX 4.4 with Errata 20030618- Volume 7


---

# Element

# Description

# FIX fields

# Usage

| returned most often on a   |                                                                                                                                        | predetermined paydown schedule    |                                                                                                                  |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Forward accrued interest   | End accrued interest                                                                                                                   | EndAccruedInterestAmt             | calculated using the day count method appropriate to the underlying security                                     |
| Forward price              | Price agreed to on the end leg of the transaction – will vary for indexed bonds                                                        | Price2                            | Denominated in the same type as Price                                                                            |
| Frequency of substitutions | Maximum frequency –                                                                                                                    |                                   | StipulationType=SUBSFREQ                                                                                         |
|                            |                                                                                                                                        | StipulationValue=, e.g. M         | monthly, semi-annually, annually                                                                                 |
| General collateral         | Securities collateralizing a repurchase agreement described generally (treasuries, corporates) rather than specifically by identifier. |                                   | Product=FINANCING                                                                                                |
|                            |                                                                                                                                        |                                   | SecurityType=REPO                                                                                                |
|                            |                                                                                                                                        | UnderlyingSecurityType            | SecuritySubType=GENERAL                                                                                          |
|                            |                                                                                                                                        | TREASURY                          |                                                                                                                  |
|                            |                                                                                                                                        | PROVINCE                          |                                                                                                                  |
|                            |                                                                                                                                        | AGENCY                            | If bonds of a particular issuer or country are wanted and UnderlyingSecurityType is not granular enough, include |
|                            |                                                                                                                                        | MORTGAGE                          | UnderlyingIssuer, UnderlyingCountryOfIssue, UnderlyingProgram, UnderlyingRegType, and/or                         |
|                            |                                                                                                                                        | CP                                |                                                                                                                  |
|                            |                                                                                                                                        | CORP                              |                                                                                                                  |
|                            |                                                                                                                                        | EQUITIES                          |                                                                                                                  |
|                            |                                                                                                                                        | SUPRA                             |                                                                                                                  |
|                            |                                                                                                                                        | CASH                              | Examples:                                                                                                        |
|                            |                                                                                                                                        | SecurityType=REPO                 | UnderlyingSecurityType=MORTGAGE                                                                                  |
|                            |                                                                                                                                        | UnderlyingIssuer=GNMA             |                                                                                                                  |
|                            |                                                                                                                                        | SecurityType=REPO                 | UnderlyingSecurityType=AGENCY                                                                                    |
|                            |                                                                                                                                        | UnderlyingIssuer=CA Housing Trust | UnderlyingCountryOfIssue=CA                                                                                      |
|                            |                                                                                                                                        | SecurityType=REPO                 | UnderlyingSecurityType=CORP                                                                                      |
|                            |                                                                                                                                        | UnderlyingNoStipulations=1        | UnderlyingStipulationType=RATING                                                                                 |
|                            |                                                                                                                                        | UnderlyingStipulationValue=>bbb-  |                                                                                                                  |
| Haircut                    | Reduction in market value taken on assigned securities in calculating their collateral value – based on market volatility and credit.  |                                   | UnderlyingStipType=HAIRCUT                                                                                       |
|                            |                                                                                                                                        | UnderlyingStipValue=              |                                                                                                                  |
| Largest piece              | Maximum size of securities                                                                                                             |                                   | StipulationType=MAXDNOM                                                                                          |

April 30, 2003June 18, 2003 92 FIX 4.4 with Errata 20030618- Volume 7


---

# Element

# Description

# FIX fields

# Usage

| Lookback days    | Number of business days prior to floating rate reset date when the benchmark price will be captured and used to determine the new rate upon reset                                                                                      | \<Stipulations>              | StipulationType=LOOKBACK StiuplationValue=\<number of days> |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- | ----------------------------------------------------------- |
| Margin           | The fraction of the cash consideration that must be collateralized, expressed as a percent. A MarginRatio of 102% indicates that the value of the collateral (after deducting for "haircut") must exceed the cash consideration by 2%. | \<FinancingDetails>          | MarginRatio                                                 |
| Margin excess    | The amount by which the total net value of collateral times margin ratio exceeds cash outstanding                                                                                                                                      |                              | MarginExcess                                                |
| Market value     | Dirty price times nominal amount                                                                                                                                                                                                       | not supported directly – see | Repo value                                                  |
| Master agreement | The name of the standard master agreement forming the basis of the financing relationship                                                                                                                                              | \<FinancingDetails>          | AgreementDesc AgreementID AgreementDate                     |

# Current list of master agreements, amendments and annexes:

- MRA 1996 Repurchase Agreement
- MRA 1996 Repurchase Agreement – Annex I 1997 (for FASB 125 compliance)
- MRA 1996 Repurchase Agreement – Amended 1997 for FASB 125
- MRA 1996 International Transaction (Annex III)
- MRA 1996 Agency Transaction (Annex IV)
- MRA 1996 Forward Transaction (Annex V)
- MRA 1996 Buy/Sell Back Transaction (Annex VI)
- MRA 1996 Equity Securities Transaction (Annex VIII, Feb 1998)
- MRA 1996 Japanese Financial Institutions Transaction (Annex IX, Aug 2002)
- MRA 1987 Repurchase Agreement
- MRA 1987 Repurchase Agreement – Amended 1997 for FASB 125
- GMRA 2000 Repurchase Agreement
- GMRA 2000 Agency Transaction
- GMRA 2000 Bills Transaction (U.K.)


---

# Element

# Description

# FIX fields

# Usage

| GMRA 2000 Forward Transaction                                                     |   |   |
| --------------------------------------------------------------------------------- | - | - |
| GMRA 2000 Buy/Sell Back Transaction                                               |   |   |
| GMRA 2000 Equities Transaction                                                    |   |   |
| GMRA 2000 Canadian Transaction                                                    |   |   |
| GMRA 2000 Italian Transaction                                                     |   |   |
| GMRA 2000 Japanese Transaction                                                    |   |   |
| GMRA 2000 Netherlands Transaction                                                 |   |   |
| GMRA 1995 Repurchase Agreement                                                    |   |   |
| GMRA 1995 Buy/Sell Back Transaction                                               |   |   |
| GMRA 1995 Agency Transaction                                                      |   |   |
| GMRA 1995 Repurchase Agreement – Amended for GMRA 2000 Conformance                |   |   |
| GMRA 1995 Buy/Sell Back Transaction – Amended for GMRA 2000 Conformance           |   |   |
| GMRA 1995 Agency Transaction – Amended for GMRA 2000 Conformance                  |   |   |
| GMRA 1995 Forward Transaction (as enabled by Amendment for GMRA 2000 conformance) |   |   |
| GMRA 1992 Repurchase Agreement                                                    |   |   |
| MSLA 2000 Securities Loan                                                         |   |   |
| MSLA 2000 Agency Transaction (Annex I)                                            |   |   |
| MSLA 2000 Term Loan                                                               |   |   |
| MSLA 1993 Securities Loan                                                         |   |   |
| MSLA 1993 Agency Transaction                                                      |   |   |
| MSLA 1993 Securities Loan – Amended 1998                                          |   |   |

# Maturity type

Open (term is indefinite and may be terminated by either party on demand) or Fixed (pre-determined, may be overnight or from one day to five years). Termination prior to maturity is open to negotiation.

# Maximum pieces

Maximum number of pieces acceptable in the transaction

# Minimum pieces

Minimum number of pieces acceptable in the transaction

# Number of substitutions

Number of substitutions permitted

# Other dynamic stipulations

# Par quantity

Face or nominal value of

April 30, 2003 June 18, 2003

94

FIX 4.4 with Errata 20030618- Volume 7


---


| Element                | Description                                                                                               | FIX fields              | Usage                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ---------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Payment calendar       | Schedule of dates based on frequency of interest payments                                                 | \<Stipulations>         | StipulationType=PAYFREQ StipulationValue=\<dates>                                                                                                                                                                                                                                                                                                                                                                                             |
| Payment interval       | Payment interval, i.e. 3 months, 6 months, etc.                                                           | \<Stipulations>         | StipulationType=PAYFREQ StipulationValue=\<interval> e.g. 3M                                                                                                                                                                                                                                                                                                                                                                                  |
| Percent of variance    | Maximum variance allowable in the value of replacement securities                                         | \<Stipulations>         | StipulationType=TRDVAR StipulationValue=\<count>                                                                                                                                                                                                                                                                                                                                                                                              |
| Rate reset calendar    | Schedule of dates based on frequency                                                                      | \<Stipulations>         | StipulationType=PRICEFREQ StipulationValue=\<dates>                                                                                                                                                                                                                                                                                                                                                                                           |
| Rate reset interval    | Reset interval, i.e. 3 months, 6 months, etc.                                                             | \<Stipulations>         | StipulationType=PRICEFREQ StipulationValue=\<frequency> e.g. 6M                                                                                                                                                                                                                                                                                                                                                                               |
| Rate type              | How the yield paid on the cash investment is to be calculated                                             | PriceType               | 9 \[yield = Fixed Rate] 6 \[spread = Floating Rate]                                                                                                                                                                                                                                                                                                                                                                                           |
| Repo rate              | The fixed yield or yield spread paid on the cash investment                                               | Price                   |                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| Repo value             | Market value rounded using the appropriate market practice convention of the security in the repo market. | \<UnderlyingInstrument> | These fields are the repo value (rounded market value) of each piece of collateral at the start, current and end of the deal. Haircut is not factored in these values. The respondent is free to populate these fields as needed based on the purpose of the current message, but we recommend UnderlyingStartValue on initial assignment and UnderlyingCurrentValue on substitution since TotalNetValue is conditionalized on these actions. |
| Securities lending fee | Used in lieu of interest rate of Fee-based transactions                                                   | MiscFeeType             | MiscFeeType 13 = Securities Lending                                                                                                                                                                                                                                                                                                                                                                                                           |
| Security rating range  | Minimum acceptable rating on any securities involved in the transaction                                   | \<Stipulations>         | StipulationType=RATING StipulationValue=\                                                                                                                                                                                                                                                                                                                                                                                     |
| Smallest piece         | Minimum size of securities acceptable in the transaction                                                  | \<Stipulations>         | StipulationType=MINDNOM StipulationValue=\<size>                                                                                                                                                                                                                                                                                                                                                                                              |
| Spot price             | Price for the start leg of the transaction                                                                | Price                   | PriceType 1 = Percentage 2 = Per unit 3 = Fixed amount                                                                                                                                                                                                                                                                                                                                                                                        |
| Start consideration    | Total cash remitted at the beginning of the term                                                          | StartCash               |                                                                                                                                                                                                                                                                                                                                                                                                                                               |

April 30, 2003June 18, 2003 95 FIX 4.4 with Errata 20030618- Volume 7


---

# Collateral Management

| Element                | Description                                                                                                                                                                                                                                                                                   | FIX fields                                       | Usage |
| ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ | ----- |
| Start date             | Settlement date for “on” date or “start leg”                                                                                                                                                                                                                                                  | StartDate                                        |       |
| Trade date             | Date of trade agreement                                                                                                                                                                                                                                                                       | TradeDate                                        |       |
| Type of financing deal | Attributes of the financing arrangement – Repo, Reverse Repo, Sell/Buy, Buy/Sell, Fee-based Loan, Fee-based Borrow, Loan vs. Cash, Borrow vs. Cash, Fee-based Loan vs. Cash, Fee-based Borrow vs. Cash, Master Forward Sell/Buy, Master Forward Buy/Sell, Sec Lend, Sec Borrow, Borrow Pledge | Product=FINANCING SecurityType SecurityType=REPO |       |
|                        | REPO – repurchase agreement                                                                                                                                                                                                                                                                   | SecuritySubType=GENERAL Side=                    |       |
|                        | FORWARD – forward                                                                                                                                                                                                                                                                             | TerminationType= StartDate= EndDate=             |       |
|                        | BUYSELL – buy/sellback or sell/buyback                                                                                                                                                                                                                                                        | UnderlyingSecurityType= AgreementDesc=           |       |
|                        | Often combined with Overnight, Term, Flexible, Open                                                                                                                                                                                                                                           | TerminationType StartDate EndDate                |       |

The following diagrams illustrate an example flow for collateral management once a repo or financing deal has been completed. Figures 8 to 11 show an example for 2-party model and Figure 12 shows an example for 3-party model.


---

FIX 4.4 with Errata 20030618 - Volume 7


# Figure 8: Example flow of Repo Trade

| Initiator                       | Respondent   |
| ------------------------------- | ------------ |
| QuoteRequest                    |              |
| QuoteReqID \<newz               |              |
| UnderlyingSecurityType \<type>  |              |
| RateType \<ratetype>            |              |
| TerminationType \<term>         |              |
| Quote                           |              |
| QuoteReqID \<initiators>        |              |
| QuotelD \<newz                  |              |
| UnderlyingSecurity Type \<type> |              |
| RateType \<ratetype>            |              |
| TerminationType \<term>         |              |
| Price \<reporate>               |              |
| New Order                       |              |
| CIOrdID \<newz                  |              |
| SecuritySubtype \<General>      |              |
| UnderlyingSecurityType \<type>  |              |
| RateType \<ratetype>            |              |
| TerminationType \<term>         |              |
| Price \<reporate>               |              |
| CashOrderPrice \<size>          |              |
| Execution Report                |              |
| OrderID \<newz                  |              |
| CIOrdID \<initiators>           |              |
| ExecID \<new?                   |              |
| OrdStatus 2 "Filled"            |              |
| UnderlyingSecurityType \<type>  |              |
| RateType \<ratetype>            |              |
| TerminationType \<term>         |              |
| Price \<reporate>               |              |
| CashOrderPrice \<size>          |              |
| NetMoney \<amtz>                |              |
| To                              | Collateral\_ |

Click here to go to “Collateral Assignment”

April 30, 2003June 18, 2003


97

---
# Figure 9: Example flow for Collateral Assignment

April 30, 2003 June 18, 2003

98 FIX 4.4 with Errata 20030618- Volume 7
---

# Collateral Management

# Initiator

| Collateral                 | Respondent                    |
| -------------------------- | ----------------------------- |
| Collateral Assignment      | CollateralAssignmentID \<newz |
| CIOrdID \<initiators>      | OrderID \<respondent's>       |
| ExecID \<respondent's>     | New Initial Assgnmnt Proposed |
| ExpireTime \         | Underlying Instrument \[1]    |
| Underlying Instrument \[n] | RateType \<ratetype>          |
| TerminationType \<term?>   | Price \<reporate>             |
| TotalNetValue \<amt>       | CashOutstanding \<amtz>       |
| MarginExcess \<amt>        |                               |

# Collateral Response

| CollatAssignmentID \<respondent's> | CIOrdID \<initiators>      |
| ---------------------------------- | -------------------------- |
| OrderID \<respondent's>            | ExecID \<respondent's>     |
| CollatResponseType Accept          | Underlying Instrument \[1] |
| Underlying Instrument \[n]         | RateType \<ratetype>       |
| TerminationType \<term?>           | Price \<reporate>          |
| TotalNetValue \<amtz>              | CashOutstanding \<amtz>    |
| MarginExcess \<amt>                |                            |

# Collateral Assignment

| CollateralAssignmentID \<newz | CIOrdID \<initiators>      |
| ----------------------------- | -------------------------- |
| OrderID \<respondent's>       | ExecID \<respondent's>     |
| New \<omit>                   | Assgnmnt Accepted          |
| Underlying Instrument \[1]    | Underlying Instrument \[n] |
| RateType \<ratetype>          | TerminationType \<term?>   |
| Price \<reporate>             | TotalNetValue \<amtz>      |
| CashOutstanding \<amtz>       | MarginExcess \<amt>        |

April 30, 2003 June 18, 2003

99 FIX 4.4 with Errata 20030618 - Volume 7


---

Figure 10: Example use of Collateral Request

# Initiator

| CollateralRequest         |                  |
| ------------------------- | ---------------- |
| CollatRequestID           | \<new>           |
| CIOrdID                   | \<initiator's>   |
| OrderID                   | \<respondent's>  |
| ExecID                    | \<respondent's>  |
| CollatAllocRsn            | MarginDeficiency |
| UnderlyingInstrument \[1] |                  |
| UnderlyingInstrument \[n] |                  |
| RateType                  | \<ratetype>      |
| TerminationType           | \<term>          |
| Price                     | \<reporate>      |
| TotalNetValue             | \<amt>           |
| CashOutstanding           | \<amt>           |
| MarginExcess              | \<amt>           |

# CollateralAssignment

| CollateralAssignmentID    | \<new?>           |
| ------------------------- | ----------------- |
| CIOrdID                   | \<initiator's>    |
| OrderID                   | \<respondent's>   |
| ExecID                    | \<respondent's>   |
| New                       | Margin Deficiency |
| Assgnmnt Proposed         |                   |
| UnderlyingInstrument \[1] |                   |
| UnderlyingInstrument \[n] |                   |
| RateType                  | \<ratetype>       |
| TerminationType           | \<term>           |
| Price                     | \<reporate>       |
| TotalNetValue             | \<amt>            |
| CashOutstanding           | \<amt>            |
| MarginExcess              | \<amt>            |

# CollateralResponse

| CollatAssignmentID        | \<respondent's> |
| ------------------------- | --------------- |
| CIOrdID                   | \<initiator's>  |
| OrderID                   | \<respondent's> |
| ExecID                    | \<respondent's> |
| CollatResponseType        | Accept          |
| UnderlyingInstrument \[1] |                 |
| UnderlyingInstrument \[n] |                 |
| RateType                  | \<ratetype>     |
| TerminationType           | \<term>         |
| Price                     | \<reporate>     |
| TotalNetValue             | \<amt>          |
| CashOutstanding           | \<amt>          |
| MarginExcess              | \<amt>          |

# CollateralAssignment

| CollateralAssignmentID    | \<new>          |
| ------------------------- | --------------- |
| CIOrdID                   | \<initiator's>  |
| OrderID                   | \<respondent's> |
| ExecID                    | \<respondent's> |
| New /                     | \<omit>         |
| Assgnmnt Accepted         |                 |
| UnderlyingInstrument \[1] |                 |
| UnderlyingInstrument \[n] |                 |
| RateType                  | \<ratetype>     |
| TerminationType           | \<term>         |
| Price                     | \<reporate>     |
| TotalNetValue             | \<amt>          |
| CashOutstanding           | \<amt>          |
| MarginExcess              | \<amt>          |


April 30, 2003June 18, 2003                                  100          FIX 4.4 with Errata 20030618- Volume 7

---

Figure 11: Collateral Inquiry


| Initiator                 | Respondent      |
| ------------------------- | --------------- |
| Collaterallnquiry         |                 |
| CollatInquirylD           | \<newz          |
| Filters                   |                 |
| CollateralReport          |                 |
| CollateralReportID        | \<new>          |
| CollaterallnquiryID       | \<initiators>   |
| CIOrdID                   | \<initiators>   |
| OrderID                   | \<respondent's> |
| ExecID                    | \<respondent's> |
| Status                    | \<omit>         |
| \<CollateralStatus?       |                 |
| UnderlyingInstrument \[1] |                 |
| Underlyinglnstrument \[n] |                 |
| RateType                  | \<ratetype>     |
| TerminationType           | \<term>         |
| Price                     | \<reporate>     |
| TotalNetValue             | \<amtz          |
| CashOutstanding           | \<amtz          |
| MarginExcess              | \<amt>          |

April 30, 2003 June 18, 2003

101 FIX 4.4 with Errata 20030618- Volume 7



---

# Figure 12: 3-Party Collateral flow

| Buyer                                                                             | TradingSystem\_IBD\_Exchange                                                                             | Seller                                                                                                       |
| --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| Collateral Report CollStatus = Unassigned Collateral Request CollStatus = Initial | Optional message Collateral Report CollStatus = TimeWarning (you have until time X to assign collateral) | Collateral Assignment ColTransType - New CollAsgnReason - Initial or TimeWarning                             |
| Collateral Report CollStatus = Assigned (accepted) or Challenged                  | Optional message Collateral Response CollAsgnRespType Accepted or Rejected                               | Collateral Response CollAsgnRespType Accepted or Rejected Collateral Report CollStatus = Assigned (accepted) |

April 30, 2003 June 18, 2003 102 FIX 4.4 with Errata 20030618 - Volume 7


---

Identifying Euro Issuers

Euro CountryOfIssue Codes:

Use ISO codes in CountryOfIssue to identify the issuing country for non-US Governments. Omit CountryOfIssue or use a value of ‘XS’ when the issuer is a supra-national agency, e.g. the first nine entries in the table below.

Euro Issuer Values:

The list below are used in the Issuer (106) field to further identify the issuer for securities such as EUSUPRA, EUSOV and PFAND (see data dictionary entry to SecurityType (167) in Volume 6. The abbreviations are from Bloomberg.

*Credit / Sovereign issued in any currency.

| COE    | Council of Europe                                |
| ------ | ------------------------------------------------ |
| DTA    | Deutsche Ausgleichsbank                          |
| EBRD   | European Bank for Reconstruction and Development |
| EIB    | European Investment Bank                         |
| HESLAN | Hessen                                           |
| KFW    | Kreditanstalt fuer Wiederaufbau                  |
| LANREN | Landwirtschaftliche Rentenbank                   |
| NORWES | Nord-Rhein-Westfalen NRW                         |
| SACHAN | Sachsen-Anhalt                                   |

| RATB  | Austrian Treasury Bill                   |
| ----- | ---------------------------------------- |
| RAGB  | Austrian Government Bond                 |
| AOBL  | Austrian Bundesobligation (OBL)          |
| RABSS | Austrian Bundesschatzscheine             |
| AUST  | Austrian Government International Bond\* |
| RAGBS | RAGB Coupon Strip (Austrian)             |
| RAGBR | RAGB Principal Strip (Austrian)          |
| RAMTB | Austria Medium Term Bill                 |

| BGTB | Belgian Treasury Bill                 |
| ---- | ------------------------------------- |
| BGB  | Belgian Government Bond               |
| BELG | Belgian Government International Bond |
| OLOS | Belgian Strip                         |
| OLOR | Belgian Principal Strip               |

| DGTB | Danish Treasury Bill                         |
| ---- | -------------------------------------------- |
| DGB  | Danish Government Bond                       |
| DENK | Danish Government International Bond\* (DKK) |

April 30, 2003 June 18, 2003 103 FIX 4.4 with Errata 20030618- Volume 7


---


# Finnish Treasury Bill

- RFTB - Finnish Treasury Bill
- RFGB - Finnish Government Bond
- FINL - Finnish Government International Bond*
- FNHF - Finnish Housing Bond

# French Treasury Instruments

- BTF - French Fixed-Rate Short Term Discount Treasury Bills
- BTNS - BTAN - French Fixed-Rate Treasury Notes
- FRTR - OAT - French Treasury Bonds
- FRTRR - OAT - French Treasury Bonds Principal STRIPS
- FRTRS - OAT - French Treasury Bonds Coupon STRIPS
- CADES - Social Security Debt Repayment Fund (French)*

# German Treasury Instruments

- BUBILL - German Treasury Bill
- DBSB - German Federal Treasury Bill (rarely used puttable &#x26; DM Ccy)
- BKO - German Two Year Notes
- FSDB - German Financing Treasury Notes (DM Ccy)
- DBR - German Government Bond
- DBRR - German Government Bond Principal STRIPS
- DBRS - German Government Bond Coupon STRIPS
- OBL - German Five Year Bonds
- DBRUF - German Unity Fund DBR – S (only 2)
- BKOUF - German Unity Fund – BKO (None)
- DBP - German Federal Post -- BUNDESPOST
- DBB - German Federal Railroad --BUNDESBAHN
- THA - Treuhand Agency Bonds
- TOBL - Treuhand Agency Obligations – All matured
- ENTFND - German Retribution Fund – Only 2 sinking funds
- GERP - European Recovery Program Special Funds (German only 2)
- BUKASS - Bundeskassenscheine – 1 matured

# Hellenic Republic Instruments

- GTB - Hellenic Republic Treasury Bill
- GGB - Hellenic Republic Government Bond
- GREECE - Hellenic Republic Government International Bond*
- GGBSTP - Hellenic Republic Government Bond Coupon STRIPS
- GGBRES - Hellenic Republic Government Bond Residual STRIPS

# Irish Government Bond

- IRISH - Irish Government Bond

April 30, 2003 June 18, 2003 104 FIX 4.4 with Errata 20030618- Volume 7



---
IRELND    Irish Government International Bond*
BOTS      Italian Treasury Bill

BTPS      Italian Government Bond

CCTS      Italian Treasury Certificate

ICTZ      Italian Zero Coupon Bonds

CTES      Italian Government Bonds Issued in EUR –Matured

CTOS      Italian Government Bonds with Put Option – All matured

ITALY     Italian International Bonds*

BTPSS     Italian Government Bond Coupon STRIPS

BTPSR     Italian Government Bond Residual STRIPS

LGB       Luxembourgeois Government Bond

NETHER    Dutch Government Bond

NETHRR    Dutch Principal Strip

NETHRS    Dutch Strip

DTB       Dutch Treasury Certificate

NBC       Dutch Bank Certificate – All matured

NGTB      Norwegian Treasury Bill

NGB       Norwegian Government Bond

NORWAY Norwegian Government International Bond* (NOK)

PORTB     Portuguese Treasury Bills

PGB       Portuguese Government Bond

PORTUG    Portuguese Government International Bond*

SPGB      Spanish Government Bond

SPGBS     Spanish Government Bond Coupon Strips

SPGBR     Spanish Government Bond Principal Strips

SPAIN     Spanish Government International Bond*

SGLT      Spanish Letras del Tesoro

SWTB      Swedish Treasury Bill

SGB       Swedish Government Bond

SWED      Swedish Government International Bond* (SEK)

SGBS      Swedish Government Bond Coupon Strip

April 30, 2003June 18, 2003                   105     FIX 4.4 with Errata 20030618- Volume 7


---
SGBR      Swedish Government Bond Residual Strip

SWISTB    Swiss Treasury Bill

SWISS     Swiss Government Bond

GENTB     Geneva Treasury Bill (CHF)

UKTB      United Kingdom GBP/EUR Treasury Bill

UKT       United Kingdom Gilt Bond

UKTS      United Kingdom Gilt Bond Coupon STRIPS

UKTR      United Kingdom Gilt Bond Residual STRIPS

UKIN      United Kingdom International Bond*

BOE       Bank of England EUR Bill

BOEN      Bank of England EUR Note

# Example usage of FI specific component blocks

# Example usage of BenchmarkCurve fields

Note: the following is a subset of possible value combinations.

| Description/  | BenchmarkCurveC | BenchmarkCurveName | BenchmarkCurvePoint |
| ------------- | --------------- | ------------------ | ------------------- |
| Common Name   | Currency        | Curve              | USD                 |
| 5 Year        | USD             | Treasury           | INTERPOLATED        |
| Old 5 Year    | USD             | Treasury           | 5Y-OLD              |
| 10 Year       | USD             | Treasury           | 10Y                 |
| Old 10 Year   | USD             | Treasury           | 10Y-OLD             |
| 30 Year       | USD             | Treasury           | 30Y                 |
| Old 30 Year   | USD             | Treasury           | 30Y-OLD             |
| 3 Month LIBOR | USD             | LIBOR              | 3M                  |
| 6 Month LIBOR | USD             | LIBOR              | 6M                  |
| Canadian      | CAD             | Treasury           | INTERPOLATED        |
| UK Curve      | GBP             | Treasury           | INTERPOLATED        |
| ECU/EURO      | EUR             | Treasury           | INTERPOLATED        |
| US Swap       | USD             | SWAP               | INTERPOLATED        |
| Euro Swap     | EUR             | SWAP               | INTERPOLATED        |
| EDFS          | EUR             | FutureSWAP         | INTERPOLATED        |
| German Bund   | DEM             | Treasury           | INTERPOLATED        |
| US MuniAAA    | USD             | MuniAAA            | 10Y                 |
| US T point    | USD             | Treasury           | 2/2031 5 3/8        |

(combination of April 30, 2003June 18, 2003                  106         FIX 4.4 with Errata 20030618- Volume 7


---
| NoStipulations | StipulationType | StipulationValue | Description of the Stipulation                            |
| -------------- | --------------- | ---------------- | --------------------------------------------------------- |
| 4              | WALA            | >=60             | Weighted average loan age Less than or equal to 60 months |
|                | TRDVAR          | .0025            | Trade variance .25%                                       |
|                | PSA             | .25              | Prepayment speed 25%                                      |
|                | GEOG            | ORANGE           | Geographics CONTRACOSTA Orange OR Contra Costa Counties   |

April 30, 2003June 18, 2003 107 FIX 4.4 with Errata 20030618- Volume 7
---

PRODUCT: FOREIGN EXCHANGE

# Foreign Exchange (F/X) Trading

# Notes:

The forex Symbol is defined in Electronic Broking Services, Ltd. (see http://www.ebs.com) format: "CCY1/CCY2".
Rates are expressed as "currency1 in currency2" (or "currency2 per currency1") and are calculated as CCY2 divided by CCY1 (NOT CCY1 divided by CCY2) (e.g. "GBP/USD" represents a rate expressed as USD per GBP, "USD/JPY" represents a rate expressed as JPY per USD, etc.). CCY1 and CCY2 are ISO currency codes.

OrderQty represents the amount expressed in units of currency specified by the Currency field. Note OrderQty must be capable of representing large values with at least 2 decimal places. The value of the Currency field represents the denomination of the quantity field(s). The Rate (specified as a “Price” field) represents CCY2 divided by CCY1 (NOT CCY1 divided by CCY2).

The “unknown quantity” can be calculated using the following rules:

- If Currency field value = CCY1 then as: (OrderQty * Rate)
- If Currency field value = CCY2 then as: (OrderQty / Rate)

| Verbal representation | Side       |      |           | OrderQty | Curr        | Symbol (CCY1/CCY2) | Rate   | Rate "Style  | "Resulting" Quantity |
| --------------------- | ---------- | ---- | --------- | -------- | ----------- | ------------------ | ------ | ------------ | -------------------- |
| Sell                  | 1,000,000  | Sell | 1,000,000 | USD      | USD/JPY     | 105.92             | Normal | 105,920,000  |                      |
|                       |            |      |           | JPY/USD  | 0.00944108  | Inverted           |        |              |                      |
| Sell                  | 50,000,000 | JPY  | USD/JPY   | 105.92   | Normal      | 472,054.38         |        |              |                      |
|                       |            |      |           | JPY/USD  | 0.00944108  | Inverted           |        |              |                      |
| Buy                   | 50,000,000 | JPY  | USD/JPY   | 105.92   | Normal      | 472,054.38         |        |              |                      |
|                       |            |      |           | JPY/USD  | 0.00944108  | Inverted           |        |              |                      |
| Buy                   | 1,000,000  | Buy  | 1,000,000 | USD      | USD/JPY     | 105.92             | Normal | 105,920,000  |                      |
|                       |            |      |           | JPY/USD  | 0.00944108  | Inverted           |        |              |                      |
| Sell                  | 1,000,000  | Sell | 1,000,000 | USD      | USD/CAD     | 1.437              | Normal | 1,437,000.00 |                      |
|                       |            |      |           | CAD/USD  | 0.695894224 | Inverted           |        |              |                      |
| Sell                  | 50,000,000 | CAD  | USD/CAD   | 1.437    | Normal      | 34,794,711.20      |        |              |                      |

April 30, 2003 June 18, 2003 108 FIX 4.4 with Errata 20030618- Volume 7


---

| 50,000,000    |      |                | 0       | CAD/USD | 0.695894224 | Inverted CAD for USD |   |   |   |
| ------------- | ---- | -------------- | ------- | ------- | ----------- | -------------------- | - | - | - |
| Buy           | Buy  | 50,000,000 CAD | USD/CAD | 1.437   | Normal      | 34,794,711.20        |   |   |   |
| 50,000,000    |      |                | 0       | CAD/USD | 0.695894224 | Inverted CAD for USD |   |   |   |
| Buy 1,000,000 | Buy  | 1,000,000 USD  | USD/CAD | 1.437   | Normal      | 1,437,000.00         |   |   |   |
| USD for CAD   |      |                |         | CAD/USD | 0.695894224 | Inverted             |   |   |   |
| Sell          | Sell | 1,000,000 USD  | GBP/USD | 1.6368  | Normal      | 610,948.19           |   |   |   |
| USD for GBP   |      |                |         | USD/GBP | 0.610948192 | Inverted             |   |   |   |
| Sell          | Sell | 50,000,000 GBP | GBP/USD | 1.6368  | Normal      | 81,840,000.00        |   |   |   |
| 50,000,000    |      |                | 0       | USD/GBP | 0.610948192 | Inverted GBP for USD |   |   |   |
| Buy           | Buy  | 50,000,000 GBP | GBP/USD | 1.6368  | Normal      | 81,840,000.00        |   |   |   |
| 50,000,000    |      |                | 0       | USD/GBP | 0.610948192 | Inverted GBP for USD |   |   |   |
| Buy 1,000,000 | Buy  | 1,000,000 USD  | GBP/USD | 1.6368  | Normal      | 610,948.19           |   |   |   |
| USD for GBP   |      |                |         | USD/GBP | 0.610948192 | Inverted             |   |   |   |
| Sell          | Sell | 1,000,000 USD  | EUR/USD | 1.001   | Normal      | 999,001.00           |   |   |   |
| USD for EUR   |      |                |         | USD/EUR | 0.999000999 | Inverted             |   |   |   |
| Sell          | Sell | 50,000,000 EUR | EUR/USD | 1.001   | Normal      | 50,050,000.00        |   |   |   |
| 50,000,000    |      |                | 0       | USD/EUR | 0.999000999 | Inverted EUR for USD |   |   |   |
| Buy           | Buy  | 50,000,000 EUR | EUR/USD | 1.001   | Normal      | 50,050,000.00        |   |   |   |
| 50,000,000    |      |                | 0       | USD/EUR | 0.999000999 | Inverted EUR for USD |   |   |   |
| Buy 1,000,000 | Buy  | 1,000,000 USD  | EUR/USD | 1.001   | Normal      | 999,001.00           |   |   |   |
| USD for EUR   |      |                |         | USD/EUR | 0.999000999 | Inverted             |   |   |   |
| Sell          | Sell | 1,000,000 EUR  | EUR/GBP | .6111   | Normal      | 611,100.00           |   |   |   |
| EUR for GBP   |      |                |         | GBP/EUR | 1.636393389 | Inverted             |   |   |   |
| Sell          | Sell | 50,000,000 GBP | EUR/GBP | .6111   | Normal      | 81,819,669.45        |   |   |   |
| 50,000,000    |      |                | 0       | GBP/EUR | 1.636393389 | Inverted GBP for EUR |   |   |   |
| Buy           | Buy  | 50,000,000 GBP | EUR/GBP | .6111   | Normal      | 81,819,669.45        |   |   |   |

April 30, 2003 June 18, 2003

109 FIX 4.4 with Errata 20030618 - Volume 7


---

| 50,000,000    | 0         | GBP/EUR    | 1.636393389 | Inverted |         |        |               |              |
| ------------- | --------- | ---------- | ----------- | -------- | ------- | ------ | ------------- | ------------ |
| Buy 1,000,000 | Buy       | 1,000,000  | EUR         | EUR/GBP  | 0.6111  | Normal | 611,100.00    |              |
| EUR for GBP   |           | GBP/EUR    | 1.636393389 | Inverted |         |        |               |              |
| Sell          | 1,000,000 | Sell       | 1,000,000   | EUR      | EUR/CHF | 1.6125 | Normal        | 1,612,500.00 |
| EUR for CHF   |           | CHF/EUR    | 0.620155039 | Inverted |         |        |               |              |
| Sell          |           | 50,000,000 | CHF         | EUR/CHF  | 1.6125  | Normal | 31,007,751.94 |              |
| 50,000,000    | 0         | CHF/EUR    | 0.620155039 | Inverted |         |        |               |              |
| CHF for EUR   |           |            |             |          |         |        |               |              |
| Buy           |           | 50,000,000 | CHF         | EUR/CHF  | 1.6125  | Normal | 31,007,751.94 |              |
| 50,000,000    | 0         | CHF/EUR    | 0.620155039 | Inverted |         |        |               |              |
| CHF for EUR   |           |            |             |          |         |        |               |              |
| Buy 1,000,000 | Buy       | 1,000,000  | EUR         | EUR/CHF  | 1.6125  | Normal | 1,612,500.00  |              |
| EUR for CHF   |           | CHF/EUR    | 0.620155039 | Inverted |         |        |               |              |

April 30, 2003 June 18, 2003

110 FIX 4.4 with Errata 20030618 - Volume 7

