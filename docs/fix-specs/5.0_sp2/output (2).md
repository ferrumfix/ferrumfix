
FINANCIAL INFORMATION
EXCHANGE PROTOCOL
(FIX)

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3 – FIX APPLICATION MESSAGES: PRE-TRADE

August 18, 2011

© Copyright, 2008-2011, FIX Protocol, Limited


---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011



# DISCLAIMER

THE INFORMATION CONTAINED HEREIN AND THE FINANCIAL INFORMATION EXCHANGE PROTOCOL (COLLECTIVELY, THE "FIX PROTOCOL") ARE PROVIDED "AS IS" AND NO PERSON OR ENTITY ASSOCIATED WITH THE FIX PROTOCOL MAKES ANY REPRESENTATION OR WARRANTY, EXPRESS OR IMPLIED, AS TO THE FIX PROTOCOL (OR THE RESULTS TO BE OBTAINED BY THE USE THEREOF) OR ANY OTHER MATTER AND EACH SUCH PERSON AND ENTITY SPECIFICALLY DISCLAIMS ANY WARRANTY OF ORIGINALITY, ACCURACY, COMPLETENESS, MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. SUCH PERSONS AND ENTITIES DO NOT WARRANT THAT THE FIX PROTOCOL WILL CONFORM TO ANY DESCRIPTION THEREOF OR BE FREE OF ERRORS. THE ENTIRE RISK OF ANY USE OF THE FIX PROTOCOL IS ASSUMED BY THE USER.

NO PERSON OR ENTITY ASSOCIATED WITH THE FIX PROTOCOL SHALL HAVE ANY LIABILITY FOR DAMAGES OF ANY KIND ARISING IN ANY MANNER OUT OF OR IN CONNECTION WITH ANY USER'S USE OF (OR ANY INABILITY TO USE) THE FIX PROTOCOL, WHETHER DIRECT, INDIRECT, INCIDENTAL, SPECIAL OR CONSEQUENTIAL (INCLUDING, WITHOUT LIMITATION, LOSS OF DATA, LOSS OF USE, CLAIMS OF THIRD PARTIES OR LOST PROFITS OR REVENUES OR OTHER ECONOMIC LOSS), WHETHER IN TORT (INCLUDING NEGLIGENCE AND STRICT LIABILITY), CONTRACT OR OTHERWISE, WHETHER OR NOT ANY SUCH PERSON OR ENTITY HAS BEEN ADVISED OF, OR OTHERWISE MIGHT HAVE ANTICIPATED THE POSSIBILITY OF, SUCH DAMAGES.

No proprietary or ownership interest of any kind is granted with respect to the FIX Protocol (or any rights therein), except as expressly set out in FIX Protocol Limited's Copyright and Acceptable Use Policy.

© Copyright 2003-2011 FIX Protocol Limited, all rights reserved

# REPRODUCTION

FIX Protocol Limited grants permission to print in hard copy form or reproduce the FIX Protocol specification in its entirety provided that the duplicated pages retain the “Copyright FIX Protocol Limited” statement at the bottom of the page.

Portions of the FIX Protocol specification may be extracted or cited in other documents (such as a document which describes one’s implementation of the FIX Protocol) provided that one reference the origin of the FIX Protocol specification (http://www.fixprotocol.org) and that the specification itself is “Copyright FIX Protocol Limited”. FIX Protocol Limited claims no intellectual property over one’s implementation (programming code) of an application which implements the behavior and details from the FIX Protocol specification.

© Copyright, 2008-2011, FIX Protocol, Limited


Page 2 of 200

---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                 August 18, 2011


# Contents – Volume 3

DISCLAIMER.............................................................................................................................................................. 2

REPRODUCTION....................................................................................................................................................... 2

# FIX APPLICATION MESSAGES: PRE-TRADE

PRE-TRADE COMPONENT BLOCKS....................................................................................................................8

LEGBENCHMARKCURVEDATA COMPONENT BLOCK.................................................................................................... 8

ROUTINGGRP COMPONENT BLOCK...............................................................................................................................8

TICKRULES COMPONENT BLOCK..................................................................................................................................9

PRICELIMITS COMPONENT BLOCK................................................................................................................................9

MARKETDATAFEEDTYPES COMPONENT BLOCK........................................................................................................ 10

LOTTYPERULES COMPONENT BLOCK........................................................................................................................ 10

MATCHRULES COMPONENT BLOCK...........................................................................................................................11

EXECINSTRULES COMPONENT BLOCK.......................................................................................................................11

TIMEINFORCERULES COMPONENT BLOCK.................................................................................................................12

ORDTYPERULES COMPONENT BLOCK........................................................................................................................12

TRADINGSESSIONRULES COMPONENT BLOCK...........................................................................................................13

BASETRADINGRULES COMPONENT BLOCK................................................................................................................ 14

# CATEGORY: INDICATION

INDICATION COMPONENT BLOCKS............................................................................................................................ 15

InstrmtLegIOIGrp component block....................................................................................................................15

IOIQualGrp component block.............................................................................................................................. 15

ADVERTISEMENTS......................................................................................................................................................16

INDICATIONS OF INTEREST.........................................................................................................................................18

# CATEGORY: EVENT COMMUNICATION

EVENT COMMUNICATION COMPONENT BLOCKS.......................................................................................................20

LinesOfTextGrp component block........................................................................................................................20

NewsREfGrp component block.............................................................................................................................20

NEWS.........................................................................................................................................................................21

EMAIL........................................................................................................................................................................ 23

# CATEGORY: QUOTATION / NEGOTIATION

QUOTATION / NEGOTIATION COMPONENT BLOCKS...................................................................................................24

LegQuotGrp component block.............................................................................................................................24

LegQuotStatGrp component block....................................................................................................................... 26

QuotCxlEntriesGrp component block..................................................................................................................27

QuoteEntryAckGrp component block...................................................................................................................27

QuotEntryGrp component block...........................................................................................................................29

QuotQualGrp component block...........................................................................................................................30

QuotReqGrp component block............................................................................................................................. 31

QuotReqLegsGrp component block......................................................................................................................33

QuotReqRjctGrp component block.......................................................................................................................34

QuotSetAckGrp component block.........................................................................................................................36

QuotSetGrp component block..................................................................................................................................37

RFQReqGrp component block..................................................................................................................................38

# QUOTE REQUEST

QUOTE RESPONSE......................................................................................................................................................41

QUOTE REQUEST REJECT...........................................................................................................................................45

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                           Page 3 of 200



---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

# RFQ REQUEST

...........................................................................................................................................................46

# Tradeable Quote Model - Using the RFQ Request

..............................................................................................................................................................46

# QUOTE

....................................................................................................................................................................... 48

# QUOTE CANCEL

.........................................................................................................................................................53

# Options usage notes:

............................................................................................................................................ 54

# Examples of the types of Quote Cancel operations:

.................................................................................................................................54

# QUOTE STATUS REQUEST

.......................................................................................................................................... 56

# QUOTE STATUS REPORT

............................................................................................................................................ 58

# Indicative Quoting Model

.....................................................................................................................................62

# Indicative Quoting Model Message Scenario

.................................................................................................................................62

# Tradeable Quote Model

........................................................................................................................................63

# Tradeable Quote Model - Reporting Quote Status back to Issuer

......................................................................................63

# Using the Execution Report to report a trade on a Tradeable Quote

..................................................................................63

# Tradeable Quote Model - Quote on Demand Message Scenario

........................................................................................64

# Tradeable Quote Model Message Scenario - Continuous markets

................................................................................65

# Tradeable Quote Model - Querying for Quote Status

.........................................................................................................66

# Restricted Tradeable Quote Model

......................................................................................................................68

# Restricted Tradeable Quote Model Message Scenario

.......................................................................................................68

# MASS QUOTE

.............................................................................................................................................................70

# MASS QUOTE ACKNOWLEDGEMENT

..........................................................................................................................74

# Mass Quote Message Scenarios

...........................................................................................................................76

# Unsolicited quote(s) no response requested

........................................................................................................................ 76

# Unsolicited quote(s) negative response only requested

...................................................................................................................... 76

# Unsolicited quote(s) full response requested

.......................................................................................................................77

# Cancel All Quotes

................................................................................................................................................................ 77

# Use of other Quote Messages in Mass Quoting

........................................................................................................................... 77

# Reporting Quote Status back to Mass Quote Issuer

....................................................................................................................77

# Querying for Mass Quote Status

.........................................................................................................................................78

# CATEGORY: MARKET DATA

.............................................................................................................................79

# MARKET DATA COMPONENT BLOCKS

...............................................................................................................................79

# InstrmtMDReqGrp component block

.........................................................................................................................79

# MDFullGrp component block

...............................................................................................................................80

# MDIncGrp component block

................................................................................................................................ 83

# MDReqGrp component block

...............................................................................................................................86

# MDRjctGrp component block

...............................................................................................................................87

# SecSizesGrp component block

.............................................................................................................................. 88

# StatsIndGrp component block

.............................................................................................................................. 88

# StrmAsgnReqGrp component block

..............................................................................................................................89

# StrmAsgnRptGrp component block

..............................................................................................................................89

# StrmAsgnReqInstrmtGrp component block

.................................................................................................................................90

# StrmAsgnRptInstrmtGrp component block

.................................................................................................................................90

# MARKET DATA REQUEST

.......................................................................................................................................... 91

# MARKET DATA - SNAPSHOT / FULL REFRESH

.................................................................................................................................94

# MARKET DATA - INCREMENTAL REFRESH

.................................................................................................................................97

# MARKET DATA REQUEST REJECT

.....................................................................................................................................99

# STREAM ASSIGNMENT REQUEST

.....................................................................................................................................100

# Example:

.............................................................................................................................................................101

# STREAM ASSIGNMENT REPORT

.......................................................................................................................................102

# STREAM ASSIGNMENT REPORT ACK

.......................................................................................................................................103

# CATEGORY: MARKET STRUCTURE REFERENCE DATA

...............................................................................................................................104

# MARKET STRUCTURE REFERENCE DATA COMPONENT BLOCKS

.............................................................................................................................104

# TrdSessLstGrp component block

...............................................................................................................................104

# MARKET DEFINITION REQUEST

...................................................................................................................................105

# MARKET DEFINITION

........................................................................................................................................106

# MARKET DEFINITION UPDATE REPORT

...................................................................................................................................108

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                    Page 4 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                    August 18, 2011

# TRADING SESSION STATUS REQUEST

...................................................................................................................................................... 110

# TRADING SESSION STATUS

...................................................................................................................................... 111

# TRADING SESSION LIST REQUEST

...........................................................................................................................113

# TRADING SESSION LIST

........................................................................................................................................... 114

# TRADING SESSION LIST UPDATE REPORT

...............................................................................................................................115

# PRODUCT REFERENCE AND MARKET STRUCTURE DATA MODEL

........................................................................... 116

# Overview

.............................................................................................................................................................116

# Message Flow Scenarios

.................................................................................................................................... 119

# Market Structure based Trading Reference data

............................................................................................................................... 120

# CATEGORY: SECURITIES REFERENCE DATA

........................................................................................... 124

# SECURITIES REFERENCE DATA COMPONENT BLOCKS

.............................................................................................124

# SecurityTradingRules component block

.............................................................................................................124

# DerivativeSecurityXML component block

.................................................................................................................124

# InstrmtLegSecListGrp component block

..................................................................................................................125

# RelSymDerivSecGrp component block

..................................................................................................................125

# SecListGrp component block

................................................................................................................................ 126

# SecTypesGrp component block

...........................................................................................................................127

# SecLstUpdRelSymGrp component block

....................................................................................................................127

# SecLstUpdRelSymsLegGrp component block

.................................................................................................................... 129

# DerivativeInstrumentPartySubIDsGrp component block

..................................................................................129

# DerivativeSecurityAltIDGrp component block

.................................................................................................................. 130

# DerivativeEventsGrp component block

..................................................................................................................130

# RelSymDerivSecUpdGrp component block

..................................................................................................................131

# StrikeRules component block

..................................................................................................................132

# MaturityRules component block

..................................................................................................................133

# SecondaryPriceLimits component block

..................................................................................................................134

# TradingSessionRulesGrp component block

.................................................................................................................. 134

# MarketSegmentGrp component block

..................................................................................................................135

# DerivativeSecurityDefinition component block

..................................................................................................................135

# NestedInstrumentAttribute component block

.................................................................................................................. 136

# DerivativeInstrumentAttribute component block

.................................................................................................................. 136

# DerivativeInstrument component block

..................................................................................................................137

# DerivativeInstrumentParties component block

.................................................................................................................. 140

# SECURITY DEFINITION REQUEST

............................................................................................................................. 140

# SECURITY DEFINITION

.............................................................................................................................................143

# SECURITY DEFINITION UPDATE REPORT

................................................................................................................................. 145

# SECURITY TYPE REQUEST

....................................................................................................................................... 146

# SECURITY TYPES

..................................................................................................................................................... 147

# SECURITY LIST REQUEST

........................................................................................................................................ 148

# SECURITY LIST

........................................................................................................................................................ 150

# SECURITY LIST UPDATE REPORT

.............................................................................................................................................................152

# DERIVATIVE SECURITY LIST REQUEST

....................................................................................................................................................154

# DERIVATIVE SECURITY LIST

....................................................................................................................................................156

# DERIVATIVE SECURITY LIST UPDATE REPORT

....................................................................................................................................................157

# SECURITY STATUS REQUEST

....................................................................................................................................................158

# SECURITY STATUS

....................................................................................................................................................159

# SECURITY DEFINITION, SECURITY STATUS, AND TRADING SESSION MESSAGE SCENARIOS

...................................................................................................................161

# Overview

.............................................................................................................................................................161

# Background

.........................................................................................................................................................161

# Definitions

...........................................................................................................................................................161

# Approach

.............................................................................................................................................................162

# Extensions to other messages

............................................................................................................................................................. 162

# Rules

....................................................................................................................................................................162

# Specifying Derivative Trading Strategies using the Security Definition message

..................................................................................................................................................163

# Scenarios

.............................................................................................................................................................164

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                             Page 5 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                            August 18, 2011

# Scenario 1 - Typical use of Security Definition message in placing an Order

164

# Scenario 2 - Inquire Securities Types Available

164

# Scenario 3 – Inquire Common Stocks Available for Trading with Counterparty

165

# Scenario 4 - Inquire all securities traded by a trading party

165

# Scenario 5 – Inquire Option Classes Available for Trading with Counterparty

166

# Scenario 6 - Inquire list of option series for a class

166

# USER DEFINED SPREADS USING SECURITY DEFINITION MESSAGES

# Creating a User Defined Spread - Business Flow

167

# Creating a User Defined Spread - FIX Message Flow

169

# CATEGORY: PARTIES REFERENCE DATA

# INTRODUCTION

171

# PARTIES REFERENCE DATA COMPONENT BLOCKS

# PartyListResponseTypeGrp component block

175

# RequestedPartyRoleGrp component block

175

# PartyRelationships component block

176

# PartyListGrp component block

176

# PartyDetail component block

177

# PartyAltIDs component block

177

# AltPtysSubGrp component block

178

# ContextParties component block

178

# ContextPtysSubGrp component block

178

# RiskLimits component block

179

# RiskInstrumentScope component block

179

# RiskSecAltIDGrp component block

181

# RiskWarningLevels component block

181

# RelatedPartyGrp component block

181

# RelatedPartyDetail component block

182

# RelatedPtysSubGrp component block

182

# RelatedPartyAltIDs component block

183

# RelatedAltPtysSubGrp component block

183

# RelatedContextParties component block

183

# RelatedContextPtysSubGrp component block

184

# RelationshipRiskLimits component block

185

# RelationshipRiskInstrumentScope component block

185

# RelationshipRiskSecAltIDGrp component block

187

# RelationshipRiskWarningLevels component block

187

# PARTY DETAILS LIST REQUEST

188

# PARTY DETAILS LIST REPORT

189

# USAGE OF PARTIES REFERENCE DATA MESSAGES

# Expressing Party Relationships and Querying for Party Relationships

190

# Expressing Risk Limits

193

# Examples

# Trader Party List Example

194

# Customer Account Party List

194

# Trading Firm Party List

196

# APPENDIX 3-A: PRE-TRADE MESSAGE TARGETING/ROUTING

# TARGETING

198

# BLOCKING

198

# OTHER ISSUES

199

© Copyright, 2008- 2009 2011, FIX Protocol, Limited                                     Page 6 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                            August 18, 2011

# FIX APPLICATION MESSAGES: PRE-TRADE

Pre-trade messaging is characterized as messages which are typically communicated prior to the placement of an order.

# The specific FIX pre-trade messaging categories are:

1. INDICATION
2. EVENT COMMUNICATIONS
3. QUOTATION / NEGOTIATION
4. MARKET DATA
5. MARKET STRUCTURE REFERENCE DATA
6. SECURITIES REFERENCE DATA
7. ~~PARTIES REFERENCE DATA~~

Descriptions and formats of the specific FIX pre-trade application messages follow.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                     Page 7 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                        August 18, 2011

# PRE-TRADE COMPONENT BLOCKS

This section lists component blocks commonly used by pre-trade messages defined in this Volume 3 of the FIX specification. Messages may also reference Common Component blocks, which are components used by messages across all the specification volumes. Common Component block definitions can be found in Volume 1 of the specification.

# LegBenchmarkCurveData component block

The LegBenchmarkCurveData is used to convey the benchmark information used for pricing in a multi-legged Fixed Income security.

| Tag | FieldName                 | Req'd | Comments |
| --- | ------------------------- | ----- | -------- |
| 676 | LegBenchmarkCurveCurrency | N     |          |
| 677 | LegBenchmarkCurveName     | N     |          |
| 678 | LegBenchmarkCurvePoint    | N     |          |
| 679 | LegBenchmarkPrice         | N     |          |
| 680 | LegBenchmarkPriceType     | N     |          |

*** = Required status should match "Req'd" setting for <legbenchmarkcurvedata> component block in message definition</legbenchmarkcurvedata>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element LegBnchmkCrvData

# RoutingGrp component block

| Tag | FieldName    | Req'd | Comments                                                                                               |
| --- | ------------ | ----- | ------------------------------------------------------------------------------------------------------ |
| 215 | NoRoutingIDs | N     | Required if any RoutingType and RoutingIDs are specified. Indicates the number within repeating group. |
| 216 | RoutingType  | N     | Indicates type of RoutingID. Required if NoRoutingIDs is > 0.                                          |
| 217 | RoutingID    | N     | Identifies routing destination. Required if NoRoutingIDs is > 0.                                       |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Rtg

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                 Page 8 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011
© Copyright, 2008-2011, FIX Protocol, Limited

Page 9 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# TickRules component block

| Tag  | FieldName           | Req'd | Comments                                                                                                                                                                                                   |
| ---- | ------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1205 | NoTickRules         | N     | Number of tick rules. This block specifies the rules for determining how a security ticks, i.e. the price increments at which it can be quoted and traded, depending on the current price of the security. |
| 1206 | StartTickPriceRange | N     | Starting price range for specified tick increment                                                                                                                                                          |
| 1207 | EndTickPriceRange   | N     | Ending price range for the specified tick increment                                                                                                                                                        |
| 1208 | TickIncrement       | N     | Tick increment for stated price range. Specifies the valid price increments at which a security can be quoted and traded                                                                                   |
| 1209 | TickRuleType        | N     | Specifies the type of tick rule which is being described                                                                                                                                                   |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TickRules

# PriceLimits component block

| Tag  | FieldName             | Req'd | Comments                                                                                                                                                                                                              |
| ---- | --------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1306 | PriceLimitType        | N     | Describes the how the price limits are expressed                                                                                                                                                                      |
| 1148 | LowLimitPrice         | N     | Allowable low limit price for the trading day. A key parameter in validating order price. Used as the lower band for validating order prices. Orders submitted with prices below the lower limit will be rejected     |
| 1149 | HighLimitPrice        | N     | Allowable high limit price for the trading day. A key parameter in validating order price. Used as the upper band for validating order prices. Orders submitted with prices above the upper limit will be rejected    |
| 1150 | TradingReferencePrice | N     | Reference price for the current trading price range usually representing the mid price between the HighLimitPrice and LowLimitPrice. The value may be the settlement price or closing price of the prior trading day. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 10 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

# Refer to FIXML element PxLmts

# MarketDataFeedTypes component block

| Tag  | FieldName     | Req'd | Comments                                                                                                                         |
| ---- | ------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------- |
| 1141 | NoMDFeedTypes | N     | The number of feed types and corresponding book depths associated with a security                                                |
| 1022 | MDFeedType    | N     | Describes a class of service for a given data feed                                                                               |
| 264  | MarketDepth   | N     | The depth of book associated with a particular feed type                                                                         |
| 1021 | MDBookType    | N     | Describes the type of book for which the feed is intended. Can be used when multiple feeds are provided over the same connection |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

# Refer to FIXML element MDFeedTyps

# LotTypeRules component block

| Tag  | FieldName      | Req'd | Comments                                                                                                                                                             |
| ---- | -------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1234 | NoLotTypeRules | N     | Number of Lot Types                                                                                                                                                  |
| 1093 | LotType        | N     | Defines the lot type assigned to the order. Use as an alternate to RoundLot(561). To be used with MinLotSize(1231). LotType + MinLotSize (max is next level minus 1) |
| 1231 | MinLotSize     | N     | Minimum lot size allowed based on lot type specified in LotType(1093)                                                                                                |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

© Copyright, 2008-2009, FIX Protocol, Limited                                             Page 11 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# MatchRules component block

| Tag  | FieldName      | Req'd | Comments                                                                                                                                                                                  |
| ---- | -------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1235 | NoMatchRules   | N     | Number of match rules                                                                                                                                                                     |
| 1142 | MatchAlgorithm | N     | The type of algorithm used to match orders in a specific security on an electronic trading platform. Possible values are FIFO, Allocation, Pro-rata, Lead Market Maker, Currency Calendar |
| 574  | MatchType      | N     | The point in the matching process at which this trade was matched.                                                                                                                        |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element MtchRules

# ExecInstRules component block

| Tag  | FieldName       | Req'd | Comments                                                                         |
| ---- | --------------- | ----- | -------------------------------------------------------------------------------- |
| 1232 | NoExecInstRules | N     | Number of execution instructions                                                 |
| 1308 | ExecInstValue   | N     | Indicates execution instructions that are valid for the specified market segment |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element ExecInstRules

© Copyright, 2008-2009, FIX Protocol, Limited                                     Page 12 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                      August 18, 2011

# TimeInForceRules component block

| Tag  | FieldName          | Req'd | Comments                                                                           |
| ---- | ------------------ | ----- | ---------------------------------------------------------------------------------- |
| 1239 | NoTimeInForceRules | N     | Number of time in force techniques                                                 |
| 59   | TimeInForce        | N     | Indicates time in force techniques that are valid for the specified market segment |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TmInForceRules

# OrdTypeRules component block

| Tag  | FieldName      | Req'd | Comments                                                               |
| ---- | -------------- | ----- | ---------------------------------------------------------------------- |
| 1237 | NoOrdTypeRules | N     | Number of order types                                                  |
| 40   | OrdType        | N     | Indicates order types that are valid for the specified market segment. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element OrdTypRules

© Copyright, 2008-2009, FIX Protocol, Limited                               Page 13 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                 August 18, 2011

# TradingSessionRules component block

| Tag             | FieldName | Req'd | Comments                                                                                                                                                                                        |
| --------------- | --------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| component block |           | N     | Specifies the order types that are valid for trading. The scope of the rule is determined by the context in which the component is used. In this case, the scope is trading session.            |
| component block |           | N     | Specifies the time in force rules that are valid for trading. The scope of the rule is determined by the context in which the component is used. In this case, the scope is trading session.    |
| component block |           | N     | Specifies the execution instructions that are valid for trading. The scope of the rule is determined by the context in which the component is used. In this case, the scope is trading session. |
| component block |           | N     | Specifies the matching rules that are valid for trading. The scope of the rule is determined by the context in which the component is used. In this case, the scope is trading session.         |
| component block |           | N     | Specifies the market data feed types that are valid for trading. The scope of the rule is determined by the context in which the component is used. In this case, the scope is trading session. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TrdgSesRules

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                         Page 14 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                 August 18, 2011

# BaseTradingRules component block

| Tag             | FieldName              | Req'd | Comments                                                                                                                                                                                  |
| --------------- | ---------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| component block | \<TickRules>           | N     | This block specifies the rules for determining how a security ticks, i.e. the price increments at which it can be quoted and traded, depending on the current price of the security       |
| component block | \<LotTypeRules>        | N     | Specifies the lot types that are valid for trading.                                                                                                                                       |
| component block | \<PriceLimits>         | N     | Specifies the price limits that are valid for trading.                                                                                                                                    |
| 827             | ExpirationCycle        | N     |                                                                                                                                                                                           |
| 562             | MinTradeVol            | N     | The minimum order quantity that can be submitted for an order.                                                                                                                            |
| 1140            | MaxTradeVol            | N     | The maximum order quantity that can be submitted for a security. For listed derivatives this indicates the minimum quantity necessary for an order or trade to qualify as a block trade   |
| 1143            | MaxPriceVariation      | N     | The maximum price variation of an execution from one event to the next for a given security. Expressed in absolute price terms.                                                           |
| 1144            | ImpliedMarketIndicator | N     |                                                                                                                                                                                           |
| 1245            | TradingCurrency        | N     | Used when the trading currency can differ from the price currency                                                                                                                         |
| 561             | RoundLot               | N     | Trading lot size of security                                                                                                                                                              |
| 1377            | MultilegModel          | N     | Used for multileg security only. Defines whether the security is pre-defined or user-defined. Not that value = 2 (User-defined, Non-Securitized, Multileg) does not apply for Securities. |
| 1378            | MultilegPriceMethod    | N     | Used for multileg security only. Defines the method used when applying the multileg price to the legs.                                                                                    |
| 423             | PriceType              | N     | Defines the default Price Type used for trading.                                                                                                                                          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element BaseTrdgRules

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                         Page 15 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011

# CATEGORY: INDICATION

# Indication Component Blocks

This section lists the component blocks used exclusively by the messages defined for Indication.

# InstrmtLegIOIGrp component block

| Tag | FieldName          |
| --- | ------------------ |
| 555 | NoLegs             |
| £   | component block    |
|     | \<InstrumentLeg>   |
| £   | 682                |
|     | LegIOIQty          |
| £   | component block    |
|     | \<LegStipulations> |

| Req'd | Comments                                       |
| ----- | ---------------------------------------------- |
| N     | Required for multileg IOIs                     |
| N     | Required for multileg IOIs                     |
|       | For Swaps one leg is Buy and other leg is Sell |
| N     | Required for multileg IOIs and for each leg.   |
| N     |                                                |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element IOI

# IOIQualGrp component block

| Tag | FieldName       | Req'd | Comments                                                                                      |
| --- | --------------- | ----- | --------------------------------------------------------------------------------------------- |
| 199 | NoIOIQualifiers | N     | Required if any IOIQualifiers are specified. Indicates the number of repeating IOIQualifiers. |
| £   | 104             | N     | Required if NoIOIQualifiers > 0                                                               |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Qual

© Copyright, 2008-2009, FIX Protocol, Limited
Page 16 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011

Advertisements

Advertisement messages are used to announce completed transactions. The advertisement message can be transmitted in various transaction types; NEW, CANCEL and REPLACE. All message types other than NEW modify the state of a previously transmitted advertisement identified in AdvRefID. The advertisement message format is as follows:

| Tag                              | FieldName           | Req'd | Comments                                                                                                                       |
| -------------------------------- | ------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader                   |                     | Y     | MsgType = 7                                                                                                                    |
| 2                                | AdvId               | Y     |                                                                                                                                |
| 5                                | AdvTransType        | Y     |                                                                                                                                |
| 3                                | AdvRefID            | N     | Required for Cancel and Replace AdvTransType messages                                                                          |
| component block \<Instrument>    |                     | Y     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages"                  |
| component block \<InstrmtLegGrp> |                     | N     | Number of legs Identifies a Multi-leg Execution if present and non-zero.                                                       |
| component block \<UndInstrmtGrp> |                     | N     | Number of underlyings                                                                                                          |
| 4                                | AdvSide             | Y     |                                                                                                                                |
| 53                               | Quantity            | Y     |                                                                                                                                |
| 854                              | QtyType             | N     |                                                                                                                                |
| 44                               | Price               | N     |                                                                                                                                |
| 15                               | Currency            | N     |                                                                                                                                |
| 75                               | TradeDate           | N     |                                                                                                                                |
| 60                               | TransactTime        | N     |                                                                                                                                |
| 58                               | Text                | N     |                                                                                                                                |
| 354                              | EncodedTextLen      | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355                              | EncodedText         | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
| 149                              | URLLink             | N     | A URL (Uniform Resource Locator) link to additional information (i.e. http\://www\.XYZ.com/research.html)                      |
| 30                               | LastMkt             | N     |                                                                                                                                |
| 336                              | TradingSessionID    | N     |                                                                                                                                |
| 625                              | TradingSessionSubID | N     |                                                                                                                                |
| StandardTrailer                  |                     | Y     |                                                                                                                                |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited

Page 17 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011

# FIXML Definition for this message

– see http://www.fixprotocol.org for details

Refer to FIXML element Adv

© Copyright, 2008-2011, FIX Protocol, Limited
Page 18 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Indications of Interest

Indication of interest messages are used to market merchandise which the broker is buying or selling in either a proprietary or agency capacity. The indications can be time bound with a specific expiration value. Indications are distributed with the understanding that other firms may react to the message first and that the merchandise may no longer be available due to prior trade. Indication messages can be transmitted in various transaction types; NEW, CANCEL, and REPLACE. All message types other than NEW modify the state of the message identified in IOIRefID.

The indication of interest message format is as follows:

| Tag             | FieldName                  | Req'd | Comments                                                                                                                                                                |
| --------------- | -------------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                            | Y     | MsgType = 6                                                                                                                                                             |
| component block | ApplicationSequenceControl |       |                                                                                                                                                                         |
| 23              | IOIID                      | Y     |                                                                                                                                                                         |
| 28              | IOITransType               | Y     |                                                                                                                                                                         |
| 26              | IOIRefID                   | N     | Required for Cancel and Replace IOITransType messages                                                                                                                   |
| component block | Instrument                 | Y     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages"                                                           |
| component block | Parties                    | N     | Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages".                                                   |
| component block | FinancingDetails           | N     | Insert here the set of "FinancingDetails" (symbology) fields defined in "Common Components of Application Messages"                                                     |
| component block | UndInstrmtGrp              | N     | Number of underlyings                                                                                                                                                   |
| 54              | Side                       | Y     | Side of Indication Valid subset of values: 1 = Buy, 2 = Sell, 7 = Undisclosed (for IOIs), B = As Defined (for multilegs), C = Opposite (for multilegs)                  |
| 854             | QtyType                    | N     |                                                                                                                                                                         |
| component block | OrderQtyData               | N     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages". The value zero is used if NoLegs repeating group is used |

© Copyright, 2008- 2009 2011, FIX Protocol, Limited Page 19 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

Applicable if needed to express CashOrder Qty (tag 152)

| 27                                                                                                                                                                           | IOIQty         | Y | The value zero is used if NoLegs repeating group is used                                                                       |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- | - | ------------------------------------------------------------------------------------------------------------------------------ |
| 15                                                                                                                                                                           | Currency       | N |                                                                                                                                |
| component block \<Stipulations> N Insert here the set of "Stipulations" (symbology) fields defined in "Common Components of Application Messages"                            |                |   |                                                                                                                                |
| component block N Required for multileg IOIs \<InstrmtLegIOIGrp>                                                                                                             |                |   |                                                                                                                                |
| 423                                                                                                                                                                          | PriceType      | N |                                                                                                                                |
| 44                                                                                                                                                                           | Price          | N |                                                                                                                                |
| 62                                                                                                                                                                           | ValidUntilTime | N |                                                                                                                                |
| 25                                                                                                                                                                           | IOIQltyInd     | N |                                                                                                                                |
| 130                                                                                                                                                                          | IOINaturalFlag | N |                                                                                                                                |
| component block \<IOIQualGrp> N Required if any IOIQualifiers are specified. Indicates the number of repeating IOIQualifiers.                                                |                |   |                                                                                                                                |
| 58                                                                                                                                                                           | Text           | N |                                                                                                                                |
| 354                                                                                                                                                                          | EncodedTextLen | N | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355                                                                                                                                                                          | EncodedText    | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
| 60                                                                                                                                                                           | TransactTime   | N |                                                                                                                                |
| 149                                                                                                                                                                          | URLLink        | N | A URL (Uniform Resource Locator) link to additional information (i.e. http\://www\.XYZ.com/research.html)                      |
| component block \<RoutingGrp> N Required if any RoutingType and RoutingIDs are specified. Indicates the number within repeating group.                                       |                |   |                                                                                                                                |
| component block N Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "Common Components of Application Messages" |                |   |                                                                                                                                |
| component block \<YieldData> N                                                                                                                                               |                |   |                                                                                                                                |
| StandardTrailer                                                                                                                                                              | Y              |   |                                                                                                                                |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element IOI

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                          Page 20 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011

# CATEGORY: EVENT COMMUNICATION

# Event Communication Component Blocks

This section lists the component blocks used exclusively by the messages defined for Event Communication.

# LinesOfTextGrp component block

| Tag | FieldName      | Req'd | Comments                                                                                                                       |
| --- | -------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
| 33  | NoLinesOfText  | Y     | Specifies the number of repeating lines of text specified                                                                      |
| 58  | Text           | Y     | Repeating field, number of instances defined in LinesOfText                                                                    |
| 354 | EncodedTextLen | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355 | EncodedText    | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TxtLn

# NewsREfGrp component block

| Tag  | FieldName    | Req'd | Comments                                                        |
| ---- | ------------ | ----- | --------------------------------------------------------------- |
| 1475 | NoNewsRefIDs | N     | Number of news item references                                  |
| 1476 | NewsRefID    | N     | Required if NoNewsRefIDs(2144) > 0. News item being referenced. |
| 1477 | NewsRefType  | N     | Type of reference.                                              |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Refs

© Copyright, 2008-2009, FIX Protocol, Limited

Page 21 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                 August 18, 2011

# News

The news message is a general free format message between the broker and institution. The message contains
flags to identify the news item's urgency and to allow sorting by subject company (symbol). The News
message can be originated at either the broker or institution side, or exchanges and other marketplace venues.
The news message also provides the capability to support categorization of news being published. This allows
the news to be filtered by the news consumer. For example:

- Exchanges may need to provide the MarketID (1301) and MarketSegmentID (1302) so users can filter
News to the segments that are of relevance for them.
- In multi-lingual environments, news may be published in a variety of languages; a user should be able
to filter out messages in irrelevant languages.
- By providing a categorization of the News messages, users can choose how to render them in different
GUIs or ignore certain categories altogether.

Additionally the news message allows news to reference other news messages. When a message references
another one, it may also need to provide the reason for the reference - e.g. an update of the previous message, a
complement or simply that it is a version in another language.

# The news message format is as follows:

| Tag             | FieldName                  | Req'd | Comments                                                                                                                           |
| --------------- | -------------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                            | Y     | MsgType = B                                                                                                                        |
| component block | ApplicationSequenceControl |       |                                                                                                                                    |
| 1472            | NewsID                     | N     | Unique identifier for News message                                                                                                 |
| component block | NewsRefGrp                 | N     | News items referenced by this News message                                                                                         |
| 1473            | NewsCategory               | N     |                                                                                                                                    |
| 1474            | LanguageCode               | N     | Used to optionally specify the national language used for the News item.                                                           |
| 42              | OrigTime                   | N     |                                                                                                                                    |
| 61              | Urgency                    | N     |                                                                                                                                    |
| 148             | Headline                   | Y     | Specifies the headline text                                                                                                        |
| 358             | EncodedHeadlineLen         | N     | Must be set if EncodedHeadline field is specified and must immediately precede it.                                                 |
| 359             | EncodedHeadline            | N     | Encoded (non-ASCII characters) representation of the Headline field in the encoded format specified via the MessageEncoding field. |
| component block | RoutingGrp                 | N     | Required if any RoutingType and RoutingIDs are specified. Indicates the number within repeating group.                             |
| 1301            | MarketID                   | N     | Used to optionally specify the market to which this News applies.                                                                  |
| 1300            | MarketSegmentID            | N     | Used to optionally specify the market segment to which this News applies.                                                          |
| component block | InstrmtGrp                 | N     | Specifies the number of repeating symbols (instruments)                                                                            |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                       Page 22 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

# Errata

| Component Block   | Required | Description                                                                                               |
| ----------------- | -------- | --------------------------------------------------------------------------------------------------------- |
| \<InstrmtLegGrp>  | N        | Number of legs Identifies a Multi-leg Execution if present and non-zero.                                  |
| \<UndInstrmtGrp>  | N        | Number of underlyings                                                                                     |
| \<LinesOfTextGrp> | Y        | Specifies the number of repeating lines of text specified                                                 |
| URLLink           | N        | A URL (Uniform Resource Locator) link to additional information (i.e. http\://www\.XYZ.com/research.html) |
| RawDataLength     | N        |                                                                                                           |
| RawData           | N        |                                                                                                           |
| StandardTrailer   | Y        |                                                                                                           |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element News

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited
Page 23 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011

# Email

The email message is similar to the format and purpose of the News message, however, it is intended for private use between two parties.

The email message format is as follows:

| Tag                               | FieldName         | Req'd | Comments                                                                                                                          |
| --------------------------------- | ----------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                    |                   | Y     | MsgType = C                                                                                                                       |
| 164                               | EmailThreadID     | Y     | Unique identifier for the email message thread                                                                                    |
| 94                                | EmailType         | Y     |                                                                                                                                   |
| 42                                | OrigTime          | N     |                                                                                                                                   |
| 147                               | Subject           | Y     | Specifies the Subject text                                                                                                        |
| 356                               | EncodedSubjectLen | N     | Must be set if EncodedSubject field is specified and must immediately precede it.                                                 |
| 357                               | EncodedSubject    | N     | Encoded (non-ASCII characters) representation of the Subject field in the encoded format specified via the MessageEncoding field. |
| component block \<RoutingGrp>     |                   | N     | Required if any RoutingType and RoutingIDs are specified. Indicates the number within repeating group.                            |
| component block \<InstrmtGrp>     |                   | N     | Specifies the number of repeating symbols (instruments) specified                                                                 |
| component block \<UndInstrmtGrp>  |                   | N     | Number of underlyings                                                                                                             |
| component block \<InstrmtLegGrp>  |                   | N     | Number of legs Identifies a Multi-leg Execution if present and non-zero.                                                          |
| 37                                | OrderID           | N     |                                                                                                                                   |
| 11                                | ClOrdID           | N     |                                                                                                                                   |
| component block \<LinesOfTextGrp> |                   | Y     | Specifies the number of repeating lines of text specified                                                                         |
| 95                                | RawDataLength     | N     |                                                                                                                                   |
| 96                                | RawData           | N     |                                                                                                                                   |
| StandardTrailer                   |                   | Y     |                                                                                                                                   |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element Email

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited

Page 24 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011

# CATEGORY: QUOTATION / NEGOTIATION

The quotation messages fall into two main sub-categories – those used for quoting in single instruments ‘Single product quoting’ and those used to quote on multiple instruments such as option series - ‘Mass quoting’. Within the ‘single product quoting’ suite of messages three business models have been identified:

# 1. Indicative quoting

The predominant business model for retail quoting, where the expected response to a quote is a ‘previously quoted’ order which may be accepted or rejected. In the retail model the quote may be preceded by a Quote Request.

# 2. Tradeable quoting

A model where the response to a quote may be an execution (rather than an order). A common model where participants are posting quotes to an exchange. Quote may be issued in response to a Quote Request in a ‘quote on demand’ market.

# 3. Restricted Tradeable quoting

As per Tradeable quoting but the response to a quote may be either an execution or an order depending on various parameters.

The Negotiation (a.k.a. counter quoting) dialog is also supported. The Negotiation dialog may begin with either an indicative quote or a tradeable quote. For specific usage guidance for Fixed Income and Exchange/Marketplace negotiation and counter quotes using the quotation messages, see Volume 7 – PRODUCT: FIXED INCOME and USER GROUP: EXCHANGES AND MARKETS respectively. The common thread linking the models is the use of the Quote message.

# Quotation / Negotiation Component Blocks

This section lists the component blocks used exclusively by the messages defined for Quotation / Negotiation.

| Tag | FieldName          | Req'd        | Comments                                                                                                                                                                                                               |
| --- | ------------------ | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 555 | NoLegs             | N            | Required for multileg quotes                                                                                                                                                                                           |
| £   | component block    | N            | Required for multileg quotes                                                                                                                                                                                           |
|     | \<InstrumentLeg>   |              | For Swaps one leg is Buy and other leg is Sell                                                                                                                                                                         |
| £   | 687                | LegQty       | N (Deprecated in FIX.5.0)                                                                                                                                                                                              |
| £   | 685                | LegOrderQty  | N                                                                                                                                                                                                                      |
|     |                    |              | When reporting an Execution, LegOrderQty may be used on Execution Report to echo back original LegOrderQty submission. This field should be used to specify OrderQty at the leg level rather than LegQty (deprecated). |
| £   | 690                | LegSwapType  | N                                                                                                                                                                                                                      |
| £   | 587                | LegSettlType | N                                                                                                                                                                                                                      |
| £   | 588                | LegSettlDate | N                                                                                                                                                                                                                      |
| £   | component block    | N            |                                                                                                                                                                                                                        |
|     | \<LegStipulations> |              |                                                                                                                                                                                                                        |
| £   | component block    | N            |                                                                                                                                                                                                                        |
|     | \<NestedParties>   |              |                                                                                                                                                                                                                        |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                          Page 25 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

| £ | 686             | LegPriceType         | N | Code to represent type of price presented in LegBidPx and LegOfferPx. Required if LegBidPx or PegOfferPx is present. |
| - | --------------- | -------------------- | - | -------------------------------------------------------------------------------------------------------------------- |
| £ | 681             | LegBidPx             | N |                                                                                                                      |
| £ | 684             | LegOfferPx           | N |                                                                                                                      |
| £ | component block |                      | N | \<LegBenchmarkCurveData>                                                                                             |
| £ | 654             | LegRefID             | N | Initiator can optionally provide a unique identifier for the specific leg. Required for FX Swaps                     |
| £ | 1067            | LegBidForwardPoints  | N |                                                                                                                      |
| £ | 1068            | LegOfferForwardPoint | N |                                                                                                                      |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Quot

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 26 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011

# LegQuotStatGrp component block

| Tag | FieldName          | Req'd        | Comments                                                                                                                                                                                                                 |
| --- | ------------------ | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 555 | NoLegs             | N            | Required for multileg quote status reports                                                                                                                                                                               |
| £   | component block    | N            | Required for multileg quote status reports                                                                                                                                                                               |
|     | \<InstrumentLeg>   |              | For Swaps one leg is Buy and other leg is Sell                                                                                                                                                                           |
| £   | 687                | LegQty       | N (Deprecated in FIX.5.0)                                                                                                                                                                                                |
| £   | 685                | LegOrderQty  | N When reporting an Execution, LegOrderQty may be used on Execution Report to echo back original LegOrderQty submission. This field should be used to specify OrderQty at the leg level rather than LegQty (deprecated). |
| £   | 690                | LegSwapType  | N                                                                                                                                                                                                                        |
| £   | 587                | LegSettlType | N                                                                                                                                                                                                                        |
| £   | 588                | LegSettlDate | N                                                                                                                                                                                                                        |
| £   | component block    | N            |                                                                                                                                                                                                                          |
|     | \<LegStipulations> |              |                                                                                                                                                                                                                          |
| £   | component block    | N            |                                                                                                                                                                                                                          |
|     | \<NestedParties>   |              |                                                                                                                                                                                                                          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element QuoteStat

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 27 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                            August 18, 2011

# QuotCxlEntriesGrp component block

| Tag | FieldName       | Req'd | Comments                                                                                                            |
| --- | --------------- | ----- | ------------------------------------------------------------------------------------------------------------------- |
| 295 | NoQuoteEntries  | N     | The number of securities (instruments) whose quotes are to be canceled Not required when cancelling all quotes.     |
| £   | component block | N     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages"       |
| £   | component block | N     | Insert here the set of "FinancingDetails" (symbology) fields defined in "Common Components of Application Messages" |
| £   | component block | N     |                                                                                                                     |
| £   | component block | N     |                                                                                                                     |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element QuotCxlEntry

# QuoteEntryAckGrp component block

| Tag | FieldName       | Req'd        | Comments                                                                                                      |                                                                                                                                                                 |
| --- | --------------- | ------------ | ------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 295 | NoQuoteEntries  | N            | The number of quotes for this Symbol (QuoteSet) that follow in this message.                                  |                                                                                                                                                                 |
| £   | 299             | QuoteEntryID | N                                                                                                             | Uniquely identifies the quote across the complete set of all quotes for a given quote provider. First field in repeating group. Required if NoQuoteEntries > 0. |
| £   | component block | N            | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" |                                                                                                                                                                 |
| £   | component block | N            |                                                                                                               |                                                                                                                                                                 |
| £   | 132             | BidPx        | N                                                                                                             | If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified.                   |
| £   | 133             | OfferPx      | N                                                                                                             | If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx,                                                      |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                           Page 28 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

| £ | 134  | BidSize              | N |                                                                                                                                        |   |
| - | ---- | -------------------- | - | -------------------------------------------------------------------------------------------------------------------------------------- | - |
| £ | 135  | OfferSize            | N |                                                                                                                                        |   |
| £ | 62   | ValidUntilTime       | N |                                                                                                                                        |   |
| £ | 188  | BidSpotRate          | N | May be applicable for F/X quotes                                                                                                       |   |
| £ | 190  | OfferSpotRate        | N | May be applicable for F/X quotes                                                                                                       |   |
| £ | 189  | BidForwardPoints     | N | May be applicable for F/X quotes                                                                                                       |   |
| £ | 191  | OfferForwardPoints   | N | May be applicable for F/X quotes                                                                                                       |   |
| £ | 631  | MidPx                | N |                                                                                                                                        |   |
| £ | 632  | BidYield             | N |                                                                                                                                        |   |
| £ | 633  | MidYield             | N |                                                                                                                                        |   |
| £ | 634  | OfferYield           | N |                                                                                                                                        |   |
| £ | 60   | TransactTime         | N |                                                                                                                                        |   |
| £ | 336  | TradingSessionID     | N |                                                                                                                                        |   |
| £ | 625  | TradingSessionSubID  | N |                                                                                                                                        |   |
| £ | 64   | SettlDate            | N | Can be used with forex quotes to specify a specific "value date"                                                                       |   |
| £ | 40   | OrdType              | N | Can be used to specify the type of order the quote is for                                                                              |   |
| £ | 193  | SettlDate2           | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.    |   |
| £ | 192  | OrderQty2            | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.  |   |
| £ | 642  | BidForwardPoints2    | N | (Deprecated in FIX.5.0) Bid F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value   |   |
| £ | 643  | OfferForwardPoints2  | N | (Deprecated in FIX.5.0) Offer F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value |   |
| £ | 15   | Currency             | N | Can be used to specify the currency of the quoted price.                                                                               |   |
| £ | 775  | BookingType          | N |                                                                                                                                        |   |
| £ | 528  | OrderCapacity        | N |                                                                                                                                        |   |
| £ | 529  | OrderRestrictions    | N |                                                                                                                                        |   |
| £ | 1167 | QuoteEntryStatus     | N |                                                                                                                                        |   |
| £ | 368  | QuoteEntryRejectReas | N | Reason Quote Entry was rejected.                                                                                                       |   |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 29 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# FIXML Definition for this Component Block

Refer to FIXML element QuotEntryAck

# QuotEntryGrp component block

| Tag                                                    | FieldName           | Req'd | Comments                                                                                                                                      |
| ------------------------------------------------------ | ------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 295                                                    | NoQuoteEntries      | Y     | The number of quotes for this Symbol (instrument) (QuoteSet) that follow in this message.                                                     |
| 299                                                    | QuoteEntryID        | Y     | Uniquely identifies the quote across the complete set of all quotes for a given quote provider.                                               |
| component block                                        |                     |       |                                                                                                                                               |
| \<Instrument>                                          |                     |       |                                                                                                                                               |
| defined in "Common Components of Application Messages" |                     |       |                                                                                                                                               |
| component block                                        |                     |       |                                                                                                                                               |
| \<InstrmtLegGrp>                                       |                     |       |                                                                                                                                               |
| 132                                                    | BidPx               | N     | If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified. |
| 133                                                    | OfferPx             | N     | If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified. |
| 134                                                    | BidSize             | N     |                                                                                                                                               |
| 135                                                    | OfferSize           | N     |                                                                                                                                               |
| 62                                                     | ValidUntilTime      | N     |                                                                                                                                               |
| 188                                                    | BidSpotRate         | N     | May be applicable for F/X quotes                                                                                                              |
| 190                                                    | OfferSpotRate       | N     | May be applicable for F/X quotes                                                                                                              |
| 189                                                    | BidForwardPoints    | N     | May be applicable for F/X quotes                                                                                                              |
| 191                                                    | OfferForwardPoints  | N     | May be applicable for F/X quotes                                                                                                              |
| 631                                                    | MidPx               | N     |                                                                                                                                               |
| 632                                                    | BidYield            | N     |                                                                                                                                               |
| 633                                                    | MidYield            | N     |                                                                                                                                               |
| 634                                                    | OfferYield          | N     |                                                                                                                                               |
| 60                                                     | TransactTime        | N     |                                                                                                                                               |
| 336                                                    | TradingSessionID    | N     |                                                                                                                                               |
| 625                                                    | TradingSessionSubID | N     |                                                                                                                                               |
| 64                                                     | SettlDate           | N     | Can be used with forex quotes to specify a specific "value date"                                                                              |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 30 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

# Errata

| Tag | FieldName           | Req'd | Comments                                                                                                                               |
| --- | ------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------- |
| 40  | OrdType             | N     | Can be used to specify the type of order the quote is for                                                                              |
| 193 | SettlDate2          | N     | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.    |
| 192 | OrderQty2           | N     | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.  |
| 642 | BidForwardPoints2   | N     | (Deprecated in FIX.5.0) Bid F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value   |
| 643 | OfferForwardPoints2 | N     | (Deprecated in FIX.5.0) Offer F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value |
| 15  | Currency            | N     | Can be used to specify the currency of the quoted price.                                                                               |
| 775 | BookingType         | N     |                                                                                                                                        |
| 528 | OrderCapacity       | N     |                                                                                                                                        |
| 529 | OrderRestrictions   | N     |                                                                                                                                        |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element QuotEntry

# QuotQualGrp component block

| Tag | FieldName         | Req'd | Comments                          |
| --- | ----------------- | ----- | --------------------------------- |
| 735 | NoQuoteQualifiers | N     |                                   |
| 695 | QuoteQualifier    | N     | Required if NoQuoteQualifiers > 1 |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element QuotQual

© Copyright, 2008-2009, FIX Protocol, Limited                                   Page 31 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# QuotReqGrp component block

| Tag | FieldName            | Req'd | Comments                                                                                                                                                                                                                                                                                                                                       |
| --- | -------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 146 | NoRelatedSym         | Y     | Number of related symbols (instruments) in Request                                                                                                                                                                                                                                                                                             |
| £   | component block      | Y     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages"                                                                                                                                                                                                                                  |
| £   | component block      | N     | Insert here the set of "FinancingDetails" (symbology) fields defined in "Common Components of Application Messages"                                                                                                                                                                                                                            |
| £   | component block      | N     |                                                                                                                                                                                                                                                                                                                                                |
| £   | UndInstrmtGrp        |       |                                                                                                                                                                                                                                                                                                                                                |
| 140 | PrevClosePx          | N     | Useful for verifying security identification                                                                                                                                                                                                                                                                                                   |
| 303 | QuoteRequestType     | N     | Indicates the type of Quote Request (e.g. Manual vs. Automatic) being generated.                                                                                                                                                                                                                                                               |
| 537 | QuoteType            | N     | Type of quote being requested from counterparty or market (e.g. Indicative, Firm, or Restricted Tradeable) Valid values used by FX in the request: 0 = Indicative, 1 = Tradeable; Absence implies a request for an indicative quote.                                                                                                           |
| 336 | TradingSessionID     | N     |                                                                                                                                                                                                                                                                                                                                                |
| 625 | TradingSessionSubID  | N     |                                                                                                                                                                                                                                                                                                                                                |
| 229 | TradeOriginationDate | N     |                                                                                                                                                                                                                                                                                                                                                |
| 54  | Side                 | N     | If OrdType = "Forex - Swap", should be the side of the future portion of a F/X swap. The absence of a side implies that a two-sided quote is being requested. For single instrument use. FX values, 1 = Buy, 2 = Sell; This is from the perspective of the Initiator. If absent then a two-sided quote is being requested for spot or forward. |
| 854 | QtyType              | N     | Type of quantity specified in a quantity field. For FX, if used, should be "0".                                                                                                                                                                                                                                                                |
| £   | component block      | N     | Required for single instrument quoting.                                                                                                                                                                                                                                                                                                        |
| £   | OrderQtyData         |       | Required for Fixed Income if QuoteType is Tradeable.                                                                                                                                                                                                                                                                                           |
| 110 | MinQty               | N     |                                                                                                                                                                                                                                                                                                                                                |
| 63  | SettlType            | N     | For NDFs either SettlType (specifying the tenor) or SettlDate must be specified.                                                                                                                                                                                                                                                               |
| 64  | SettlDate            | N     | Can be used (e.g. with forex quotes) to specify the desired "value date". For NDFs either SettlType (specifying the tenor) or SettlDate must be specified.                                                                                                                                                                                     |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 32 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                              August 18, 2011

# Errata

| £ | 193             | SettlDate2     | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.                        |
| - | --------------- | -------------- | - | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| £ | 192             | OrderQty2      | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.                      |
| £ | 15              | Currency       | N | Can be used to specify the desired currency of the quoted price. May differ from the 'normal' trading currency of the instrument being quote requested.    |
| £ | 120             | SettlCurrency  | N | Required for NDFs to specify the settlement currency (fixing currency).                                                                                    |
| £ | component block |                | N | \<RateSource>                                                                                                                                              |
| £ | component block |                | N | Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "Common Components of Application Messages"         |
| £ | 1               | Account        | N |                                                                                                                                                            |
| £ | 660             | AcctIDSource   | N |                                                                                                                                                            |
| £ | 581             | AccountType    | N |                                                                                                                                                            |
| £ | component block |                | N | \<QuotReqLegsGrp>                                                                                                                                          |
| £ | component block |                | N | \<QuotQualGrp>                                                                                                                                             |
| £ | 692             | QuotePriceType | N | Initiator can specify the price type the quote needs to be quoted at. If not specified, the Respondent has option to specify how quote is quoted.          |
| £ | 40              | OrdType        | N | Can be used to specify the type of order the quote request is for.                                                                                         |
| £ | 62              | ValidUntilTime | N | Used by the quote initiator to indicate the period of time the resulting Quote must be valid until.                                                        |
| £ | 126             | ExpireTime     | N | The time when Quote Request will expire.                                                                                                                   |
| £ | 60              | TransactTime   | N | Time transaction was entered.                                                                                                                              |
| £ | component block |                | N | Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "Common Components of Application Messages" |
| £ | 423             | PriceType      | N |                                                                                                                                                            |
| £ | 44              | Price          | N | Quoted or target price.                                                                                                                                    |
| £ | 640             | Price2         | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the Quoted or target price for the future portion of a F/X swap.              |
| £ | component block |                | N | Insert here the set of "YieldData" (yield-related) fields defined in "Common Components of Application Messages"                                           |

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                      Page 33 of 200


---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011

# FIXML Definition for this Component Block

Refer to FIXML element QuotReq

# QuotReqLegsGrp component block

| Tag | FieldName                | Req'd | Comments                                                                                                                                                                                                               |
| --- | ------------------------ | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 555 | NoLegs                   | N     | Required for multileg quotes.                                                                                                                                                                                          |
|     | component block          | N     | Required for multileg quotes                                                                                                                                                                                           |
|     | \<InstrumentLeg>         |       | For Swaps one leg is Buy and other leg is Sell                                                                                                                                                                         |
| 687 | LegQty                   | N     | (Deprecated in FIX.5.0)                                                                                                                                                                                                |
| 685 | LegOrderQty              | N     | When reporting an Execution, LegOrderQty may be used on Execution Report to echo back original LegOrderQty submission. This field should be used to specify OrderQty at the leg level rather than LegQty (deprecated). |
| 690 | LegSwapType              | N     |                                                                                                                                                                                                                        |
| 587 | LegSettlType             | N     |                                                                                                                                                                                                                        |
| 588 | LegSettlDate             | N     |                                                                                                                                                                                                                        |
|     | component block          | N     |                                                                                                                                                                                                                        |
|     | \<LegStipulations>       |       |                                                                                                                                                                                                                        |
|     | component block          | N     |                                                                                                                                                                                                                        |
|     | \<NestedParties>         |       |                                                                                                                                                                                                                        |
|     | component block          | N     |                                                                                                                                                                                                                        |
|     | \<LegBenchmarkCurveData> |       |                                                                                                                                                                                                                        |
| 654 | LegRefID                 | N     | Initiator can optionally provide a unique identifier for the specific leg.                                                                                                                                             |

# FIXML Definition for this Component Block

Refer to FIXML element Leg

© Copyright, 2008-2009, FIX Protocol, Limited
Page 34 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# QuotReqRjctGrp component block

| Tag | FieldName       | Req'd                | Comments                                                                                                                                                         |                                                                                                                                                                                                               |
| --- | --------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 146 | NoRelatedSym    | Y                    | Number of related symbols (instruments) in Request                                                                                                               |                                                                                                                                                                                                               |
| £   | component block | Y                    | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages"                                                    |                                                                                                                                                                                                               |
| £   | component block | N                    | Insert here the set of "FinancingDetails" (symbology) fields defined in "Common Components of Application Messages"                                              |                                                                                                                                                                                                               |
| £   | component block | N                    |                                                                                                                                                                  |                                                                                                                                                                                                               |
| £   |                 |                      |                                                                                                                                                                  |                                                                                                                                                                                                               |
| £   | 140             | PrevClosePx          | N                                                                                                                                                                | Useful for verifying security identification                                                                                                                                                                  |
| £   | 303             | QuoteRequestType     | N                                                                                                                                                                | Indicates the type of Quote Request (e.g. Manual vs. Automatic) being generated.                                                                                                                              |
| £   | 537             | QuoteType            | N                                                                                                                                                                | Type of quote being requested from counterparty or market (e.g. Indicative, Firm, or Restricted Tradeable)                                                                                                    |
| £   | 336             | TradingSessionID     | N                                                                                                                                                                |                                                                                                                                                                                                               |
| £   | 625             | TradingSessionSubID  | N                                                                                                                                                                |                                                                                                                                                                                                               |
| £   | 229             | TradeOriginationDate | N                                                                                                                                                                |                                                                                                                                                                                                               |
| £   | 54              | Side                 | N                                                                                                                                                                | If OrdType = "Forex - Swap", should be the side of the future portion of a F/X swap. The absence of a side implies that a two-sided quote is being requested. Required if specified in Quote Request message. |
| £   | 854             | QtyType              | N                                                                                                                                                                |                                                                                                                                                                                                               |
| £   | component block | N                    | Insert here the set of "OrderQytData" fields defined in "Common Components of Application Messages" Required if component is specified in Quote Request message. |                                                                                                                                                                                                               |
| £   | 63              | SettlType            | N                                                                                                                                                                |                                                                                                                                                                                                               |
| £   | 64              | SettlDate            | N                                                                                                                                                                | Can be used (e.g. with forex quotes) to specify the desired "value date"                                                                                                                                      |
| £   | 193             | SettlDate2           | N                                                                                                                                                                | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.                                                                           |
| £   | 192             | OrderQty2            | N                                                                                                                                                                | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.                                                                         |
| £   | 15              | Currency             | N                                                                                                                                                                | Can be used to specify the desired currency of the quoted price. May differ from the 'normal' trading currency of the instrument being quote requested.                                                       |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 35 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                            August 18, 2011

# Component Blocks

# Stipulations

Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "Common Components of Application Messages"

| £ | component block | N            | Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "Common Components of Application Messages" |
| - | --------------- | ------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| £ | 1               | Account      | N                                                                                                                                                  |
| £ | 660             | AcctIDSource | N                                                                                                                                                  |
| £ | 581             | AccountType  | N                                                                                                                                                  |

# QuotReqLegsGrp

component block

# QuotQualGrp

component block

| £ | 692 | QuotePriceType | N | Initiator can specify the price type the quote needs to be quoted at. If not specified, the Respondent has option to specify how quote is quoted. |
| - | --- | -------------- | - | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| £ | 40  | OrdType        | N | Can be used to specify the type of order the quote request is for                                                                                 |
| £ | 126 | ExpireTime     | N | The time when Quote Request will expire.                                                                                                          |
| £ | 60  | TransactTime   | N | Time transaction was entered                                                                                                                      |

# SpreadOrBenchmarkCurveData

component block

Insert here the set of "SpreadOrBenchmarkCurveData" (Fixed Income spread or benchmark curve) fields defined in "Common Components of Application Messages"

| £ | 423 | PriceType | N |                                                                                                                                               |
| - | --- | --------- | - | --------------------------------------------------------------------------------------------------------------------------------------------- |
| £ | 44  | Price     | N | Quoted or target price                                                                                                                        |
| £ | 640 | Price2    | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the Quoted or target price for the future portion of a F/X swap. |

# YieldData

component block

Insert here the set of "YieldData" (yield-related) fields defined in "Common Components of Application Messages"

# Parties

component block

Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages"

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element QuotReqRej

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                    Page 36 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# QuotSetAckGrp component block

| Tag                                                                                                                                      | FieldName              | Req'd | Comments                                                                                                                                                                                                            |
| ---------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 296                                                                                                                                      | NoQuoteSets            | N     | The number of sets of quotes in the message                                                                                                                                                                         |
| 302                                                                                                                                      | QuoteSetID             | N     | First field in repeating group. Required if NoQuoteSets > 0                                                                                                                                                         |
| component block                                                                                                                          |                        |       |                                                                                                                                                                                                                     |
| \<UnderlyingInstrument> (underlying symbology) fields defined in "Common Components of Application Messages" Required if NoQuoteSets > 0 |                        |       |                                                                                                                                                                                                                     |
| 367                                                                                                                                      | QuoteSetValidUntilTime | N     |                                                                                                                                                                                                                     |
| 304                                                                                                                                      | TotNoQuoteEntries      | N     | Total number of quotes for the quote set across all messages. Should be the sum of all NoQuoteEntries in each message that has repeating quotes that are part of the same quote set. Required if NoQuoteEntries > 0 |
| 1168                                                                                                                                     | TotNoCxldQuotes        | N     | Total number of quotes canceled for the quote set across all messages.                                                                                                                                              |
| 1169                                                                                                                                     | TotNoAccQuotes         | N     | Total number of quotes accepted for the quote set across all messages.                                                                                                                                              |
| 1170                                                                                                                                     | TotNoRejQuotes         | N     | Total number of quotes rejected for the quote set across all messages.                                                                                                                                              |
| 893                                                                                                                                      | LastFragment           | N     | Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.                                                                                    |
| component block                                                                                                                          |                        |       |                                                                                                                                                                                                                     |
| \<QuotEntryAckGrp>                                                                                                                       |                        |       |                                                                                                                                                                                                                     |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element QuotSetAck

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 37 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# QuotSetGrp component block

| Tag                                                                                                          | FieldName              | Req'd | Comments                                                                                                                                                                             |
| ------------------------------------------------------------------------------------------------------------ | ---------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 296                                                                                                          | NoQuoteSets            | Y     | The number of sets of quotes in the message                                                                                                                                          |
| 302                                                                                                          | QuoteSetID             | Y     | Sequential number for the Quote Set. For a given QuoteID - assumed to start at 1. Must be the first field in the repeating group.                                                    |
| component block                                                                                              |                        |       |                                                                                                                                                                                      |
| \<UnderlyingInstrument> (underlying symbology) fields defined in "Common Components of Application Messages" |                        |       |                                                                                                                                                                                      |
| 367                                                                                                          | QuoteSetValidUntilTime | N     |                                                                                                                                                                                      |
| 304                                                                                                          | TotNoQuoteEntries      | Y     | Total number of quotes for the quote set across all messages. Should be the sum of all NoQuoteEntries in each message that has repeating quotes that are part of the same quote set. |
| 893                                                                                                          | LastFragment           | N     | Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.                                                     |
| component block                                                                                              |                        |       |                                                                                                                                                                                      |
| \<QuotEntryGrp>                                                                                              |                        |       |                                                                                                                                                                                      |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element QuotSet

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 38 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3                                                August 18, 2011

# RFQReqGrp component block

| Tag | FieldName       | Req'd               | Comments                                                                                                      |                                                                                                            |
| --- | --------------- | ------------------- | ------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| 146 | NoRelatedSym    | Y                   | Number of related symbols (instruments) in Request                                                            |                                                                                                            |
| £   | component block | Y                   | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" |                                                                                                            |
| £   | component block | N                   | \<UndInstrmtGrp>                                                                                              |                                                                                                            |
| £   | component block | N                   | \<InstrmtLegGrp>                                                                                              |                                                                                                            |
| £   | 140             | PrevClosePx         | N                                                                                                             | Useful for verifying security identification                                                               |
| £   | 303             | QuoteRequestType    | N                                                                                                             | Indicates the type of Quote Request (e.g. Manual vs. Automatic) being generated.                           |
| £   | 537             | QuoteType           | N                                                                                                             | Type of quote being requested from counterparty or market (e.g. Indicative, Firm, or Restricted Tradeable) |
| £   | 336             | TradingSessionID    | N                                                                                                             |                                                                                                            |
| £   | 625             | TradingSessionSubID | N                                                                                                             |                                                                                                            |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element RFQReq

© Copyright, 2008-2009, FIX Protocol, Limited                                     Page 39 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Quote Request

In some markets it is the practice to request quotes from brokers prior to placement of an order. The quote request message is used for this purpose. This message is commonly referred to as an Request For Quote (RFQ).

Quotes can be requested on specific securities, on specified stipulations when specific security is not known or forex rates. The quote request message can be used to request quotes on single products or multiple products. Securities quotes can be requested as either market quotes or for a specific quantity and side. If OrderQty and Side are absent, a market-style quote (bid x offer, size x size) will be returned.

In the tradeable and restricted tradeable quote models the Quote Request may be preceded by the RFQ Request message described further below.

For tradeable quote requests it is possible to specify the time period in which the request is valid for and the time period which the resulting quote must be valid for.

See VOLUME 7 - PRODUCT: FOREIGN EXCHANGE and USER GROUP: EXCHANGES AND MARKETS sections for detailed usage notes specific to Foreign Exchange and exchanges/marketplaces respectively.

The quote request message format is as follows:

| Tag                            | FieldName         | Req'd | Comments                                                                                                                                                                                                                                             |
| ------------------------------ | ----------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                 |                   | Y     | MsgType = R                                                                                                                                                                                                                                          |
| 131                            | QuoteReqID        | Y     |                                                                                                                                                                                                                                                      |
| 644                            | RFQReqID          | N     | For tradeable quote model - used to indicate to which RFQ Request this Quote Request is in response.                                                                                                                                                 |
| 11                             | ClOrdID           | N     | Required only in two party models when QuoteType(537) = '1' (Tradeable) and the OrdType(40) = '2' (Limit).                                                                                                                                           |
| 775                            | BookingType       | N     |                                                                                                                                                                                                                                                      |
| 528                            | OrderCapacity     | N     |                                                                                                                                                                                                                                                      |
| 529                            | OrderRestrictions | N     |                                                                                                                                                                                                                                                      |
| 1171                           | PrivateQuote      | N     | Used to indicate whether a private negotiation is requested or if the response should be public. Only relevant in markets supporting both Private and Public quotes. If field is not provided in message, the model used must be bilaterally agreed. |
| 1172                           | RespondentType    | N     |                                                                                                                                                                                                                                                      |
| 1091                           | PreTradeAnonymity | N     |                                                                                                                                                                                                                                                      |
| component block \<RootParties> |                   | N     | Insert here the set of "Root Parties" fields defined in "common components of application messages" Used for acting parties that applies to the whole message, not individual legs, sides, etc.                                                      |
| component block \<QuotReqGrp>  |                   | Y     | Number of related symbols (instruments) in Request                                                                                                                                                                                                   |
| 58                             | Text              | N     |                                                                                                                                                                                                                                                      |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                            Page 40 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

354     EncodedTextLen                 N          Must be set if EncodedText field is specified and must immediately precede it.

355     EncodedText                    N          Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.

StandardTrailer                         Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element QuotReq

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                          Page 41 of 200


---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                   August 18, 2011

# Quote Response

The Quote Response message is used to respond to a IOI message or Quote message. It is also used to counter a Quote or end a negotiation dialog. For usage of this message in a negotiation or counter quote dialog for fixed income and exchanges/marketplace see Volume 7, Fixed Income and Exchanges and Markets sections respectively.

The Quote Response message format is as follows:

| Tag                                 | FieldName           | Req'd | Comments                                                                                                                                                                     |
| ----------------------------------- | ------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                      |                     | Y     | MsgType = AJ                                                                                                                                                                 |
| 693                                 | QuoteRespID         | Y     | Unique ID as assigned by the Initiator                                                                                                                                       |
| 117                                 | QuoteID             | N     | Required only when responding to a Quote.                                                                                                                                    |
| 1166                                | QuoteMsgID          | N     | Optionally used when responding to a Quote.                                                                                                                                  |
| 694                                 | QuoteRespType       | Y     | Type of response this Quote Response is.                                                                                                                                     |
| 11                                  | ClOrdID             | N     | Unique ID as assigned by the Initiator. Required only in two-party models when QuoteRespType(694) = 1 (Hit/Lift) or 2 (Counter quote).                                       |
| 528                                 | OrderCapacity       | N     |                                                                                                                                                                              |
| 529                                 | OrderRestrictions   | N     |                                                                                                                                                                              |
| 23                                  | IOIID               | N     | Required only when responding to an IOI.                                                                                                                                     |
| 537                                 | QuoteType           | N     | (Deprecated in FIX.5.0) Default is Indicative.                                                                                                                               |
| 1091                                | PreTradeAnonymity   | N     |                                                                                                                                                                              |
| component block \<QuotQualGrp>      |                     | N     |                                                                                                                                                                              |
| component block \<Parties>          |                     | N     | Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages"                                                         |
| 336                                 | TradingSessionID    | N     |                                                                                                                                                                              |
| 625                                 | TradingSessionSubID | N     |                                                                                                                                                                              |
| component block \<Instrument>       |                     | Y     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages". For multilegs supply minimally a value for Symbol (55).       |
| component block \<FinancingDetails> |                     | N     | Insert here the set of "FinancingDetails" (symbology) fields defined in "Common Components of Application Messages". For multilegs supply minimally a value for Symbol (55). |
| component block \<UndInstrmtGrp>    |                     | N     | Number of underlyings                                                                                                                                                        |
| 54                                  | Side                | N     | Required when countering a single instrument quote or "hit/lift" an IOI or Quote.                                                                                            |

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                         Page 42 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Component Blocks

# component block &#x3C;OrderQtyData>

N

Insert here the set of "OrderQtyData" fields defined in "Common Components of Application Messages". Required when countering a single instrument quote or "hit/lift" an IOI or Quote.

| 110 | MinQty     | N |                                                                                                                                         |
| --- | ---------- | - | --------------------------------------------------------------------------------------------------------------------------------------- |
| 63  | SettlType  | N |                                                                                                                                         |
| 64  | SettlDate  | N | Can be used with forex quotes to specify a specific "value date".                                                                       |
| 193 | SettlDate2 | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.     |
| 192 | OrderQty2  | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.   |
| 15  | Currency   | N | Can be used to specify the currency of the quoted prices. May differ from the 'normal' trading currency of the instrument being quoted. |

# component block &#x3C;Stipulations>

N

| 1   | Account      | N |                                                     |
| --- | ------------ | - | --------------------------------------------------- |
| 660 | AcctIDSource | N | Used to identify the source of the Account code.    |
| 581 | AccountType  | N | Type of account associated with the order (Origin). |

# component block &#x3C;LegQuotGrp>

N

| 132 | BidPx          | N | If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified. |
| --- | -------------- | - | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 133 | OfferPx        | N | If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified. |
| 645 | MktBidPx       | N | Can be used by markets that require showing the current best bid and offer.                                                                   |
| 646 | MktOfferPx     | N | Can be used by markets that require showing the current best bid and offer.                                                                   |
| 647 | MinBidSize     | N | Specifies the minimum bid size. Used for markets that use a minimum and maximum bid size.                                                     |
| 134 | BidSize        | N | Specifies the bid size. If MinBidSize is specified, BidSize is interpreted to contain the maximum bid size.                                   |
| 648 | MinOfferSize   | N | Specifies the minimum offer size. If MinOfferSize is specified, OfferSize is interpreted to contain the maximum offer size.                   |
| 135 | OfferSize      | N | Specified the offer size. If MinOfferSize is specified, OfferSize is interpreted to contain the maximum offer size.                           |
| 62  | ValidUntilTime | N | The time when the quote will expire.                                                                                                          |

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                           Page 43 of 200
---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011


# Errata

Required for FI when the QuoteRespType is 2 (Counter quote) to indicate to the Respondent when the counter offer is valid until.

| 188  | BidSpotRate           | N | May be applicable for F/X quotes                                                                                                                                        |
| ---- | --------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 190  | OfferSpotRate         | N | May be applicable for F/X quotes                                                                                                                                        |
| 189  | BidForwardPoints      | N | May be applicable for F/X quotes                                                                                                                                        |
| 191  | OfferForwardPoints    | N | May be applicable for F/X quotes                                                                                                                                        |
| 631  | MidPx                 | N |                                                                                                                                                                         |
| 632  | BidYield              | N |                                                                                                                                                                         |
| 633  | MidYield              | N |                                                                                                                                                                         |
| 634  | OfferYield            | N |                                                                                                                                                                         |
| 60   | TransactTime          | N |                                                                                                                                                                         |
| 40   | OrdType               | N | Can be used to specify the type of order the quote is for.                                                                                                              |
| 642  | BidForwardPoints2     | N | (Deprecated in FIX.5.0)Bid F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value                                     |
| 643  | OfferForwardPoints2   | N | (Deprecated in FIX.5.0)Offer F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value                                   |
| 656  | SettlCurrBidFxRate    | N | Can be used when the quote is provided in a currency other than the instrument's 'normal' trading currency. Applies to all bid prices contained in this quote message   |
| 657  | SettlCurrOfferFxRate  | N | Can be used when the quote is provided in a currency other than the instrument's 'normal' trading currency. Applies to all offer prices contained in this quote message |
| 156  | SettlCurrFxRateCalc   | N | Can be used when the quote is provided in a currency other than the instruments trading currency.                                                                       |
| 12   | Commission            | N | Can be used to show the counterparty the commission associated with the transaction.                                                                                    |
| 13   | CommType              | N | Can be used to show the counterparty the commission associated with the transaction.                                                                                    |
| 582  | CustOrderCapacity     | N | For Futures Exchanges                                                                                                                                                   |
| 100  | ExDestination         | N | Used when routing quotes to multiple markets                                                                                                                            |
| 1133 | ExDestinationIDSource | N |                                                                                                                                                                         |
| 58   | Text                  | N |                                                                                                                                                                         |
| 354  | EncodedTextLen        | N | Must be set if EncodedText field is specified and must immediately precede it.                                                                                          |
| 355  | EncodedText           | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                          |

© Copyright, 2008-      ~~2009~~2011, FIX Protocol, Limited                                          Page 44 of 200



---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

# 44 Price

N

# 423 PriceType

N

# component block

N

Insert here the set of "SpreadOrBenchmarkCurveData"

# &#x3C;SpreadOrBenchmarkCurveData>

fields defined in "Common Components of Application Messages"

# component block &#x3C;YieldData>

N

Insert here the set of "YieldData" fields defined in "Common Components of Application Messages"

# StandardTrailer

Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element QuotRsp

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited

Page 45 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

# Quote Request Reject

The Quote Request Reject message is used to reject Quote Request messages for all quoting models. The quote request reject message format is as follows:

| Tag                               | FieldName                | Req'd | Comments                                                                                                                                                                                        |
| --------------------------------- | ------------------------ | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                    |                          | Y     | MsgType = AG                                                                                                                                                                                    |
| 131                               | QuoteReqID               | Y     |                                                                                                                                                                                                 |
| 644                               | RFQReqID                 | N     | For tradeable quote model - used to indicate to which RFQ Request this Quote Request is in response.                                                                                            |
| 658                               | QuoteRequestRejectReason | Y     | Reason Quote was rejected                                                                                                                                                                       |
| 1171                              | PrivateQuote             | N     | Used to indicate whether a private negotiation is requested or if the response should be public. Only relevant in markets supporting both Private and Public quotes.                            |
| 1172                              | RespondentType           | N     |                                                                                                                                                                                                 |
| 1091                              | PreTradeAnonymity        | N     |                                                                                                                                                                                                 |
| component block \<RootParties>    |                          | N     | Insert here the set of "Root Parties" fields defined in "common components of application messages" Used for acting parties that applies to the whole message, not individual legs, sides, etc. |
| component block \<QuotReqRjctGrp> |                          | Y     | Number of related symbols (instruments) in Request                                                                                                                                              |
| 58                                | Text                     | N     |                                                                                                                                                                                                 |
| 354                               | EncodedTextLen           | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                                                                                  |
| 355                               | EncodedText              | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                                                  |
| StandardTrailer                   |                          | Y     |                                                                                                                                                                                                 |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element QuotReqRej

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                          Page 46 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                   August 18, 2011

# RFQ Request

In tradeable and restricted tradeable quoting markets – Quote Requests are issued by counterparties interested in ascertaining the market for an instrument. Quote Requests are then distributed by the market to liquidity providers who make markets in the instrument. The RFQ Request is used by liquidity providers to indicate to the market for which instruments they are interested in receiving Quote Requests. It can be used to register interest in receiving quote requests for a single instrument or for multiple instruments.

The RFQ Request message format is as follows:

| Tag                          | FieldName               | Req'd | Comments                                                                                                                                                                                                                                             |
| ---------------------------- | ----------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader               |                         | Y     | MsgType = AH                                                                                                                                                                                                                                         |
| 644                          | RFQReqID                | Y     |                                                                                                                                                                                                                                                      |
| component block \<Parties>   |                         | N     | Insert here the set of Parties (firm identification) fields defined in COMMON COMPONENTS OF APPLICATION MESSAGES                                                                                                                                     |
| component block \<RFQReqGrp> |                         | Y     | Number of related symbols (instruments) in Request                                                                                                                                                                                                   |
| 263                          | SubscriptionRequestType | N     | Used to subscribe for Quote Requests that are sent into a market                                                                                                                                                                                     |
| 1171                         | PrivateQuote            | N     | Used to indicate whether a private negotiation is requested or if the response should be public. Only relevant in markets supporting both Private and Public quotes. If field is not provided in message, the model used must be bilaterally agreed. |
| StandardTrailer              |                         | Y     |                                                                                                                                                                                                                                                      |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element RFQReq

# Tradeable Quote Model - Using the RFQ Request

In the quote on demand model – markets are not necessarily available until someone interested in the market generates a request.

| First Party | Market                                                                                       | Second Party (usually market maker or specialist) |
| ----------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| RFQ Request | Subscribes for Quote Requests for instruments in which party is interested in making markets | Quote Request                                     |
|             | Submits Quote Requests for instruments                                                       |                                                   |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                      Page 47 of 200
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011



| First Party                                   | Market                                                                    | Second Party (usually market maker or specialist) |
| --------------------------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------- |
| Quote Requests are distributed to subscribers | Receives Quote Request                                                    |                                                   |
| Quote                                         | Sends Quote in response to Quote Request                                  |                                                   |
| Market Data                                   | Quote results in change to market – causing Market Data to be distributed |                                                   |


© Copyright, 2008-2009, 2011, FIX Protocol, Limited

Page 48 of 200


---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Quote

The Quote message is used as the response to a Quote Request or a Quote Response message in both
indicative, tradeable, and restricted tradeable quoting markets.

In tradeable and restricted tradeable quoting models, the market maker sends quotes into a market as opposed
to sending quotes directly to a counterparty.

For Fixed Income in the indicative and tradeable quoting models, the quotes are typically sent directly to an
interested counterparty as opposed to a market place. See Volume 7 – PRODUCT: FIXED INCOME for
specific descriptions and usage details.

The quote message can be used to send unsolicited quotes in both indicative, tradeable, and restricted tradeable
quoting markets.

The quote message contains a quote for a single product.

If the issuer of the quote requires a response (i.e. notification that the quote message has been accepted) then
the QuoteResponseLevel field should be populated on the quote message – the response would be made using
the Quote Status Report message.

The quote should not be used in tradeable and restricted tradeable quoting markets, such as electronic trading
systems, to broadcast quotes to market participants. The recommended approach to reporting market state
changes that result from quotes received by a market is to use the market data messages.

Quotes supplied as the result of a Quote Request message will specify the appropriate QuoteReqID, unsolicited
quotes can be identified by the absence of a QuoteReqID.

See VOLUME 7 - PRODUCT: FOREIGN EXCHANGE and USER GROUP: EXCHANGES AND MARKETS sections for more
detailed usage notes specific to Foreign Exchange and Exchanges/Marketplaces respectively.

Orders can be generated based on Quotes. Quoted orders include the QuoteID and are OrdType=Previously
Quoted.

The time in force for a quote is determined by agreement between counterparties.

A quote can be canceled either using the Quote Cancel message or by sending a quote message with bid and
offer prices and sizes all set to zero (BidPx, OfferPx, BidSize, OfferSize).

# The quote message format is as follows:

| Tag            | FieldName   | Req'd | Comments                                                                                                             |
| -------------- | ----------- | ----- | -------------------------------------------------------------------------------------------------------------------- |
| StandardHeader |             | Y     | MsgType = S                                                                                                          |
| 131            | QuoteReqID  | N     | Required when quote is in response to a Quote Request message                                                        |
| 117            | QuoteID     | Y     |                                                                                                                      |
| 1166           | QuoteMsgID  | N     | Optionally used to supply a message identifier for a quote.                                                          |
| 693            | QuoteRespID | N     | Required when responding to the Quote Response message. The counterparty specified ID of the Quote Response message. |
| 537            | QuoteType   | N     | Quote Type. If not specified, the default is an indicative quote.                                                    |

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                          Page 49 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                            August 18, 2011

# Errata

| 1171                                | PrivateQuote        | N | Used to indicate whether a private negotiation is requested or if the response should be public. Only relevant in markets supporting both Private and Public quotes. If field is not provided in message, the model used must be bilaterally agreed. |
| ----------------------------------- | ------------------- | - | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| component block \<QuotQualGrp>      | N                   |   | Level of Response requested from receiver of quote messages.                                                                                                                                                                                         |
| component block \<Parties>          | N                   |   | Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages"                                                                                                                                 |
| 336                                 | TradingSessionID    | N |                                                                                                                                                                                                                                                      |
| 625                                 | TradingSessionSubID | N |                                                                                                                                                                                                                                                      |
| component block \<Instrument>       | Y                   |   | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages"                                                                                                                                        |
| component block \<FinancingDetails> | N                   |   | Insert here the set of "FinancingDetails" (symbology) fields defined in "Common Components of Application Messages"                                                                                                                                  |
| component block \<UndInstrmtGrp>    | N                   |   | Number of underlyings                                                                                                                                                                                                                                |
| 54                                  | Side                | N | Required for Tradeable or Counter quotes of single instruments                                                                                                                                                                                       |
| component block \<OrderQtyData>     | N                   |   | Required for Tradeable quotes or Counter quotes of single instruments                                                                                                                                                                                |
| 63                                  | SettlType           | N |                                                                                                                                                                                                                                                      |
| 64                                  | SettlDate           | N | Can be used with forex quotes to specify a specific "value date". For NDFs this is required.                                                                                                                                                         |
| 193                                 | SettlDate2          | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.                                                                                                                  |
| 192                                 | OrderQty2           | N | (Deprecated in FIX.5.0) Can be used with OrdType = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.                                                                                                                |
| 15                                  | Currency            | N | Can be used to specify the currency of the quoted prices. May differ from the 'normal' trading currency of the instrument being quoted                                                                                                               |
| 120                                 | SettlCurrency       | N | Required for NDFs to specify the settlement currency (fixing currency).                                                                                                                                                                              |
| component block \<RateSource>       | N                   |   |                                                                                                                                                                                                                                                      |
| component block \<Stipulations>     | N                   |   | Insert here the set of "Stipulations" (repeating group of Fixed Income stipulations) fields defined in "Common Components of Application Messages"                                                                                                   |

© Copyright, 2008-      ~~2009~~2011, FIX Protocol, Limited                                       Page 50 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                   August 18, 2011

# 1 Account

|                               | AcctIDSource       | N |                                                                                                                                               |   |
| ----------------------------- | ------------------ | - | --------------------------------------------------------------------------------------------------------------------------------------------- | - |
|                               | AccountType        | N | Type of account associated with the order (Origin)                                                                                            |   |
| component block \<LegQuotGrp> |                    | N | Required for multileg quotes                                                                                                                  |   |
| BidPx                         |                    | N | If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified. |   |
|                               | OfferPx            | N | If F/X quote, should be the "all-in" rate (spot rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified. |   |
|                               | MktBidPx           | N | Can be used by markets that require showing the current best bid and offer                                                                    |   |
|                               | MktOfferPx         | N | Can be used by markets that require showing the current best bid and offer                                                                    |   |
|                               | MinBidSize         | N | Specifies the minimum bid size. Used for markets that use a minimum and maximum bid size.                                                     |   |
|                               | BidSize            | N | Specifies the bid size. If MinBidSize is specified, BidSize is interpreted to contain the maximum bid size.                                   |   |
|                               | MinOfferSize       | N | Specifies the minimum offer size. If MinOfferSize is specified, OfferSize is interpreted to contain the maximum offer size.                   |   |
|                               | OfferSize          | N | Specified the offer size. If MinOfferSize is specified, OfferSize is interpreted to contain the maximum offer size.                           |   |
| MinQty                        |                    | N | For use in private/directed quote negotiations.                                                                                               |   |
|                               | ValidUntilTime     | N | The time when the quote will expire                                                                                                           |   |
|                               | BidSpotRate        | N | May be applicable for F/X quotes                                                                                                              |   |
|                               | OfferSpotRate      | N | May be applicable for F/X quotes                                                                                                              |   |
|                               | BidForwardPoints   | N | May be applicable for F/X quotes                                                                                                              |   |
|                               | OfferForwardPoints | N | May be applicable for F/X quotes                                                                                                              |   |
|                               | BidSwapPoints      | N | Bid swap points of an FX Swap quote.                                                                                                          |   |
|                               | OfferSwapPoints    | N |                                                                                                                                               |   |
| MidPx                         |                    | N |                                                                                                                                               |   |
|                               | BidYield           | N |                                                                                                                                               |   |
|                               | MidYield           | N |                                                                                                                                               |   |
|                               | OfferYield         | N |                                                                                                                                               |   |
|                               | TransactTime       | N |                                                                                                                                               |   |
|                               | OrdType            | N | Can be used to specify the type of order the quote is for                                                                                     |   |
|                               | BidForwardPoints2  | N | (Deprecated in FIX.5.0)Bid F/X forward points of the future portion of a F/X swap quote added to spot rate.                                   |   |

© Copyright, 2008-      ~~2009~~2011, FIX Protocol, Limited                                           Page 51 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

| 643                           | OfferForwardPoints2   | N | (Deprecated in FIX.5.0) Offer F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value                                  |
| ----------------------------- | --------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 656                           | SettlCurrBidFxRate    | N | Can be used when the quote is provided in a currency other than the instrument's 'normal' trading currency. Applies to all bid prices contained in this quote message   |
| 657                           | SettlCurrOfferFxRate  | N | Can be used when the quote is provided in a currency other than the instrument's 'normal' trading currency. Applies to all offer prices contained in this quote message |
| 156                           | SettlCurrFxRateCalc   | N | Can be used when the quote is provided in a currency other than the instruments trading currency.                                                                       |
| 13                            | CommType              | N | Can be used to show the counterparty the commission associated with the transaction.                                                                                    |
| 12                            | Commission            | N | Can be used to show the counterparty the commission associated with the transaction.                                                                                    |
| 582                           | CustOrderCapacity     | N | For Futures Exchanges                                                                                                                                                   |
| 100                           | ExDestination         | N | Used when routing quotes to multiple markets                                                                                                                            |
| 1133                          | ExDestinationIDSource | N |                                                                                                                                                                         |
| 775                           | BookingType           | N |                                                                                                                                                                         |
| 528                           | OrderCapacity         | N |                                                                                                                                                                         |
| 529                           | OrderRestrictions     | N |                                                                                                                                                                         |
| 423                           | PriceType             | N |                                                                                                                                                                         |
| component block               |                       |   |                                                                                                                                                                         |
| \<SpreadOrBenchmarkCurveData> |                       |   |                                                                                                                                                                         |
| component block \<YieldData>  |                       |   |                                                                                                                                                                         |
| 58                            | Text                  | N |                                                                                                                                                                         |
| 354                           | EncodedTextLen        | N | Must be set if EncodedText field is specified and must immediately precede it.                                                                                          |
| 355                           | EncodedText           | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                          |
| StandardTrailer               |                       |   |                                                                                                                                                                         |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element Quot

# Example: Quote for Single Security

QuoteID=XXX

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                          Page 52 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                        August 18, 2011

QuoteReqID=YYY

Symbol=AA

MaturyMonthYear=199901

StrikePrice=25.00

CFICode=”OCXXXS”

BixPx=5.00

OfferPx=5.25

BidSize=10

OfferSize=10

© Copyright, 2008-2009, 2011, FIX Protocol, Limited
Page 53 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

The Quote Cancel message is used by an originator of quotes to cancel quotes. The Quote Cancel message supports cancelation of:

- All quotes
- Quotes for a specific symbol or security ID
- All quotes for a security type
- All quotes for an underlying

Canceling a Quote is accomplished by indicating the type of cancelation in the QuoteCancelType field. It is recommended that all Cancel messages be acknowledged using the Quote Status Report message. The Quote Cancelation only applies to quotes made by the current FIX user. The Quote Cancel message format is as follows:

| Tag                              | FieldName           | Req'd | Comments                                                                                                                                                                                    |
| -------------------------------- | ------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                   |                     | Y     | MsgType = Z                                                                                                                                                                                 |
| 131                              | QuoteReqID          | N     | Required when quote is in response to a Quote Request message                                                                                                                               |
| 117                              | QuoteID             | N     | Conditionally required when QuoteCancelType(298) = 5 (cancel quote specified in QuoteID). Maps to QuoteID(117) of a single Quote(MsgType=S) or QuoteEntryID(299) of a MassQuote(MsgType=i). |
| 1166                             | QuoteMsgID          | N     | Optionally used to supply a message identifier for a quote cancel.                                                                                                                          |
| 298                              | QuoteCancelType     | Y     | Identifies the type of Quote Cancel request.                                                                                                                                                |
| 537                              | QuoteType           | N     | Conditional Required when QuoteCancelType(298)=6\[Cancel by QuoteType]                                                                                                                      |
| 301                              | QuoteResponseLevel  | N     | Level of Response requested from receiver of quote messages.                                                                                                                                |
| component block \<Parties>       |                     | N     | Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages"                                                                        |
| component block \<TargetParties> |                     | N     | Can be used to specify the parties to whom the Quote Cancel should be applied.                                                                                                              |
| 1                                | Account             | N     |                                                                                                                                                                                             |
| 660                              | AcctIDSource        | N     |                                                                                                                                                                                             |
| 581                              | AccountType         | N     | Type of account associated with the order (Origin)                                                                                                                                          |
| 336                              | TradingSessionID    | N     |                                                                                                                                                                                             |
| 625                              | TradingSessionSubID | N     |                                                                                                                                                                                             |
| component block                  |                     | N     | The number of securities (instruments) whose quotes are                                                                                                                                     |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                      Page 54 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

# FIXML Definition for this message

See http://www.fixprotocol.org for details

Refer to FIXML element QuotCxl

# Options usage notes:

Normal usage would be to cancel the quotes for a symbol. This is the reason that the use of further nesting similar to the quote is not used in this message. You are able to cancel quotes for specific series by specifying each option series in the repeating group.

# Examples of the types of Quote Cancel operations:

# Cancel for Symbol(s)

Cancel all option quotes for symbol: IBM

| QuoteID         | user defined identifier for this cancel request |
| --------------- | ----------------------------------------------- |
| QuoteCancelType | 1                                               |
| NoQuoteEntries  | 1                                               |
| Symbol          | IBM                                             |
| CFICode         | O                                               |

# Cancel for Security Type(s)

Cancel all futures quotes for symbol: T (notice that CFICode is specified not SecurityType).

| QuoteID         | user defined identifier for this cancel request |
| --------------- | ----------------------------------------------- |
| QuoteCancelType | 2                                               |
| NoQuoteEntries  | 1                                               |
| Symbol          | N/A                                             |
| CFICode         | F                                               |

# Cancel Quotes for underlying symbols

Cancel all quotes for options with an underlying symbol of IBM

| QuoteID         | user defined identifier for this cancel request |
| --------------- | ----------------------------------------------- |
| QuoteCancelType | 3                                               |
| NoQuoteEntries  | 1                                               |
| Symbol          | IBM                                             |
| CFICode         | O                                               |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                        Page 55 of 200
---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                 August 18, 2011


# Cancel All Quotes

# Cancel all quotes associated with this FIX Session

- QuoteID= user defined identifier for this cancel request
- QuoteCancelType=4

# Cancel all quotes for a specific trading session

- QuoteID= user defined identifier for this cancel request
- QuoteCancelType=4
- TradingSessionID=a trading session identifier in a market

# Cancel All Quotes for specific parties

- QuoteID= user defined identifier for this cancel request
- QuoteCancelType=4
- PartyID=party identifier
- NoPartyIDs=1


© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                         Page 56 of 200

---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                              August 18, 2011

# Quote Status Request

The quote status request message is used for the following purposes in markets that employ tradeable or restricted tradeable quotes:

- For the issuer of a quote in a market to query the status of that quote (using the QuoteID to specify the target quote).
- To subscribe and unsubscribe for Quote Status Report messages for one or more securities.

The format of the quote status request message is:

| Tag                                 | FieldName               | Req'd | Comments                                                                                                             |
| ----------------------------------- | ----------------------- | ----- | -------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                      |                         | Y     | MsgType = a (lowercase)                                                                                              |
| 649                                 | QuoteStatusReqID        | N     |                                                                                                                      |
| 117                                 | QuoteID                 | N     | Maps to: - QuoteID(117) of a single Quote - QuoteEntryID(299) of a Mass Quote.                                       |
| component block \<Instrument>       |                         | N     | Conditionally required when requesting status of a single security quote.                                            |
| component block \<FinancingDetails> |                         | N     | Insert here the set of "FinancingDetails" (symbology) fields defined in "Common Components of Application Messages"  |
| component block \<UndInstrmtGrp>    |                         | N     | Number of underlyings                                                                                                |
| component block \<InstrmtLegGrp>    |                         | N     | Required for multileg quotes                                                                                         |
| component block \<Parties>          |                         | N     | Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages" |
| component block \<TargetParties>    |                         | N     | Can be used to specify the parties to whom the Quote Status Request should apply.                                    |
| 1                                   | Account                 | N     |                                                                                                                      |
| 660                                 | AcctIDSource            | N     |                                                                                                                      |
| 581                                 | AccountType             | N     | Type of account associated with the order (Origin)                                                                   |
| 336                                 | TradingSessionID        | N     |                                                                                                                      |
| 625                                 | TradingSessionSubID     | N     |                                                                                                                      |
| 263                                 | SubscriptionRequestType | N     | Used to subscribe for Quote Status Report messages                                                                   |
| StandardTrailer                     |                         | Y     |                                                                                                                      |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element QuotStatReq

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                         Page 57 of 200
---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011


# Application of Quote Status Request to Options Markets using tradeable or restricted tradeable quoting models:

To retrieve status of all quotes for a given underlying symbol for options enter the Symbol55 and optionally the SecurityID167 along with a CFICode537=”OXXXXX”.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                         Page 58 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

# Quote Status Report

The quote status report message is used:

- as the response to a Quote Status Request message
- as a response to a Quote Cancel message
- as a response to a Quote Response message in a negotiation dialog (see Volume 7 – PRODUCT: FIXED INCOME and USER GROUP: EXCHANGES AND MARKETS)

# Quote Status Report

| Tag                                 | FieldName           | Req'd | Comments                                                                                                              |
| ----------------------------------- | ------------------- | ----- | --------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                      |                     | Y     | MsgType = AI                                                                                                          |
| 649                                 | QuoteStatusReqID    | N     |                                                                                                                       |
| 131                                 | QuoteReqID          | N     | Required when quote is in response to a Quote Request message                                                         |
| 117                                 | QuoteID             | N     | Maps to QuoteID(117) of a single Quote(MsgType=S) or QuoteEntryID(299) of a MassQuote(MsgType=i).                     |
| 1166                                | QuoteMsgID          | N     | Maps to QuoteMsgID(1166) of a single Quote(MsgType=S) or QuoteID(117) of a MassQuote(MsgType=i).                      |
| 693                                 | QuoteRespID         | N     | Required when responding to a Quote Response message.                                                                 |
| 537                                 | QuoteType           | N     | Quote Type. If not specified, the default is an indicative quote.                                                     |
| 298                                 | QuoteCancelType     | N     |                                                                                                                       |
| component block \<Parties>          |                     | N     | Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages". |
| component block \<TargetParties>    |                     | N     | Can be populated with the values provided on the associated QuoteStatusRequest(MsgType=A).                            |
| 336                                 | TradingSessionID    | N     |                                                                                                                       |
| 625                                 | TradingSessionSubID | N     |                                                                                                                       |
| component block \<Instrument>       |                     | N     | Conditionally required when reporting status of a single security quote.                                              |
| component block \<FinancingDetails> |                     | N     | Insert here the set of "FinancingDetails" (symbology) fields defined in "Common Components of Application Messages".  |
| component block \<UndInstrmtGrp>    |                     | N     | Number of underlyings.                                                                                                |
| 54                                  | Side                | N     |                                                                                                                       |
| component block \<OrderQtyData>     |                     | N     | Required for Tradeable quotes of single instruments.                                                                  |
| 63                                  | SettlType           | N     |                                                                                                                       |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                   Page 59 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                              August 18, 2011

# 64

SettlDate                       N        Can be used with forex quotes to specify a specific "value date"

# 193

SettlDate2                      N        (Deprecated in FIX.5.0)Can be used with OrdType = "Forex - Swap" to specify the "value date" for the future portion of a F/X swap.

# 192

OrderQty2                       N        (Deprecated in FIX.5.0)Can be used with OrdType = "Forex - Swap" to specify the order quantity for the future portion of a F/X swap.

# 15

Currency                        N        Can be used to specify the currency of the quoted prices. May differ from the 'normal' trading currency of the instrument being quoted

# component block &#x3C;Stipulations>

N

# 1

Account                         N

# 660

AcctIDSource                    N

# 581

AccountType                     N        Type of account associated with the order (Origin)

# component block &#x3C;LegQuotStatGrp>

N        Required for multileg quote status reports

# component block &#x3C;QuotQualGrp>

N

# 126

ExpireTime                      N

# 44

Price                           N

# 423

PriceType                       N

# component block

N

# &#x3C;SpreadOrBenchmarkCurveData>

# component block &#x3C;YieldData>

N

# 132

BidPx                           N        If             F/X quote, should be the "all-in" rate (spot  rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified.

# 133

OfferPx                         N        If             F/X quote, should be the "all-in" rate (spot  rate adjusted for forward points). Note that either BidPx, OfferPx or both must be specified.

# 645

MktBidPx                        N        Can be used by markets that require showing the current best bid and offer

# 646

MktOfferPx                      N        Can be used by markets that require showing the current best bid and offer

# 647

MinBidSize                      N        Specifies the minimum bid size. Used for markets that use a minimum and maximum bid size.

# 134

BidSize                         N        Specifies          the    bid size.  If MinBidSize is specified, BidSize is interpreted to contain the maximum bid size.

# 648

MinOfferSize                    N        Specifies the minimum offer size. If MinOfferSize is specified,         OfferSize     is  interpreted  to  contain the maximum offer size.

# 135

OfferSize                       N        Specified the offer size. If MinOfferSize is specified, OfferSize is interpreted to contain the maximum offer

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                           Page 60 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

# Errata

| 110  | MinQty                | N |                                                                                                                                                                   |
| ---- | --------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 62   | ValidUntilTime        | N |                                                                                                                                                                   |
| 188  | BidSpotRate           | N | May be applicable for F/X quotes                                                                                                                                  |
| 190  | OfferSpotRate         | N | May be applicable for F/X quotes                                                                                                                                  |
| 189  | BidForwardPoints      | N | May be applicable for F/X quotes                                                                                                                                  |
| 191  | OfferForwardPoints    | N | May be applicable for F/X quotes                                                                                                                                  |
| 631  | MidPx                 | N |                                                                                                                                                                   |
| 632  | BidYield              | N |                                                                                                                                                                   |
| 633  | MidYield              | N |                                                                                                                                                                   |
| 634  | OfferYield            | N |                                                                                                                                                                   |
| 60   | TransactTime          | N |                                                                                                                                                                   |
| 40   | OrdType               | N | Can be used to specify the type of order the quote is for                                                                                                         |
| 642  | BidForwardPoints2     | N | (Deprecated in FIX.5.0)Bid F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value                               |
| 643  | OfferForwardPoints2   | N | (Deprecated in FIX.5.0)Offer F/X forward points of the future portion of a F/X swap quote added to spot rate. May be a negative value                             |
| 656  | SettlCurrBidFxRate    | N | Can be used when the quote is provided in a currency other than the instrument's 'normal' trading currency. Applies to all bid prices contained in this message   |
| 657  | SettlCurrOfferFxRate  | N | Can be used when the quote is provided in a currency other than the instrument's 'normal' trading currency. Applies to all offer prices contained in this message |
| 156  | SettlCurrFxRateCalc   | N | Can be used when the quote is provided in a currency other than the instruments trading currency.                                                                 |
| 13   | CommType              | N | Can be used to show the counterparty the commission associated with the transaction.                                                                              |
| 12   | Commission            | N | Can be used to show the counterparty the commission associated with the transaction.                                                                              |
| 582  | CustOrderCapacity     | N | For Futures Exchanges                                                                                                                                             |
| 100  | ExDestination         | N | Used when routing quotes to multiple markets                                                                                                                      |
| 1133 | ExDestinationIDSource | N |                                                                                                                                                                   |
| 775  | BookingType           | N |                                                                                                                                                                   |
| 528  | OrderCapacity         | N |                                                                                                                                                                   |
| 529  | OrderRestrictions     | N |                                                                                                                                                                   |
| 297  | QuoteStatus           | N | Quote Status                                                                                                                                                      |
| 300  | QuoteRejectReason     | N | Reason Quote was rejected                                                                                                                                         |

© Copyright, 2008-      ~~2009~~2011, FIX Protocol, Limited                                 Page 61 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                        August 18, 2011

# 58

Text                           N

# 354

EncodedTextLen                 N

# 355

EncodedText                    N

# StandardTrailer

Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element QuotStatRpt

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 62 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                              August 18, 2011

# Indicative Quoting Model

FIX supports an Indicative Quoting Model that is frequently used between two counterparties. In the Indicative Quoting Model a party interested in a particular security issues a Quote Request to a counterparty. The counterparty responds with an indicative quote. The first party – assuming the quote meets their requirements – can send back a New Order – Single (order type = Previously Quoted). The New Order – Single message should contain the QuoteID of the Quote. The issuer of the quote does not necessarily have to execute the order – based upon market conditions or characteristics contained on the New Order Message.

# Indicative Quoting Model Message Scenario

| First Party                                                                                                                                                            | Second Party                                                                                                                                                    |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| QuoteRequest                                                                                                                                                           | Accepts Quote Request                                                                                                                                           |
| This is an optional first step. Counterparties may agree to provide indicative quotes in a continuous manner.                                                          | Creates a Quote for the product specified in the Quote Request                                                                                                  |
| Accepts Quote – after examining market indicated in quote decides whether to place a New Order                                                                         | Send Quote message (can be a one or two sided market). The QuoteReqID should be set to the QuoteReqID from the Quote Request to which this Quote is a response. |
| New Order –Single – should reference the QuoteID for which the New Order message in which the New Order is a response. The OrdType should be set to previously quoted. | Accepts the New Order message.                                                                                                                                  |
|                                                                                                                                                                        | Should be acknowledged as New.                                                                                                                                  |
|                                                                                                                                                                        | Sends Execution Report for NEW (Optional)                                                                                                                       |
|                                                                                                                                                                        | Sends Execution Report OrdStatus=FILL if the order is acceptable or                                                                                             |
|                                                                                                                                                                        | Or                                                                                                                                                              |
|                                                                                                                                                                        | Send Execution Report OrdStatus=PARTIALLY FILLED                                                                                                                |
|                                                                                                                                                                        | Or                                                                                                                                                              |
|                                                                                                                                                                        | Send Execution Report OrdStatus=REJECTED                                                                                                                        |

Indicative quotes can also be sent out on an unsolicited basis. The correct response is the New Order (previously quoted) as above.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                     Page 63 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Tradeable Quote Model

Beginning with FIX 4.2 support was provided for markets requiring tradeable quotes. A tradeable quote market has market makers or specialist issue quotes that are kept as part of a market. A tradeable quote can be directly traded against orders or other quotes (depending on market rules). The market created by these quotes should be distributed using the Market Data messages. When orders are entered in response to the markets created by the tradeable quotes – trades may result. Trades are reported with an Execution Report.

Tradeable Quote model markets can be continuously quoted or quoted on demand or a combination of the two. In continuously quoted markets – market makers or specialists are required to maintain two sided markets which comply with market requirements for bid-ask spread and minimum quantity. In the quote on demand market – market makers and specialists are usually required to respond to Quote Requests (RFQs) within a market prescribed time limit with a quote which complies with exchange prescribed bid-ask spread and minimum quantity.

# Tradeable Quote Model - Reporting Quote Status back to Issuer

The market should provide unsolicited quote status back to the quote issuer if the state of a quote changes with the exception of trades (fills) that occur against a quote. Trades (fills) are reported using the Execution Report.

NOTE: The Quote Message should not be used to report trades. Only the Execution Report should be used to report fills against a tradeable or restricted tradeable quote.

| Market maker or specialist                                                                                                                                                                                                                                                                                                                       | Quote                                                                         |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------- |
| Accepts Quote and applies to the market                                                                                                                                                                                                                                                                                                          |                                                                               |
| Valid tradeable or restricted tradeable quote sent into market – either unsolicited or in reply to a Quote Request from the market.                                                                                                                                                                                                              | Accepts Quote and updates trading system based upon status reported by market |
| Based upon market rules or the QuoteResponseLevel requested by Quote Issuer the market will send Quote Status Report messages back to the quote issuer to report quote status (using the QuoteStatus field). If a trade (fill) occurs against a tradeable quote an Execution Report (ExecType=Fill or Partial Fill) is sent to the quote issuer. |                                                                               |

# Using the Execution Report to report a trade on a Tradeable Quote

The Execution Report should be used to report trades involving a tradeable quote. Because quotes are usually replaced or replenished – often times with the same QuoteID – it is not always possible, nor does it necessarily make sense for markets to keep track and transmit the detailed quantity information required on the quote. Execution Reports for trades against a tradeable quote can use the quantity fields in the following manner.

| Tag# | Field Name | Re | Usage in reporting trades on tradeable or restricted tradeable quotes |
| ---- | ---------- | -- | --------------------------------------------------------------------- |
| 38   | OrderQty   | N  | Quote quantity when the fill occurred.                                |
| 32   | LastQty    | N  | Same as for a fill against an order                                   |
| 31   | LastPx     | N  | Same as for a fill against an order                                   |
| 151  | LeavesQty  | Y  | Quantity remaining open in the market                                 |
| 14   | CumQty     | Y  | Use 0.0 if market is unable to provide a cumulative total.            |
| 6    | AvgPx      | Y  | Use 0.0 if market is unable to provide an average price               |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 64 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Tradeable Quote Model - Quote on Demand Message Scenario

In the quote on demand model – markets are not necessarily available until someone interested in the market generates a request.

| First Party                                                                                                                                                                          | Market                                                                                                       | Second Party (usually market maker or specialist)  |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------- |
| Optional Quote Status Request to subscribe for Quote Status for one or more instruments (some markets may choose to configure this out of band).                                     |                                                                                                              |                                                    |
| Tracks Subscription Requests for each party connected to market                                                                                                                      | RFQ Request                                                                                                  | Subscribe for Quote Requests                       |
| NOTE: Some markets may choose to configure subscription and dissemination of Quote Request out-of-band – instead of in-band.                                                         |                                                                                                              |                                                    |
| Quote Request (Optional request for quote if no quote exists in the market)                                                                                                          | Market checks validity of Quote Request and then sends it to subscribed participants                         | Accepts Quote Request                              |
| Generates a quote based upon request                                                                                                                                                 | Interprets quotes and applies them to a market                                                               | Quote                                              |
| Interprets QuoteResponse Level to determine if quote status should be sent back to the quote issuer using a Quote Status Report message with the QuoteStatus field set appropriately | Valid quote that changes market should be disseminated using Market Data messages                            | Optional Quote Status Report                       |
| Receives Market Data                                                                                                                                                                 | If the Quote is valid and has an impact on the market Market Data is published                               | Receives Market Data                               |
| Will use Market Data to make market participation and pricing decision                                                                                                               | (NOTE: The process of subscribing for market data is omitted from this example)                              | Useful in creating subsequent quotes               |
| Sends New Order – Single                                                                                                                                                             | Order is matched against other orders and quotes according to market rules.                                  | Receives Execution Report – Pending New (optional) |
| Received Execution Report – NEW                                                                                                                                                      | (NOTE: This can be either open-outcry based markets with or without limit book or a fully electronic market) |                                                    |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                       Page 65 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

# Errata

# Tradeable Quote Model Message Scenario - Continuous markets

| First Party                 | Market                                                                                                                                                           | Second Party (usually market maker or specialist)                                                                                                                                                                |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Receipt of Execution Report | If the order is matched against the tradeable or restricted tradeable quote resulting in a trade – Execution Reports are sent to the counterparties of the trade | Receipt of Market Maker side Execution Report reporting Fill against the previously submitted tradeable or restricted tradeable Quote (Optionally can choose to replenish market or wait for next Quote Request) |

| First Party                                                                       | Market                                                                                                                                                         | Second Party (usually market maker or specialist)                                                                                                                                    |
| --------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Uses market data to determine market participation and pricing on orders          | Market Data is disseminated (NOTE: This may include the need to transmit expected opening prices based upon current state of the book at the opening)          | Uses market data to create subsequent quotes                                                                                                                                         |
| Interprets quotes and applies them to a market                                    | Quote Market Makers / Specialist are expected to maintain two sided quotes that comply with market required bid-ask spread and minimum quantities              | Interprets QuoteResponse Level to determine if quote status should be sent back to the quote issuer using a Quote Status Report message with the QuoteStatus field set appropriately |
| Market Data will be generated to report state of the book is changed by the quote | Optional Quote Status Report                                                                                                                                   | Receives Market Data                                                                                                                                                                 |
| Will use Market Data to make market participation and pricing decision            | If the Quote is valid and has an impact on the market Market Data is published (NOTE: The process of subscribing for market data is omitted from this example) | Receives Market Data                                                                                                                                                                 |
|                                                                                   | Used to create subsequent quotes                                                                                                                               |                                                                                                                                                                                      |

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                    Page 66 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# First Party

# Market

# Second Party (usually market maker or specialist)

Sends New Order – Single

Receives Execution Report – Pending New (optional)

Received Execution Report – NEW

Order is matched against other orders and quotes according to market rules.

(NOTE: This can be either open-outcry based markets with or without limit book or a fully electronic market)

Receipt of Execution Report – Reporting Fill or Partial Fill

If the order is matched against the tradeable or restricted tradeable quote resulting in a trade – Execution Reports are sent to the counterparties of the trade.

Receipt of Market Maker side Execution Report reporting Fill against the previously submitted tradeable or restricted tradeable Quote.

(Optionally can choose to replenish market or wait for next Quote Request)

Quote is processed as above – market data is generated – an optional Quote Status Report message is generated.

# Tradeable Quote Model - Querying for Quote Status

Market participants may need to query the status of their current quotes. Normally a market will provide status in an unsolicited manner back to the quote issuer. However, to support system or session recovery – the Quote Status Request can be used to query the current state of quotes within a market.

| Market maker or specialist                                                                                                                | Market                                                                                                                                                                                      |
| ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Quote Status Request                                                                                                                      | Accepts Quote Status Request                                                                                                                                                                |
| Contains information on the securities for which the quote status request is being issued or the QuoteID of a previously submitted quote. | Accepts Quote and updates trading system.                                                                                                                                                   |
|                                                                                                                                           | Sends Quote Status Report messages with the QuoteStatus field set, bid and ask prices, and quantities for each quote belonging to the request issuer that meet the criteria in the request. |
|                                                                                                                                           | If there is a current quote in the market – the Quote Status Report in response to a Quote Status Request should be sent with a QuoteStatus of “Query”.                                     |
|                                                                                                                                           | The Quote Status Report message can also contain a QuoteStatus of “Quote Not Found” if no quote currently exists.                                                                           |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 67 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                 August 18, 2011

# Restricted Tradeable Quote Model

The Restricted Tradeable Quote Model extends the behavior of the Tradeable Quote Model to place limits on quantity or price. Orders received against the Restricted Tradeable Quote that are within limits set by the market – will execute against the quote automatically – just as in the case of the Tradeable Quote Model. If the order is outside the limits specified by the market – the order is forwarded to the quote issuer(s) to be filled, partially filled with remaining quantity cancelled, or canceled.

# Restricted Tradeable Quote Model Message Scenario

The Restricted Tradeable Quote Model will automatically trade against orders within restrictions specified by the market in terms of quantity or price.

| First Party                                                              | Market                                                                                                                                                         | Second Party (usually market maker or specialist)                                                                                                                                    |
| ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Uses market data to determine market participation and pricing on orders | Market Data is disseminated (NOTE: This may include the need to transmit expected opening prices based upon current state of the book at the opening)          | Uses market data to create subsequent quotes                                                                                                                                         |
| Interprets quotes and applies them to a market                           | Market Makers / Specialist are expected to maintain two sided quotes that comply with market required bid-ask spread and minimum quantities                    | Interprets QuoteResponse Level to determine if quote status should be sent back to the quote issuer using a Quote Status Report message with the QuoteStatus field set appropriately |
| Receives Market Data                                                     | If the Quote is valid and has an impact on the market Market Data is published (NOTE: The process of subscribing for market data is omitted from this example) | Receives Market Data                                                                                                                                                                 |
| Will use Market Data to make market participation and pricing decision   |                                                                                                                                                                | Used to create subsequent quotes                                                                                                                                                     |
| Sends New Order – Single                                                 | Order is matched against other orders and quotes according to market rules.                                                                                    | Receives Execution Report – Pending New (optional)                                                                                                                                   |
| Received Execution Report – NEW                                          | (NOTE: This can be either open-outcry based markets with or without limit book or a fully electronic market)                                                   |                                                                                                                                                                                      |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited
Page 68 of 200
---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                    August 18, 2011

# Receipt of Execution Report

| First Party                                                                                            | Market                                                                                                                                                           | Second Party (usually market maker or specialist)                                                                                                                                                                |
| ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Reporting Fill or Partial Fill                                                                         | If the order is matched against the tradeable or restricted tradeable quote resulting in a trade – Execution Reports are sent to the counterparties of the trade | Receipt of Market Maker side Execution Report reporting Fill against the previously submitted tradeable or restricted tradeable Quote (Optionally can choose to replenish market or wait for next Quote Request) |
| Quote is processed as above – market data is generated – an optional quote status message is generated | Replenishes Quote – possibly changing prices and quantities                                                                                                      |                                                                                                                                                                                                                  |
| Sends New Order – Single that is outside the restrictions specified by the market                      | Order is identified as being outside automatic execution parameters. The order is sent to the quote issuer(s)                                                    | Receives order and decides if the order is acceptable                                                                                                                                                            |
|                                                                                                        |                                                                                                                                                                  | Sends back an execution for partial quantity, full quantity, or cancels the order                                                                                                                                |

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                         Page 69 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                         August 18, 2011

# Mass Quote

The Mass Quote message can contain quotes for multiple securities to support applications that allow for the mass quoting of an option series. Two levels of repeating groups have been provided to minimize the amount of data required to submit a set of quotes for a class of options (e.g. all option series for IBM). A QuoteSet specifies the first level of repeating fields for the Mass Quote message. It represents a group of related quotes and can, for example, represent an option class.

Each QuoteSet contains an optional repeating group of QuoteEntries which can represent an option series.

It is possible the number of Quote Entries for a Quote Set (option class) could exceed one’s physical or practical message size. It may be necessary to fragment a message across multiple quote messages. Message size limits must be mutually agreed to with one’s counterparties.

# The grouping of quotes is as follows:

- NoQuoteSets – specifies the number of sets of quotes contained in the message
- QuoteSetID – Is a unique ID given to the quote set
- Information regarding the security to which all of the quotes belong
- TotQuoteEntries – defines the number of quotes for the quote set across all messages
- NoQuoteEntries – defines the number of quotes contained within this message for this quote set
- QuoteEntryID – Is a unique ID given to a specific quote entry
- Information regarding the specific quote (bid/ask size and price)

If there are too many Quote Entries for a Quote Set to fit into one physical message, then the quotes can be continued in another Mass Quote message by repeating all of the QuoteSet information and then specifying the number of Quote Entries (related symbols) in the continued message. The TotQuoteEntries is provided to optionally indicate to the counterparty the total number of Quote Entries for a Quote Set in multiple quote messages. This permits, but does not require, a receiving application to react in a stateful manner where it can determine if it has received all quotes for a Quote Set before carrying out some action. However, the overall approach to fragmentation is to permit each mass quote message to be processed in a stateless manner as it is received. Each mass quote message should contain enough information to have the Quote Entries applied to a market without requiring the next message if fragmentation has occurred. Also, a continued message should not require any information from the previous message.

Maximum message size for fragmentation purposes can be determined by using the optional MaxMessageSize field in the Logon message or by mutual agreement between counterparties.

# Requesting Acknowledgement for Mass Quotes

Applications can optionally support acknowledgement of quotes using the QuoteResponseLevel field. The QuoteResponseLevel is used to specify the level of acknowledgement requested from the counterparty. A QuoteResponseLevel of 0 indicates that no acknowledgement is requested. A ResponseLevel of 1 requests

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                              Page 70 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

acknowledgement of invalid or erroneous quotes. A QuoteResponseLevel of 2 requests acknowledgement of each Mass Quote message.

See “Mass Quote Message Scenarios”

# Mass Quote message format

| Tag                           | FieldName          | Req'd | Comments                                                                                                             |
| ----------------------------- | ------------------ | ----- | -------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                |                    | Y     | MsgType = i (lowercase)                                                                                              |
| 131                           | QuoteReqID         | N     | Required when quote is in response to a Quote Request message                                                        |
| 117                           | QuoteID            | Y     |                                                                                                                      |
| 537                           | QuoteType          | N     | Type of Quote Default is Indicative if not specified                                                                 |
| 301                           | QuoteResponseLevel | N     | Level of Response requested from receiver of quote messages.                                                         |
| component block \<Parties>    |                    | N     | Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages" |
| 1                             | Account            | N     |                                                                                                                      |
| 660                           | AcctIDSource       | N     |                                                                                                                      |
| 581                           | AccountType        | N     | Type of account associated with the order (Origin)                                                                   |
| 293                           | DefBidSize         | N     | Default Bid Size for quote contained within this quote message - if not explicitly provided.                         |
| 294                           | DefOfferSize       | N     | Default Offer Size for quotes contained within this quote message - if not explicitly provided.                      |
| component block \<QuotSetGrp> |                    | Y     | The number of sets of quotes in the message                                                                          |
| StandardTrailer               |                    | Y     |                                                                                                                      |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element MassQuot

# Notes on usage:

For many markets, the Mass Quote message will be used to generate quotes in high volumes in an unsolicited manner. This means that multiple quotes will be sent to the counterparty (an exchange) without acknowledgement. The Mass Quote message can be used to send quotes for multiple classes, each with multiple series.

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                      Page 71 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                      August 18, 2011

# Example: Multiple Option Series for a single Option Class (No Fragmentation)

QuoteID=XXX
QuoteReqID=YYY
NoQuoteSets=1
QuoteSetID=1
Symbol=AA
TotQuoteEntries=2
NoQuoteEntries=2
Other quote set fields
QuoteEntryID=1
MaturyMonthYear=199901
StrikePrice=25.00
CFICode=”OCXXXS”
BixPx=5.00
OfferPx=5.25
BidSize=10
OfferSize=10
QuoteEntryID=2
MaturyMonthYear=199901
StrikePrice=30.00
CFICode=”OCXXXS”
BixPx=3.00
OfferPx=3.25
BidSize=10
OfferSize=10

# Example: Multiple Option Series for a single Option Class (Fragmentation)

# First Message:

QuoteID=XXX
QuoteReqID=YYY
NoQuoteSets=1
QuoteSetID=1
Symbol=AA
TotQuoteEntries=3
NoQuoteEntries=2
Other quote set fields
QuoteEntryID=1
MaturyMonthYear=199901
StrikePrice=25.00
CFICode=”OCXXXX”
BixPx=5.00
OfferPx=5.25
BidSize=10
OfferSize=10
QuoteEntryID=2
MaturyMonthYear=199901
StrikePrice=30.00
CFICode=”OCXXXX”
BixPx=3.00
OfferPx=3.25
BidSize=10
OfferSize=10

# Second Message:

QuoteID=XXX

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                               Page 72 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                      August 18, 2011

# QuoteReqID=YYY

NoQuoteSets=1

- QuoteSetID=1
- Symbol=AA
- Other quote set fields
- TotQuoteEntries=3
- NoQuoteEntries=1
- QuoteEntryID=3

# Example: Multiple Quotes for Fixed Income publishing

QuoteID=XXX

NoQuoteSets=1

- QuoteSetID=1
- TotQuoteEntries=3
- NoQuoteEntries=3
- Other quote set fields
- QuoteEntryID=1
- - Symbol=DE10003453
- SecurityID=DE10003453
- SecurityIDSource=4
- BixPx=105
- BidYield=.043
- OfferPx=102.3
- OfferYield=.0525
- BidSize=10
- OfferSize=10

QuoteEntryID=2
- - Symbol=NL0000102606
- SecurityID=NL0000102606
- SecurityIDSource=4
- MidPx=105
- MidYield=4.3

QuoteEntryID=3

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                               Page 73 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                 August 18, 2011

# Mass Quote Acknowledgement

Mass Quote Acknowledgement is used as the application level response to a Mass Quote message. The Mass Quote Acknowledgement contains a field for reporting the reason in the event that the entire quote is rejected (QuoteRejectReason[300]). The Mass Quote Acknowledgement also contains a field for each quote that is used in the event that the quote entry is rejected (QuoteEntryRejectReason[368]). The ability to reject an individual quote entry is important so that the majority of quotes can be successfully applied to the market instead of having to reject the entire Mass Quote for a minority of rejected quotes.

Derivative markets are characterized by high bandwidth consumption – due to a change in an underlying security price causing multiple (often in the hundreds) of quotes to be recalculated and retransmitted to the market. For that reason the ability for market participants (and the market) to be able to set the level of response requested to a Mass Quote message is specified using the QuoteResponseLevel[301] field.

# The Mass Quote Acknowledgement message format is as follows:

| Tag                              | FieldName          | Req'd | Comments                                                                                                                                                                                                                                       |
| -------------------------------- | ------------------ | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                   |                    | Y     | MsgType = b (lowercase)                                                                                                                                                                                                                        |
| 131                              | QuoteReqID         | N     | Required when acknowledgment is in response to a Quote Request message                                                                                                                                                                         |
| 117                              | QuoteID            | N     | Required when acknowledgment is in response to a Mass Quote, mass Quote Cancel or mass Quote Status Request message. Maps to:- QuoteID(117) of a Mass Quote
- QuoteMsgID(1166) of Quote Cancel
- QuoteStatusReqID(649) of Quote Status Request |
| 297                              | QuoteStatus        | Y     | Status of the mass quote acknowledgement.                                                                                                                                                                                                      |
| 300                              | QuoteRejectReason  | N     | Reason Quote was rejected.                                                                                                                                                                                                                     |
| 301                              | QuoteResponseLevel | N     | Level of Response requested from receiver of quote messages. Is echoed back to the counterparty.                                                                                                                                               |
| 537                              | QuoteType          | N     | Type of Quote                                                                                                                                                                                                                                  |
| 298                              | QuoteCancelType    | N     |                                                                                                                                                                                                                                                |
| component block \<Parties>       |                    | N     | Insert here the set of "Parties" (firm identification) fields defined in "Common Components of Application Messages"                                                                                                                           |
| component block \<TargetParties> |                    | N     | Should be populated if the Mass Quote Acknowledgement is acknowledging a mass quote cancellation by party.                                                                                                                                     |
| 1                                | Account            | N     |                                                                                                                                                                                                                                                |
| 660                              | AcctIDSource       | N     |                                                                                                                                                                                                                                                |
| 581                              | AccountType        | N     | Type of account associated with the order (Origin)                                                                                                                                                                                             |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                    Page 74 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011

# 58     Text

N

# 354     EncodedTextLen

N

# 355     EncodedText

N

# component block &#x3C;QuotSetAckGrp>

N          The number of sets of quotes in the message

# StandardTrailer

Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element MassQuotAck

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 75 of 200
---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                       August 18, 2011

# Mass Quote Message Scenarios

# Unsolicited quote(s) no response requested

Mass Quote is sent from first party to second party. The quote has the QuoteResponseLevel set to 0 or omitted. The second party does not acknowledge the quote. If the quote is later hit, resulting in a trade, an Execution Report is sent to the first party.

| First Party                                   | Second Party                                              |
| --------------------------------------------- | --------------------------------------------------------- |
| Mass Quote message                            | Interprets quotes applies them to a market                |
| Options:                                      | Interprets Response Level – provides response accordingly |
| One or more sets of quotes                    |                                                           |
| Set QuoteResponseLevel is set to 0 or omitted | No response is sent                                       |
|                                               | Execution Report                                          |
|                                               | Quote Results in Trade                                    |

# Unsolicited quote(s) negative response only requested

Mass Quote is sent from first party to second party. The quote has the QuoteResponseLevel set to 1. The second party only acknowledges the quote if there is an error. If the second party encounters an error while processing the quote a Mass Quote Acknowledgement message is sent with the QuoteRejectReason set to the error encountered.

| First Party                        | Second Party                               |
| ---------------------------------- | ------------------------------------------ |
| Mass Quote message                 | Interprets quotes applies them to a market |
| Options:                           |                                            |
| One or more sets of quotes         |                                            |
| Set Response Level to 1            | Interprets Mass Quote Acknowledgement      |
| If error – then send revised quote | If an error is encountered                 |
| Mass Quote message                 | Interprets quotes applies them to a market |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                              Page 76 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                        August 18, 2011

# Unsolicited quote(s) full response requested

Mass Quote is sent from first party to second party. The quote has the QuoteResponseLevel set to 2. The second party acknowledges each quote.

| First Party        | Second Party                               |
| ------------------ | ------------------------------------------ |
| Mass Quote message | Interprets quotes applies them to a market |

# Options:

- One or more sets of quotes
- Set Response Level to 2
- Interpret Mass Quote Acknowledgement
- Mass Quote Acknowledgement

# Cancel All Quotes

The First Party asks the second party to cancel all quotes. Quotes with a quote status are sent in response to the Cancel All Quotes message.

| First Party          | Second Party                                        |
| -------------------- | --------------------------------------------------- |
| Quote Cancel message | Interprets Quote Cancel message and cancels quotes. |

QuoteCancelType = 4 (Cancel all quotes)

# Interpret Mass Quote Acknowledgement

Mass Quote Acknowledgement

# Use of other Quote Messages in Mass Quoting

Once the Mass Quote message is submitted to a market and after the initial Mass Quote Acknowledgement - the Quote Entries are treated as separate quotes. Report of Quote Status should be done using the Quote Status Request and Quote messages. Fills are reported for each QuoteEntry using the Execution Report.

# Reporting Quote Status back to Mass Quote Issuer

Markets should report the status of quotes back to the quote issuer when the state of one of the quotes in a Mass Quote changes. Quote Status Report messages should be issued for each change in state of a quote entry. The QuoteID of the original Mass Quote message should be used as the QuoteID on the Quote Status Report. It is acceptable to append the QuoteSetID and QuoteEntryID to indicate the specific quote in the Mass Quote message referred to in the Quote Status Report if this information is maintained by the market. NOTE: The Quote Message should not be used to report trades. Only the Execution Report should be used to report fills against a tradeable or restricted tradeable quote.

Market maker or specialist

Market

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                              Page 77 of 200
---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                  August 18, 2011

# Mass Quote

Accepts Mass Quote and applies to the market valid tradeable or restricted tradeable quote sent into market – either unsolicited or in reply to a Quote Request from the market.

Accepts Quote and updates trading system based upon status reported by market based upon the QuoteResponseLevel requested by Quote Issuer the market will send Mass Quote Acknowledgement message back to the quote issuer to report quote status in the QuoteStatus field.

Updates trading system with quote status. Quote messages are sent back unsolicited as the quote state changes. The QuoteEntryID should be used as the QuoteID.

Updates trading system with execution report. If a trade (fill) occurs against a tradeable or restricted tradeable quote an Execution Report (ExecType=Trade) is sent to the quote issuer.

# Querying for Mass Quote Status

If the issuer of a Mass Quote queries the current status of the quote the market should reply with a sequence of individual quote messages with status. This is recommended to eliminate the need for markets to store QuoteSetIds and QuoteEntryIds that were provided as part of the Mass Quote message. Also, as quote status is very dynamic data – sending quote status on securities as soon as it is available – instead of combining it into a single message – will provide more timely information to the quote issuer. The use of a Quote Status Request for a Mass Quote is provided as a method of recovery for market maker trading systems – due to the volume of information that can be generated and the short lived nature of quote status – this usage is not recommended for normal processing.

# Market maker or specialist

# Market

Quote Status Request accepts Quote Status Request contains the QuoteId of a previously submitted Mass Quote.

Accepts Quote and updates trading system. Sends Quote messages with the QuoteStatus field, bid and ask prices and quantities for each quote belonging to the request issuer that meet the criteria in the request. If there is a current quote in the market – the Quote in response to a Quote Status Request should be sent with a QuoteStatus of “Query”. The Quote message can also contain a QuoteStatus of “Quote Not Found” if no quote currently exists.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                                   Page 78 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011

# CATEGORY: MARKET DATA

# Market Data Component Blocks

This section lists the component blocks used exclusively by the messages defined for Market Data.

# InstrmtMDReqGrp component block

| Tag | FieldName       | Req'd       | Comments                                                                                                      |
| --- | --------------- | ----------- | ------------------------------------------------------------------------------------------------------------- |
| 146 | NoRelatedSym    | Y           | Number of symbols (instruments) requested.                                                                    |
| £   | component block | Y           | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" |
| £   | component block | N           | \<UndInstrmtGrp>                                                                                              |
| £   | component block | N           | \<InstrmtLegGrp>                                                                                              |
| £   | 15              | Currency    | N                                                                                                             |
| £   | 537             | QuoteType   | N                                                                                                             |
| £   | 63              | SettlType   | N                                                                                                             |
| £   | 64              | SettlDate   | N                                                                                                             |
| £   | 271             | MDEntrySize | N                                                                                                             |
| £   | 1500            | MDStreamID  | N                                                                                                             |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element InstReq

© Copyright, 2008-2009, FIX Protocol, Limited

Page 79 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                    August 18, 2011

# MDFullGrp component block

| Tag                                                                                                  | FieldName        | Req'd | Comments                                                                                                                                                                     |
| ---------------------------------------------------------------------------------------------------- | ---------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 268                                                                                                  | NoMDEntries      | Y     | Number of entries following.                                                                                                                                                 |
| 269                                                                                                  | MDEntryType      | Y     | Must be the first field in this repeating group.                                                                                                                             |
| 278                                                                                                  | MDEntryID        | N     | Conditionally required when maintaining an order-depth book, that is, when AggregatedBook (266) is "N". allows subsequent Incremental changes to be applied using MDEntryID. |
| 270                                                                                                  | MDEntryPx        | N     | Conditionally required if MDEntryType is not Imbalance(A), Trade Volume (B), or Open Interest(C); Conditionally required when MDEntryType = "auction clearing price"         |
| 423                                                                                                  | PriceType        | N     |                                                                                                                                                                              |
| component block                                                                                      |                  |       |                                                                                                                                                                              |
| \<YieldData>                                                                                         |                  |       |                                                                                                                                                                              |
| defined in "Common Components of Application Messages"                                               |                  |       |                                                                                                                                                                              |
| component block                                                                                      |                  |       |                                                                                                                                                                              |
| \<SpreadOrBenchmarkCurveData>                                                                        |                  |       |                                                                                                                                                                              |
| (Fixed Income spread or benchmark curve) fields defined in Common Components of Application Messages |                  |       |                                                                                                                                                                              |
| 40                                                                                                   | OrdType          | N     | Used to support market mechanism type; limit order, market order, committed principal order                                                                                  |
| 15                                                                                                   | Currency         | N     | Can be used to specify the currency of the quoted price.                                                                                                                     |
| 120                                                                                                  | SettlCurrency    | N     | Required for NDFs to specify the settlement currency (fixing currency).                                                                                                      |
| component block                                                                                      |                  |       |                                                                                                                                                                              |
| \<RateSource>                                                                                        |                  |       |                                                                                                                                                                              |
| 271                                                                                                  | MDEntrySize      | N     | Conditionally required if MDEntryType = Bid(0), Offer(1), Trade(2), Trade Volume (B), or Open Interest(C) conditionally required when MDEntryType = "auction clearing price" |
| component block                                                                                      |                  |       |                                                                                                                                                                              |
| \<SecSizesGrp>                                                                                       |                  |       |                                                                                                                                                                              |
| 1093                                                                                                 | LotType          | N     | Can be used to specify the lot type of the quoted size in order depth books.                                                                                                 |
| 272                                                                                                  | MDEntryDate      | N     |                                                                                                                                                                              |
| 273                                                                                                  | MDEntryTime      | N     |                                                                                                                                                                              |
| 274                                                                                                  | TickDirection    | N     |                                                                                                                                                                              |
| 275                                                                                                  | MDMkt            | N     | (Deprecated in FIX.5.0) Market posting quote / trade. Valid values: See Volume 6: Appendix 6-C                                                                               |
| 336                                                                                                  | TradingSessionID | N     |                                                                                                                                                                              |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                            Page 80 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Errata

| £ | 625 | TradingSessionSubID   | N |                                                                                                                                           |   |
| - | --- | --------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------- | - |
| £ | 326 | SecurityTradingStatus | N |                                                                                                                                           |   |
| £ | 327 | HaltReason            | N |                                                                                                                                           |   |
| £ | 276 | QuoteCondition        | N | Space-delimited list of conditions describing a quote.                                                                                    |   |
| £ | 277 | TradeCondition        | N | Space-delimited list of conditions describing a trade                                                                                     |   |
| £ | 282 | MDEntryOriginator     | N | (Deprecated in FIX.5.0)                                                                                                                   |   |
| £ | 283 | LocationID            | N | (Deprecated in FIX.5.0)                                                                                                                   |   |
| £ | 284 | DeskID                | N | (Deprecated in FIX.5.0)                                                                                                                   |   |
| £ | 286 | OpenCloseSettlFlag    | N | Used if MDEntryType = Opening Price(4), Closing Price(5), or Settlement Price(6).                                                         |   |
| £ | 59  | TimeInForce           | N | For optional use when this Bid or Offer represents an order                                                                               |   |
| £ | 432 | ExpireDate            | N | For optional use when this Bid or Offer represents an order. ExpireDate and ExpireTime cannot both be specified in one Market Data Entry. |   |
| £ | 126 | ExpireTime            | N | For optional use when this Bid or Offer represents an order. ExpireDate and ExpireTime cannot both be specified in one Market Data Entry. |   |
| £ | 110 | MinQty                | N | For optional use when this Bid or Offer represents an order                                                                               |   |
| £ | 18  | ExecInst              | N | Can contain multiple instructions, space delimited.                                                                                       |   |
| £ | 287 | SellerDays            | N |                                                                                                                                           |   |
| £ | 37  | OrderID               | N | For optional use when this Bid, Offer, or Trade represents an order                                                                       |   |
| £ | 198 | SecondaryOrderID      | N | For optional use to support Hit/Take (selecting a specific order from the feed) without disclosing a private order id.                    |   |
| £ | 299 | QuoteEntryID          | N | For optional use when this Bid, Offer, or Trade represents a quote                                                                        |   |
| £ | 288 | MDEntryBuyer          | N | For optional use in reporting Trades                                                                                                      |   |
| £ | 289 | MDEntrySeller         | N | For optional use in reporting Trades                                                                                                      |   |
| £ | 346 | NumberOfOrders        | N | In an Aggregated Book, used to show how many individual orders make up an MDEntry                                                         |   |
| £ | 290 | MDEntryPositionNo     | N | Display position of a bid or offer, numbered from most competitive to least competitive, per market side, beginning with 1                |   |
| £ | 546 | Scope                 | N |                                                                                                                                           |   |
| £ | 811 | PriceDelta            | N |                                                                                                                                           |   |
| £ | 828 | TrdType               | N | Specifies trade type when a trade is being reported. Must be used when MDEntryType(269) = Trade(2).                                       |   |

© Copyright, 2008-2009, FIX Protocol, Limited

Page 81 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                   August 18, 2011

# Errata

| £ | 58                         | Text                | N | Text to describe the Market Data Entry. Part of repeating group.                                                               |
| - | -------------------------- | ------------------- | - | ------------------------------------------------------------------------------------------------------------------------------ |
| £ | 354                        | EncodedTextLen      | N | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| £ | 355                        | EncodedText         | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
| £ | 1023                       | MDPriceLevel        | N | Display position of a bid or offer, numbered from most competitive to least competitive, per market side, beginning with 1     |
| £ | 528                        | OrderCapacity       | N | Designates the capacity of the firm placing the order                                                                          |
| £ | 1024                       | MDOriginType        | N |                                                                                                                                |
| £ | 332                        | HighPx              | N | Used to report high price in association with trade, bid or ask rather than a separate entity                                  |
| £ | 333                        | LowPx               | N | Used to report low price in association with trade, bid or ask rather than a separate entity                                   |
| £ | 1025                       | FirstPx             | N | Indicates the first price of a trading session; can be a bid, ask, or trade price.                                             |
| £ | 31                         | LastPx              | N | Indicates the last price of a trading session; can be a bid, ask, or trade price.                                              |
| £ | 1020                       | TradeVolume         | N | Used to report trade volume in association with trade, bid or ask rather than a separate entity                                |
| £ | 63                         | SettlType           | N |                                                                                                                                |
| £ | 64                         | SettlDate           | N | Indicates date on which instrument will settle. For NDFs required for specifying the "value date".                             |
| £ | 1070                       | MDQuoteType         | N |                                                                                                                                |
| £ | 83                         | RptSeq              | N | Used to identify the sequence number within a feed type                                                                        |
| £ | 1048                       | DealingCapacity     | N | Identifies role of dealer; Agent, Principal, RisklessPrincipal                                                                 |
| £ | 1026                       | MDEntrySpotRate     | N |                                                                                                                                |
| £ | 1027                       | MDEntryForwardPoint | N |                                                                                                                                |
| £ | component block \<Parties> |                     |   |                                                                                                                                |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Full

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                          Page 82 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

# MDIncGrp component block

| Tag                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | FieldName       | Req'd | Comments                                                                                                                                                                                                                                                                                                                                                                                       |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 268                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | NoMDEntries     | Y     | Number of entries following.                                                                                                                                                                                                                                                                                                                                                                   |
| 279                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | MDUpdateAction  | Y     | Must be first field in this repeating group.                                                                                                                                                                                                                                                                                                                                                   |
| 285                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | DeleteReason    | N     | (Deprecated in FIX.5.0) If MDUpdateAction = Delete(2), can be used to specify a reason for the deletion.                                                                                                                                                                                                                                                                                       |
| 1173                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | MDSubBookType   | N     | Can be used to define a subordinate book.                                                                                                                                                                                                                                                                                                                                                      |
| 264                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | MarketDepth     | N     | Can be used to define the current depth of the book.                                                                                                                                                                                                                                                                                                                                           |
| 269                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | MDEntryType     | N     | Conditionally required if MDUpdateAction = New(0). Cannot be changed.                                                                                                                                                                                                                                                                                                                          |
| 278                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | MDEntryID       | N     | If specified, must be unique among currently active entries if MDUpdateAction = New (0), must be the same as a previous MDEntryID if MDUpdateAction = Delete (2), and must be the same as a previous MDEntryID if MDUpdateAction = Change (1) and MDEntryRefID is not specified, or must be unique among currently active entries if MDUpdateAction = Change(1) and MDEntryRefID is specified. |
| 280                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | MDEntryRefID    | N     | If MDUpdateAction = New(0), for the first Market Data Entry in a message, either this field or a Symbol must be specified. If MDUpdateAction = Change(1), this must refer to a previous MDEntryID.                                                                                                                                                                                             |
| 1500                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | MDStreamID      | N     |                                                                                                                                                                                                                                                                                                                                                                                                |
| component block                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |                 |       |                                                                                                                                                                                                                                                                                                                                                                                                |
| \<Instrument>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |                 |       |                                                                                                                                                                                                                                                                                                                                                                                                |
| defined in "Common Components of Application Messages" Either Symbol (the instrument component block) or MDEntryRefID must be specified if MDUpdateAction = New(0) for the first Market Data Entry in a message. For subsequent Market Data Entries where MDUpdateAction = New(0), the default is the instrument used in the previous Market Data Entry if neither Symbol nor MDEntryRefID are specified, or in the case of options and futures, the previous instrument with changes specified in MaturityMonthYear, MaturityDay, StrikePrice, OptAttribute, and SecurityExchange. May not be changed. |                 |       |                                                                                                                                                                                                                                                                                                                                                                                                |
| component block                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |                 |       |                                                                                                                                                                                                                                                                                                                                                                                                |
| \<UndInstrmtGrp>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                 |       |                                                                                                                                                                                                                                                                                                                                                                                                |
| component block                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |                 |       |                                                                                                                                                                                                                                                                                                                                                                                                |
| \<InstrmtLegGrp>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                 |       |                                                                                                                                                                                                                                                                                                                                                                                                |
| 291                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | FinancialStatus | N     |                                                                                                                                                                                                                                                                                                                                                                                                |
| 292                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | CorporateAction | N     |                                                                                                                                                                                                                                                                                                                                                                                                |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                            Page 83 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

| £ | 270             | MDEntryPx             | N                                                                                                                                                      | Conditionally required when MDUpdateAction = New(0) and MDEntryType is not Imbalance(A), Trade Volume (B), or Open Interest (C). Conditionally required when MDEntryType = "auction clearing price"        |
| - | --------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| £ | 423             | PriceType             | N                                                                                                                                                      |                                                                                                                                                                                                            |
| £ | component block | N                     | Insert here the set of YieldData (yield-related) fields defined in Common Components of Application Messages                                           |                                                                                                                                                                                                            |
| £ | component block | N                     | Insert here the set of SpreadOrBenchmarkCurveData (Fixed Income spread or benchmark curve) fields defined in Common Components of Application Messages |                                                                                                                                                                                                            |
| £ | 40              | OrdType               | N                                                                                                                                                      | Used to support market mechanism type; limit order, market order, committed principal order                                                                                                                |
| £ | 15              | Currency              | N                                                                                                                                                      | Can be used to specify the currency of the quoted price.                                                                                                                                                   |
| £ | 120             | SettlCurrency         | N                                                                                                                                                      | Required for NDFs to specify the settlement currency (fixing currency).                                                                                                                                    |
| £ | component block | N                     |                                                                                                                                                        |                                                                                                                                                                                                            |
| £ | 271             | MDEntrySize           | N                                                                                                                                                      | Conditionally required when MDUpdateAction = New(0) and MDEntryType = Bid(0), Offer(1), Trade(2), Trade Volume(B), or Open Interest(C). Conditionally required when MDEntryType = "auction clearing price" |
| £ | component block | N                     |                                                                                                                                                        |                                                                                                                                                                                                            |
| £ | 1093            | LotType               | N                                                                                                                                                      | Can be used to specify the lot type of the quoted size in order depth books.                                                                                                                               |
| £ | 272             | MDEntryDate           | N                                                                                                                                                      |                                                                                                                                                                                                            |
| £ | 273             | MDEntryTime           | N                                                                                                                                                      |                                                                                                                                                                                                            |
| £ | 274             | TickDirection         | N                                                                                                                                                      |                                                                                                                                                                                                            |
| £ | 275             | MDMkt                 | N                                                                                                                                                      | (Deprecated in FIX.5.0) Market posting quote / trade. Valid values: See Volume 6: Appendix 6-C                                                                                                             |
| £ | 336             | TradingSessionID      | N                                                                                                                                                      |                                                                                                                                                                                                            |
| £ | 625             | TradingSessionSubID   | N                                                                                                                                                      |                                                                                                                                                                                                            |
| £ | 326             | SecurityTradingStatus | N                                                                                                                                                      |                                                                                                                                                                                                            |
| £ | 327             | HaltReason            | N                                                                                                                                                      |                                                                                                                                                                                                            |
| £ | 276             | QuoteCondition        | N                                                                                                                                                      | Space-delimited list of conditions describing a quote.                                                                                                                                                     |
| £ | 277             | TradeCondition        | N                                                                                                                                                      | Space-delimited list of conditions describing a trade                                                                                                                                                      |
| £ | 828             | TrdType               | N                                                                                                                                                      | For optional use in reporting Trades                                                                                                                                                                       |
| £ | 574             | MatchType             | N                                                                                                                                                      | For optional use in reporting Trades                                                                                                                                                                       |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 84 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Errata

| Field Number | Field Name         | Required | Description                                                                                                                               |
| ------------ | ------------------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| 282          | MDEntryOriginator  | N        | (Deprecated in FIX.5.0)                                                                                                                   |
| 283          | LocationID         | N        | (Deprecated in FIX.5.0)                                                                                                                   |
| 284          | DeskID             | N        | (Deprecated in FIX.5.0)                                                                                                                   |
| 286          | OpenCloseSettlFlag | N        | Used if MDEntryType = Opening Price(4), Closing Price(5), or Settlement Price(6).                                                         |
| 59           | TimeInForce        | N        | For optional use when this Bid or Offer represents an order                                                                               |
| 432          | ExpireDate         | N        | For optional use when this Bid or Offer represents an order. ExpireDate and ExpireTime cannot both be specified in one Market Data Entry. |
| 126          | ExpireTime         | N        | For optional use when this Bid or Offer represents an order. ExpireDate and ExpireTime cannot both be specified in one Market Data Entry. |
| 110          | MinQty             | N        | For optional use when this Bid or Offer represents an order                                                                               |
| 18           | ExecInst           | N        | Can contain multiple instructions, space delimited.                                                                                       |
| 287          | SellerDays         | N        |                                                                                                                                           |
| 37           | OrderID            | N        | For optional use when this Bid, Offer, or Trade represents an order                                                                       |
| 198          | SecondaryOrderID   | N        | For optional use to support Hit/Take (selecting a specific order from the feed) without disclosing a private order id.                    |
| 299          | QuoteEntryID       | N        | For optional use when this Bid, Offer, or Trade represents a quote                                                                        |
| 1003         | TradeID            | N        | For optional use in reporting Trades                                                                                                      |
| 288          | MDEntryBuyer       | N        | For optional use in reporting Trades                                                                                                      |
| 289          | MDEntrySeller      | N        | For optional use in reporting Trades                                                                                                      |
| 346          | NumberOfOrders     | N        | In an Aggregated Book, used to show how many individual orders make up an MDEntry                                                         |
| 290          | MDEntryPositionNo  | N        | Display position of a bid or offer, numbered from most competitive to least competitive, per market side, beginning with 1                |
| 546          | Scope              | N        |                                                                                                                                           |
| 811          | PriceDelta         | N        |                                                                                                                                           |
| 451          | NetChgPrevDay      | N        |                                                                                                                                           |
| 58           | Text               | N        | Text to describe the Market Data Entry. Part of repeating group.                                                                          |
| 354          | EncodedTextLen     | N        | Must be set if EncodedText field is specified and must immediately precede it.                                                            |
| 355          | EncodedText        | N        | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the                                   |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 85 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                   August 18, 2011

| £ | 1023            | MDPriceLevel        | N               |                                                                                                                    |
| - | --------------- | ------------------- | --------------- | ------------------------------------------------------------------------------------------------------------------ |
| £ | 528             | OrderCapacity       | N               |                                                                                                                    |
| £ | 1024            | MDOriginType        | N               |                                                                                                                    |
| £ | 332             | HighPx              | N               |                                                                                                                    |
| £ | 333             | LowPx               | N               |                                                                                                                    |
| £ | 1025            | FirstPx             | N               | Indicates the first price of a trading session; can be a bid, ask, or a trade price.                               |
| £ | 31              | LastPx              | N               | Indicates the last price of a trading session; can be a bid, ask, or a trade price.                                |
| £ | 1020            | TradeVolume         | N               |                                                                                                                    |
| £ | 63              | SettlType           | N               |                                                                                                                    |
| £ | 64              | SettlDate           | N               | Indicates date on which instrument will settle. For NDFs required for specifying the "value date".                 |
| £ | 483             | TransBkdTime        | N               | For optional use in reporting Trades. Used to specify the time of trade agreement for privately negotiated trades. |
| £ | 60              | TransactTime        | N               | For optional use in reporting Trades. Used to specify the time of matching.                                        |
| £ | 1070            | MDQuoteType         | N               |                                                                                                                    |
| £ | 83              | RptSeq              | N               | Allows sequence number to be specified within a feed type                                                          |
| £ | 1048            | DealingCapacity     | N               | Identifies role of dealer; Agent, Principal, RisklessPrincipal                                                     |
| £ | 1026            | MDEntrySpotRate     | N               |                                                                                                                    |
| £ | 1027            | MDEntryForwardPoint | N               |                                                                                                                    |
| £ | component block |                     | N               |                                                                                                                    |
| £ | StatsIndGrp     | component block     |                 | N                                                                                                                  |
| £ |                 | Parties             | component block | N                                                                                                                  |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Inc

| Tag | FieldName | Req'd | Comments |
| --- | --------- | ----- | -------- |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 86 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                           August 18, 2011

# 267 NoMDEntryTypes

Y Number of MDEntryType fields requested.

# 269 MDEntryType

Y Must be the first field in this repeating group. This is a list of all the types of Market Data Entries that the firm requesting the Market Data is interested in receiving.

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Req

# MDRjctGrp component block

| Tag | FieldName     | Req'd | Comments                       |
| --- | ------------- | ----- | ------------------------------ |
| 816 | NoAltMDSource | N     |                                |
| 817 | AltMDSourceID | N     | Alternative Market Data Source |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Rjct

© Copyright, 2008-2009, FIX Protocol, Limited                                 Page 87 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                 August 18, 2011

# SecSizesGrp component block

| Tag  | FieldName     | Req'd | Comments                                                                                                               |
| ---- | ------------- | ----- | ---------------------------------------------------------------------------------------------------------------------- |
| 1177 | NoOfSecSizes  | N     | Number of entries following. Conditionally required when MDUpdateAction = New(0) and MDEntryType = Bid(0) or Offer(1). |
| 1178 | MDSecSizeType | N     | Defines the type of secondary size specified in MDSecSize(1179). Must be first field in this repeating group.          |
| 1179 | MDSecSize     | N     |                                                                                                                        |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SecSizesGrp

# StatsIndGrp component block

| Tag  | FieldName         | Req'd | Comments                                                                                                                                                         |
| ---- | ----------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1175 | NoStatsIndicators | N     | Number of statistics indicators                                                                                                                                  |
| 1176 | StatsType         | N     | Indicates that the MD Entry is eligible for inclusion in the type of statistic specified by the StatsType. Must be provided if NoStatsIndicators greater than 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element StatsIndGrp

© Copyright, 2008-2009, 2011, FIX Protocol, Limited                                        Page 88 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# StrmAsgnReqGrp component block

| Tag  | FieldName                  | Req'd | Comments                    |
| ---- | -------------------------- | ----- | --------------------------- |
| 1499 | NoAsgnReqs                 | N     | Stream Assignment Requests. |
| £    | component block \<Parties> | N     |                             |
| £    | component block            | N     |                             |
|      | \<StrmAsgnReqInstrmtGrp>   |       |                             |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Reqs

# StrmAsgnRptGrp component block

| Tag  | FieldName                  | Req'd | Comments                   |
| ---- | -------------------------- | ----- | -------------------------- |
| 1499 | NoAsgnReqs                 | N     | Stream Assignment Reports. |
| £    | component block \<Parties> | N     |                            |
| £    | component block            | N     |                            |
|      | \<StrmAsgnRptInstrmtGrp>   |       |                            |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Rpts

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited Page 89 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                 August 18, 2011

# StrmAsgnReqInstrmtGrp component block

| Tag | FieldName       | Req'd       | Comments |
| --- | --------------- | ----------- | -------- |
| 146 | NoRelatedSym    | N           |          |
|     | component block | N           |          |
|     | \<Instrument>   |             |          |
|     | 63              | SettlType   | N        |
|     | 271             | MDEntrySize | N        |
|     | 1500            | MDStreamID  | N        |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Instrmts

# StrmAsgnRptInstrmtGrp component block

| Tag | FieldName       | Req'd               | Comments |
| --- | --------------- | ------------------- | -------- |
| 146 | NoRelatedSym    | N                   |          |
|     | component block | N                   |          |
|     | \<Instrument>   |                     |          |
|     | 63              | SettlType           | N        |
|     | 1617            | StreamAsgnType      | N        |
|     | 1500            | MDStreamID          | N        |
|     | 1502            | StreamAsgnRejReason | N        |
|     | 58              | Text                | N        |
|     | 354             | EncodedTextLen      | N        |
|     | 355             | EncodedText         | N        |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Instrmts

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                         Page 90 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Market Data Request

Some systems allow the transmission of real-time quote, order, trade, trade volume, open interest, and/or other price information on a subscription basis. A Market Data Request is a general request for market data on specific securities or forex quotes.

A successful Market Data Request returns one or more Market Data messages containing one or more Market Data Entries. Each Market Data Entry is a Bid, an Offer, a Trade associated with a security, the opening, closing, or settlement price of a security, the buyer or seller imbalance for a security, the value of an index, the trading session high price, low price, or VWAP, or the trade volume or open interest in a security. Market Data Entries usually have a price and a quantity associated with them. For example, in an order book environment, requesting just the top of book will result in only two active Market Data Entries at a time – one for the best Bid and one for the best Offer. For a full book, the Bid and Offer side may each have several Market Data Entries. Each Market Data Entry might represent an aggregate for each price tier, and only one Market Data Entry per side per price would be active at a time. This is referred to as an Aggregated book.

When several Market Data Entries at one price tier could each represent a broker, Market Maker, ECN or Exchange’s quote in a security, or individual orders in a book, this is a Non-Aggregated book. Alternately, a Market Data Entry could represent a completed trade in a security, the value of an index, the opening, closing, or settlement price of an instrument, the trading session high price, low price, or VWAP, or the volume traded or open interest in a security.

If the message is used for disseminating imbalance information, conventions are as follows:

- MDEntrySize represents the size of the imbalance and is always a positive integer.
- A TradeCondition of either P or Q is required to indicate the side of the imbalance.
- Markets may wish to indicate the presence of an imbalance but not the actual size. In this case, MDEntrySize need not be specified.

One specifies whether a list of trades, a 1-sided or 2-sided book, index, opening, closing, settlement, high, low and VWAP prices and imbalance volumes should be returned by using the NoMDEntryTypes field and MDEntryType repeating group to list all MDEntryType values that should be returned.

# Types of Market Data Requests

1. A market data feed may consist of both Market Data Snapshot Full Refresh messages and Market Data Incremental Refresh messages.
2. The Market Data Request message is used to request a static book snapshot or subscribe to a stream of snapshots and updates.
3. Market Data Snapshot Full Refresh should be used to provide a snapshot of the market when Snapshot is requested using SubscriptionRequestType (263). Use of Market Data Incremental Refresh is being discouraged for this purpose.
4. Market Data Snapshot Full Refresh will be used to provide initial snapshot when Snapshot + Updates are requested using SubscriptionRequestType (263).
5. The Market Data Request scenarios that will be supported are as follows:

| Customer Requests                                                                                                                       | Subscription RequestType (263) | MDUpdateType(265) | Response Messages                                                    |
| --------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ | ----------------- | -------------------------------------------------------------------- |
| Requests state of the book and receives one and only one snapshot for each request (i.e. customer only wants single snapshot of prices) | 0=Snapshot                     | Not Provided      | Market Data Snapshot/Full Refresh message (only one message is sent) |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 91 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Requests state of the book

| 1 = Snapshot | 0 = Full Refresh        | Market Data Updates | Snapshot/Full Refresh messages only                 |
| ------------ | ----------------------- | ------------------- | --------------------------------------------------- |
| 1 = Snapshot | 1 = Incremental Refresh | Market Data Updates | Snapshot/Full Refresh message with updates provided |

# Indicating an Empty Book

1. An empty book contains no bids or asks and indicates that the market has no open orders in a given instrument. This can also be referred to as a “null” book.
2. When this occurs in a scenario in which the Snapshot Full Refresh Message is being used to provide a static snapshot or snapshot + updates then a special MDEntryType (tag 269) of “J” (Null Market) should be used.
3. The Snapshot Full Refresh Message should contain a single MDEntry with MDEntryType (269) = J specified. MDEntryPrice (270) = 0 and MDEntrySize (271) = 0 may also be provided but are not required. Other tags may be specified as well in order to convey the time and conditions under which the market generated a null book.

# Indicating a Crossed Book

1. If MDBookType = Top-of-Book or Price Depth, indicates that the market is crossed.
2. If MDBookType = Order Depth, indicates that the (order) entry is associated with conditions that can cause the book to lock or be locked or crossed. Such conditions include quantity conditions as All-Or-None (AON), MinQty and MatchIncement but also counterparties conditions as Acceptable or Unacceptable Counterparty. In the case such orders are included in the same book feed as normal orders, the user may choose to display crossed orders in a separate book view or indicate the “crossed” fact in another way.

While this document specifies many parameters and modes in a request, the recipient of the request is not required to support all of them. A Market Data Request Reject may be sent in response to a request indicating that it cannot be honored.

See VOLUME 7 - PRODUCT: FOREIGN EXCHANGE section for more detailed usage notes specific to Foreign Exchange.

The Market Data Request message format is as follows:

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 92 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                              August 18, 2011

# Market Data Request

| Tag                           | FieldName               | Req'd | Comments                                                                                                                                                                                                                                                                             |
| ----------------------------- | ----------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader                |                         | Y     | MsgType = V                                                                                                                                                                                                                                                                          |
| 262                           | MDReqID                 | Y     | Must be unique, or the ID of previous Market Data Request to disable if SubscriptionRequestType = Disable previous Snapshot + Updates Request (2).                                                                                                                                   |
| 263                           | SubscriptionRequestType | Y     | SubscriptionRequestType indicates to the other party what type of response is expected. A snapshot request only asks for current information. A subscribe request asks for updates as the status changes. Unsubscribe will cancel any future update messages from the counter party. |
| component block \<Parties>    |                         | N     | Insert here the set of Parties (firm identification) fields defined in "Common Components of Application Messages"                                                                                                                                                                   |
| 264                           | MarketDepth             | Y     |                                                                                                                                                                                                                                                                                      |
| 265                           | MDUpdateType            | N     | Required if SubscriptionRequestType = Snapshot + Updates (1).                                                                                                                                                                                                                        |
| 266                           | AggregatedBook          | N     |                                                                                                                                                                                                                                                                                      |
| 286                           | OpenCloseSettlFlag      | N     | Can be used to clarify a request if MDEntryType = Opening Price(4), Closing Price(5), or Settlement Price(6).                                                                                                                                                                        |
| 546                           | Scope                   | N     | Defines the scope(s) of the request                                                                                                                                                                                                                                                  |
| 547                           | MDImplicitDelete        | N     | Can be used when MarketDepth >= 2 and MDUpdateType = Incremental Refresh(1).                                                                                                                                                                                                         |
| component block \<MDReqGrp>   |                         | Y     | Number of MDEntryType fields requested.                                                                                                                                                                                                                                              |
| component block               |                         | Y     | Number of symbols (instruments) requested.                                                                                                                                                                                                                                           |
| \<InstrmtMDReqGrp>            |                         |       |                                                                                                                                                                                                                                                                                      |
| component block \<TrdgSesGrp> |                         | N     | Number of trading sessions for which the request is valid.                                                                                                                                                                                                                           |
| 815                           | ApplQueueAction         | N     | Action to take if application level queuing exists                                                                                                                                                                                                                                   |
| 812                           | ApplQueueMax            | N     | Maximum application queue depth that must be exceeded before queuing action is taken.                                                                                                                                                                                                |
| 1070                          | MDQuoteType             | N     |                                                                                                                                                                                                                                                                                      |
| StandardTrailer               |                         | Y     |                                                                                                                                                                                                                                                                                      |

FIXML Definition for this message – see http://www.fixprotocol.org for details

MktDataReq

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                           Page 93 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011

# Market Data - Snapshot / Full Refresh

The Market Data messages are used as the response to a Market Data Request message. In all cases, one Market Data message refers only to one Market Data Request. It can be used to transmit a 2-sided book of orders or list of quotes, a list of trades, index values, opening, closing, settlement, high, low, or VWAP prices, the trade volume or open interest for a security, or any combination of these.

Market Data messages sent as the result of a Market Data Request message will specify the appropriate MDReqID. Unsolicited Market Data messages can be sent; in such cases, MDReqID will not be present.

Market Data messages include many fields, and not all are required to be used. A firm may, at its option, choose to send the minimum fields required, or may choose to send more information, such as tick direction, tagging of best quotes, etc.

Market Data messages can take two forms. The first Market Data message format used for a Snapshot, or a Snapshot + Updates where MDUpdateType = Full Refresh (0) is as follows:

- For Market Data Requests where a Bid or Offer is added, changed, or deleted, every update to a Market Data Entry results in a new Market Data message that contains the entirety of the data requested for that instrument, not just the changed Market Data Entry. In other words, both sides of the market, or just one side in the case of a request of only bids or offers, for the depth requested, must be sent in one FIX Market Data message.
- A Market Data message may contain several trades, imbalances, an index value, opening, closing, settlement, high, low, and/or VWAP price for one instrument, as well as the traded volume and open interest, but only one instrument per message.
- Messages containing bids and/or offers cannot contain trades, imbalances, index value, opening, closing, settlement, high, low, and/or VWAP prices, trade volume, or open interest as separate entries.

# Refreshing Market Data in a Multicast Environment

Dissemination of market data messages in a multicast environment creates an issue that recovery of lost packets is not always feasible using a query method in high message volume situations. The Market Data Snapshot / Full Refresh message can be used to disseminate periodic full snapshots of the data (e.g. order book data). Recipients that join late or otherwise miss packets can get their data aligned by processing the Market Data Snapshots for one complete pass of the instruments.

The snapshot messages will always transmit the market data in the state that it was as of the last incremental refresh message. Snapshots never provide updates and can be ignored in regular processing except in the case of a system failure. Upon system restart the data flow will begin with a snapshot of each instrument. For the most part the recipient cannot ignore these snapshots. However, in some cases the snapshots cannot be ignored by the recipient. The RefreshIndicator (1187) is used to indicate to the recipient of which Snapshot message are redundant and can be ignored, and which are mandatory and must be processed because the message contains new data.

When connecting to the data feed, or after a loss of data, recipients should process Snapshot messages to recover their data, especially if the feed is for orderbook data. Once recovered, recipients can ignore snapshots that have RefreshIndicator = N. If RefreshIndicator = Y then the recipient should discard their data and replace it with the information in the Snapshot message.

See VOLUME 7 - PRODUCT: FOREIGN EXCHANGE section for more detailed usage notes specific to Foreign Exchange.

# Market Data - Snapshot / Full Refresh

| Tag                                                    | FieldName | Req'd | Comments |
| ------------------------------------------------------ | --------- | ----- | -------- |
| © Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited |           |       |          |

Page 94 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# StandardHeader

| MsgType         | W |
| --------------- | - |
| component block | N |

# ApplicationSequenceControl

| 911  | TotNumReports        | N | Total number or reports returned in response to a request.                                                                       |
| ---- | -------------------- | - | -------------------------------------------------------------------------------------------------------------------------------- |
| 963  | MDReportID           | N | Unique identifier for Market Data Report                                                                                         |
| 715  | ClearingBusinessDate | N |                                                                                                                                  |
| 1021 | MDBookType           | N | Describes the type of book for which the feed is intended. Can be used when multiple feeds are provided over the same connection |
| 1173 | MDSubBookType        | N | Can be used to define a subordinate book.                                                                                        |
| 264  | MarketDepth          | N | Can be used to define the current depth of the book.                                                                             |
| 1022 | MDFeedType           | N | Describes a class of service for a given data feed, ie Regular and Market Maker                                                  |
| 1187 | RefreshIndicator     | N |                                                                                                                                  |
| 75   | TradeDate            | N | Used to specify the trading date for which a set of market data applies                                                          |
| 262  | MDReqID              | N | Conditionally required if this message is in response to a Market Data Request.                                                  |
| 1500 | MDStreamID           | N |                                                                                                                                  |

# component block &#x3C;Instrument>

Y Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages"

# component block &#x3C;UndInstrmtGrp>

N Number of underlyings

# component block &#x3C;InstrmtLegGrp>

N Required for multileg quotes

# 291

FinancialStatus N

# 292

CorporateAction N

# 451

NetChgPrevDay N

# component block &#x3C;MDFullGrp>

Y Number of entries following.

# 813

ApplQueueDepth N Depth of application messages queued for transmission as of delivery of this message

# 814

ApplQueueResolution N Action taken to resolve application queuing

# component block &#x3C;RoutingGrp>

N

# StandardTrailer

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to the FIXML element MktDataSnpFullRefresh

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited
Page 95 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Market Data - Incremental Refresh

The second Market Data message format is used for incremental updates. Market Data Entries may have an MDEntryID unique among all currently active Market Data Entries so they can be referenced for the purposes of deleting and changing them later. When changing a Market Data Entry, it may keep the same MDEntryID, in which case only MDEntryID would be populated, or the MDEntryID may change, in which case MDEntryID will contain the new ID, and MDEntryRefID will contain the ID of the Market Data Entry being changed. An MDEntryID can be reused within a day only if it has first been deleted. Alternately, in the case of displaying the best quotes of Market Makers or Exchanges, and not orders in an order book, MDEntryID can be omitted for simplification. In this case, a New Market Data Entry will replace the previous best quote for that side and symbol for the specified Market Maker or Exchange. Deletion of a Market Data Entry would not specify an MDEntryID or MDRefID, and would remove the most recent Market Data Entry for the specified symbol, side, and Market Maker or Exchange. A Change of a Market Data Entry would not specify an MDEntryID or MDRefID, and would replace the most recent Market Data Entry for the specified symbol, side, and Market Maker or Exchange.

The Market Data message for incremental updates may contain any combination of new, changed, or deleted Market Data Entries, for any combination of instruments, with any combination of trades, imbalances, quotes, index values, open, close, settlement, high, low, and VWAP prices, trade volume and open interest so long as the maximum FIX message size is not exceeded. All of these types of Market Data Entries can be changed and deleted.

Adding, Changing, or Deleting Market Data Entries requires special consideration of the MDEntryPositionNo field, if the sender wishes to specify it and the receiver wishes to process it. For example, assume ten bids for a security. Adding a bid with MDEntryPositionNo = 4 requires the receiver to shift down other Market Data Entries, i.e. the Market Data Entry in the 4th display position will shift to the 5th, the 5th shifts to the 6th, etc. until the 10th shifts to the 11th. The sender must NOT send a modification of all MDEntries in the 4th through 10th positions just to update the MDEntryPositionNo field; the recipient must infer the change. Similarly, deleting a Market Data Entry in the 7th position causes the 8th Market Data Entry to move into the 7th position, the 9th to shift into the 8th position, etc. A Change of the MDEntryPositionNo field of a Market Data Entry causes the Market Data Entries lying between the old and new positions to shift. For instance, a Market Data Entry that occupied the 5th position is changed to the 8th position. This means that the Market Data Entry in the 6th position shifts up to the 5th position, the 7th position shifts to the 6th, and what was in the 8th position shifts into the 7th to make room for the changed Market Data Entry that is being moved into the 8th position.

Several techniques are employed to conserve bandwidth:

- An instrument only needs to be identified when a Market Data Entry is first created.
- In cases where the identification of an instrument is long, the sender has the option of referring to a previous active Market Data Entry of the same instrument instead of duplicating the information.
- A new Market Data Entry will default to the same instrument of the previous Market Data Entry in the same Market Data message if neither Symbol nor MDEntryRefID are specified.
- In the case of a change in a Market Data Entry, only the fields changing need to be sent as part of the change to the Market Data Entry; for example, a change of the MDEntrySize but not the MDEntryPx or other attributes of the Market Data Entry only requires listing the MDEntrySize field, in addition to MDUpdateAction and MDEntryID if used in the original Market Data Entry.
- When creating a new Market Data Entry with a future or option instrument similar to the instrument in the previous Market Data Entry in the same FIX message, one may send just symbol identification fields that have changed, such as MaturityMonthYear, MaturityDay, StrikePrice, OptAttribute, and SecurityExchange.
- MDEntryID can be reused within the same day after it is deleted. This is helpful for distributing order books because an order that is suspended and then reinstated can have its MDEntryID deleted upon suspension and later reused, with MDUpdateAction = New(0) upon reinstatement, thus avoiding having to re-map the MDEntryID.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 96 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Market Data - Incremental Refresh

| Tag             | FieldName           | Req'd | Comments                                                                                                                         |
| --------------- | ------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                     | Y     | MsgType = X                                                                                                                      |
| component block |                     | N     |                                                                                                                                  |
|                 |                     |       |                                                                                                                                  |
| 1021            | MDBookType          | N     | Describes the type of book for which the feed is intended. Can be used when multiple feeds are provided over the same connection |
| 1022            | MDFeedType          | N     | Describes a class of service for a given data feed, ie Regular and Market Maker                                                  |
| 75              | TradeDate           | N     | Used to specify the trading date for which a set of market data applies                                                          |
| 262             | MDReqID             | N     | Conditionally required if this message is in response to a Market Data Request.                                                  |
| component block |                     | Y     | Number of entries following.                                                                                                     |
| 813             | ApplQueueDepth      | N     | Depth of application messages queued for transmission as of delivery of this message                                             |
| 814             | ApplQueueResolution | N     | Action taken to resolve application queuing                                                                                      |
| component block |                     | N     |                                                                                                                                  |
| StandardTrailer |                     | Y     |                                                                                                                                  |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to the FIXML element MktDataIncRefresh

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                      Page 97 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                             August 18, 2011

# Market Data Request Reject

The Market Data Request Reject is used when the broker cannot honor the Market Data Request, due to business or technical reasons. Brokers may choose to limit various parameters, such as the size of requests, whether just the top of book or the entire book may be displayed, and whether Full or Incremental updates must be used.

The market data request reject message format is as follows:

| Tag                          | FieldName      | Req'd | Comments                                                                                                                       |
| ---------------------------- | -------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader               |                | Y     | MsgType = Y                                                                                                                    |
| 262                          | MDReqID        | Y     | Must refer to the MDReqID of the request.                                                                                      |
| component block \<Parties>   |                | N     | Insert here the set of Parties (firm identification) fields defined in "Common Components of Application Messages"             |
| 281                          | MDReqRejReason | N     |                                                                                                                                |
| component block \<MDRjctGrp> |                | N     |                                                                                                                                |
| 58                           | Text           | N     |                                                                                                                                |
| 354                          | EncodedTextLen | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355                          | EncodedText    | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
| StandardTrailer              |                | Y     |                                                                                                                                |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element MktDataReqRej

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                          Page 98 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Stream Assignment Request

In certain markets where market data aggregators fan out to end clients the pricing streams provided by the price makers, the price maker may assign the clients to certain pricing streams that the price maker publishes via the aggregator. An example of this use is in the FX markets where clients may be assigned to different pricing streams based on volume bands and currency pairs.

| Price maker                                                 | Aggregator                                                  |
| ----------------------------------------------------------- | ----------------------------------------------------------- |
| (e.g. Bank of America)                                      | (e.g. RTFX)                                                 |
| End Client                                                  | End Client                                                  |
| Stream A1 – EURUSD 0-2M, 2-10M, 10-20M volume bands, tier 1 | Stream B1 – GBPUSD 0-5M, 5-10M, 10-15M volume bands, tier 1 |
| Stream A2 – EURUSD 0-2M, 2-10M, 10-20M volume bands, tier 2 | Stream B2 – GBPUSD 0-5M, 5-10M, 10-15M volume bands, tier 2 |

The Stream Assignment set of messages facilitates the automation of assigning clients to specific price streams by the price makers and allowing the price maker to notify the aggregator of these assignments.

| Price Maker                      | Aggregator                |
| -------------------------------- | ------------------------- |
| Replies with                     | Stream Assignment Request |
| Stream ID                        | Client                    |
| Ccy pair                         | Tenor                     |
| Stream assignment                | Volume band               |
| +/-ccy pair                      | Market Data Request       |
| +/-tenor                         | If stream doesn’t exist   |
| +/-volume band                   |                           |
| Market Data Snapshot/Incremental |                           |
| -Ccy pair                        |                           |
| -Size                            |                           |
| -Tenor                           |                           |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited

Page 99 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                       August 18, 2011

# Stream Assignment Request

The Stream Assignment Request message is used by the aggregator and sent to the price maker to request stream assignments for one or more clients. The response to this message is the Stream Assignment Report.

The message definition of the StreamAssignmentRequest is:

| Tag                               | FieldName         | Req'd | Comments                            |
| --------------------------------- | ----------------- | ----- | ----------------------------------- |
| StandardHeader                    |                   | Y     | MsgType = CC                        |
| 1497                              | StreamAsgnReqID   | Y     | Unique identifier of the request.   |
| 1498                              | StreamAsgnReqType | Y     | Type of assignment being requested. |
| component block \<StrmAsgnReqGrp> |                   | Y     | Assignment requests                 |
| StandardTrailer                   |                   | Y     |                                     |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element StrmAsgnReq

# Example:

This is an example of how the NoAssgnReq repeating group is used for three types of requests. In the first assignment request instance, a party (FirmA) needs two stream assignments for two CCY pairs (EUR/USD and USD/JPY), in the second instance there are two parties (FirmB and FirmC) that needs an assignment for a single CCY pair stream (EUR/JPY), and the third instance is primarily to illustrate the use of the PartySubID fields to further identify the party.

| NoAssgnReq     | 3                  |
| -------------- | ------------------ |
| NoPartyIDs     | 1                  |
| PartyID        | FirmA              |
| PartyIDSource  | (ID source scheme) |
| PartyRole      | 11                 |
| NoRelatedSym   | 2                  |
| Symbol         | EUR/USD            |
| Symbol         | USD/JPY            |
| NoPartyIDs     | 2                  |
| PartyID        | FirmB              |
| PartyIDSource  | (ID source scheme) |
| PartyRole      | 11                 |
| PartyID        | FirmC              |
| PartyIDSource  | (ID source scheme) |
| PartyRole      | 11                 |
| NoRelatedSym   | 1                  |
| Symbol         | EUR/JPY            |
| NoPartyIDs     | 1                  |
| PartyID        | FirmE              |
| PartyIDSource  | (ID source scheme) |
| PartyRole      | 11                 |
| NoPartySubIDs  | 1                  |
| PartySubID     | xyz                |
| PartySubIDType | taker group        |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                             Page 100 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                            August 18, 2011

# Stream Assignment Report

The StreamAssignmentReport message is in response to the StreamAssignmentRequest message. It provides information back to the aggregator as to which clients to assign to receive which price stream based on requested CCY pair. This message can be sent unsolicited to the Aggregator from the Price Maker.

The message definition for StreamAssignmentReport is:

| Tag                               | FieldName         | Req'd | Comments                                                                                                                                                                |
| --------------------------------- | ----------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                    |                   | Y     | MsgType = CD                                                                                                                                                            |
| 1501                              | StreamAsgnRptID   | Y     | Unique identifier of the Stream Assignment Report.                                                                                                                      |
| 1498                              | StreamAsgnReqType | N     | Required if report is being sent in response to a StreamAssignmentRequest. The value should be the same as the value in the corresponding request.                      |
| 1497                              | StreamAsgnReqID   | N     | Conditionally required if Stream Assignment Report is being sent in response to a StreamAssignmentRequest(MsgType=CC). Not required for unsolicited stream assignments. |
| component block \<StrmAsgnRptGrp> |                   | N     | Stream assignments                                                                                                                                                      |
| StandardTrailer                   |                   | Y     |                                                                                                                                                                         |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element StrmAsgnRpt

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                         Page 101 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                        August 18, 2011

# Stream Assignment Report Ack

This message is used to respond to the Stream Assignment Report, to either accept or reject an unsolicited assignment.

The message definition for StreamAssignmentReportAck is:

| Tag             | FieldName           | Req'd | Comments                                                                                                   |
| --------------- | ------------------- | ----- | ---------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                     | Y     | MsgType = CE                                                                                               |
| 1503            | StreamAsgnAckType   | Y     |                                                                                                            |
| 1501            | StreamAsgnRptID     | Y     |                                                                                                            |
| 1502            | StreamAsgnRejReason | N     |                                                                                                            |
| 58              | Text                | N     | Can be used to provide additional information regarding the assignment report, such as reject description. |
| 354             | EncodedTextLen      | N     |                                                                                                            |
| 355             | EncodedText         | N     |                                                                                                            |
| StandardTrailer |                     | Y     |                                                                                                            |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element StrmAsgnRptACK

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                              Page 102 of 200
---
Version 5.0 Service Pack 2 - Errata        VOLUME 3                                              August 18, 2011

# CATEGORY: MARKET STRUCTURE REFERENCE DATA

# Market Structure Reference Data Component Blocks

This section lists the component blocks used exclusively by the messages defined for Market Structure Reference Data.

# TrdSessLstGrp component block

| Tag             | FieldName            | Req'd | Comments                                                                                                   |
| --------------- | -------------------- | ----- | ---------------------------------------------------------------------------------------------------------- |
| 386             | NoTradingSessions    | Y     |                                                                                                            |
| 336             | TradingSessionID     | Y     | Identifier for Trading Session                                                                             |
| 625             | TradingSessionSubID  | N     |                                                                                                            |
| 1327            | TradSesUpdateAction  | N     |                                                                                                            |
| 207             | SecurityExchange     | N     | (Deprecated in FIX.5.0SP1)                                                                                 |
| 1301            | MarketID             | N     | Market for which Trading Session applies                                                                   |
| 1300            | MarketSegmentID      | N     | Market Segment for which Trading Session applies                                                           |
| 1326            | TradingSessionDesc   | N     |                                                                                                            |
| 338             | TradSesMethod        | N     | Method of Trading                                                                                          |
| 339             | TradSesMode          | N     | Trading Session Mode                                                                                       |
| 325             | UnsolicitedIndicator | N     | "Y" if message is sent unsolicited as a result of a previous subscription request.                         |
| 340             | TradSesStatus        | Y     | State of trading session.                                                                                  |
| 567             | TradSesStatusRejReas | N     | Used with TradSesStatus = "Request Rejected"                                                               |
| 341             | TradSesStartTime     | N     | Starting time of trading session                                                                           |
| 342             | TradSesOpenTime      | N     | Time of the opening of the trading session                                                                 |
| 343             | TradSesPreCloseTime  | N     | Time of pre-close of trading session                                                                       |
| 344             | TradSesCloseTime     | N     | Closing time of trading session                                                                            |
| 345             | TradSesEndTime       | N     | End time of trading session                                                                                |
| 387             | TotalVolumeTraded    | N     |                                                                                                            |
| component block |                      | N     | Insert here the set of "TradingSessionRules" fields defined in "common components of application messages" |
| 60              | TransactTime         | N     |                                                                                                            |
| 58              | Text                 | N     |                                                                                                            |
| 354             | EncodedTextLen       | N     | Must be set if EncodedText field is specified and must immediately precede it.                             |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                           Page 103 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

£ 355 EncodedText N Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element TrdSessLstGrp

# Market Definition Request

The Market Definition Request message is used to request for market structure information from the Respondent that receives this request. Fields that are specified will act as "filters" for the request. For example, if MarketID is specified then only market structure information for that specified market should be sent back if available. If MarketID is not specified then the request is for all available market structure information. The Market Definition Request can also indicate to the Respondent whether the request is for a snapshot of requested information, subscribe to market structure information, or to unsubscribe to an earlier subscription request. This is done via the SubscriptionRequestType (263) field.

| Tag             | FieldName               | Req'd | Comments                                                                                                                                             |
| --------------- | ----------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                         | Y     | MsgType = BT                                                                                                                                         |
| 1393            | MarketReqID             | Y     | Must be unique, or the ID of previous Market Segment Request to disable if SubscriptionRequestType = Disable previous Snapshot + Updates Request(2). |
| 263             | SubscriptionRequestType | Y     |                                                                                                                                                      |
| 1301            | MarketID                | N     | Conditionally required if MarketSegmentID(1300) is specified on the request                                                                          |
| 1300            | MarketSegmentID         | N     |                                                                                                                                                      |
| 1325            | ParentMktSegmID         | N     | Specifies that the Market Segment is a sub segment of the Market Segment defined in this field.                                                      |
| StandardTrailer |                         | Y     |                                                                                                                                                      |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element MktDefReq

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 104 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                              August 18, 2011

# Market Definition

The Market Definition message is used to respond to Market Definition Request. In a subscription, it will be used to provide the initial snapshot of the information requested. Subsequent updates are provided by the Market Definition Update Report. This message is associated with a list of trading sessions (and subsessions) applicable for the segment - the list is published using the Trading Session List message.

# Market Definition

| Tag             | FieldName             | Req'd | Comments                                                                                                                                 |
| --------------- | --------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                       | Y     | MsgType = BU                                                                                                                             |
| component block |                       | N     |                                                                                                                                          |
|                 |                       |       |                                                                                                                                          |
| 1394            | MarketReportID        | Y     | Unique identifier for each Market Definition message                                                                                     |
| 1393            | MarketReqID           | N     |                                                                                                                                          |
| 1301            | MarketID              | Y     |                                                                                                                                          |
| 1300            | MarketSegmentID       | N     |                                                                                                                                          |
| 1396            | MarketSegmentDesc     | N     |                                                                                                                                          |
| 1397            | EncodedMktSegmDescLen | N     | Must be set if EncodedMktSegmDesc field is specified and must immediately precede it.                                                    |
| 1398            | EncodedMktSegmDesc    | N     | Encoded (non-ASCII characters) representation of the MarketSegmDesc field in the encoded format specified via the MessageEncoding field. |
| 1325            | ParentMktSegmID       | N     | Specifies that the Market Segment is a sub segment of the Market Segment defined in this field.                                          |
| 15              | Currency              | N     | The default trading currency                                                                                                             |
| component block |                       | N     | Insert here the set of "BaseTradingRules" fields defined in "common components of application messages"                                  |
|                 |                       |       |                                                                                                                                          |
| component block |                       | N     | Insert here the set of "OrdTypeRules" fields defined in "common components of application messages"                                      |
| component block |                       | N     | Insert here the set of "TimeInForceRules" fields defined in "common components of application messages"                                  |
| component block |                       | N     | Insert here the set of "ExecInstRules" fields defined in "common components of application messages"                                     |
| 60              | TransactTime          | N     |                                                                                                                                          |
| 58              | Text                  | N     | Comment, instructions, or other identifying information.                                                                                 |
| 354             | EncodedTextLen        | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                           |
| 355             | EncodedText           | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.           |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                 Page 105 of 200


---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011


# FIXML Definition for this message

– see http://www.fixprotocol.org for details

Refer to FIXML element MktDef

© Copyright, 2008-2011, FIX Protocol, Limited

Page 106 of 200



---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                             August 18, 2011

# Market Definition Update Report

In a subscription for market structure information, this message is used once the initial snapshot of the information has been sent using the Market Definition message.

# Market Definition Update Report

| Tag             | FieldName             | Req'd | Comments                                                                                                                                 |
| --------------- | --------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                       | Y     | MsgType = BV                                                                                                                             |
| component block |                       | N     |                                                                                                                                          |
|                 |                       |       |                                                                                                                                          |
| 1394            | MarketReportID        | Y     | Unique identifier for each Market Definition message                                                                                     |
| 1393            | MarketReqID           | N     |                                                                                                                                          |
| 1395            | MarketUpdateAction    | N     | Specifies the action taken                                                                                                               |
| 1301            | MarketID              | Y     |                                                                                                                                          |
| 1300            | MarketSegmentID       | N     |                                                                                                                                          |
| 1396            | MarketSegmentDesc     | N     |                                                                                                                                          |
| 1397            | EncodedMktSegmDescLen | N     | Must be set if EncodedMktSegmDesc field is specified and must immediately precede it.                                                    |
| 1398            | EncodedMktSegmDesc    | N     | Encoded (non-ASCII characters) representation of the MarketSegmDesc field in the encoded format specified via the MessageEncoding field. |
| 1325            | ParentMktSegmID       | N     | Specifies that the Market Segment is a sub segment of the Market Segment defined in this field.                                          |
| 15              | Currency              | N     | The default trading currency                                                                                                             |
| component block |                       | N     | Insert here the set of "BaseTradingRules" fields defined in "common components of application messages"                                  |
|                 |                       |       |                                                                                                                                          |
| component block |                       | N     | Insert here the set of "OrdTypeRules" fields defined in "common components of application messages"                                      |
| component block |                       | N     | Insert here the set of "TimeInForceRules" fields defined in "common components of application messages"                                  |
| component block |                       | N     | Insert here the set of "ExecInstRules" fields defined in "common components of application messages"                                     |
| 60              | TransactTime          | N     |                                                                                                                                          |
| 58              | Text                  | N     | Comment, instructions, or other identifying information.                                                                                 |
| 354             | EncodedTextLen        | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                           |
| 355             | EncodedText           | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.           |
| StandardTrailer |                       | Y     |                                                                                                                                          |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                 Page 107 of 200
---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# FIXML Definition for this message

– see http://www.fixprotocol.org for details

Refer to FIXML element MktDefUpdtRpt

© Copyright, 2008-2011, FIX Protocol, Limited

Page 108 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Trading Session Status Request

The Trading Session Status Request is used to request information on the status of a market. With the move to multiple sessions occurring for a given trading party (morning and evening sessions for instance) there is a need to be able to provide information on what product is trading on what market. The Trading Session Status Request message can be used to inquire the trading status of a trading party. The Trading Session Status message can be used to subscribe to updates to the status of a trading session by setting the RequestType field to 1. To list the securities available during a particular trading session, see the SecurityDefinitionRequest message.

| Tag             | FieldName               |
| --------------- | ----------------------- |
| StandardHeader  |                         |
| 335             | TradSesReqID            |
| 1301            | MarketID                |
| 1300            | MarketSegmentID         |
| 336             | TradingSessionID        |
| 625             | TradingSessionSubID     |
| 338             | TradSesMethod           |
| 339             | TradSesMode             |
| 263             | SubscriptionRequestType |
| 207             | SecurityExchange        |
| StandardTrailer |                         |

# Trading Session Status Request

| Req'd | Comments                                                                                                                                                      |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Y     | MsgType = g (lowercase)                                                                                                                                       |
| Y     | Must be unique, or the ID of previous Trading Session Status Request to disable if SubscriptionRequestType = Disable previous Snapshot + Updates Request (2). |
| N     | Market for which Trading Session applies                                                                                                                      |
| N     | Market Segment for which Trading Session applies                                                                                                              |
| N     | Trading Session for which status is being requested                                                                                                           |
| N     |                                                                                                                                                               |
| N     | Method of trading                                                                                                                                             |
| N     | Trading Session Mode                                                                                                                                          |
| Y     |                                                                                                                                                               |
| N     | (Deprecated in FIX.5.0SP1)                                                                                                                                    |
| Y     |                                                                                                                                                               |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element TrdgSesStatReq

© Copyright, 2008-2011, FIX Protocol, Limited

Page 109 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011

# Trading Session Status

The Trading Session Status provides information on the status of a market. For markets multiple trading sessions on multiple-markets occurring (morning and evening sessions for instance), this message is able to provide information on what products are trading on what market during what trading session.

# Trading Session Status

| Tag             | FieldName              | Req'd | Comments                                                                                                                       |
| --------------- | ---------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader  |                        | Y     | MsgType = h (lowercase)                                                                                                        |
| component block |                        | N     |                                                                                                                                |
|                 |                        |       |                                                                                                                                |
| 335             | TradSesReqID           | N     | Provided for a response to a specific Trading Session Status Request message (snapshot).                                       |
| 1301            | MarketID               | N     | Market for which Trading Session applies                                                                                       |
| 1300            | MarketSegmentID        | N     | Market Segment for which Trading Session applies                                                                               |
| 336             | TradingSessionID       | Y     | Identifier for Trading Session                                                                                                 |
| 625             | TradingSessionSubID    | N     |                                                                                                                                |
| 338             | TradSesMethod          | N     | Method of trading:                                                                                                             |
| 339             | TradSesMode            | N     | Trading Session Mode                                                                                                           |
| 325             | UnsolicitedIndicator   | N     | Set to 'Y' if message is sent unsolicited as a result of a previous subscription request.                                      |
| 340             | TradSesStatus          | Y     | State of the trading session                                                                                                   |
| 1368            | TradSesEvent           | N     | Identifies an event related to the trading status of a trading session                                                         |
| 567             | TradSesStatusRejReason | N     | Use with TradSesStatus = "Request Rejected"                                                                                    |
| 341             | TradSesStartTime       | N     | Starting time of the trading session                                                                                           |
| 342             | TradSesOpenTime        | N     | Time of the opening of the trading session                                                                                     |
| 343             | TradSesPreCloseTime    | N     | Time of the pre-close of the trading session                                                                                   |
| 344             | TradSesCloseTime       | N     | Closing time of the trading session                                                                                            |
| 345             | TradSesEndTime         | N     | End time of the trading session                                                                                                |
| 387             | TotalVolumeTraded      | N     |                                                                                                                                |
| 58              | Text                   | N     |                                                                                                                                |
| 354             | EncodedTextLen         | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355             | EncodedText            | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
| component block | \<Instrument>          | N     |                                                                                                                                |
| StandardTrailer |                        | Y     |                                                                                                                                |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                     Page 110 of 200
---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011


# FIXML Definition for this message

– see http://www.fixprotocol.org for details

Refer to FIXML element TrdgSesStat

© Copyright, 2008-2011, FIX Protocol, Limited

Page 111 of 200



---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

# Trading Session List Request

The Trading Session List Request is used to request a list of trading sessions available in a market place and
the state of those trading sessions. The request can be modified to request status on a particular trading session
(by specifying the TradingSessionID (tag 336) and TradingSessionSubID (tag 625) (if used by the market
place). The request can be used to request a list of trading sessions that use a particular trading method or mode
(such as electronic) by specifying the TradSesMethod (tag 338) and/or TradSesMode( tag 339).

A successful request will result in a response from the counterparty of a Trading Session List (MsgType=BJ)
message that contains a list of zero or more trading sessions.
It is recommended that the TradSesReqID be used to provide a unique identifier for the request. This value
should be returned by the counterparty in the Trading Session List messages sent in response to the request.
The Trading Session List Request follows the standard request model in providing the
SubscriptionRequestType (tag 263) field which can be used to obtain a snapshot of trading session
information, subscribe for a snapshot with subsequent updates, or to unsubscribe from a previous subscription
request.

| Tag             | FieldName        |              |                         |                     |
| --------------- | ---------------- | ------------ | ----------------------- | ------------------- |
| StandardHeader  | 335              | TradSesReqID |                         |                     |
| 1301            | MarketID         |              |                         |                     |
| 1300            | MarketSegmentID  |              |                         |                     |
|                 |                  | 336          | TradingSessionID        |                     |
|                 |                  |              | 625                     | TradingSessionSubID |
| 207             | SecurityExchange |              |                         |                     |
|                 |                  |              | 338                     | TradSesMethod       |
|                 |                  |              | 339                     | TradSesMode         |
|                 |                  | 263          | SubscriptionRequestType |                     |
| StandardTrailer |                  |              |                         |                     |

# Trading Session List Request

| Req'd | Comments                                                                                                                                                     |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Y     | MsgType = BI                                                                                                                                                 |
| Y     | Must be unique, or the ID of previous Trading Session Status Request to disable if SubscriptionRequestType = Disable previous Snapshot + Update Request (2). |
| N     | Market for which Trading Session applies                                                                                                                     |
| N     | Market Segment for which Trading Session applies                                                                                                             |
| N     | Trading Session for which status is being requested                                                                                                          |
| N     |                                                                                                                                                              |
| N     | (Deprecated in FIX.5.0SP1)                                                                                                                                   |
| N     | Method of Trading                                                                                                                                            |
| N     | Trading Session Mode                                                                                                                                         |
| Y     |                                                                                                                                                              |
| Y     |                                                                                                                                                              |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element TrdSessListReq

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                        Page 112 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Trading Session List

The Trading Session List message is sent as a response to a Trading Session List Request. The Trading Session List should contain the characteristics of the trading session and the current state of the trading session. The message could be relayed every trading day, or at least when trading sessions are changed. The user of the message has the ability to relay either Trading Sessions only or, if applicable, Trading SubSessions. Depending on characteristics of the market, the various Time fields may apply. The Trading Session List should return the TradSesReqID(tag 335) value from the Trading Session List Request originally sent by a counterparty.

# Trading Session List

| Tag             | FieldName    | Req'd | Comments                                                                               |
| --------------- | ------------ | ----- | -------------------------------------------------------------------------------------- |
| StandardHeader  |              | Y     | MsgType = BJ                                                                           |
| component block |              | N     |                                                                                        |
|                 |              |       |                                                                                        |
| 335             | TradSesReqID | N     | Provided for a response to a specific Trading Session List Request message (snapshot). |
| component block |              | Y     |                                                                                        |
| StandardTrailer |              | Y     |                                                                                        |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element TrdSessList

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                      Page 113 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011

# Trading Session List Update Report

The Trading Session List Update Report is used by marketplaces to provide intra-day updates of trading sessions when there are changes to one or more trading sessions.

# Trading Session List Update Report

| Tag             | FieldName        | Req'd | Comments                                                                               |
| --------------- | ---------------- | ----- | -------------------------------------------------------------------------------------- |
| StandardHeader  |                  | Y     | MsgType = BS                                                                           |
| component block |                  | N     |                                                                                        |
|                 | 335 TradSesReqID | N     | Provided for a response to a specific Trading Session List Request message (snapshot). |
| component block |                  | Y     |                                                                                        |
| StandardTrailer |                  | Y     |                                                                                        |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element TrdSessListUpdt

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                               Page 114 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                   August 18, 2011

# Product Reference and Market Structure Data Model

# Overview

A marketplace may group trading into separate markets (“exchanges”). Such grouping is frequent when various asset classes are traded in one and the same market place, e.g. Stock, Fixed Income, Options and Futures. Another type of segmentation occurs when one exchange covers multiple domiciles. The various markets may have different members (or trading participants) but are served by separate trading systems.

Within a market, various market segments (or product groups) could be devised to cover specific market needs. One market segment could, for example, be specialized for a wholesale market, another for retail. Separate market segments could be used for liquid stock using continuous auto-execution facilities, while another segment uses recurring call auctions to best fit less liquid instruments. Market segments could also represent trading venues as in the case an exchange offers both a floor and electronic trading.

Each market segment covers a number of instruments (or order books) and could have a distinct trading schedule and specific trading rules. It should be noted that Instruments and others are not created as an effect of the Market Definition message. Instrument creation or definition is done via the Security Definition message.

A variety of facts can be associated with a Market Segment:

- The identifier for the market for which the segment applies
- A description or free text name of the market segment
- A reference to a higher level market segment (enabling a hierarchy of segments)
- A list of instruments traded at the segment. It is proposed that this list is relayed using the Security List message or multiple Security Definition messages.
- The trading schedule for the market segment. It is proposed that the Trading Session List message is used to relay the schedule.
- Default trading rules applying to all instruments unless overridden at the trading session or individual security level:
- Trading currency
- Price Type used for standard quoting and trading
- Tick rules. Although many markets use a single tick size, other markets supports so-called “tick size tables” where the tick increases with the size of the price. With "tick size tables" penny price increments may be used for security traded in smaller unit prices, while nickel, for example, increments are used for higher unit prices.
- Lot sizes. Although many markets use a single lot size, some markets support integrated books with a separation of odd and round lot orders. In some cases additional lot sizes are also used, for example, for block trades.

As trading sessions in most cases are defined per market segment, trading sessions often need to be qualified by the market segment and sometimes by the exchange/market. Note the same TradingSessionID can apply to many Market Segments. For example, an “Opening” trading session may apply to all market segments, so when a Trading Session Status is relayed the TradingSessionID needs to be qualified by the MarketID and MarketSegmentID. In Orders, however, specifying the TradingSessionID when the Order should expire does not need qualification of MarketID or MarketSegmentID as the context is normally implied by the security being traded.

The diagrams below depicts the data model used in the Security Definition, Derivative Security List, Security List and Market Definition message sets.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                        Page 115 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                    August 18, 2011

# Figure 1: Security and Trading Session rules components

| Base Trading Rules            |                         | ~~~~                             |
| ----------------------------- | ----------------------- | -------------------------------- |
| xTickRules>                   | TrudlingS essionID      | NaStrikeRules                    |
| NoTickRules                   | TrudingSessiunSubID     | SuikeRulelD                      |
| StatTickPriceRange            |                         | EndStrikePxRange                 |
| cAnctrment                    | \~rdTypeRules >         | StrikeExerciseStyle              |
| TickRuleType                  | NoOrTypeRules           | (rdTyre                          |
| \~tTypeRules>                 | NLulTypeRules           | TimlnForceRules >                |
| NoMaturityRules               | MazurityRul\[D          | LotType                          |
| Nol ielnFonekults             | MMYFormat               | Minl otSize                      |
|                               |                         | TittleInForce                    |
|                               |                         | StatMMY                          |
|                               |                         | EndMMY                           |
| FriceLimits >                 |                         | NoExeclnsRules                   |
| PriceLimitType                | LowLimitPrice           | HighLimiPrice                    |
| ExeclinstValu?                |                         | Geconde PriccLimicType           |
| Secondun LowLimitPnce         | SeconduryHighLimitPtice | Securidury TradingRelerenc-Piice |
| ExpiratinnCycle               |                         | NoMulchRules                     |
| MinTrude Vol                  | MaxTiidleVol            | MaxPrice Varation                |
| Match-Algorithtt              | TmpliedMark elndicator  | MatchType                        |
| IradingCuenCY                 | RourdLol                |                                  |
|                               | FcrdTyp >               | NoMDFcedTyes                     |
| MDFerdTyp:                    | Murke Depth             | MDBookType                       |
| NestexPriceLimitType          | NestexLowLimnitPrice    | Neste- Highl .imitPrice          |
| Nestexl rudlingRetirencePrice |                         |                                  |

© Copyright, 2008-      ~~2009~~2011, FIX Protocol, Limited                                      Page 116 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Figure 2: Security Definition

| Security Definition Request       | Derivative Security List Request       | Security List Request        |
| --------------------------------- | -------------------------------------- | ---------------------------- |
| MarketlD                          | Marketlp                               | MarketlD                     |
| MarketSegrnentiD                  | MarketSegrnentiD                       | MarkelSegienid               |
| \<Instrument>                     | \<Underlying Instrument>               | \<Instrument>                |
| \<Instrumont Extension>           | \<Derivativo Instrument>               | \<sInstrument Extenslon>     |
| Security Definition               | Derivative Security List /             | Security List /              |
| Security Definition Update Report | Derivative Security List Update Report | Security List Update Report  |
| sInstrumont?                      | \<Underlying Instrument>               | MarketlD                     |
| \<Instrument Extension>           | \<Derivative Security Definition>      | \<Instrument>                |
| \<Market Segment Grp>             | \<Derivative Instrument>               | \<Instrument Extension>      |
| MarketiD                          | \<Derivative Instrument Attribute>     | \<Security Trading Rules>    |
| \            | MarketlD                               | \<Trading Session Rules Grp> |
| \<Nested Instrument Attribute>    | \<Strike Rules>                        |                              |
| \<Strike Rules>                   |                                        |                              |
| \<Instrument>                     | \<Instrument Extonsion>                | \<SecondaryPriceLimits>      |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 117 of 200
---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                  August 18, 2011


# Figure 3: Market Definition

| Market Definition Request | Trading Session List Request                           |
| ------------------------- | ------------------------------------------------------ |
| MarketID                  | MarketID                                               |
| MarketSegmentID           | MarketSegmentID                                        |
| Market Definition         | Trading Session List / Market Definition Update Report |
| MarketSegmentID           | MarketSegmentID                                        |
| Trading Rules             | Trading Session ID                                     |
| Trading Session SubID     |                                                        |
| OrdTypeRules              |                                                        |
| NoOrdTypeRules            |                                                        |
| OrdTypes                  |                                                        |
| TimeInForceRule           |                                                        |
| NoTimeInForceRules        |                                                        |
| TimeInForce               |                                                        |
| ExcelInstRules            |                                                        |
| NoExcelInstRules          |                                                        |
| ExcelInstValue            |                                                        |

# Message Flow Scenarios

The Market Definition message is associated with a list of trading sessions (and subsessions) applicable for the segment – the list is published using the Trading Session List message. It is foreseen that the message will be relayed every trading day, or at least when trading sessions are changed. The user of the message has the ability to relay either Trading Sessions only – or, if applicable, Trading SubSessions. Depending on characteristics of the market, the various Time fields apply or not.

A user can obtain the securities traded at a Market and/or Market Segment through the use of the Security List or Security Definition messages. A market can choose to push the Security List message out as part of a master file feed or provide queries/subscriptions capabilities through the Security List Request and related messages.


© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                        Page 118 of 200

---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Market Structure based Trading Reference data

Users access the market structure either through query/subscription messages (such as the Market Definition Request) or via separate download or feed provided by the marketplace. An example sequence of messages is:

1. A Market Definition message for each Market
1. A Market Definition message for each segment per the Market
1. A Trading Session List message for each Market Segment, listing the applicable trading sessions and subsessions
2. A Security List message for each Market Segment, listing the applicable securities
3. (A Security Types message for each Market Segment, listing the applicable security types)

The download typically occurs at start of day, week or other relevant period subject to bilateral agreement. In between downloads, real time updates can be provided through the associated Update Report messages.

The marketplace may continuously relay status information either subscription based or as a part of a market data feed:

- Security Status / Security Status Request. If securities are traded in multiple markets or segments, the status needs to be qualified by Market / Market segment
- Trading Session Status / Trading Session Status Request. If there are multiple markets / market segments, the Trading Session status needs to be qualified by the applicable Market and Market Segment to make sense

Further, other messages may also be filtered per Market and Market Segment:

- Security Type / Security Type Request

# "Start of day" download

The diagram depicts how start of day (or other periodicity) market structure trading reference data can be relayed.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                      Page 119 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Figure 4: Start-of-day flow

| Marketplace                      | Market Participant       |
| -------------------------------- | ------------------------ |
| Market Definition Request        | MarketD                  |
| Market Definition                | MarketSegmentID          |
| Trading Session List Request     | MarketD                  |
| Trading Session List             | MarketSegmentID          |
| Trading SessionID                | TradingSessionWbID       |
| Security List Request            | MarketD                  |
| MarketSegmentID                  | Security List            |
| MarketID                         | MarketSegmentID          |
| No RelatedSH                     | Symbol                   |
| SecurityID                       |                          |
| Security Definition Request      | MarketD                  |
| MarketSegmentID                  | Security Definition      |
| Symbol                           | Security                 |
| No Market Segments               | MarketServiceID          |
| Derivative Security List Request | MarketD                  |
| MarketSegmentID                  | Derivative Security List |
| Underlying SecurityID            | No RelatedSVT            |
| Symbol                           | SecurityID               |
| No Market Segments               | MarketServiceID          |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 120 of 200
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011



Note that whether a request is needed, what filtering parameters are applicable, what messages are returned and what fields are included are all bilaterally agreed. A marketplace may also choose to make the information available by other means as e.g. a down-loadable file.

The same message flow applies to a situation after the "Start of Day" where reference data has to be obtained anew.

# "Intra-day" updates

The following diagram depicts how intra-day real time updates of the market structure trading reference data can be relayed.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited

Page 121 of 200
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011



# Figure 5: Intra-day flow

| Marketplace                            | Marker Participant         |
| -------------------------------------- | -------------------------- |
| Market Delinitiun Request              | Start Of Day               |
| MarketID                               | MarketSegmentID "JJ"       |
| Trading Session List Request           | Subscriptions (if needed)  |
| Security List Request                  | Naret                      |
| Market Segment                         | Security Delintion Request |
| MarketID                               | MarkedSegreD               |
| Derivative Security List Request       | Market                     |
| MarkedSegmentID                        |                            |
| Market Delinitivn Update               | Real time updates          |
| Market                                 | MarkedSegmentID J          |
| Trading Session List Update            | MarketD                    |
| Markedsegeewtin                        |                            |
| Security List Update Report            | Market                     |
| MarkedSegmentID 7                      | Yvo"                       |
| Security Definition Update Report      | Symbol                     |
| SecuriD                                | MarketD)                   |
| MarkedSegmentID "17"                   |                            |
| Derivative Security List Update Report | UnderlvangSymbol           |
| Underlving Secw itI)                   | YVo"                       |
| SecurityID                             | 'Ogo"                      |
| Markedsewentin                         |                            |


© Copyright, 2008-2009, 2011, FIX Protocol, Limited

Page 122 of 200


---

Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# CATEGORY: SECURITIES REFERENCE DATA

# Securities Reference Data Component Blocks

This section lists the component blocks used exclusively by the messages defined for Securities Reference Data.

# SecurityTradingRules component block

This SecurityTradingRules component block is used as part of security definition to specify the specific security's standard trading parameters such as trading session eligibility and other attributes of the security.

| Tag                          | FieldName       | Req'd | Comments                                                            |
| ---------------------------- | --------------- | ----- | ------------------------------------------------------------------- |
| component block              |                 | N     | This block contains the base trading rules                          |
| \<BaseTradingRules>          | component block | N     | This block contains the trading rules specific to a trading session |
| \<TradingSessionRulesGrp>    | component block | N     |                                                                     |
| \<NestedInstrumentAttribute> |                 | N     |                                                                     |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SecurityTradingRules

# DerivativeSecurityXML component block

| Tag  | FieldName                  | Req'd | Comments                                                                       |
| ---- | -------------------------- | ----- | ------------------------------------------------------------------------------ |
| 1282 | DerivativeSecurityXMLLen   | N     |                                                                                |
| 1283 | DerivativeSecurityXML      | N     | Must be set if SecurityXML field is specified and must immediately precede it. |
| 1284 | DerivativeSecurityXMLSchem | N     | XML Data Stream describing the Security.                                       |

XML Schema used to validate the XML used to describe the Security.

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SecXML

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited


Page 123 of 200

---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                            August 18, 2011

# InstrmtLegSecListGrp component block

| Tag | FieldName       | Req'd        | Comments                                                                                                                                            |
| --- | --------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| 555 | NoLegs          | N            | Number of legs that make up the Security                                                                                                            |
| £   | component block | N            | Insert here the set of "Instrument Legs" (leg symbology) fields defined in "Common Components of Application Messages" Required if NoLegs > 0       |
| £   | 690             | LegSwapType  | N                                                                                                                                                   |
| £   | 587             | LegSettlType | N                                                                                                                                                   |
| £   | component block | N            | Insert here the set of "LegStipulations" (leg symbology) fields defined in "Common Components of Application Messages" Required if NoLegs > 0       |
| £   | component block | N            | Insert here the set of "LegBenchmarkCurveData" (leg symbology) fields defined in "Common Components of Application Messages" Required if NoLegs > 0 |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SecL

# RelSymDerivSecGrp component block

| Tag | FieldName            | Req'd           | Comments                                                          |
| --- | -------------------- | --------------- | ----------------------------------------------------------------- |
| 146 | NoRelatedSym         | N               | Specifies the number of repeating symbols (instruments) specified |
| £   | component block      | N               |                                                                   |
| £   | Instrument           |                 |                                                                   |
| £   | component block      | N               | Secondary price limit rules                                       |
| £   | SecondaryPriceLimits |                 |                                                                   |
| £   | 15                   | Currency        | N                                                                 |
| £   | 292                  | CorporateAction | N                                                                 |
| £   | component block      | N               |                                                                   |
| £   | InstrumentExtension  |                 |                                                                   |
| £   | component block      | N               |                                                                   |
| £   | InstrmtLegGrp        |                 |                                                                   |

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                   Page 124 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

# 1504 RelSymTransactTime

N

# 58 Text

N          Comment, instructions, or other identifying information.

# 354 EncodedTextLen

N          Must be set if EncodedText field is specified and must immediately precede it.

# 355 EncodedText

N          Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element RelSym

# SecListGrp component block

| Tag | FieldName    |
| --- | ------------ |
| 146 | NoRelatedSym |

£      component block

&#x3C;Instrument>

£     component block

&#x3C;InstrumentExtension>

£      component block

&#x3C;FinancingDetails>

£      component block

&#x3C;SecurityTradingRules>

£      component block

&#x3C;StrikeRules>

£      component block

&#x3C;UndInstrmtGrp>

£       15      Currency

£      component block

&#x3C;Stipulations>

£      component block

&#x3C;InstrmtLegSecListGrp>

£      component block

&#x3C;SpreadOrBenchmarkCurveData>

# Req'd Comments

| Req'd | Comments                                                                                                                                |
| ----- | --------------------------------------------------------------------------------------------------------------------------------------- |
| N     | Specifies the number of repeating symbols (instruments) specified                                                                       |
| N     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" of the requested Security |
| N     | Insert here the set of "InstrumentExtension" fields defined in "Common Components of Application Messages"                              |
| N     | Insert here the set of "FinancingDetails" fields defined in "Common Components of Application Messages"                                 |
| N     | Used to provide listing rules                                                                                                           |
| N     | Used to provide listing rules                                                                                                           |
| N     | Insert here the set of "Stipulations" fields defined in "Common Components of Application Messages"                                     |
| N     | Insert here the set of "SpreadOrBenchmarkCurveData" fields defined in "Common Components of Application Messages"                       |

© Copyright, 2008-2009, FIX Protocol, Limited                                     Page 125 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                     August 18, 2011

£     component block                N            Insert   here the set of "YieldData" fields defined in

&#x3C;YieldData>                                 "Common Components of Application Messages"

£     1504     RelSymTransactTime    N

£      58      Text                  N            Comment, instructions, or other identifying information.

£      354     EncodedTextLen        N            Must be set if EncodedText field is specified and must

immediately precede it.

£      355     EncodedText           N            Encoded (non-ASCII characters) representation of the

Text     field in the encoded format specified via  the

MessageEncoding field.

FIXML Definition for           this Component           Block–   see http://www.fixprotocol.org  for

details

Refer to FIXML element SecL

# SecTypesGrp component block

| Tag   | FieldName       | Req'd | Comments                        |
| ----- | --------------- | ----- | ------------------------------- |
| 558   | NoSecurityTypes | N     |                                 |
| £ 167 | SecurityType    | N     | Required if NoSecurityTypes > 0 |
| £ 762 | SecuritySubType | N     |                                 |
| £ 460 | Product         | N     |                                 |
| £ 461 | CFICode         | N     |                                 |
| £ 60  | TransactTime    | N     |                                 |

FIXML Definition for           this Component           Block–   see http://www.fixprotocol.org  for

details

Refer to FIXML element SecT

# SecLstUpdRelSymGrp component block

| Tag    | FieldName        | Req'd | Comments                                                |
| ------ | ---------------- | ----- | ------------------------------------------------------- |
| 146    | NoRelatedSym     | N     | Specifies the number of repeating symbols (instruments) |
| £ 1324 | ListUpdateAction | N     |                                                         |

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                       Page 126 of 200


---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

# Component Blocks

| £ | component block | N                  | Insert here the set of "Instrument" (symbology) fields defined in "common components of application messages" of the requested Security | \<Instrument>                                                                                                                  |
| - | --------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| £ | component block | N                  | Insert here the set of "InstrumentExtension" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                              | \<InstrumentExtension>                                                                                                         |
| £ | component block | N                  | Insert here the set of "FinancingDetails" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                 | \<FinancingDetails>                                                                                                            |
| £ | component block | N                  | \<SecurityTradingRules>                                                                                                                 |                                                                                                                                |
| £ | component block | N                  | \<StrikeRules>                                                                                                                          |                                                                                                                                |
| £ | component block | N                  | \<UndInstrmtGrp>                                                                                                                        |                                                                                                                                |
| £ | 15              | Currency           | N                                                                                                                                       | \<Stipulations>                                                                                                                |
| £ | component block | N                  | \<SecLstUpdRelSymsLegGrp>                                                                                                               |                                                                                                                                |
| £ | component block | N                  | Insert here the set of "SpreadOrBenchmarkCurveData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                       | \<SpreadOrBenchmarkCurveData>                                                                                                  |
| £ | component block | N                  | Insert here the set of "YieldData" fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                        | \<YieldData>                                                                                                                   |
| £ | 1504            | RelSymTransactTime | N                                                                                                                                       |                                                                                                                                |
| £ | 58              | Text               | N                                                                                                                                       | Comment, instructions, or other identifying information.                                                                       |
| £ | 354             | EncodedTextLen     | N                                                                                                                                       | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| £ | 355             | EncodedText        | N                                                                                                                                       | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SecL

© Copyright, 2008-2011, FIX Protocol, Limited                                  Page 127 of 200
---
Version 5.0 Service Pack 2 - Errata         VOLUME 3                                                August 18, 2011

# SecLstUpdRelSymsLegGrp component block

| Tag | FieldName                | Req'd | Comments                                                                                                                                            |
| --- | ------------------------ | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| 555 | NoLegs                   | N     | Number of legs that make up the Security                                                                                                            |
| £   | component block          | N     | Insert here the set of "Instrument Legs" (leg symbology)                                                                                            |
|     | \<InstrumentLeg>         |       | fields defined in "common components of application messages" Required if NoLegs > 0                                                                |
| £   | 690 LegSwapType          | N     |                                                                                                                                                     |
| £   | 587 LegSettlType         | N     |                                                                                                                                                     |
| £   | component block          | N     | Insert here the set of "LegStipulations" (leg symbology)                                                                                            |
|     | \<LegStipulations>       |       | fields defined in "common components of application messages" Required if NoLegs > 0                                                                |
| £   | component block          | N     | Insert here the set of "LegBenchmarkCurveData" (leg symbology) fields defined in "common components of application messages" Required if NoLegs > 0 |
|     | \<LegBenchmarkCurveData> |       |                                                                                                                                                     |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SecLstUpdRelSymsLegGrp

# DerivativeInstrumentPartySubIDsGrp component block

| Tag  | FieldName                               | Req'd | Comments |
| ---- | --------------------------------------- | ----- | -------- |
| 1296 | NoDerivativeInstrumentPartySubIDs       | N     |          |
| £    | 1297 DerivativeInstrumentPartySubID     | N     |          |
| £    | 1298 DerivativeInstrumentPartySubIDType | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                           Page 128 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# DerivativeSecurityAltIDGrp component block

| Tag  | FieldName                     | Req'd | Comments |
| ---- | ----------------------------- | ----- | -------- |
| 1218 | NoDerivativeSecurityAltID     | N     |          |
| 1219 | DerivativeSecurityAltID       | N     |          |
| 1220 | DerivativeSecurityAltIDSource | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element AID

# DerivativeEventsGrp component block

| Tag  | FieldName           | Req'd | Comments                                                                 |
| ---- | ------------------- | ----- | ------------------------------------------------------------------------ |
| 1286 | NoDerivativeEvents  | N     |                                                                          |
| 1287 | DerivativeEventType | N     | Indicates type of event describing security                              |
| 1288 | DerivativeEventDate | N     |                                                                          |
| 1289 | DerivativeEventTime | N     | Specific time of event. To be used in combination with EventDate \[1288] |
| 1290 | DerivativeEventPx   | N     |                                                                          |
| 1291 | DerivativeEventText | N     |                                                                          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Evnt

© Copyright, 2008-2011, FIX Protocol, Limited Page 129 of 200
---

Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011


# RelSymDerivSecUpdGrp component block

| Tag  | FieldName               | Req'd | Comments                                                                                                                       |
| ---- | ----------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
| 146  | NoRelatedSym            | N     |                                                                                                                                |
| 1324 | ListUpdateAction        | N     | If provided, then Instrument occurrence has explicitly changed                                                                 |
| 292  | CorporateAction         | N     |                                                                                                                                |
|      | component block         | N     |                                                                                                                                |
|      | \<Instrument>           |       |                                                                                                                                |
|      | component block         | N     |                                                                                                                                |
|      | \<InstrumentExtension>  |       |                                                                                                                                |
|      | component block         | N     | Secondary price limit rules                                                                                                    |
|      | \<SecondaryPriceLimits> |       |                                                                                                                                |
| 15   | Currency                | N     |                                                                                                                                |
|      | component block         | N     |                                                                                                                                |
|      | \<InstrmtLegGrp>        |       |                                                                                                                                |
| 1504 | RelSymTransactTime      | N     |                                                                                                                                |
| 58   | Text                    | N     | Comment, instructions, or other identifying information.                                                                       |
| 354  | EncodedTextLen          | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355  | EncodedText             | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element RelSym


© Copyright, 2008-2009, FIX Protocol, Limited Page 130 of 200

---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# StrikeRules component block

| Tag                                                                                                | FieldName           | Req'd | Comments                                                                                                                                                                  |
| -------------------------------------------------------------------------------------------------- | ------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1201                                                                                               | NoStrikeRules       | N     | Number of strike rule entries. This block specifies the rules for determining how new strikes should be listed within the stated price range of the underlying instrument |
| 1223                                                                                               | StrikeRuleID        | N     | Allows strike rule to be referenced via an identifier so that rules do not need to be explicitly enumerated                                                               |
| 1202                                                                                               | StartStrikePxRange  | N     | Starting price for the range to which the StrikeIncrement applies. Price refers to the price of the underlying                                                            |
| 1203                                                                                               | EndStrikePxRange    | N     | Ending price of the range to which the StrikeIncrement applies. Price refers to the price of the underlying                                                               |
| 1204                                                                                               | StrikeIncrement     | N     | Value by which strike price should be incremented within the specified price                                                                                              |
| 1304                                                                                               | StrikeExerciseStyle | N     | Enumeration that represents the exercise style for a class of options Same values as ExerciseStyle                                                                        |
| component block                                                                                    |                     |       |                                                                                                                                                                           |
| \<MaturityRules> Describes the maturity rules for a given set of strikes as defined by StrikeRules |                     |       |                                                                                                                                                                           |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element StrkRules

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 131 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# MaturityRules component block

| Tag  | FieldName                       | Req'd | Comments                                                                                                                                                                    |
| ---- | ------------------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1236 | NoMaturityRules                 | N     | Number of maturity rule entries. This block specifies the rules for determining how new strikes should be listed within the stated price range of the underlying instrument |
| 1222 | MaturityRuleID                  | N     | Allows maturity rule to be referenced via an identifier so that rules do not need to be explicitly enumerated                                                               |
| 1303 | MaturityMonthYearFormat         | N     | Format used to generate the MMY for each option contract:                                                                                                                   |
| 1302 | MaturityMonthYearIncrementUnits | N     | Enumeration specifying the increment unit:                                                                                                                                  |
| 1241 | StartMaturityMonthYear          | N     | Starting maturity for the range to which the StrikeIncrement applies. Price refers to the price of the underlying                                                           |
| 1226 | EndMaturityMonthYear            | N     | Ending maturity month year to which the StrikeIncrement applies. Price refers to the price of the underlying                                                                |
| 1229 | MaturityMonthYearIncrement      | N     | Value by which maturity month year should be incremented within the specified price range.                                                                                  |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element MatRules

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited
Page 132 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# SecondaryPriceLimits component block

| Tag  | FieldName                      | Req'd | Comments |
| ---- | ------------------------------ | ----- | -------- |
| 1305 | SecondaryPriceLimitType        | N     |          |
| 1221 | SecondaryLowLimitPrice         | N     |          |
| 1230 | SecondaryHighLimitPrice        | N     |          |
| 1240 | SecondaryTradingReferencePrice | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element PxLmts2

# TradingSessionRulesGrp component block

| Tag                                                                                                  | FieldName             | Req'd | Comments                                                                                                                                 |
| ---------------------------------------------------------------------------------------------------- | --------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| 1309                                                                                                 | NoTradingSessionRules | N     | Allows trading rules to be expressed by trading session                                                                                  |
| 336                                                                                                  | TradingSessionID      | N     | Identifier for the trading session Must be provided if NoTradingSessions > 0 Set to \[N/A] if values are not specific to trading session |
| 625                                                                                                  | TradingSessionSubID   | N     | Identifier for the trading session Set to \[N/A] if values are not specific to trading session sub id                                    |
| component block \<TradingSessionRules> Contains trading rules specified at the trading session level |                       |       |                                                                                                                                          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TrdgSesRulesGrp

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 133 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# MarketSegmentGrp component block

| Tag                                                                                                                                         | FieldName        | Req'd | Comments                                                                                         |   |
| ------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- | ----- | ------------------------------------------------------------------------------------------------ | - |
| 1310                                                                                                                                        | NoMarketSegments | N     | Number of Market Segments on which a security may trade.                                         |   |
| 1301                                                                                                                                        | MarketID         | N     | Identifies the market which lists and trades the instrument.                                     |   |
| 1300                                                                                                                                        | MarketSegmentID  | N     | Identifies the segment of the market to which the specify trading rules and listing rules apply. |   |
|                                                                                                                                             | component block  |       |                                                                                                  |   |
| \<SecurityTradingRules>                                                                                                                     |                  |       |                                                                                                  |   |
|                                                                                                                                             | component block  |       |                                                                                                  |   |
| \<StrikeRules>                                                                                                                              |                  |       |                                                                                                  |   |
| This block specifies the rules for determining how new strikes should be listed within the stated price range of the underlying instrument. |                  |       |                                                                                                  |   |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element MktSegGrp

# DerivativeSecurityDefinition component block

| Tag                              | FieldName | Req'd | Comments |
| -------------------------------- | --------- | ----- | -------- |
| component block                  |           |       |          |
| \<DerivativeInstrument>          |           |       |          |
| component block                  |           |       |          |
| \<DerivativeInstrumentAttribute> |           |       |          |
| component block                  |           |       |          |
| \<MarketSegmentGrp>              |           |       |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element DerivSecDef

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 134 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# NestedInstrumentAttribute component block

| Tag    | FieldName              | Req'd |
| ------ | ---------------------- | ----- |
| 1312   | NoNestedInstrAttrib    | N     |
| £ 1210 | NestedInstrAttribType  | N     |
| £ 1211 | NestedInstrAttribValue | N     |

Comments

Code to represent the type of instrument attribute

Attribute value appropriate to the NestedInstrAttribType field

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Attrb

# DerivativeInstrumentAttribute component block

| Tag    | FieldName                  | Req'd | Comments |
| ------ | -------------------------- | ----- | -------- |
| 1311   | NoDerivativeInstrAttrib    | N     |          |
| £ 1313 | DerivativeInstrAttribType  | N     |          |
| £ 1314 | DerivativeInstrAttribValue | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Attrb

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 135 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Derivative Instrument component block

| Tag                 | FieldName                      | Req'd | Comments                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ------------------------------ | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1214                | DerivativeSymbol               | N     | Common, "human understood" representation of the security. SecurityID value can be specified if no symbol exists (e.g. non-exchange traded Collective Investment Vehicles). Use "\[N/A]" for products which do not have a symbol.                                                                                                                       |
| 1215                | DerivativeSymbolSfx            | N     | Used in Fixed Income with a value of "WI" to indicate "When Issued" for a security to be reissued under an old CUSIP or ISIN or with a value of "CD" to indicate a EUCP with lump-sum interest rather than discount price.                                                                                                                              |
| 1216                | DerivativeSecurityID           | N     | Takes precedence in identifying security to counterparty over SecurityAltID block. Requires SecurityIDSource if specified.                                                                                                                                                                                                                              |
| 1217                | DerivativeSecurityIDSource     | N     | Required if SecurityID is specified.                                                                                                                                                                                                                                                                                                                    |
| **component block** |                                |       |                                                                                                                                                                                                                                                                                                                                                         |
| 1246                | DerivativeProduct              | N     | Indicates the type of product the security is associated with (high-level category).                                                                                                                                                                                                                                                                    |
| 1228                | DerivativeProductComplex       | N     | Identifies an entire suite of products for a given market. In Futures this may be "interest rates", "agricultural", "equity indexes", etc.                                                                                                                                                                                                              |
| 1243                | DerivFlexProductEligibilityInd | N     | Used to indicate if a product or group of product supports the creation of flexible securities.                                                                                                                                                                                                                                                         |
| 1247                | DerivativeSecurityGroup        | N     | An exchange specific name assigned to a group of related securities which may be concurrently affected by market events and actions.                                                                                                                                                                                                                    |
| 1248                | DerivativeCFICode              | N     | Indicates the type of security using ISO 10962 standard, Classification of Financial Instruments (CFI code) values. It is recommended that CFICode be used instead of SecurityType for non-Fixed Income instruments.                                                                                                                                    |
| 1249                | DerivativeSecurityType         | N     | It is recommended that CFICode be used instead of SecurityType for non-Fixed Income instruments. Required for Fixed Income. Refer to Volume 7 - Fixed Income. Futures and Options should be specified using the CFICode\[461] field instead of SecurityType\[167] (Refer to Volume 7 - Recommendations and Guidelines for Futures and Options Markets.) |
| 1250                | DerivativeSecuritySubType      | N     | Sub-type qualification/identification of the SecurityType (e.g. for SecurityType=MLEG). If specified, SecurityType is required.                                                                                                                                                                                                                         |
| 1251                | DerivativeMaturityMonthYear    | N     | Specifies the month and year of maturity. Applicable for                                                                                                                                                                                                                                                                                                |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 136 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Errata

| 1252 | DerivativeMaturityDate             | N | Specifies date of maturity (a full date). Note that standardized derivatives which are typically only referenced by month and year (e.g. S and P futures). may use MaturityMonthYear and or this field. When using MaturityMonthYear, it is recommended that markets and sell sides report the MaturityDate on all outbound messages as a means of data enrichment. |
| ---- | ---------------------------------- | - | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1253 | DerivativeMaturityTime             | N |                                                                                                                                                                                                                                                                                                                                                                     |
| 1254 | DerivativeSettleOnOpenFlag         | N | Indicator to determine if Instrument is Settle on Open.                                                                                                                                                                                                                                                                                                             |
| 1255 | DerivativeInstrmtAssignment Method | N |                                                                                                                                                                                                                                                                                                                                                                     |
| 1256 | DerivativeSecurityStatus           | N | Gives the current state of the instrument                                                                                                                                                                                                                                                                                                                           |
| 1276 | DerivativeIssueDate                | N | Date instrument was issued. For Fixed Income IOIs for new issues, specifies the issue date.                                                                                                                                                                                                                                                                         |
| 1257 | DerivativeInstrRegistry            | N | The location at which records of ownership are maintained for this instrument, and at which ownership changes must be recorded. Can be used in conjunction with ISIN to address ISIN uniqueness issues.                                                                                                                                                             |
| 1258 | DerivativeCountryOfIssue           | N | ISO Country code of instrument issue (e.g. the country portion typically used in ISIN). Can be used in conjunction with non-ISIN SecurityID (e.g. CUSIP for Municipal Bonds without ISIN) to provide uniqueness.                                                                                                                                                    |
| 1259 | DerivativeStateOrProvinceOfIssue   | N | A two-character state or province abbreviation.                                                                                                                                                                                                                                                                                                                     |
| 1260 | DerivativeLocaleOfIssue            | N | The three-character IATA code for a locale (e.g. airport code for Municipal Bonds).                                                                                                                                                                                                                                                                                 |
| 1261 | DerivativeStrikePrice              | N | Used for derivatives, such as options and covered warrants                                                                                                                                                                                                                                                                                                          |
| 1262 | DerivativeStrikeCurrency           | N | Used for derivatives                                                                                                                                                                                                                                                                                                                                                |
| 1263 | DerivativeStrikeMultiplier         | N | Used for derivatives. Multiplier applied to the strike price for the purpose of calculating the settlement value.                                                                                                                                                                                                                                                   |
| 1264 | DerivativeStrikeValue              | N | Used for derivatives. The number of shares/units for the financial instrument involved in the option trade.                                                                                                                                                                                                                                                         |
| 1265 | DerivativeOptAttribute             | N | Used for derivatives, such as options and covered warrants to indicate a versioning of the contract when required due to corporate actions to the underlying. Should not be used to indicate type of option - use the CFICode\[461] for this purpose.                                                                                                               |
| 1266 | DerivativeContractMultiplier       | N | For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g. contracts vs. shares) amount.                                                                                                                                                                                                           |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 137 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Errata

| 1438 | DerivativeContractMultiplierU    | N | Unit                                                                                                                                                                                            |
| ---- | -------------------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1442 | DerivativeFlowScheduleType       | N |                                                                                                                                                                                                 |
| 1267 | DerivativeMinPriceIncrement      | N | Minimum price increment for the instrument. Could also be used to represent tick value.                                                                                                         |
| 1268 | DerivativeMinPriceIncrement      | N | Minimum price increment amount associated with the MinPriceIncrement \[969]. For listed derivatives, the value can be calculated by multiplying MinPriceIncrement by ContractValueFactor \[231] |
| 1269 | DerivativeUnitOfMeasure          | N |                                                                                                                                                                                                 |
| 1270 | DerivativeUnitOfMeasureQty       | N |                                                                                                                                                                                                 |
| 1315 | DerivativePriceUnitOfMeasure     | N |                                                                                                                                                                                                 |
| 1316 | DerivativePriceUnitOfMeasure     | N | Qty                                                                                                                                                                                             |
| 1317 | DerivativeSettlMethod            | N | Settlement method for a contract. Can be used as an alternative to CFI Code value                                                                                                               |
| 1318 | DerivativePriceQuoteMethod       | N | Method for price quotation                                                                                                                                                                      |
| 1319 | DerivativeValuationMethod        | N | For futures, indicates type of valuation method applied                                                                                                                                         |
| 1320 | DerivativeListMethod             | N | Indicates whether strikes are pre-listed only or can also be defined via user request                                                                                                           |
| 1321 | DerivativeCapPrice               | N | Used to express the ceiling price of a capped call                                                                                                                                              |
| 1322 | DerivativeFloorPrice             | N | Used to express the floor price of a capped put                                                                                                                                                 |
| 1323 | DerivativePutOrCall              | N |                                                                                                                                                                                                 |
| 1299 | DerivativeExerciseStyle          | N | Type of exercise of a derivatives security                                                                                                                                                      |
| 1225 | DerivativeOptPayAmount           | N | Cash amount indicating the pay out associated with an option. For binary options this is a fixed amount                                                                                         |
| 1271 | DerivativeTimeUnit               | N | Used to indicate a time unit for the contract (e.g., days, weeks, months, etc.)                                                                                                                 |
| 1272 | DerivativeSecurityExchange       | N | Can be used to identify the security.                                                                                                                                                           |
| 1273 | DerivativePositionLimit          | N | Position Limit for the instrument.                                                                                                                                                              |
| 1274 | DerivativeNTPositionLimit        | N | Near-term Position Limit for the instrument.                                                                                                                                                    |
| 1275 | DerivativeIssuer                 | N |                                                                                                                                                                                                 |
| 1277 | DerivativeEncodedIssuerLen       | N | Must be set if EncodedIssuer field is specified and must immediately precede it.                                                                                                                |
| 1278 | DerivativeEncodedIssuer          | N | Encoded (non-ASCII characters) representation of the Issuer field in the encoded format specified via the MessageEncoding field.                                                                |
| 1279 | DerivativeSecurityDesc           | N |                                                                                                                                                                                                 |
| 1280 | DerivativeEncodedSecurityDescLen | N | Must be set if EncodedSecurityDesc field is specified and must immediately precede it.                                                                                                          |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 138 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# 1281 DerivativeEncodedSecurityDe

N Encoded (non-ASCII characters) representation of the SecurityDesc field in the encoded format specified via the MessageEncoding field.

component block N Embedded XML document describing security.

&#x3C;DerivativeSecurityXML>

# 1285 DerivativeContractSettlMonth

N Must be present for MBS or TBA

component block N

&#x3C;DerivativeEventsGrp>
component block N

&#x3C;DerivativeInstrumentParties>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element DerivInstrmt

# DerivativeInstrumentParties component block

| Tag  | FieldName                         | Req'd | Comments                                                                                                                                |
| ---- | --------------------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------- |
| 1292 | NoDerivativeInstrumentParties     | N     | Should contain unique combinations of DerivativeInstrumentPartyID, DerivativeInstrumentPartyIDSource, and DerivativeInstrumentPartyRole |
| 1293 | DerivativeInstrumentPartyID       | N     | Used to identify party id related to instrument series                                                                                  |
| 1294 | DerivativeInstrumentPartyIDSource | N     | Used to identify source of instrument series party id                                                                                   |
| 1295 | DerivativeInstrumentPartyRole     | N     | Used to identify the role of instrument series party id                                                                                 |
| £    | component block                   | N     | \<DerivativeInstrumentPartySubIDsGrp>                                                                                                   |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Pty

# Security Definition Request

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 139 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Security Definition Request

The Security Definition Request message is used for the following:

1. Request a specific Security to be traded with the second party. The request security can be defined as a multileg security made up of one or more instrument legs.
2. Request a set of individual securities for a single market segment.
3. Request all securities, independent of market segment.

Subscription for security status can be optionally specified by including the SubscriptionRequestType[263] field on the message.

See “Security Definition, Security Status, and Trading Session Message Scenarios”

| Tag                              | FieldName              | Req'd | Comments                                                                                                                                          |
| -------------------------------- | ---------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                   |                        | Y     | MsgType = c (lowercase)                                                                                                                           |
| 320                              | SecurityReqID          | Y     |                                                                                                                                                   |
| 321                              | SecurityRequestType    | Y     |                                                                                                                                                   |
| 1301                             | MarketID               | N     | Identifies the market for which the security definition request is being made.                                                                    |
| 1300                             | MarketSegmentID        | N     | Identifies the segment of the market for which the security definition request is being made.                                                     |
| component block \<Instrument>    |                        | N     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" of the requested Security           |
| component block                  | \<InstrumentExtension> | N     | Insert here the set of "InstrumentExtension" fields defined in "Common Components of Application Messages"                                        |
| component block \<UndInstrmtGrp> |                        | N     | Number of underlyings                                                                                                                             |
| 15                               | Currency               | N     |                                                                                                                                                   |
| 58                               | Text                   | N     | Comment, instructions, or other identifying information.                                                                                          |
| 354                              | EncodedTextLen         | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                                    |
| 355                              | EncodedText            | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                    |
| 336                              | TradingSessionID       | N     | Optional Trading Session Identifier to specify a particular trading session for which you want to obtain a list of securities that are tradeable. |
| 625                              | TradingSessionSubID    | N     |                                                                                                                                                   |
| component block \<Stipulations>  |                        | N     |                                                                                                                                                   |
| component block \<InstrmtLegGrp> |                        | N     | Number of legs that make up the Security                                                                                                          |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                           Page 140 of 200
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011



# component block

N

# &#x3C;SpreadOrBenchmarkCurveData>

# component block &#x3C;YieldData>

N

| 827 | ExpirationCycle         | N |                                                                                |
| --- | ----------------------- | - | ------------------------------------------------------------------------------ |
| 263 | SubscriptionRequestType | N | Subscribe or unsubscribe for security status to security specified in request. |

# StandardTrailer

Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to the FIXML element SecDefReq

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 141 of 200


---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                      August 18, 2011

# Security Definition

The Security Definition message is used for the following:

1. Accept the security defined in a Security Definition message.
2. Accept the security defined in a Security Definition message with changes to the definition and/or identity of the security.
3. Reject the security requested in a Security Definition message.
4. Respond to a request for securities within a specified market segment.
5. Convey comprehensive security definition for all market segments that the security participates in.
6. Convey the security's trading rules that differ from default rules for the market segment.

# Security Definition

| Tag                        | FieldName            | Req'd | Comments                                                                                                                                |
| -------------------------- | -------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader             |                      | Y     | MsgType = d (lowercase)                                                                                                                 |
| component block            |                      | N     |                                                                                                                                         |
| ApplicationSequenceControl |                      |       |                                                                                                                                         |
| 964                        | SecurityReportID     | N     | Identifier for Security Definition message                                                                                              |
| 715                        | ClearingBusinessDate | N     |                                                                                                                                         |
| 320                        | SecurityReqID        | N     |                                                                                                                                         |
| 322                        | SecurityResponseID   | N     | Identifier for the Security Definition message                                                                                          |
| 323                        | SecurityResponseType | N     | Response to the Security Definition Request                                                                                             |
| 292                        | CorporateAction      | N     | Identifies the type of Corporate Action                                                                                                 |
| component block            | \<Instrument>        | N     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" of the requested Security |
| component block            |                      | N     | Insert here the set of "InstrumentExtension" fields defined in "Common Components of Application Messages"                              |
| component block            | \<UndInstrmtGrp>     | N     | Number of underlyings                                                                                                                   |
| 15                         | Currency             | N     | Currency in which the price is denominated                                                                                              |
| 58                         | Text                 | N     | Comment, instructions, or other identifying information.                                                                                |
| 354                        | EncodedTextLen       | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                          |
| 355                        | EncodedText          | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.          |
| component block            | \<Stipulations>      | N     |                                                                                                                                         |
| component block            | \<InstrmtLegGrp>     | N     | Number of legs that make up the Security                                                                                                |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                         Page 142 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011

component block                        N

&#x3C;SpreadOrBenchmarkCurveData>

component block &#x3C;YieldData>            N

component block                        N          Contains all the security details related to listing and

&#x3C;MarketSegmentGrp>

60    TransactTime                    N

StandardTrailer                        Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element SecDef

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 143 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Security Definition Update Report

This message is used for reporting updates to a Product Security Masterfile. Updates could be the result of corporate actions or other business events. Updates may include additions, modifications or deletions.

| Tag             | FieldName                     |
| --------------- | ----------------------------- |
| StandardHeader  | component block               |
|                 |                               |
| 964             | SecurityReportID              |
| 320             | SecurityReqID                 |
| 322             | SecurityResponseID            |
| 323             | SecurityResponseType          |
| 715             | ClearingBusinessDate          |
| 980             | SecurityUpdateAction          |
| 292             | CorporateAction               |
| component block | \<Instrument>                 |
| component block | \<InstrumentExtension>        |
| component block | \<UndInstrmtGrp>              |
| 15              | Currency                      |
| 58              | Text                          |
| 354             | EncodedTextLen                |
| 355             | EncodedText                   |
| component block | \<Stipulations>               |
| component block | \<InstrmtLegGrp>              |
| component block | \<SpreadOrBenchmarkCurveData> |
| component block | \<YieldData>                  |
| component block | \<MarketSegmentGrp>           |
| 60              | TransactTime                  |
| StandardTrailer |                               |

# Security Definition Update Report

| Req'd | Comments                                                                                                                       |
| ----- | ------------------------------------------------------------------------------------------------------------------------------ |
| Y     | MsgType = BP                                                                                                                   |
| N     |                                                                                                                                |
| N     | Identifier for the Security Definition Update message in a bulk transfer environment (No Request/Response)                     |
| N     | Identifier for the Security Definition message.                                                                                |
| N     | Response to the Security Definition Request.                                                                                   |
| N     |                                                                                                                                |
| N     | Identifies the type of Corporate Action                                                                                        |
| N     |                                                                                                                                |
| N     |                                                                                                                                |
| N     | Comment, instructions, or other identifying information.                                                                       |
| N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
| N     |                                                                                                                                |
| N     |                                                                                                                                |
| N     | Contains all the security details related to listing and trading the security                                                  |
| N     |                                                                                                                                |
| Y     |                                                                                                                                |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 144 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                   August 18, 2011

# Security Type Request

The Security Type Request message is used to return a list of security types available from a counterparty or market. The request can include a specific TradingSessionID for which Security Types should be returned.

# Security Type Request

| Tag             | FieldName           | Req'd | Comments                                                                                                                                          |
| --------------- | ------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                     | Y     | MsgType = v (lowercase V)                                                                                                                         |
| 320             | SecurityReqID       | Y     |                                                                                                                                                   |
| 58              | Text                | N     | Comment, instructions, or other identifying information.                                                                                          |
| 354             | EncodedTextLen      | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                                    |
| 355             | EncodedText         | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                    |
| 1301            | MarketID            | N     | Optional MarketID to specify a particular trading session for which you want to obtain a list of securities that are tradeable.                   |
| 1300            | MarketSegmentID     | N     | Optional Market Segment Identifier to specify a particular trading session for which you want to obtain a list of securities that are tradeable.  |
| 336             | TradingSessionID    | N     | Optional Trading Session Identifier to specify a particular trading session for which you want to obtain a list of securities that are tradeable. |
| 625             | TradingSessionSubID | N     |                                                                                                                                                   |
| 460             | Product             | N     | Used to qualify which security types are returned.                                                                                                |
| 167             | SecurityType        | N     | Used to qualify which security type is returned.                                                                                                  |
| 762             | SecuritySubType     | N     | Used to qualify which security types are returned.                                                                                                |
| StandardTrailer |                     | Y     |                                                                                                                                                   |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element SecTypReq

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                           Page 145 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

# Security Types

The Security Type message is used to return a list of security types available from a counterparty or market.

# Security Types

| Tag                            | FieldName               | Req'd | Comments                                                                                                                                          |
| ------------------------------ | ----------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                 |                         | Y     | MsgType = w (lowercase W)                                                                                                                         |
| component block                |                         | N     |                                                                                                                                                   |
|                                |                         |       |                                                                                                                                                   |
| 320                            | SecurityReqID           | Y     |                                                                                                                                                   |
| 322                            | SecurityResponseID      | Y     | Identifier for the security response message                                                                                                      |
| 323                            | SecurityResponseType    | Y     | The result of the security request identified by SecurityReqID                                                                                    |
| 557                            | TotNoSecurityTypes      | N     | Indicates total number of security types in the event that multiple Security Type messages are used to return results                             |
| 893                            | LastFragment            | N     | Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.                  |
| component block \<SecTypesGrp> |                         | N     |                                                                                                                                                   |
| 58                             | Text                    | N     | Comment, instructions, or other identifying information.                                                                                          |
| 354                            | EncodedTextLen          | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                                    |
| 355                            | EncodedText             | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                    |
| 1301                           | MarketID                | N     | Optional MarketID to specify a particular trading session for which you want to obtain a list of securities that are tradeable.                   |
| 1300                           | MarketSegmentID         | N     | Optional Market Segment Identifier to specify a particular trading session for which you want to obtain a list of securities that are tradeable.  |
| 336                            | TradingSessionID        | N     | Optional Trading Session Identifier to specify a particular trading session for which you want to obtain a list of securities that are tradeable. |
| 625                            | TradingSessionSubID     | N     |                                                                                                                                                   |
| 263                            | SubscriptionRequestType | N     | Subscribe or unsubscribe for security status to security specified in request.                                                                    |
| StandardTrailer                |                         | Y     |                                                                                                                                                   |

FIXML Definition for this message – see http://www.fixprotocol.org for details

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                             Page 146 of 200
---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                            August 18, 2011

# Refer to FIXML element SecTyps

© Copyright, 2008-2011, FIX Protocol, Limited


Page 147 of 200

---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                August 18, 2011

# Security List Request

The Security List Request message is used to return a list of securities from the counterparty that match criteria provided on the request. Subscription for security status can be optionally specified by including the SubscriptionRequestType[263] field on the message. SecurityListRequestType[559] specifies the criteria of the request:

- 0 - Symbol
- 1 - SecurityType and/or CFICode
- 2 - Product
- 3 - TradingSessionID
- 4 - All Securities

The Security List Request may also be used to request a list of securities for a given market segment.

# Security List Request

| Tag                                 | FieldName               | Req'd | Comments                                                                                                                                                                                              |
| ----------------------------------- | ----------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader                      |                         | Y     | MsgType = x (lowercase X)                                                                                                                                                                             |
| 320                                 | SecurityReqID           | Y     |                                                                                                                                                                                                       |
| 559                                 | SecurityListRequestType | Y     | Type of Security List Request being made                                                                                                                                                              |
| 1465                                | SecurityListID          | N     | Identifies a specific list                                                                                                                                                                            |
| 1470                                | SecurityListType        | N     | Identifies a list type                                                                                                                                                                                |
| 1471                                | SecurityListTypeSource  | N     | Identifies the source a list type                                                                                                                                                                     |
| 1301                                | MarketID                | N     | Identifies the market which lists and trades the instrument.                                                                                                                                          |
| 1300                                | MarketSegmentID         | N     | Identifies the segment of the market to which the specify trading rules and listing rules apply. The segment may indicate the venue, whether retail or wholesale, or even segregation by nationality. |
| component block \<Instrument>       |                         | N     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" of the requested Security                                                               |
| component block                     |                         | N     | Insert here the set of "InstrumentExtension" fields defined in "Common Components of Application Messages"                                                                                            |
| component block \<FinancingDetails> |                         | N     | Insert here the set of "FinancingDetails" fields defined in "Common Components of Application Messages"                                                                                               |
| component block \<UndInstrmtGrp>    |                         | N     | Number of underlyings                                                                                                                                                                                 |
| component block \<InstrmtLegGrp>    |                         | N     | Number of legs that make up the Security                                                                                                                                                              |
| 15                                  | Currency                | N     |                                                                                                                                                                                                       |
| 58                                  | Text                    | N     | Comment, instructions, or other identifying information.                                                                                                                                              |
| 354                                 | EncodedTextLen          | N     | Must be set if EncodedText field is specified and must                                                                                                                                                |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                     Page 148 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                              August 18, 2011

355     EncodedText                     N        Encoded (non-ASCII characters) representation of the Text       field in the encoded format specified via     the MessageEncoding field.

336     TradingSessionID                N        Optional         Trading Session  Identifier  to  specify  a particular trading session for which you want to obtain a list of securities that are tradeable.

625     TradingSessionSubID             N

263     SubscriptionRequestType         N        Subscribe or unsubscribe for security status to security specified in request.

StandardTrailer                          Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element SecListReq

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                           Page 149 of 200


---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                 August 18, 2011

# Security List

The Security List message is used to return a list of securities that matches the criteria specified in a Security List Request.

# Security List

| Tag             | FieldName                  | Req'd | Comments                                                                                                                                                                                              |
| --------------- | -------------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader  |                            | Y     | MsgType = y (lowercase Y)                                                                                                                                                                             |
| component block |                            | N     |                                                                                                                                                                                                       |
|                 |                            |       |                                                                                                                                                                                                       |
| 964             | SecurityReportID           | N     |                                                                                                                                                                                                       |
| 715             | ClearingBusinessDate       | N     |                                                                                                                                                                                                       |
| 1465            | SecurityListID             | N     | Identifies a specific Security List Entry                                                                                                                                                             |
| 1466            | SecurityListRefID          | N     | Provides a reference to another Security List                                                                                                                                                         |
| 1467            | SecurityListDesc           | N     |                                                                                                                                                                                                       |
| 1468            | EncodedSecurityListDescLen | N     |                                                                                                                                                                                                       |
| 1469            | EncodedSecurityListDesc    | N     |                                                                                                                                                                                                       |
| 1470            | SecurityListType           | N     | Identifies a list type                                                                                                                                                                                |
| 1471            | SecurityListTypeSource     | N     | Identifies the source of a list type                                                                                                                                                                  |
| 320             | SecurityReqID              | N     |                                                                                                                                                                                                       |
| 322             | SecurityResponseID         | N     | Identifier for the Security List message                                                                                                                                                              |
| 560             | SecurityRequestResult      | N     | Result of the Security Request identified by the SecurityReqID                                                                                                                                        |
| 60              | TransactTime               | N     |                                                                                                                                                                                                       |
| 393             | TotNoRelatedSym            | N     | Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation is required.                                                            |
| 1301            | MarketID                   | N     | Identifies the market which lists and trades the instrument.                                                                                                                                          |
| 1300            | MarketSegmentID            | N     | Identifies the segment of the market to which the specify trading rules and listing rules apply. The segment may indicate the venue, whether retail or wholesale, or even segregation by nationality. |
| 893             | LastFragment               | N     | Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.                                                                      |
| component block |                            | N     | Specifies the number of repeating symbols (instruments) specified                                                                                                                                     |
| StandardTrailer |                            | Y     |                                                                                                                                                                                                       |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                       Page 150 of 200


---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011



FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element SecList

© Copyright, 2008-2011, FIX Protocol, Limited

Page 151 of 200



---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                 August 18, 2011

# Security List Update Report

The Security List Update Report is used for reporting updates to a Contract Security Masterfile. Updates could be due to Corporate Actions or other business events. Update may include additions, modifications and deletions.

# Security List Update Report

| Tag                        | FieldName                  | Req'd | Comments                                                                                                                                   |
| -------------------------- | -------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader             |                            | Y     | MsgType = BK                                                                                                                               |
| component block            |                            | N     |                                                                                                                                            |
| ApplicationSequenceControl |                            |       |                                                                                                                                            |
| 964                        | SecurityReportID           | N     | Identifier for the Security List Update message in a bulk transfer environment (No Request/Response)                                       |
| 1465                       | SecurityListID             | N     | Identifies a specific Security List entity                                                                                                 |
| 1466                       | SecurityListRefID          | N     | Provides a reference to another Security List                                                                                              |
| 1467                       | SecurityListDesc           | N     |                                                                                                                                            |
| 1468                       | EncodedSecurityListDescLen | N     |                                                                                                                                            |
| 1469                       | EncodedSecurityListDesc    | N     |                                                                                                                                            |
| 1470                       | SecurityListType           | N     | Identifies a list type                                                                                                                     |
| 1471                       | SecurityListTypeSource     | N     | Identifies the source as a list type                                                                                                       |
| 320                        | SecurityReqID              | N     |                                                                                                                                            |
| 322                        | SecurityResponseID         | N     | Identifier for the Security List message.                                                                                                  |
| 560                        | SecurityRequestResult      | N     | Result of the Security Request identified by the SecurityReqID.                                                                            |
| 393                        | TotNoRelatedSym            | N     | Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation is required. |
| 715                        | ClearingBusinessDate       | N     |                                                                                                                                            |
| 980                        | SecurityUpdateAction       | N     |                                                                                                                                            |
| 292                        | CorporateAction            | N     | Identifies the type of Corporate Action that triggered the update                                                                          |
| 1301                       | MarketID                   | N     | Identifies the market which lists and trades the instrument.                                                                               |
| 1300                       | MarketSegmentID            | N     | Identifies the segment of the market specified in MarketID(96)                                                                             |
| 60                         | TransactTime               | N     |                                                                                                                                            |
| 893                        | LastFragment               | N     | Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.           |
| component block            |                            | N     | Specifies the number of repeating symbols (instruments) specified                                                                          |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited

Page 152 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011
StandardTrailer                        Y

© Copyright, 2008-2011, FIX Protocol, Limited                               Page 153 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Derivative Security List Request

The Derivative Security List Request message is used to return a list of securities from the counterparty that match criteria provided on the request. Subscription for security status can be optionally specified by including the SubscriptionRequestType[263] field on the message.

SecurityListRequestType[559] specifies the criteria of the request:

- 0 - Symbol
- 1 - SecurityType and/or CFICode
- 2 - Product
- 3 - TradingSessionID
- 4 - All Securities

Derivative SecurityListRequest may also be used to:

1. Request for option classes for a given market segment.
2. Allows a request all derivative securities to be made independent of Market Segment. The option classes may carry all relevant Market Segments and their corresponding trading rules.

# Derivative Security List Request

| Tag                     | FieldName               | Req'd | Comments                                                                                                                                          |
| ----------------------- | ----------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader          |                         | Y     | MsgType = z (lowercase Z)                                                                                                                         |
| 320                     | SecurityReqID           | Y     |                                                                                                                                                   |
| 559                     | SecurityListRequestType | Y     |                                                                                                                                                   |
| 1301                    | MarketID                | N     |                                                                                                                                                   |
| 1300                    | MarketSegmentID         | N     |                                                                                                                                                   |
| component block         |                         | N     | Specifies the underlying instrument                                                                                                               |
| \<UnderlyingInstrument> |                         |       |                                                                                                                                                   |
| component block         |                         | N     | Group block which contains all information for an option family.                                                                                  |
| \<DerivativeInstrument> |                         |       |                                                                                                                                                   |
| 762                     | SecuritySubType         | N     |                                                                                                                                                   |
| 15                      | Currency                | N     |                                                                                                                                                   |
| 58                      | Text                    | N     | Comment, instructions, or other identifying information.                                                                                          |
| 354                     | EncodedTextLen          | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                                    |
| 355                     | EncodedText             | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                    |
| 336                     | TradingSessionID        | N     | Optional Trading Session Identifier to specify a particular trading session for which you want to obtain a list of securities that are tradeable. |
| 625                     | TradingSessionSubID     | N     |                                                                                                                                                   |
| 263                     | SubscriptionRequestType | N     | Subscribe or unsubscribe for security status to security specified in request.                                                                    |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                           Page 154 of 200
---

Version 5.0 Service Pack 2 - Errata   VOLUME 3                                         August 18, 2011

# FIXML Definition for this message

– see http://www.fixprotocol.org for details

Refer to FIXML element DerivSecListReq

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                               Page 155 of 200



---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Derivative Security List

The Derivative Security List message is used to return a list of securities that matches the criteria specified in a Derivative Security List Request. The Derivative Security List message is used to send a predefined list of securities (usually options) based on a common underlying and option class. It can also be used to send the rules for security creation (usually options) which imply the existence of a set of securities. Other uses of this message may include:

1. Convey comprehensive set of option classes for all market segments in which these option classes participate in.
2. Convey the option classes' trading rules that differ from the default trading rules for the market segment.

# Derivative Security List

| Tag                          | FieldName             | Req'd | Comments                                                                                                                                                           |
| ---------------------------- | --------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader               |                       | Y     | MsgType = AA (2 A's)                                                                                                                                               |
| component block              |                       | N     |                                                                                                                                                                    |
| ApplicationSequenceControl   |                       |       |                                                                                                                                                                    |
| 964                          | SecurityReportID      | N     |                                                                                                                                                                    |
| 320                          | SecurityReqID         | N     |                                                                                                                                                                    |
| 322                          | SecurityResponseID    | N     | Identifier for the Derivative Security List message                                                                                                                |
| 560                          | SecurityRequestResult | N     | Result of the Security Request identified by SecurityReqID                                                                                                         |
| 715                          | ClearingBusinessDate  | N     |                                                                                                                                                                    |
| component block              |                       | N     | Underlying security for which derivatives are being returned                                                                                                       |
| UnderlyingInstrument         |                       |       |                                                                                                                                                                    |
| component block              |                       | N     | Group block which contains all information for an option family. If provided DerivativeSecurityDefinition qualifies the strikes specified in the Instrument block. |
| DerivativeSecurityDefinition |                       |       |                                                                                                                                                                    |
| 60                           | TransactTime          | N     |                                                                                                                                                                    |
| 393                          | TotNoRelatedSym       | N     | Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation is required.                         |
| 893                          | LastFragment          | N     | Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.                                   |
| component block              |                       | N     | Specifies the number of repeating symbols (instruments) specified                                                                                                  |
| RelSymDerivSecGrp            |                       |       |                                                                                                                                                                    |
| StandardTrailer              |                       | Y     |                                                                                                                                                                    |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element DerivSecList

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 156 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Derivative Security List Update Report

The Derivative Security List Update Report message is used to send updates to an option family or the strikes that comprise an option family.

# Derivative Security List Update Report

| Tag                        | FieldName             | Req'd | Comments                                                                                                                                                                                                                                                                                                 |
| -------------------------- | --------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader             |                       | Y     | MsgType = BR                                                                                                                                                                                                                                                                                             |
| component block            |                       | N     |                                                                                                                                                                                                                                                                                                          |
| ApplicationSequenceControl |                       |       |                                                                                                                                                                                                                                                                                                          |
| 320                        | SecurityReqID         | N     |                                                                                                                                                                                                                                                                                                          |
| 322                        | SecurityResponseID    | N     | Identifier for the Derivative Security List message                                                                                                                                                                                                                                                      |
| 560                        | SecurityRequestResult | N     | Result of the Security Request identified by SecurityReqID                                                                                                                                                                                                                                               |
| 980                        | SecurityUpdateAction  | N     | Updates can be applied to Underlying or option class. If Series information provided, then Series has explicitly changed                                                                                                                                                                                 |
| component block            |                       | N     | Underlying security for which derivatives are being returned                                                                                                                                                                                                                                             |
| UnderlyingInstrument       |                       | N     |                                                                                                                                                                                                                                                                                                          |
| component block            |                       | N     | Group block which contains all information for an option family. If provided DerivativeSecurityDefinition qualifies the strikes specified in the Instrument block. DerivativeSecurityDefinition contains the following components: DerivativeInstrument. DerivativeInstrumentExtension, MarketSegmentGrp |
| 60                         | TransactTime          | N     |                                                                                                                                                                                                                                                                                                          |
| 393                        | TotNoRelatedSym       | N     | Used to indicate the total number of securities being returned for this request. Used in the event that message fragmentation is required.                                                                                                                                                               |
| 893                        | LastFragment          | N     | Indicates whether this is the last fragment in a sequence of message fragments. Only required where message has been fragmented.                                                                                                                                                                         |
| component block            |                       | N     |                                                                                                                                                                                                                                                                                                          |
| RelSymDerivSecUpdGrp       |                       |       |                                                                                                                                                                                                                                                                                                          |
| StandardTrailer            |                       | Y     |                                                                                                                                                                                                                                                                                                          |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element DerivativeSecurityListUpdate

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 157 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                               August 18, 2011

# Security Status Request

The Security Status Request message provides for the ability to request the status of a security. One or more Security Status messages are returned as a result of a Security Status Request message.

The Security Status Request message contains a SubscriptionRequestType field. This tells the counter party what type of request is being made:

- 0 – indicates that the requestor only wants a snapshot or the current status.
- 1 – indicates that the requestor wants a snapshot (the current status) plus updates as the status changes. This is similar to subscribing for information and can be implemented in applications as a subscription mechanism.
- 2 – indicates that the requestor wishes to cancel any pending snapshots or updates – in essence making this an unsubscribe operation.

# Security Status Request

| Tag                              | FieldName               | Req'd | Comments                                                                                                                                                                                                                                                                             |
| -------------------------------- | ----------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader                   |                         | Y     | MsgType = e (lowercase)                                                                                                                                                                                                                                                              |
| 324                              | SecurityStatusReqID     | Y     | Must be unique, or the ID of previous Security Status Request to disable if SubscriptionRequestType = Disable previous Snapshot + Updates Request (2).                                                                                                                               |
| component block \<Instrument>    |                         | Y     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages"                                                                                                                                                                        |
| component block                  |                         | N     | Insert here the set of "InstrumentExtension" fields defined in "Common Components of Application Messages"                                                                                                                                                                           |
| component block \<UndInstrmtGrp> |                         | N     | Number of underlyings                                                                                                                                                                                                                                                                |
| component block \<InstrmtLegGrp> |                         | N     | Number of legs that make up the Security                                                                                                                                                                                                                                             |
| 15                               | Currency                | N     |                                                                                                                                                                                                                                                                                      |
| 263                              | SubscriptionRequestType | Y     | SubscriptionRequestType indicates to the other party what type of response is expected. A snapshot request only asks for current information. A subscribe request asks for updates as the status changes. Unsubscribe will cancel any future update messages from the counter party. |
| 1301                             | MarketID                | N     |                                                                                                                                                                                                                                                                                      |
| 1300                             | MarketSegmentID         | N     |                                                                                                                                                                                                                                                                                      |
| 336                              | TradingSessionID        | N     |                                                                                                                                                                                                                                                                                      |
| 625                              | TradingSessionSubID     | N     |                                                                                                                                                                                                                                                                                      |
| StandardTrailer                  |                         | Y     |                                                                                                                                                                                                                                                                                      |

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element SecStatReq

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                        Page 158 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                     August 18, 2011

# Security Status

The Security Status message provides for the ability to report changes in status to a security. The Security
Status message contains fields to indicate trading status, corporate actions, financial status of the company. The
Security Status message is used by one trading entity (for instance an exchange) to report changes in the state
of a security.

It is expected that the Security Status message that is sent as a response should indicate what type of request is
being provided. If the message is being generated as a result of a RequestType =1, then the response should
have a RequestType=1 to permit the requestor to determine why the message was sent.

| Tag             | FieldName             |
| --------------- | --------------------- |
| StandardHeader  | component block       |
|                 | SecurityStatusReqID   |
| component block |                       |
| component block |                       |
| component block |                       |
| component block |                       |
| 15              | Currency              |
| 1301            | MarketID              |
| 1300            | MarketSegmentID       |
| 336             | TradingSessionID      |
| 625             | TradingSessionSubID   |
| 325             | UnsolicitedIndicator  |
| 326             | SecurityTradingStatus |
| 1174            | SecurityTradingEvent  |
| 291             | FinancialStatus       |
| 292             | CorporateAction       |
| 327             | HaltReason            |
| 328             | InViewOfCommon        |
| 329             | DueToRelated          |
| 1021            | MDBookType            |

# Security Status

| Req'd | Comments                                                                                                      |
| ----- | ------------------------------------------------------------------------------------------------------------- |
| Y     | MsgType = f (lowercase)                                                                                       |
| N     |                                                                                                               |
| N     |                                                                                                               |
| Y     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" |
| N     | Insert here the set of "InstrumentExtension" fields defined in "Common Components of Application Messages"    |
| N     | Number of underlyings                                                                                         |
| N     | Required for multileg quotes                                                                                  |
| N     |                                                                                                               |
| N     |                                                                                                               |
| N     |                                                                                                               |
| N     |                                                                                                               |
| N     | Set to 'Y' if message is sent as a result of a subscription request not a snapshot request                    |
| N     | Identifies the trading status applicable to the transaction.                                                  |
| N     | Identifies an event related to the trading status                                                             |
| N     | Denotes the reason for the Opening Delay or Trading Halt.                                                     |
| N     |                                                                                                               |
| N     |                                                                                                               |
| N     | Used to relay changes in the book type                                                                        |

© Copyright, 2008-      ~~2009~~2011, FIX Protocol, Limited                                      Page 159 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                          August 18, 2011

| 264  | MarketDepth    | N | Used to relay changes in Market Depth.                                                                                                  |
| ---- | -------------- | - | --------------------------------------------------------------------------------------------------------------------------------------- |
| 330  | BuyVolume      | N |                                                                                                                                         |
| 331  | SellVolume     | N |                                                                                                                                         |
| 332  | HighPx         | N |                                                                                                                                         |
| 333  | LowPx          | N |                                                                                                                                         |
| 31   | LastPx         | N | Represents the last price for that security either on a Consolidated or an individual participant basis at the time it is disseminated. |
| 60   | TransactTime   | N | Trade Dissemination Time                                                                                                                |
| 334  | Adjustment     | N |                                                                                                                                         |
| 1025 | FirstPx        | N | Represents the price of the first fill of the trading session.                                                                          |
| 58   | Text           | N | Comment, instructions, or other identifying information.                                                                                |
| 354  | EncodedTextLen | N | Must be set if EncodedText field is specified and must immediately precede it.                                                          |
| 355  | EncodedText    | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.          |

StandardTrailer                          Y

FIXML Definition for this message – see http://www.fixprotocol.org for details

Refer to FIXML element SecStat

© Copyright, 2008-      ~~2009~~2011, FIX Protocol, Limited                                Page 160 of 200
---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                    August 18, 2011

# Security Definition, Security Status, and Trading Session Message Scenarios

# Overview

A set of messages has been defined for the definition and dissemination of securities information traded between two parties. These messages allow for the ability to define complex, multi-leg financial securities, such as options strategies, futures spreads, underlying-derivative combinations, indexes, and baskets. The Security Definition Request message is used to define a security to the counterparty for trading and to retrieve definitions for securities available for trading with the counterparty.

The Security Definition message can also be used to query a list of securities offered by a trading party. This message is useful for obtaining lists of products that are traded on a market. Although intended to support exchange style trading – this capability should also be of use in trading between any two trading partners.

Two additional messages have been added for status purposes: The Security Status message and the Trading Session Status message. The Security Status message is based upon the Trade Related message proposal from SIAC.

The Security Status message provides solicited or unsolicited status information on securities. An exchange can use this message to transmit change in trading state of a product. The Security Status Request message can be used to query the state of a product or to subscribe for security state changes.

The Trading Session Status message has been added to provide status on a market. An exchange can use this to indicate status on the overall market and to provide a list of securities traded during that trading session. Two trading parties can also use this message to communicate information on two-party trading. The Trading Session Status Request message is used to query the state of a product.

Both the Security Status message and Trading Session Status message include a SubscriptionRequestType field, which is used to tell the counterparty application if the requesting application wants to receive a snapshot of status or wants to subscribe for unsolicited messages as the status of the security (or trading session) changes.

# Background

The motivation behind these messages was to identify a method to be able to trade derivative strategies (butterfly spread, vertical spread, calendar spread, covered write, etc.) and to provide a mechanism to define FLEX Options using the FIX protocol. Most exchange trading systems have some type of product definition service. Although the motivation for the new messages was to support the communication between trading party and exchange, it was important to make any message flexible enough to support a variety of applications, including the ability to retrieve information about securities available for trading with a counterparty. The ability to query for a list of securities is very important in an exchange environment – where the retrieval of “standing data” from the exchange is needed by many trading systems.

# Definitions

# Strategy

A group of related securities that are traded atomically at a net price.

# Examples:

- Vertical Spread
- Butterfly Spread
- Calendar Spread
- Covered Write

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                           Page 161 of 200
---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                     August 18, 2011

# Strategy

- Leg - One Security within a strategy
- Spread - combination of derivative securities whose maturity date or strike price is spread, creating a synthetic Security.
- Synthetic - A financial security that is the result of holding positions in multiple securities.
- Combination - alias for spread or strategy.

# Approach

A Security Definition Request message can be used to define and/or request a specific Security to be traded with a counterparty.

The Security Definition message is used to:

- Indicate acceptance of a Security defined in a previous Security Definition Request message.
- Indicate acceptance of a Security defined in a previous Security Definition Request message with changes to the definition and/or symbol or security ID.
- Reject the request for security.

# Extensions to other messages

One additional field, MultiLegReportingType, is to be used on the Execution Report to indicate if the Execution Report is for the multileg security itself or an individual leg of the multileg security. Absence of this field in the Execution Report implies that the report pertains to the entire security – not an individual leg.

The agreement on how parties report multileg security execution is left to individual trading parties and is to be configured out of band. The FIX protocol will not provide a mechanism to specify how multileg execution reporting should be done.

For an example:

A straddle is an option strategy that consists of simultaneously buying a call option and a put option at the same strike price and maturity date. The straddle is defined for trading using the Security Definition Request Message. Once the straddle is defined, via receipt of the Security Definition Message from the counterparty (in this case an options exchange), a New Order – Single is used to submit the order to trade this newly defined multileg security. If the parties agree to report multileg execution by individual legs– then an execution report will be generated for each leg of the option strategy. If the parties agree to report multileg execution by multileg security only, then only one Execution Report will be issued for the fill.

Reporting by leg is required for equity options as clearing houses will only understand the individual option series legs. Reporting by legs permits the trading parties to accurately maintain positions.

# Rules

- The Security identification negotiated during the session is, by default, assumed valid only during the session.
- This eliminates the requirement for, but does not prevent the use, of a service to define and keep Securities persistent.
- Once a Security is defined, it will be traded as a regular Security.
- Once a Security is defined, it will be traded at a single net price.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                           Page 162 of 200
---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                     August 18, 2011

Once a Security is defined, it can be traded by FIX 4.1 compatible systems (This provides for backward compatibility and the ability to maintain Security information outside of FIX so that FIX 4.1 engines can participate).

# Specifying Derivative Trading Strategies using the Security Definition message

The Security Definition message can be used to specify multiple legs of a derivative trading strategy. The first set of security related fields are used to name and identify the proposed strategy. This is followed by the NoRelatedSym field (146), which indicates the number of legs in the proposed security. After the NoRelatedSym field, security related fields are repeated for each leg in the proposed security.

Two additional pieces are needed to specify the strategy.

- RatioQty is a quantity field that indicates the ratio of the leg to other legs in the strategy.
- Side indicates if that particular leg will be bought or sold as part of the strategy.

# Example using RatioQty and Side:

A Butterfly strategy consists of simultaneously:

- Buying 1 Call at Strike Price #1
- Selling 2 Calls at the next higher strike price (Strike Price #2)
- Buying 1 call at the next higher strike price (Strike Price #3)

The Legs that would describe this strategy are as follows:

| PutOrCall | RatioQty | Side   |
| --------- | -------- | ------ |
| 1=Call    | 1        | 1=Buy  |
| 1=Call    | 2        | 2=Sell |
| 1=Call    | 1        | 1=Buy  |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                            Page 163 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Scenarios

# Scenario 1 - Typical use of Security Definition message in placing an Order

This scenario has the first party defining a strategy order using a Security Definition message.

| First Party                                                                                   | Second Party                                                                                   |
| --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| Security Definition Request message                                                           | Interprets Security request                                                                    |
| SecurityRequest = 1                                                                           | Propose an identity for the Security or Request an identity for the Security from second party |
| If second party accepted Security then the first party is free to use the Security in a trade | Security Definition message                                                                    |
|                                                                                               | SecurityResponse=0                                                                             |
| New Order – Single message                                                                    | Order is handled by exchange                                                                   |
| Product = Security information from the Security Definition message                           |                                                                                                |
|                                                                                               | Execution Report                                                                               |
|                                                                                               | Order received                                                                                 |
|                                                                                               | (Most likely will need to add Security information to the Execution report)                    |
|                                                                                               | Execution Report                                                                               |
|                                                                                               | Fill Information on Order                                                                      |

# Scenario 2 - Inquire Securities Types Available

This scenario has the first party requesting a list of Security types supported by the second party.

| First Party                                           | Second Party                                                              |
| ----------------------------------------------------- | ------------------------------------------------------------------------- |
| Security Definition Request message                   | Processes Security Definition message                                     |
| SecurityRequest = 2                                   |                                                                           |
| First party can use this to select a list of messages | In this scenario, the trading party only trades three types of securities |
|                                                       | SecurityResponseType= 2                                                   |
|                                                       | NoRelatedSym=3                                                            |
|                                                       | UnderlyingSecuritySymbol=SecurityType#1                                   |
|                                                       | UnderlyingSecuritySymbol=SecurityType#2                                   |
|                                                       | UnderlyingSecuritySymbol=SecurityType#3                                   |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 164 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Scenario 3 – Inquire Common Stocks Available for Trading with Counterparty.

This example shows how the Security Definition Request Message and Security Definition Messages can be used to return a list of common stocks available for trading with a counterparty. The first party specifies the SecurityRequest equal to 3 and specifies the SecurityType of common stock. The second party returns a list of common stocks available on its market. Note: This is intended to return standing data (static data) or a list of products available for trading – it is not intended to return an order book (see Market Data messages for this purpose). This is most applicable but not limited, to the case when the second party is an exchange.

| First Party                                                                                                        | Second Party                                                               |
| ------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------- |
| Security Definition Request message                                                                                | Processes Security request                                                 |
| In this scenario the initiator wants to obtain a list of common stock available for trading with the counterparty. | Create a list of common stocks that are available for trading.             |
| SecurityRequest=3                                                                                                  | Security Definition message                                                |
| SecurityType=”CS”                                                                                                  | Contains list of common stocks available for trading with the second party |
|                                                                                                                    | SecurityResponse=3                                                         |
|                                                                                                                    | NoRelatedSym=25                                                            |
|                                                                                                                    | UnderlyingSecuritySymbol=”AOL”                                             |
|                                                                                                                    | ….Other fields for this security                                           |
|                                                                                                                    | UnderlyingSecuritySymbol=”GM”                                              |
|                                                                                                                    | ….Other fields for this security                                           |
|                                                                                                                    | UnderlyingSecuritySymbol=”IBM”                                             |
|                                                                                                                    | ….Other fields for this security                                           |

# Scenario 4 - Inquire all securities traded by a trading party

This scenario has the first party requesting a list of Security types supported by the second party.

| First Party                         | Second Party                                                                                         |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------- |
| Security Definition Request message | Processes Security request                                                                           |
| SecurityRequest=3                   | Create a list of the Securities available for the specified SecurityType                             |
|                                     | Security Definition message                                                                          |
|                                     | Contains list of Securities available for the specified the Security Types supported by second party |
|                                     | SecurityResponse=3                                                                                   |
|                                     | NoRelatedSym=XX                                                                                      |
|                                     | Security information for each security is provided for each of the XX securities.                    |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited Page 165 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Scenario 5 – Inquire Option Classes Available for Trading with Counterparty.

This example shows how the Security Definition Request Message and Security Definition Messages can be used to return a list of option classes available for trading with a counterparty. The first party specifies a Security Request Type equal to 3 (Request List of Securities) and the SecurityType of options. The second party returns a list of option classes available on its markets. Note: This is intended to return standing data (static data) or a list of products available for trading – it is not intended to return an order book (see Market Data messages).

| First Party                                                                                                                               | Second Party                                                               |
| ----------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Security Definition Request message                                                                                                       | Processes Security request                                                 |
| In this scenario the initiator wants to see a list of option series for IBM that are traded by the counterparty (that may be an exchange) | Create a list of common stocks that are available for trading.             |
| SecurityRequest=3                                                                                                                         | Security Definition message                                                |
| SecurityType=”OPT”                                                                                                                        | Contains list of common stocks available for trading with the second party |
|                                                                                                                                           | SecurityResponse=3                                                         |
|                                                                                                                                           | NoRelatedSym=25                                                            |
|                                                                                                                                           | UnderlyingSecuritySymbol=”AOL”                                             |
|                                                                                                                                           | UnderlyingSecuritySymbol=”GM”                                              |
|                                                                                                                                           | UnderlyingSecuritySymbol=”IBM”                                             |

# Scenario 6 - Inquire list of option series for a class

This scenario has the first party requesting a list of option classes by setting the SecurityRequest equal to 3, the SecurityType to “OPT”, and a security symbol = “IBM”. Because a symbol is given, the second party sends back a list of option series for the class specified with a symbol or securityID.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 166 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Security Definition Request message

| First Party                                                               | Second Party                                                                      |
| ------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| SecurityRequest=3                                                         | Processes Security request                                                        |
| SecurityType=”OPT”                                                        | Because a symbol is provided the second party sends back a list of option series. |
| Symbol=”IBM”                                                              |                                                                                   |
| Any of the security identification fields can be populated for this query |                                                                                   |

# Security Definition message

Contains list of option series available for the specified the class specified in the request.

| SecurityResponse=3                                                                | NoRelatedSym=XX |
| --------------------------------------------------------------------------------- | --------------- |
| Security information for each security is provided for each of the XX securities. |                 |

# User Defined Spreads using Security Definition Messages

User Defined Spreads (UDS) allow users to construct strategies that support their unique trading and risk needs. In an exchange-centric model, a user may request a custom-designed strategy when the pre-listed instruments offered by an exchange or counterparty are insufficient to meet these needs. If accepted by the exchange or counterparty, it will become a listed instrument.

FIX currently provides support for User Defined Spreads through the Security Definition Request and Security Definition messages. These messages allow single-leg or multi-leg requests to be submitted for instrument creation, and provide confirmation of the fully elaborated instrument. Once the UDS has been established, the requestor will generally submit a subsequent Order or Request for Quote on the newly defined instrument.

# Creating a User Defined Spread - Business Flow

The Business Process for User Defined Spreads is expressed by the workflow shown below. One-step and two-step processes are illustrated as they represent the recommended flow in FIX 4.4 (and above). The requesting party makes known its desire to define an instrument which has not been pre-listed by the Respondent - usually an exchange entity - by sending in a Request for a New Strategy. The Strategy will generally be a complex, multi-legged strategy or an options strategy which will provide neutral risk.

Upon receiving the request, the Respondent will perform validation and either accept or reject the request. If accepted, the Respondent will create a new instrument which is now considered to be “listed” [on the exchange], and send back confirmation that a new instrument has been created. Generally, the Respondent will not revise the requested instrument definition but will simply reject the request. The confirmation will carry all the details of the new instrument. The Requestor will then submit orders and/or request for quotes on the newly established instrument which will then follow the normal flow for these processes.

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 167 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Exhibit 1: One-step process

| t e r0 c e                  |                   | |
| --------------------------- | ----------------- |---|
| e $ p 0 nd a (7             | e 9 U e $ ( 0 \[  | |
| a \| k e (                  | c h               |
| V a Iid a t e               |                   | |
| R e q U e s t f0 r N tT U m |                   | |
| $ (f0                       | 5 i n (r a (e 9 y | |
| rd e \[ It i                | r d0 \[           | |
| t r ate 9y L \|sSt e        |                   |
| A c k n 0 w le d g e        |                   | |
| e W In $ tr u me n t        |                   | |
| re a ti0 0 f                |                   | |
| d is $ in a te d            |                   | |
| In $ (T U me n t a n d      |                   | |
| a rk e                      |                   | |
| e c e ip 0 rd e             |                   | |
| r ke (                      | it h              | |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 168 of 200
---
Version 5.0 Service Pack 2 - Errata           VOLUME 3                                                                                           August 18, 2011

# Exhibit 2: Two-step process

t e                  r0c  e

e  5 p0   nd a n

e q Ue $ ( 0 [                                                                                                                                a | k e (

E * c h a   n

V a lid a (e

e  $ ( f 0 r    tr at e                                          e q Ue $ t fo

In s tr u m

q Ue  s (       e | ec ( e                                            Id e n tifie

A $ $ ig n e

tra te g y

tr at eg  y        L |$ ( @    4

e  q U e 5 ( A c ce p t e d                                                    N e W In $ tru m      e n (

1nS ( r U     e n (    f i n  i t i 0         e ( Ur n                                                 d is $  in a te d

a r k e

q ue $ ( 00 [                   ( e

s i n            (r a[e 9 Y                                       R F Q S U b m itte

U $ in g   n e W

tifie

c c @      p t e

a [ k e ( Re  $

i t h

rd e | $ U             i ( [ e

r de |  c k

# Creating a User Defined Spread - FIX Message Flow

The message flow for creation of a User Defined Spread is shown in the Exhibit below. The requesting party submits a Security Definition Request Message with the objective of defining a new instrument. The requestor will submit the specifications for the new instrument as part of the Security Definition Request. The Security ID of the strategy will not be provided as it is not yet know. It is important to note that an Instrument Block need not be included on the message. The InstrumentLeg block will be used to convey the legs of the strategy. The respondent will validate the Security Definition Request, create the instrument, and respond with a Security Definition Message which will carry a Security ID or Symbol for the new instrument. The Security Definition Message will carry all the details of the new instrument. Upon receiving the Security Definition Message, the requestor will then submit an order and/or request for quote on the newly established instrument which will then follow the normal flow for these processes.

© Copyright, 2008-    ~~2009~~  2011, FIX Protocol, Limited                                                                                    Page 169 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Exhibit 3: FIX Message flow for User Defined Spread

| e $ p t /                | e q Ue $ ( 0 \[                 | a r k e (                      |                              |                     |   |   | |
| ------------------------ | ------------------------------- | ------------------------------ | ---------------------------- | ------------------- | - | - |---|
|                          | c h                             | V a Iid a te                   | c Urity                      |                     |   |   | |
|                          | f i niti 0                      | e q u eS \[                    | R e q U e $ t f0 rN e W      |                     |   |   | |
| c Urity                  | e q u e $ tT y p e = " 1        | In tr u m                      |                              |                     |   |   | |
| U D $ : U Iti-le g g e d | 0 V e r e d                     | Multi-le g g e d               | A $ $ ig n e d               | S tra te g          |   |   | |
| fin itio n               | L e g $ L e gP ric e            | 5 tra (e g y0 rL e g $ , L e g | P ric S , Ita C 0 V e r in g | u r it y fin it i 0 |   |   | |
| c U \[ i t y             | f i nit i 0                     | e \| e                         |                              |                     |   |   |
|                          |                                 | e W In $ tru m @ n t           | d is $ m in a te d           | a rk e              |   |   | |
| c Urity                  | f i nit i 0                     | u rity R e $ p 0n $ e T y pe = |                              |                     |   |   | |
| U D 5 : c U rityID       | V e r e d :S e € u rityID       | M u Iti-le g g e d fin itio    | u Iti-le g g e dD e fin itio | F Q 5 U b mitte d   |   |   | |
| L e g $ L e g P ric e    | 5 tra te g y0 \[ L e g $ ,L e g | s in g                         | rity                         |                     |   |   | |
|                          | ric e $ ,                       | Ita 0 V e rin g                | F u tu r e                   |                     |   |   | |

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited

Page 170 of 200
---
Version 5.0 Service Pack 2 - Errata    VOLUME 3                                                       August 18, 2011

# CATEGORY: PARTIES REFERENCE DATA

# Introduction

The Parties Reference Data message set provides support for the dissemination of party and related party reference information and party risk limit reference information from a master file/source to interested parties or systems that need this information. The primary use of this information is for interested parties or systems to enforce trading and clearing relationships and risk limits.

The party and related party reference information provides information for a particular party that may include:

- account owner and associated information
- primary account identifier and alternate identifiers
- risk limits
- parties related to the account - e.g. parent company, clearing firm for the account, trader of the account

The party risk limit reference information provides their risk limit information between the disseminator of the information and the recipient.

The diagram below represents how the party and risk limit information will be provided in order to enforce trading and clearing relationships.

1. The Trading Platform will have a set of Traders permissioned to trade on the platform
2. Each Trader will have a set of product entitlements that specify which products can be traded
3. Each Trader will have a set of Customer Accounts for which they are a permissioned to trade
4. Each Customer Account will have a set of risk limits which daily trading activity cannot exceed.
5. Each relationship between a Trader and a Customer Account may have an optional set of risk limits that differ from the risk limits established on the Customer Account.
6. Each Customer Account will have a single parent Trading Firm through which clearing services are offered
7. Each Trading Firm will have a set of risk limits which daily trading activity cannot exceed.
8. Each Trading Firm has a relationship with a Clearing Firm which provides a clearing guarantee for all trading activity.
9. Each Clearing Firm provides clearing services for one or more Trading Firms and their accounts. Risk limits may be applied for the platform.
10. Clearing Firm can be implied from the Trading Firm relationship

© Copyright, 2008-2011, FIX Protocol, Limited                                      Page 171 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Figure 6: Party reference structure

|                  | Trading Platform |          |
| ---------------- | ---------------- | -------- |
| Trader           | Owner            |          |
| Customer Account |                  |          |
| Trading Firm     | Risk Limit       | Platform |
| Clearing Firm    | Risk Limit       | Platform |

[Note that product based trading entitlements are currently not supported. This will be a future enhancement.]

Based on the Account Reference Structure described above, the diagrams below illustrate the physical message structure for the new Party Details List Report message and Related Party component.

© Copyright, 2008-2011, FIX Protocol, Limited Page 172 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Figure 7: Party Details List Report message structure

| PartyDetailsListReportID | PartyDetailsListRequestID | PartyDetailsRequestResult |                     |                          |                  |
| ------------------------ | ------------------------- | ------------------------- | ------------------- | ------------------------ | ---------------- |
|                          |                           | PartyListGrp              | RelatedPartyGrp     | (See Figure Structure)   |                  |
| PartyDetail              | PartyID                   | PartyIDSource             | PartyRole           |                          |                  |
| PartyAltIDs              | PartyAltID                | ContextParties            |                     |                          |                  |
|                          |                           |                           | ContextPartyID      | ContextPartyIDSource     | ContextPartyRole |
| AllPtysSubGrp            |                           | PtysSubGrp                | ContextPtysSubGrp   |                          |                  |
| PartyAltSubID            |                           | PartySubID                | ContextPartySubID   |                          |                  |
| PartyAltSubIDType        | PartySubIDType            | ContextPartySubIDType     |                     |                          |                  |
| RiskLimits               | RiskLimitType             | RiskLimitAmount           | RiskLimitCurrency   | RiskLimitPlatform        |                  |
| RiskWarningLevels        | RiskWarningLevelPercent   | RiskWarningLevelName      | RiskInstrumentScope |                          |                  |
| RiskInstrumentOperator   |                           |                           | RiskSymbol          | RiskInstrumentMultiplier |                  |
| RiskSecAIDGrp            | RiskSecurityAltID         | RiskSecurityAltIDSource   |                     |                          |                  |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 173 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Figure 8: Related Party component structure

RelatedPartyCrp
PartyRelationships
PartyRelationship
RelatedPartyDetail
RelatedPartyID
RelatedPartyIDSource
RelatedPartyRole
RelatedContext Parties
RelatedPartyAltIDs
RelatedContext PartyID
RelatedPartyAllID
RelatedContextPartyIDSource
RelatedContextPartyRole
RelatedContextPtysSubGrp
RelatedAlPtysSubGrp
RelatedPtysSubGrp
RelatedContextPartySubID
RelatedPartyAllSubID
RelatedPartySubID
RelatedContextPartySubIDType
RelatedPartyAllSubIDType
RelatedPartySubIDType
RelationshipRiskLimits
RelationshipRiskLimitType
RelationshipRiskLimitAmount
RelationshipRiskLimitCurrency
RelationshipRiskLimitPlatform
RelationshipRiskWarningLevels
RelationshipRiskInstrumentScope
RelationshipRiskWarningLevelPercent
RelationshipRiskInstrumentOperator
RelationshipRiskWarningLevelName
RelationshipRiskSymbol
RelationshipRiskInstrumentMultiplier
RelationshipRiskSecAltIDGp
RelationshipRiskSecurityAMIID
RelationshipRiskSecurityAltIDSource

The diagrams above illustrate the structure that will be provided to the trading platform. The structure allows for:

1. A set of primary party information describing the party in the list (Party Block)
2. A set of alternate party identifiers for each Party (Party AltID Block)
3. A set of secondary information for each Party (Party Sub Type Block)
4. A set of parent parties to which the primary party is subordinate (Parent Party Block) – note that each Parent Party will have its own Party List Group
5. A set of product entitlements that pertains to the Party (Product Entitlement Grp) (future consideration)
6. A set of risk limits that pertains to the primary Party (Risk Control Grp)

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 174 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Figure 9: Normal message flow

Initiator

Respondent

| Party Details List Request |                                                                                                                                                                                                                                        |   |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | - |
| PartyDetailsList RequestID | \<nev>                                                                                                                                                                                                                                 |   |
| PartylistResponseType      |                                                                                                                                                                                                                                        |   |
| PartyDetailsListReport     |                                                                                                                                                                                                                                        |   |
| PartyDetailsListRequestID  | \<raquestParty Details List Request&#xA;PartyDetailsList RequestID	\<nev>&#xA;PartylistResponseType&#xA;PartyDetailsListReport&#xA;PartyDetailsListRequestID	\<raquest $>&#xA;PartyList ReportID	\<rew?>&#xA;PartyDetailsRequestResult |   |
gt;PartyList ReportID&#x3C;rew?>PartyDetailsRequestResult

# Parties Reference Data Component Blocks

This section lists the component blocks used exclusively by the messages defined for Parties Reference Data.

# PartyListResponseTypeGrp component block

| Tag  | FieldName                | Req'd | Comments |
| ---- | ------------------------ | ----- | -------- |
| 1506 | NoPartyListResponseTypes | Y     |          |
| 1507 | PartyListResponseTyp     | Y     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element RspTyp

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 175 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Tag

# FieldName

# Req'd

# Comments

| 1508 | NoRequestedPartyRoles | N |                                                                                |
| ---- | --------------------- | - | ------------------------------------------------------------------------------ |
| 1509 | RequestedPartyRole    | N | Identifies the type of party requested. Required if NoRequestedPartyRoles > 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element ReqR

# PartyRelationships component block

| Tag  | FieldName            | Req'd | Comments                                |
| ---- | -------------------- | ----- | --------------------------------------- |
| 1514 | NoPartyRelationships | N     |                                         |
| 1515 | PartyRelationship    | N     | Required when NoPartyRelationships > 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Rltnshp

# PartyListGrp component block

| Tag  | FieldName          | Req'd | Comments                     |
| ---- | ------------------ | ----- | ---------------------------- |
| 1513 | NoPartyList        | N     |                              |
|      | component block    | N     | Required if NoPartyList > 0. |
|      | \<PartyDetail>     |       |                              |
|      | component block    | N     |                              |
|      | \<RelatedPartyGrp> |       |                              |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element PartyListGrp

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 176 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# PartyDetail component block

| Tag                               | FieldName     |
| --------------------------------- | ------------- |
| 448                               | PartyID       |
| 447                               | PartyIDSource |
| 452                               | PartyRole     |
| component block \<PtysSubGrp>     |               |
| component block \<PartyAltIDs>    |               |
| component block \<ContextParties> |               |
| component block \<RiskLimits>     |               |

| Req'd | Comments                                                                     |
| ----- | ---------------------------------------------------------------------------- |
| Y     | The identification of the party.                                             |
| Y     | Used to identify source of PartyID value (e.g. BIC).                         |
| Y     | Identifies the type of PartyID (e.g. Executing Broker).                      |
| N     |                                                                              |
| N     | Optionally used to specify alternate IDs to identify the party specified.    |
| N     | Optionally used to specify parties that identify the context of the PartyID. |
| N     | Optionally used to specify risk limits.                                      |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element PtyDetl

# PartyAltIDs component block

| Tag                              | FieldName        | Req'd | Comments                         |
| -------------------------------- | ---------------- | ----- | -------------------------------- |
| 1516                             | NoPartyAltIDs    | N     |                                  |
| 1517                             | PartyAltID       | N     | Required when NoPartyAltIDs > 0. |
| 1518                             | PartyAltIDSource | N     | Required when NoPartyAltIDs > 0. |
| component block \<AltPtysSubGrp> |                  |       |                                  |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element AltPty

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 177 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# AltPtysSubGrp component block

| Tag  | FieldName         | Req'd | Comments                            |
| ---- | ----------------- | ----- | ----------------------------------- |
| 1519 | NoPartyAltSubIDs  | N     |                                     |
| 1520 | PartyAltSubID     | N     | Required when NoPartyAltSubIDs > 0. |
| 1521 | PartyAltSubIDType | N     | Required when NoPartyAltSubIDs > 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

# ContextParties component block

| Tag                  | FieldName            | Req'd | Comments                            |
| -------------------- | -------------------- | ----- | ----------------------------------- |
| 1522                 | NoContextPartyIDs    | N     |                                     |
| 1523                 | ContextPartyID       | N     | Required when NoContextParties > 0. |
| 1524                 | ContextPartyIDSource | N     | Required when NoContextParties > 0. |
| 1525                 | ContextPartyRole     | N     | Required when NoContextParties > 0. |
| component block      |                      |       |                                     |
| \<ContextPtysSubGrp> |                      |       |                                     |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element CntxtPty

# ContextPtysSubGrp component block

| Tag  | FieldName            | Req'd | Comments                                |
| ---- | -------------------- | ----- | --------------------------------------- |
| 1526 | NoContextPartySubIDs | N     |                                         |
| 1527 | ContextPartySubID    | N     | Required when NoContextPartySubIDs > 0. |
| 1528 | ContextPartySubIDTy  | N     | Required when NoContextPartySubIDs > 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 178 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Refer to FIXML element Sub

# RiskLimits component block

| Tag                    | FieldName         | Req'd | Comments                        |
| ---------------------- | ----------------- | ----- | ------------------------------- |
| 1529                   | NoRiskLimits      | N     |                                 |
| 1530                   | RiskLimitType     | N     | Required when NoRiskLimits > 0. |
| 1531                   | RiskLimitAmount   | N     | Required when NoRiskLimits > 0. |
| 1532                   | RiskLimitCurrency | N     |                                 |
| 1533                   | RiskLimitPlatform | N     |                                 |
| component block        |                   |       |                                 |
| \<RiskInstrumentScope> |                   |       |                                 |
| component block        |                   |       |                                 |
| \<RiskWarningLevels>   |                   |       |                                 |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

# Refer to FIXML element RiskLmt

# RiskInstrumentScope component block

| Tag                | FieldName            | Req'd | Comments                             |
| ------------------ | -------------------- | ----- | ------------------------------------ |
| 1534               | NoRiskInstruments    | N     |                                      |
| 1535               | RiskInstrumentOperat | N     | Required when NoRiskInstruments > 0. |
|                    | or                   |       |                                      |
| 1536               | RiskSymbol           | N     |                                      |
| 1537               | RiskSymbolSfx        | N     |                                      |
| 1538               | RiskSecurityID       | N     |                                      |
| 1539               | RiskSecurityIDSource | N     |                                      |
| component block    |                      |       |                                      |
| \<RiskSecAltIDGrp> |                      |       |                                      |
| 1543               | RiskProduct          | N     |                                      |
| 1544               | RiskProductComplex   | N     |                                      |
| 1545               | RiskSecurityGroup    | N     |                                      |
| 1546               | RiskCFICode          | N     |                                      |
| 1547               | RiskSecurityType     | N     |                                      |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 179 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Risk Security Types

| £ | 1548 | RiskSecuritySubType   | N |                                   |
| - | ---- | --------------------- | - | --------------------------------- |
| £ | 1549 | RiskMaturityMonthYe   | N |                                   |
| £ | 1550 | RiskMaturityTime      | N |                                   |
| £ | 1551 | RiskRestructuringType | N |                                   |
| £ | 1552 | RiskSeniority         | N |                                   |
| £ | 1553 | RiskPutOrCall         | N |                                   |
| £ | 1554 | RiskFlexibleIndicator | N |                                   |
| £ | 1555 | RiskCouponRate        | N |                                   |
| £ | 1616 | RiskSecurityExchange  | N |                                   |
| £ | 1556 | RiskSecurityDesc      | N |                                   |
| £ | 1620 | RiskEncodedSecurity   | N |                                   |
| £ | 1621 | RiskEncodedSecurity   | N |                                   |
| £ | 1557 | RiskInstrumentSettlTy | N | Can be used to specify FX tenors. |
| £ | 1558 | RiskInstrumentMultipl | N |                                   |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element InstrmtScope

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 180 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# RiskSecAltIDGrp component block

| Tag  | FieldName            | Req'd | Comments                               |
| ---- | -------------------- | ----- | -------------------------------------- |
| 1540 | NoRiskSecurityAltID  | N     |                                        |
| 1541 | RiskSecurityAltID    | N     | Required when NoRiskSecurityAltID > 0. |
| 1542 | RiskSecurityAltIDSou | N     | Required when NoRiskSecurityAltID > 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element AID

# RiskWarningLevels component block

| Tag  | FieldName           | Req'd | Comments                               |
| ---- | ------------------- | ----- | -------------------------------------- |
| 1559 | NoRiskWarningLevels | N     |                                        |
| 1560 | RiskWarningLevelPer | N     | Required when NoRiskWarningLevels > 0. |
| 1561 | RiskWarningLevelNa  | N     |                                        |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element WarnLvl

# RelatedPartyGrp component block

| Tag             | FieldName         | Req'd | Comments                                                                                                                                                 |
| --------------- | ----------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1562            | NoRelatedPartyIDs | N     |                                                                                                                                                          |
| component block |                   | N     | Required when NoRelatedPartyIDs > 0. The identification of the related party.                                                                            |
| component block |                   | N     | Can be used to define a list of relationships that exist between the party specified at a higher level and the party specified in \<RelatedPartyDetail>. |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 181 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element ReltdPty

# RelatedPartyDetail component block

| Tag                       | FieldName            | Req'd | Comments                                         |
| ------------------------- | -------------------- | ----- | ------------------------------------------------ |
| 1563                      | RelatedPartyID       | N     | Required when this component block is specified. |
| 1564                      | RelatedPartyIDSource | N     | Required when this component block is specified. |
| 1565                      | RelatedPartyRole     | N     | Required when this component block is specified. |
| component block           |                      |       |                                                  |
| \<RelatedPtysSubGrp>      |                      |       |                                                  |
| component block           |                      |       |                                                  |
| \<RelatedPartyAltIDs>     |                      |       |                                                  |
| component block           |                      |       |                                                  |
| \<RelatedContextParties>  |                      |       |                                                  |
| component block           |                      |       |                                                  |
| \<RelationshipRiskLimits> |                      |       |                                                  |

# FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element PtyDetl

# RelatedPtysSubGrp component block

| Tag  | FieldName            | Req'd | Comments                                |
| ---- | -------------------- | ----- | --------------------------------------- |
| 1566 | NoRelatedPartySubIDs | N     |                                         |
| 1567 | RelatedPartySubID    | N     | Required when NoRelatedPartySubIDs > 0. |
| 1568 | RelatedPartySubIDTy  | N     | Required when NoRelatedPartySubIDs > 0. |

# FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 182 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# RelatedPartyAltIDs component block

| Tag                     | FieldName            | Req'd | Comments                                |
| ----------------------- | -------------------- | ----- | --------------------------------------- |
| 1569                    | NoRelatedPartyAltIDs | N     |                                         |
| 1570                    | RelatedPartyAltID    | N     | Required when NoRelatedPartyAltIDs > 0. |
| 1571                    | RelatedPartyAltIDSou | N     | Required when NoRelatedPartyAltIDs > 0. |
| component block         |                      |       |                                         |
| \<RelatedAltPtysSubGrp> |                      |       |                                         |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element AltPty

# RelatedAltPtysSubGrp component block

| Tag  | FieldName               | Req'd | Comments                                   |
| ---- | ----------------------- | ----- | ------------------------------------------ |
| 1572 | NoRelatedPartyAltSubIDs | N     |                                            |
| 1573 | RelatedPartyAltSubID    | N     | Required when NoRelatedPartyAltSubIDs > 0. |
| 1574 | RelatedPartyAltSubID    | N     | Required when NoRelatedPartyAltSubIDs > 0. |
| Type |                         |       |                                            |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

# RelatedContextParties component block

| Tag  | FieldName                | Req'd | Comments                                   |
| ---- | ------------------------ | ----- | ------------------------------------------ |
| 1575 | NoRelatedContextPartyIDs | N     |                                            |
| 1576 | RelatedContextPartyI     | N     | Required when NoRelatedContextParties > 0. |
| 1577 | RelatedContextPartyI     | N     | Required when NoRelatedContextParties > 0. |
| 1578 | RelatedContextPartyR     | N     | Required when NoRelatedContextParties > 0. |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 183 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# RelatedContextPtysSubGrp component block

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element CntxtPty

| Tag  | FieldName                    | Req'd | Comments                                       |
| ---- | ---------------------------- | ----- | ---------------------------------------------- |
| 1579 | NoRelatedContextPartySubIDs  | N     |                                                |
| 1580 | RelatedContextPartySubID     | N     | Required when NoRelatedContextPartySubIDs > 0. |
| 1581 | RelatedContextPartySubIDType | N     | Required when NoRelatedContextPartySubIDs > 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 184 of 200
---
Version 5.0 Service Pack 2 - Errata                             VOLUME 3           August 18, 2011

# RelationshipRiskLimits component block

| Tag  | FieldName                | Req'd | Comments                                    |
| ---- | ------------------------ | ----- | ------------------------------------------- |
| 1582 | NoRelationshipRiskLimits | N     |                                             |
| 1583 | RelationshipRiskLimit    | N     | Required when NoRelationshipRiskLimits > 0. |
| 1584 | RelationshipRiskLimit    | N     | Required when NoRelationshipRiskLimits > 0. |
| 1585 | RelationshipRiskLimit    | N     | Currency                                    |
| 1586 | RelationshipRiskLimit    | N     | Platform                                    |
|      | component block          | N     |                                             |
|      | component block          | N     |                                             |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element RiskLmt

# RelationshipRiskInstrumentScope component block

| Tag  | FieldName                          | Req'd | Comments                                         |
| ---- | ---------------------------------- | ----- | ------------------------------------------------ |
| 1587 | NoRelationshipRiskInstrument       | N     | s                                                |
| 1588 | RelationshipRiskInstrumentOperator | N     | Required when NoRelationshipRiskInstruments > 0. |
| 1589 | RelationshipRiskSymbol             | N     |                                                  |
| 1590 | RelationshipRiskSymbolSfx          | N     |                                                  |
| 1591 | RelationshipRiskSecurityID         | N     |                                                  |
| 1592 | RelationshipRiskSecurityIDSource   | N     |                                                  |
|      | component block                    | N     |                                                  |

© Copyright, 2008-2009 2011, FIX Protocol, Limited                                                 Page 185 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Relationship Risk Products

| £ | 1596 | RelationshipRiskProdu  | N |                                   |
| - | ---- | ---------------------- | - | --------------------------------- |
| £ | 1597 | RelationshipRiskProdu  | N |                                   |
| £ | 1598 | RelationshipRiskSecur  | N |                                   |
| £ | 1599 | RelationshipRiskCFIC   | N |                                   |
| £ | 1600 | RelationshipRiskSecur  | N |                                   |
| £ | 1601 | RelationshipRiskSecur  | N |                                   |
| £ | 1602 | RelationshipRiskMatur  | N |                                   |
| £ | 1603 | RelationshipRiskMatur  | N |                                   |
| £ | 1604 | RelationshipRiskRestr  | N |                                   |
| £ | 1605 | RelationshipRiskSenio  | N |                                   |
| £ | 1606 | RelationshipRiskPutOr  | N |                                   |
| £ | 1607 | RelationshipRiskFlexi  | N |                                   |
| £ | 1608 | RelationshipRiskCoup   | N |                                   |
| £ | 1609 | RelationshipRiskSecur  | N |                                   |
| £ | 1610 | RelationshipRiskSecur  | N |                                   |
| £ | 1618 | RelationshipRiskEnco   | N |                                   |
| £ | 1619 | RelationshipRiskEnco   | N |                                   |
| £ | 1611 | RelationshipRiskInstru | N | Can be used to specify FX tenors. |
| £ | 1612 | RelationshipRiskInstru | N |                                   |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 186 of 200
---
Version 5.0 Service Pack 2 - Errata                              VOLUME 3             August 18, 2011

# Refer to FIXML element InstrmtScope

# RelationshipRiskSecAltIDGrp component block

| Tag  | FieldName                           | Req'd | Comments                                           |
| ---- | ----------------------------------- | ----- | -------------------------------------------------- |
| 1593 | NoRelationshipRiskSecurityAltID     | N     |                                                    |
| 1594 | RelationshipRiskSecurityAltID       | N     | Required when NoRelationshipRiskSecurityAltID > 0. |
| 1595 | RelationshipRiskSecurityAltIDSource | N     | Required when NoRelationshipRiskSecurityAltID > 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

# Refer to FIXML element AID

# RelationshipRiskWarningLevels component block

| Tag  | FieldName                           | Req'd | Comments                                           |
| ---- | ----------------------------------- | ----- | -------------------------------------------------- |
| 1613 | NoRelationshipRiskWarningLevels     | N     |                                                    |
| 1614 | RelationshipRiskWarningLevelPercent | N     | Required when NoRelationshipRiskWarningLevels > 0. |
| 1615 | RelationshipRiskWarningLevelName    | N     |                                                    |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

# Refer to FIXML element WarnLvl

© Copyright, 2008-2009 2011, FIX Protocol, Limited                   Page 187 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Party Details List Request

The Party Details List Request message is used to request party reference information from a central master reference system or another party that stores and maintains party reference information. The central master reference system can be an exchange that provides such information to trading applications that connect to it. Reference information may include relationships between parties, and details such as risk limits. The response to this message is the PartyDetailsListReport.

# Types of requests may include:

- Request all party information available from counterparty.
- Request party information for a specific party identifier or list of identifiers.
- Request party information for one or more party roles.
- Request party information for one or more types of party relationships.
- Request all related party information.
- Request party risk limits, or risk limits specific to a relationship between parties.

A request may specify one or more PartyID values, one or more RequestedPartyRole values, one or more RelationshipType values, or none of these. A request without these fields returns the requested details on all parties, as determined by PartyListResponseType. A request specifying only one or more PartyIDs returns details about those parties. This may not include risk limits and/or related parties, as determined by PartyListResponseType. A request specifying only one or more RequestedPartyRole returns details about all parties with a matching PartyRole. This may not include risk limits and/or related parties, as determined by PartyListResponseType.

# The message definition for PartyDetailsListRequest is:

| Tag                         | FieldName                 | Req'd | Comments     |
| --------------------------- | ------------------------- | ----- | ------------ |
| StandardHeader              |                           | Y     | MsgType = CF |
| 1505                        | PartyDetailsListRequestID | Y     |              |
| component block             |                           | Y     |              |
| \<PartyListResponseTypeGrp> | component block           | N     |              |
| \<Parties>                  | component block           | N     |              |
| component block             |                           | N     |              |
| \<RequestedPartyRoleGrp>    | component block           | N     |              |
| \<PartyRelationships>       |                           | N     |              |
| 263                         | SubscriptionRequestType   | N     |              |
| 58                          | Text                      | N     |              |
| 354                         | EncodedTextLen            | N     |              |
| 355                         | EncodedText               | N     |              |
| StandardTrailer             |                           | Y     |              |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 188 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Party Details List Report

The Party Details List Report message is used by a central master file system or another party that stores and maintains party reference information to disseminate party reference information. Reference information may include relationships between parties, and details such as risk limits. This message can be sent in response to the Party Details List Request or it may be sent unsolicited without a request.

# The message definition for PartyDetailsListReport is:

| Tag             | FieldName                 | Req'd | Comments                                                    |
| --------------- | ------------------------- | ----- | ----------------------------------------------------------- |
| StandardHeader  |                           | Y     | MsgType = CG                                                |
| component block |                           | N     |                                                             |
| 1510            | PartyDetailsListReportID  | Y     |                                                             |
| 1505            | PartyDetailsListRequestID | N     | Required when responding to the Party Details List Request. |
| 1511            | PartyDetailsRequestResult | N     | Required when responding to the Party Details List Request. |
| 1512            | TotNoPartyList            | N     |                                                             |
| 893             | LastFragment              | N     |                                                             |
| component block |                           | N     |                                                             |
| 58              | Text                      | N     |                                                             |
| 354             | EncodedTextLen            | N     |                                                             |
| 355             | EncodedText               | N     |                                                             |
| StandardTrailer |                           | Y     |                                                             |

© Copyright, 2008-2009 2011, FIX Protocol, Limited

Page 189 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Usage of Parties Reference Data Messages

# Expressing Party Relationships and Querying for Party Relationships

Party relationships, as indicated in the response, can either be inferred based on PartyRole, or made explicit. For example, if PartyRole is Executing Firm(1) and RelatedPartyRole is Customer Account(24), it is inferred that the executing firm trades for the customer account. This relationship can also be made explicit by using the PartyRelationships component block, specifying PartyRelationship as Trades for(3).

Relationships between parties are modeled as a web or mesh. PartyRelationship can indicate that an entity of a specific PartyRole is also an entity of different PartyRole, e.g. an Executing Firm with one ID is also a Clearing Firm with another ID.

With the exception of “Is also,” PartyRelationship is specified as a list of reciprocal relationships. This enables a relationship to be conveyed regardless of which party is specified in PartyID and which is specified in RelatedPartyID. The list of reciprocal party relationships is as follows:

| 1  | Clears for                    | 2  | Clears through             |
| -- | ----------------------------- | -- | -------------------------- |
| 3  | Trades for                    | 4  | Trades through             |
| 5  | Sponsors                      | 6  | Sponsored though           |
| 7  | Provides guarantee for        | 8  | Is guaranteed by           |
| 9  | Member of                     | 10 | Has members                |
| 11 | Provides marketplace for      | 12 | Participant of marketplace |
| 13 | Carries positions for         | 14 | Post trades to             |
| 15 | Enters trades for             | 16 | Enters trades through      |
| 17 | Provides quotes to            | 18 | Requests quotes from       |
| 19 | Invests for                   | 20 | Invests through            |
| 21 | Brokers trades for            | 22 | Brokers trades through     |
| 23 | Provides trading services for | 24 | Uses trading services of   |
| 25 | Approves of                   | 26 | Approved by                |
| 27 | Parent firm for               | 28 | Subsidiary of              |
| 29 | Regulatory owner of           | 30 | Owned by (regulatory)      |
| 31 | Controls                      | 32 | Is controlled by           |
| 33 | Legal / titled owner of       | 34 | Owned by (legal / title)   |
| 35 | Beneficial owner of           | 36 | Owned by (beneficial)      |

If Executing Firm A “Trades for” Customer Account B, then Customer Account B “Trades through” Executing Firm A. If Executing Firm A were specified in PartyID, and Customer Account B were specified in RelatedPartyID, then PartyRelationship would be Trades for(3). Alternately, if Customer Account B were specified in PartyID, and Executing Firm A were specified in RelatedPartyID, then PartyRelationship would be Trades through(4).

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 190 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

Examples illustrating each odd numbered PartyRelationship type, with several possible PartyRole and RelatedPartyRole values, are given in the table below. Only odd numbered PartyRelationship values are illustrated because the even numbered cases can be obtained by swapping PartyRole and RelatedPartyRole. This list is not exclusive, but rather illustrative of some relations that can be modeled.

| PartyRelationship             | Applicable PartyRole                | Applicable RelatedPartyRole |
| ----------------------------- | ----------------------------------- | --------------------------- |
| 1 - Clears for                | Clearing Firm                       | Executing Firm              |
|                               | Prime Broker                        | Customer Account            |
|                               | Clearing Account                    | Introducing Broker          |
|                               |                                     | Broker Clearing ID          |
| 3 - Trades for                | Executing Firm                      | Customer Account            |
|                               | Executing Trader                    | Clearing Account            |
| 5 - Sponsors                  | Executing Firm                      | Executing Trader            |
| 7 - Provides guarantee for    | Clearing Organization               | Clearing Firm               |
|                               | Executing Firm                      | Investor ID                 |
|                               | Customer Account                    | Clearing Account            |
| 9 - Member of                 | Clearing Firm                       | Clearing Organization       |
|                               | Executing Firm                      | Exchange                    |
|                               | Prime Broker                        | Regulated Market (RM)       |
|                               | Market Maker                        |                             |
| 11 - Provides marketplace for | Exchange                            | Executing Firm              |
|                               | Systematic internaliser (SA)        | Executing Trader            |
|                               | Multilateral Trading Facility (MTF) | Investor ID                 |
|                               | Prime Broker                        | Regulated Market (RM)       |
|                               | Market Maker                        |                             |
| 13 - Carries positions for    | Position Account                    | Executing Firm              |
|                               | Customer Account                    | Clearing Account            |
| 15 - Enters trades for        | Entering Firm                       | Executing Trader            |
|                               | Entering Trader                     |                             |
| 17 - Provides quotes to       | Market Maker                        | Executing Trader            |
|                               | Exchange                            |                             |
| 19 - Invests for              | Investor ID                         | Customer Account            |
|                               | Clearing Account                    |                             |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 191 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Party Relationship and Applicable Roles

| PartyRelationship                  | Applicable PartyRole                    | Applicable RelatedPartyRole            |
| ---------------------------------- | --------------------------------------- | -------------------------------------- |
| 21 - Brokers trades for            | Introducing Broker                      | Customer Account                       |
|                                    | Broker Clearing ID                      | Clearing Account                       |
| 23 - Provides trading services for | Executing Firm                          | Executing Trader                       |
|                                    | Prime Broker                            |                                        |
| 25 - Approves of                   | Clearing Firm                           | Introducing Broker                     |
|                                    | Broker Clearing ID                      |                                        |
| 27 – Parent firm for               | Any role applicable for the parent firm | Any role applicable for the subsidiary |
| 29 – Regulatory owner of           | Clearing Firm                           | Customer Account                       |
|                                    | Executing Firm                          | Position Account                       |
|                                    | Clearing Account                        |                                        |
| 31 – Controls                      | Clearing Firm                           | Customer Account                       |
|                                    | Executing Firm                          | Position Account                       |
|                                    | Executing Trader                        | Clearing Account                       |
|                                    | Investor ID                             | Asset Manager                          |
| 33 - Legal / titled owner of       | Investor ID                             | Customer Account                       |
|                                    | Executing Firm                          | Clearing Account                       |
| 35 - Beneficial owner of           | Investor ID                             | Customer Account                       |
|                                    | Executing Firm                          | Clearing Account                       |

Multiple PartyRelationship values can be specified if multiple types of relationships exist between two parties. For example, a Clearing Firm might both “Clears for” and “Approves of” an Introducing Broker.

Parties can also have one or more parties specified to clarify the context in which the original party is used. An example might be a Clearing Firm that is a member of multiple Clearing Organizations and has a different ID assigned by each Clearing Organization. When referencing the ID assigned by just one Clearing Organization, the Clearing Firm is the party, and that specific Clearing Organization is the ContextPartyID.

PartyRelationship matches parties that participate as the PartyID in the relationship specified. For example, a query with PartyRelationship = Trades for(3) may return all Executing Firms that trade for various Customer Accounts.

Whether they are paired with their related party (e.g. the Customer Account) or not depends upon the PartyListResponseType. Should PartyListResponseType = 0 or 2, then both the Executing Firms and the Customer Accounts are returned. (Note that the party order can be reversed and the relationship expressed as Trades through(4) as described above.) If neither 0 nor 2 are specified, then just the Executing Firms are returned.

Specifying more than one of these criteria further limits the results set. For example, requests including one or more PartyID and one or more PartyRelationship restrict the query to those parties who are in the specified relationship(s) with the party or parties specified. In the example above, a query with PartyID = Executing Firm A and PartyRelationship = Trades for(3), then Customer Account B is selected.

If PartyListResponseType = 0 or 2, then a response including both Executing Firm A and Customer Account B would be returned. In the response, if PartyID = Executing Firm A and RelatedPartyID = Customer Account B, RelationshipType = Trades for (3) would be sent. Another valid response would be PartyID = Customer Account B, RelatedPartyID = Executing Firm A, with RelationshipType = Trades through (4).

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 192 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

PartyListResponseType = 1, then a record with PartyID = Customer Account B would be sent in response with no related parties or party relationships specified.

# Expressing Risk Limits

Certain party details, e.g. risk limits, are conveyed in both the PartyDetail and RelatedPartyDetail component blocks. Such information in the PartyDetail component block refers to the PartyID identified therein. However, such information in the RelatedPartyDetail component block may refer only to the relationship between the PartyID and the RelatedPartyID, and is not a property of the RelatedPartyID itself. For example, if PartyID = Customer Account A and RiskLimitAmount = 10000000, then Customer Account A has a $10MM risk limit. If RelatedPartyID=Executing Trader B and RelationshipRiskLimitAmount=2000000, then Executing Trader B has a $2MM risk limit while trading for Customer Account A. This limit is valid in Customer Account A only; Executing Trader B may have a different risk limit in other accounts, and a different combined risk limit among all accounts that Executing Trader B trades for. The RelatedPartyDetail component block will explicitly indicate if information refers to the relationship and not the related party. Multiple risk limits may apply to a given entity or relationship. For example, a customer account may have a $10MM risk limit, a $7MM risk limit on index futures, and a $7MM risk limit on agricultural futures. When multiple risk limits are specified, then all relevant limits must be satisfied or a trade cannot occur. For example, purchasing $8MM in pork bellies is allowed by the customer account limit, is unaffected by the index futures limit, and is prohibited by the agricultural futures limit; therefore, the trade is not allowed. But a trader can buy $6MM pork bellies. If the trader then attempts to buy an additional $6MM S&#x26;P500 index futures, the action would be allowed under the index futures limit, and would be unaffected by the agricultural futures limit, but the overall account limit of $10MM would be exceeded. The trader could only purchase at most $4MM in S&#x26;P500 futures while still satisfying all risk limits.

When a risk limit applies only to specific instruments, the RiskInstrumentScope and RelationshipRiskInstrumentScope component blocks define the instrument or instruments in question. The RiskInstrumentOperator and RelationshipRiskLimitOperator determine whether the matching instruments are to be included or excluded in the risk limit. All fields in the component block are optional. Any absent field will match all instruments. Specifying a value in a field will restrict the risk limit to instruments with matching values. Lists of instruments, or complex matching criteria, can be specified for a single risk limit. When multiple include and exclude operations are specified, all of the rules are applied in order. For example, Include Futures, Exclude Agricultural Futures, Include Corn would:

1. Build a result set consisting of all futures
2. Remove all agricultural futures (including corn) from the result set
3. Add corn back into the result set

S&#x26;P500 futures would be part of the risk limit (as they were added in #1 and not removed by #2.) Pork bellies would not be part of the risk limit (as they were added in #1 and removed in #2.) Corn would be part of the risk limit (even though #2 removes it, #3 adds it back in again.)

RiskInstrumentMultiplier and RelationshipRiskInstrumentMultipler allow different instruments to contribute to risk limits proportionally. Two examples where this may be used are FX and Treasuries. For example, a risk limit of $100MM may be set for spot FX, but 3 month FX carries more risk and has a limit of $50MM. This could be modeled as a single $100MM risk limit, with 2 instrument types (spot and 3 month) where the 3 month instruments have a RiskInstrumentMuliplier of 2.0. Trading $30MM of 3 month FX would then consume $60MM of the risk limit due to the multiplier of 2.0, so only $40MM of spot FX could then be traded. Or, a risk limit of $100MM may be set for 30 year Treasuries, but 10 year Treasuries carry less risk so $200MM of these may be allowed. In this example, the limit is $100MM, and both 30 year and 10 year Treasuries are listed, with the 10 year Treasuries having a RiskMultiplier of 0.5. So if a trader buys $40MM of 30 year Treasuries, then $60MM of the risk limit remains. Since 10 year Treasuries carry a RiskMultiplier of 0.5, then $120MM of 10 year Treasuries may be purchased.

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 193 of 200
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011



# Examples

The following are examples of unsolicited Party Details List Report message. The standard header and trailer has been omitted for clarity, and the text names of the FIX fields are used.

# Trader Party List Example

Valid Trader ID’s representing account owners have been defined in this list. A trader is linked to one or more customer accounts in the Customer Account Party List. It will be necessary for the trading platform to link an operator to a specific trader id.

PartyDetailsListReportID = 1
NoPartyList = 4
PartyID = 1234567890                 // Investor ID
PartyIDSource = D                     // Proprietary / Custom code
PartyRole = 5                        // Investor ID
NoPartySubIDs = 2
PartySubID = GOGOL AND ASSOCIATES
PartySubIDType = 5                   // Full legal name of firm
PartySubID = GOGOL
PartySubIDType = 1                   // Firm
PartyID = 2345678901                 // Investor ID
PartyIDSource = D                     // Proprietary / Custom code
PartyRole = 5                        // Investor ID
NoPartySubIDs = 2
PartySubID = PAULSON INVESTMENTS
PartySubIDType = 5                   // Full legal name of firm
PartySubID = PAULSON
PartySubIDType = 1                   // Firm
PartyID = 3456789012                 // Investor ID
PartyIDSource = D                     // Proprietary / Custom code
PartyRole = 5                        // Investor ID
NoPartySubIDs = 2
PartySubID = BERNANKE TRADING
PartySubIDType = 5                   // Full legal name of firm
PartySubID = BERNANKE
PartySubIDType = 1                   // Firm
PartyID = 346894                     // Investor ID
PartyIDSource = D                     // Proprietary / Custom code
PartyRole = 5                        // Investor ID
NoPartySubIDs = 2
PartySubID = HELBERG, LESLIE
PartySubIDType = 5                   // Full legal name of firm
PartySubID = HELBERG
PartySubIDType = 1                   // Firm

# Customer Account Party List

Valid customer accounts have been defined in this list. The customer account alias is provided as an alternate id. The Related Parties for each customer account are the authorized Trader (in this case, the Investor ID), Trading Firm, and Clearing Firm. Risk controls are defined at the levels of platform and product group. The first account can trade any product to an exposure of $90M. At the product group level, a product restriction is placed on CDS-IG where a gross limit has been set at $50M and a net limit at $10M. The second account can only trade up to $7M gross in CDS-IG and up to $5M net. The account level exposure limit is set at $7.5M. All of these restrictions apply to the platform “DVS” only.

PartyDetailsListReportID = 3
NoPartyList = 2
PartyID = CUST601986                 // Customer Account
PartyIDSource = D                     // Proprietary / Custom code
PartyRole = 24                       // Customer Account
NoPartySubIDs = 2
PartySubID = GOGOL AND ASSOCIATES                  // Account Name
PartySubIDType = 5                   // Full legal name of firm


© Copyright, 2008-2009, FIX Protocol, Limited

Page 194 of 200


---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# Party Information

# Party Sub IDs

| PartySubID     | 1  |
| -------------- | -- |
| PartySubIDType | 26 |

# Party Alternate IDs

| NoPartyAltIDs | 1            |
| ------------- | ------------ |
| PartyAltID    | 987ABC654XYZ |

# Risk Limits

| NoRiskLimits           | 3        |
| ---------------------- | -------- |
| RiskLimitType          | 3        |
| RiskLimitAmount        | 90000000 |
| RiskLimitCurrency      | USD      |
| RiskLimitPlatform      | DRS      |
| RiskLimitType          | 2        |
| RiskLimitAmount        | 10000000 |
| RiskLimitCurrency      | USD      |
| RiskLimitPlatform      | DRS      |
| NoRiskInstruments      | 1        |
| RiskInstrumentOperator | 1        |
| RiskSecurityGroup      | IG       |
| RiskSecurityType       | CDS      |
| RiskLimitType          | 1        |
| RiskLimitAmount        | 50000000 |
| RiskLimitCurrency      | USD      |
| RiskLimitPlatform      | DRS      |

# Related Party IDs

| NoRelatedPartyIDs    | 3          |
| -------------------- | ---------- |
| RelatedPartyID       | 1234567890 |
| RelatedPartyIDSource | D          |
| RelatedPartyRole     | 5          |
| NoPartyRelationships | 1          |
| PartyRelationship    | 20         |
| RelatedPartyID       | 313        |
| RelatedPartyIDSource | D          |
| RelatedPartyRole     | 1          |

# Related Context Parties

| NoRelatedContextParties     | 1   |
| --------------------------- | --- |
| RelatedContextPartyID       | CME |
| RelatedContextPartyIDSource | D   |
| RelatedContextPartyRole     | 22  |

# Additional Party Relationships

| NoPartyRelationships | 1   |
| -------------------- | --- |
| PartyRelationship    | 4   |
| RelatedPartyID       | 312 |
| RelatedPartyIDSource | D   |
| RelatedPartyRole     | 4   |

# Clearing Context Parties

| NoRelatedContextParties     | 1   |
| --------------------------- | --- |
| RelatedContextPartyID       | CME |
| RelatedContextPartyIDSource | D   |
| RelatedContextPartyRole     | 21  |

# Clearing Relationships

| NoPartyRelationships | 1          |
| -------------------- | ---------- |
| PartyRelationship    | 2          |
| PartyID              | CUSTHNG57Y |
| PartyIDSource        | D          |
| PartyRole            | 24         |

# Party Sub IDs

| NoPartySubIDs  | 2                   |
| -------------- | ------------------- |
| PartySubID     | PAULSON INVESTMENTS |
| PartySubIDType | 5                   |
| PartySubID     | 1                   |
| PartySubIDType | 26                  |

# Party Alternate IDs

| NoPartyAltIDs | 1            |
| ------------- | ------------ |
| PartyAltID    | 987ABC654XYZ |

# Risk Limits

| NoRiskLimits           | 3       |
| ---------------------- | ------- |
| RiskLimitType          | 3       |
| RiskLimitAmount        | 7500000 |
| RiskLimitCurrency      | USD     |
| RiskLimitPlatform      | DRS     |
| RiskLimitType          | 2       |
| RiskLimitAmount        | 5000000 |
| RiskLimitCurrency      | USD     |
| RiskLimitPlatform      | DRS     |
| NoRiskInstruments      | 1       |
| RiskInstrumentOperator | 1       |
| RiskSecurityGroup      | IG      |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 195 of 200
---
Version 5.0 Service Pack 2 - Errata VOLUME 3 August 18, 2011

# 1. Risk Information

RiskSecurityType = CDS

RiskLimitType = 1 // Gross Limit (for CDS-IG)

RiskLimitAmount = 7000000

RiskLimitCurrency = USD // Currency – USD

RiskLimitPlatform = DRS // Limit applies to platform DRS only

NoRiskInstruments = 1

RiskInstrumentOperator = 1 // Include

RiskSecurityGroup = IG

RiskSecurityType = CDS

NoRelatedPartyIDs = 3

RelatedPartyID = 2345678901 // Investor ID

RelatedPartyIDSource = D // Proprietary / Custom code

RelatedPartyRole = 5 // Investor ID

NoPartyRelationships = 1

PartyRelationship = 20 // Invests through

RelatedPartyID = 313 // Trading Firm

RelatedPartyIDSource = D // Proprietary / Custom code

RelatedPartyRole = 1 // Executing Firm

NoRelatedContextParties = 1

RelatedContextPartyID = CME // Trading Firm Exchange

RelatedContextPartyIDSource = D // Proprietary / Custom code

RelatedContextPartyRole = 22 // Exchange

NoPartyRelationships = 1

PartyRelationship = 4 // Trades through

RelatedPartyID = 312 // Clearing Firm

RelatedPartyIDSource = D // Proprietary / Custom code

RelatedPartyRole = 4 // Clearing Firm

NoRelatedContextParties = 1

RelatedContextPartyID = CME // Clearing Firm Organization

RelatedContextPartyIDSource = D // Proprietary / Custom code

RelatedContextPartyRole = 21 // Clearing Organization

NoPartyRelationships = 1

PartyRelationship = 2 // Clears through

# 2. Trading Firm Party List

Valid Trading Firms have been defined in this list. Each Trading Firm ID has an Exchange ID specified to provide context. The Party is the Trading Firm, and the Related Party is the Clearing Firm, which consists of a Clearing Firm ID and a Clearing Organization to provide context. In this example, a Trading Firm, within the context of a given Exchange, may have only one Clearing Firm.

# 2.1 Party Details

PartyDetailsListReportID = 2

NoPartyList = 4

PartyID = 313 // Trading Firm ID

PartyIDSource = D // Proprietary / Custom code

PartyRole = 1 // Executing Firm

NoPartySubIDs = 1

PartySubID = BANC OF AMERICA // Trading Firm Name

PartySubIDType = 5 // Full legal name of firm

NoContextParties = 1

ContextPartyID = CME // Trading Firm Exchange

ContextPartyIDSource = D // Proprietary / Custom code

ContextPartyRole = 22 // Exchange

NoRelatedPartyIDs = 1

RelatedPartyID = 312 // Clearing Firm

RelatedPartyIDSource = D // Proprietary / Custom code

RelatedPartyRole = 4 // Clearing Firm

NoRelatedContextParties = 1

RelatedContextPartyID = CME // Clearing Firm Organization

RelatedContextPartyIDSource = D // Proprietary / Custom code

RelatedContextPartyRole = 21 // Clearing Organization

NoPartyRelationships = 1

PartyRelationship = 2 // Clears through

PartyID = 112 // Trading Firm ID

PartyIDSource = D // Proprietary / Custom code

PartyRole = 1 // Executing Firm

NoPartySubIDs = 1

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 196 of 200
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011



PartySubID = PRUDENTIAL SECURITES // Trading Firm Name

PartySubIDType = 5 // Full legal name of firm

NoContextParties = 1

ContextPartyID = CME // Trading Firm Exchange

ContextPartyIDSource = D // Proprietary / Custom code

ContextPartyRole = 22 // Exchange

NoRelatedPartyIDs = 1

RelatedPartyID = 112 // Clearing Firm

RelatedPartyIDSource = D // Proprietary / Custom code

RelatedPartyRole = 4 // Clearing Firm

NoRelatedContextParties = 1

RelatedContextPartyID = CME // Clearing Firm Organization

RelatedContextPartyIDSource = D // Proprietary / Custom code

RelatedContextPartyRole = 21 // Clearing Organization

NoPartyRelationships = 1

PartyRelationship = 2 // Clears through

PartyID = 710 // Trading Firm ID

PartyIDSource = D // Proprietary / Custom code

PartyRole = 1 // Executing Firm

NoPartySubIDs = 1

PartySubID = DEUTSCHE BANK // Trading Firm Name

PartySubIDType = 5 // Full legal name of firm

NoContextParties = 1

ContextPartyID = CME // Trading Firm Exchange

ContextPartyIDSource = D // Proprietary / Custom code

ContextPartyRole = 22 // Exchange

NoRelatedPartyIDs = 1

RelatedPartyID = 709 // Clearing Firm

RelatedPartyIDSource = D // Proprietary / Custom code

RelatedPartyRole = 4 // Clearing Firm

NoRelatedContextParties = 1

RelatedContextPartyID = CME // Clearing Firm Organization

RelatedContextPartyIDSource = D // Proprietary / Custom code

RelatedContextPartyRole = 21 // Clearing Organization

NoPartyRelationships = 1

PartyRelationship = 2 // Clears through

PartyID = 709 // Trading Firm ID

PartyIDSource = D // Proprietary / Custom code

PartyRole = 1 // Executing Firm

NoPartySubIDs = 1

PartySubID = DEUTSCHE BANK CLEARING // Trading Firm Name

PartySubIDType = 5 // Full legal name of firm

NoContextParties = 1

ContextPartyID = CME // Trading Firm Exchange

ContextPartyIDSource = D // Proprietary / Custom code

ContextPartyRole = 22 // Exchange

NoRelatedPartyIDs = 1

RelatedPartyID = 709 // Clearing Firm

RelatedPartyIDSource = D // Proprietary / Custom code

RelatedPartyRole = 4 // Clearing Firm

NoRelatedContextParties = 1

RelatedContextPartyID = CME // Clearing Firm Organization

RelatedContextPartyIDSource = D // Proprietary / Custom code

RelatedContextPartyRole = 21 // Clearing Organization

NoPartyRelationships = 1

PartyRelationship = 2 // Clears through


© Copyright, 2008-2009, 2011, FIX Protocol, Limited

Page 197 of 200


---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                      August 18, 2011

# Appendix 3-A: Pre-Trade Message Targeting/Routing

Three fields, NoRoutingID, RoutingType, and RoutingID have been added to support list processing on third party networks. Vendor "indication of interest" systems generally have list management capabilities. These capabilities include blocking and targeting. To mirror the functionality of the vendor indication systems both blocking and targeting were supported.

# Targeting

Targeting relates to the message that contains a list of targeted firms or targeted vendor maintained list identifiers to receive the indication. Generally, most vendor "indication of interest" systems maintain list identifiers that contain firm identifiers for their broker connections. For example, a broker has a list called "JapanList" that contains three institutions JapaneseFirm1, JapaneseFirm2, and JapaneseFirm3. The three firm identifiers are created by the vendor.

Targeting allows for the definition of the universe of firms to receive the indication of interest. A indication of interest message without the targeting identifiers (either firm or list) is assumed to be sent to the whole list of indication receiving firms managed by the vendor (i.e. every institution connected to the broker).

Specific targeting can be accomplished through the combination of firm identifiers and list identifiers. For example, a broker needs to send an indication of interest to a vendor maintained list of U.K. based clients called "UKList" and two U.S. based firms. The targeting section of the indication of interest would look as follows:

215=3^216=1^217=USFirm1^216=1^217=USFirm2^216=2^217=UKList^
Note: The ^ character represents the SOH delimiter.

# Tag Explanation

| 215=3       | Three pairs of routing types and IDs to be processed |
| ----------- | ---------------------------------------------------- |
| 216=1       | Target ID to follow                                  |
| 217=USFirm1 | Target ID named USFirm1                              |
| 216=1       | Target ID to follow                                  |
| 217=USFirm2 | Target ID named USFirm2                              |
| 216=2       | Target list to follow                                |
| 217=UKList  | Target list named UKList                             |

The vendor would assemble the destination list based on the two firm identifiers and the one list identifier.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                            Page 198 of 200
---
Version 5.0 Service Pack 2 - Errata   VOLUME 3                                                   August 18, 2011

# Blocking

An indication with blocking contains a list of firm identifiers or vendor maintained list identifiers that will be excluded from the targeted list of indication receiving firms managed by the vendor. Using the blocking fields without targeting fields implies that indication of interest is being blocked from the whole universe of institutions available to the broker (i.e. everyone on the vendor's system but these firms).

Many "indication of interest" systems have sophisticated list handling mechanisms that need to be replicated. Blocking is not always performed from the whole universe of firms on the system (i.e. ALL).

Using a combination of targeting and blocking fields can allow for sophisticated list management capabilities. For example, let's assume that the broker intends to send an indication of interest to the universe defined by the broker's UKList and two U.S. based firms. However, the broker needs to exclude one UK based firm from the UKList. The targeting and blocking section would appear as follows:

215=4^216=2^217=UKList^216=1^217=USFirm1^216=1^217=USFirm2^216=3^217=UKFirm1^
Note: The ^ character represents the SOH delimiter.

# Tag Explanation

| 215=4       | Four pairs of routing types and IDs to be processed |
| ----------- | --------------------------------------------------- |
| 216=2       | Target list to follow                               |
| 217=UKList  | Target list named UKList                            |
| 216=1       | Target firm to follow                               |
| 217=USFirm1 | Target firm named USFirm1                           |
| 216=1       | Target firm to follow                               |
| 217=USFirm2 | Target firm named USFirm2                           |
| 216=3       | Blocked firm to follow                              |
| 217=UKFirm1 | UKFirm1 is blocked from receiving IOI               |

The vendor would assemble the targets based on the supplied UKList and two firm identifiers (USFirm1 and USFirm2) and then remove UKFirm1 from the combined list.

# Other Issues

It is expected that every indication of interest message will have a unique IOIid for the FIX session for the trading day.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                         Page 199 of 200
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 3

# August 18, 2011


For canceling and replacing, the vendor system would cancel or replace every destination that has been identified on the previous indication of interest by the IOIid. Blocking and targeting information would not be required on the canceled or replaced indication of interest.

The use of vendor based firm identifiers requires periodic updates to the brokers to ensure proper blocking and targeting. It is expected that vendors will provide file base transfers of firm identifiers and company names until a more automated solution becomes available.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited

Page 200 of 200