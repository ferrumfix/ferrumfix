
FINANCIAL INFORMATION
EXCHANGE PROTOCOL
(FIX)
Version 4.4 with Errata 20030618


# VOLUME 1 – INTRODUCTION TO THE FIX PROTOCOL

Includes Errata adjustments as of June 18, 2003

# Errata Purpose:

This document includes a list of minor adjustments to the FIX 4.4 Specification document due to typographical errors or ambiguities. The nature and scope of Errata adjustments do not introduce new functionality, additional fields, new values for existing fields, or new messages. Regretably some functionality was introduced in FIX 4.4 which contained errors that required a new value or field on a specific message in order to make the intended functionality implementable. Any such exceptions to the “do not introduce”, “additional fields”, or “new messages” Errata rules were kept to a minimum using the “required to make the intended functionality implementable” rationale. The list of items has been reviewed and approved by the FIX Technical Committee and Steering Committees. Implementers of FIX version 4.4 should refer to this document to ensure the most consistent implementation and clearest understanding of the FIX protocol.

The specific adjustments made to the original FIX version 4.4 specification as a result of the Errata can be seen and printed via Microsoft Word’s revision feature of this document. A separate document with an itemized list of changes is available via the FIX website.

~~April 30, 2003~~ June 18, 2003

~~April 30, 2003~~ June 18, 2003

i FIX 4.4 with Errata 20030618 - Volume 1

Copyright 2003 FIX Protocol Limited
---

DISCLAIMER


THE INFORMATION CONTAINED HEREIN AND THE FINANCIAL INFORMATION EXCHANGE PROTOCOL (COLLECTIVELY, THE "FIX PROTOCOL") ARE PROVIDED "AS IS" AND NO PERSON OR ENTITY ASSOCIATED WITH THE FIX PROTOCOL MAKES ANY REPRESENTATION OR WARRANTY, EXPRESS OR IMPLIED, AS TO THE FIX PROTOCOL (OR THE RESULTS TO BE OBTAINED BY THE USE THEREOF) OR ANY OTHER MATTER AND EACH SUCH PERSON AND ENTITY SPECIFICALLY DISCLAIMS ANY WARRANTY OF ORIGINALITY, ACCURACY, COMPLETENESS, MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. SUCH PERSONS AND ENTITIES DO NOT WARRANT THAT THE FIX PROTOCOL WILL CONFORM TO ANY DESCRIPTION THEREOF OR BE FREE OF ERRORS. THE ENTIRE RISK OF ANY USE OF THE FIX PROTOCOL IS ASSUMED BY THE USER.

NO PERSON OR ENTITY ASSOCIATED WITH THE FIX PROTOCOL SHALL HAVE ANY LIABILITY FOR DAMAGES OF ANY KIND ARISING IN ANY MANNER OUT OF OR IN CONNECTION WITH ANY USER'S USE OF (OR ANY INABILITY TO USE) THE FIX PROTOCOL, WHETHER DIRECT, INDIRECT, INCIDENTAL, SPECIAL OR CONSEQUENTIAL (INCLUDING, WITHOUT LIMITATION, LOSS OF DATA, LOSS OF USE, CLAIMS OF THIRD PARTIES OR LOST PROFITS OR REVENUES OR OTHER ECONOMIC LOSS), WHETHER IN TORT (INCLUDING NEGLIGENCE AND STRICT LIABILITY), CONTRACT OR OTHERWISE, WHETHER OR NOT ANY SUCH PERSON OR ENTITY HAS BEEN ADVISED OF, OR OTHERWISE MIGHT HAVE ANTICIPATED THE POSSIBILITY OF, SUCH DAMAGES.

No proprietary or ownership interest of any kind is granted with respect to the FIX Protocol (or any rights therein).

Copyright 2003 FIX Protocol Limited, all rights reserved

~~April 30, 2003~~ June 18, 2003 ii FIX 4.4 with Errata 20030618- Volume 1 Copyright 2003 FIX Protocol Limited



---

# PREFACE


The Financial Information eXchange (FIX) effort was initiated in 1992 by a group of institutions and brokers interested in streamlining their trading processes. These firms felt that they, and the industry as a whole, could benefit from efficiencies derived through the electronic communication of indications, orders and executions. The result is FIX, an open message standard controlled by no single entity, that can be structured to match the business requirements of each firm. The benefits are:

- From the business flow perspective, FIX provides institutions, brokers, and other market participants a means of reducing the clutter of unnecessary telephone calls and scraps of paper, and facilitates targeting high quality information to specific individuals.
- For technologists, FIX provides an open standard that leverages the development effort so that they can efficiently create links with a wide range of counter-parties.
- For vendors, FIX provides ready access to the industry, with the incumbent reduction in marketing effort and increase in potential client base.

Openness has been the key to FIX's success. For that reason, while encouraging vendors to participate with the standard, FIX has remained vendor neutral. Similarly, FIX avoids over-standardization. It does not demand a single type of carrier (e.g., it will work with leased lines, frame relay, Internet, etc.), nor a single security protocol. It leaves many of these decisions to the individual firms that are using it. We do expect that, over time, the rules of engagement in these non-standardized areas will converge as technologies mature.

FIX is now used by a variety of firms and vendors. It has clearly emerged as the inter-firm messaging protocol of choice. FIX has grown from its original buyside-to-sellside equity trading roots. It is now used by markets (exchanges, “ECNs”, etc) and other market participants. In addition to equities, FIX currently supports four other products: Collective Investment Vehicles (CIVs), Derivatives, Fixed Income, and Foreign Exchange. The process for modifications to the specification is very open with input and feedback encouraged from the community. Those interested in providing input to the protocol are encouraged to use the FIX website Discussion section or contact the FIX Global Technical Committee Chairpersons, Scottt Atwell, American Century Investments, (US) 816-340-7053 (scott_atwell@americancentury.com) or Dean Kauffman, TradeWeb LLC, (US) 201-536-5827 (dean.kauffman@tradeweb.com).

The FIX website at http://www.fixprotocol.org is the main source of information, discussion, and notification of FIX-related events.

We look forward to your participation.

FIX Protocol Ltd ~~April 30, 2003~~June 16, 2003

~~April 30, 2003~~June 18, 2003 iii FIX 4.4 with Errata 20030618- Volume 1

Copyright 2003 FIX Protocol Limited


---

About FIX Protocol Limited

FIX Protocol Limited (FPL) oversees and manages the development of the FIX Protocol specification and encourages its use throughout the industry. FPL is open to due paying members representing business and technology professionals interested in guiding the growth and adoption of the FIX Protocol that work for: Buy-side Firms, Sell-side Firms, Exchanges, ECNs/ATSs, Utilities, Vendors, and Other Associations. See the FIX website for more information about membership.

# Global Steering Committee:

| Americas Region | Global Derivatives           |
| --------------- | ---------------------------- |
| AsialPac Region | Global Education & Marketing |
| Europe Region   | Global Fixed Income          |
| Japan Region    | Global Technical             |

Position on committee through appointment election consists of members of each Region/Product Education Marketing Subcommittee. Consists of members of each Region/Product Technical Subcommittee. Note: Position on committee governance boards also by appointment/election.

# FIX Protocol Limited includes the following firms (as of the time of this writing):

| ABN Amro                              | Capital Group                 | Dresdner Kleinwort Wasserstein |
| ------------------------------------- | ----------------------------- | ------------------------------ |
| Alliance Capital                      | Cazenove                      | E\*TRADE Group                 |
| Allianz Dresdner Asset Management     | CBOE                          | Euronext·Liffe                 |
| American Century                      | CBOT                          | F\&C Management Ltd            |
| AXA Rosenburg                         | Charles River Development     | Fidelity Capital Markets       |
| Baillie Gifford                       | Citigroup Global Markets Inc. | Fidelity Investments           |
| Bank of America Securities            | CityIQ                        | Fimat                          |
| Barclays Global Investors             | CLSA Limited                  | Financial Fusion               |
| Barclays Stockbrokers                 | CME                           | Franklin Templeton Investments |
| Baring Asset Management               | ~~Commerzbank Securities~~    | Gartmore Investment Management |
| Bear Stearns                          | CSFB                          | GL Trade                       |
| BFT- Beauchchamp Financial Technology | Daiwa Securities Group Inc.   | Goldman Sachs                  |
| Bloomberg                             | DCE Consultants Ltd.          | HSBC                           |
| Boston Stock Exchange                 | Deutsche Asset Management     | ING                            |
| Cameron Systems                       | Deutsche Securities           | Insight Investment             |

~~April 30, 2003~~ June 18, 2003

iv FIX 4.4 with Errata 20030618- Volume 1

Copyright 2003 FIX Protocol Limited


---

Instinet                             New York Stock Exchange
ITG                                  Newton Investment Management Ltd.
JF Asset Management                  Nikko Asset Management
JP Morgan Securities                 Nomura Asset Management
Knight Securities                    Nomura Securities
Lehman Brothers                      Northern Trust
London Stock Exchange                NYFIX/Javelin Technologies
MacGregor                            OM
Market Axess                         Omgeo
Massachusetts Financial              Options Clearing Corporation (OCC)
Merrill Lynch                        Putnam Investments
Mitsui Asset Trust and Banking Co., Ltd. Radianz
Morgan Stanley                       Reuters
National Futures Association         Royal Blue
SimCorp
SolutionForge Ltd.
SunGard
Syntegra
SWIFT
The Sumitomo Trust &#x26; Banking Co., Ltd
Thomson Financial
TNS
Townsend Analytics
TradeWeb
UBS Warburg
Virt-x
Worldwide Business Research


~~April 30, 2003~~June 18, 2003

v FIX 4.4 with Errata 20030618- Volume 1

Copyright 2003 FIX Protocol Limited
---
NO_CONTENT_HERE
---

VOLUME INDEX


# VOLUME 1 - INTRODUCTION

- INTRODUCTION
- DOCUMENT NAVIGATION
- FIX PROTOCOL SYNTAX
- COMMON COMPONENTS OF APPLICATION MESSAGES
- COMMON APPLICATION MESSAGES
- GLOSSARY

# VOLUME 2 - FIX SESSION PROTOCOL

- TRANSMITTING FIXML OR OTHER XML-BASED CONTENT
- FIX MESSAGE DELIVERY
- SESSION PROTOCOL
- ADMINISTRATIVE MESSAGES
- CHECKSUM CALCULATION
- FIX SESSION USING A MULTICAST TRANSPORT
- FIX SESSION-LEVEL TEST CASES AND EXPECTED BEHAVIORS

# VOLUME 3 - FIX APPLICATION MESSAGES: PRE-TRADE

- CATEGORY: INDICATION
- CATEGORY: EVENT COMMUNICATION
- CATEGORY: QUOTATION
- CATEGORY: MARKET DATA
- CATEGORY: SECURITY AND TRADING SESSION DEFINITION/STATUS

# VOLUME 4 - FIX APPLICATION MESSAGES: ORDERS AND EXECUTIONS (TRADE)

- CATEGORY: SINGLE/GENERAL ORDER HANDLING
- CATEGORY: CROSS ORDERS
- CATEGORY: MULTILEG ORDERS (SWAPS, OPTION STRATEGIES, ETC)
- CATEGORY: LIST/PROGRAM/BASKET TRADING

# VOLUME 5 - FIX APPLICATION MESSAGES: POST-TRADE
---


# CATEGORY: ALLOCATION AND READY-TO-BOOK

# CATEGORY: SETTLEMENT INSTRUCTIONS

# CATEGORY: TRADE CAPTURE ("STREETSIDE") REPORTING

# CATEGORY: REGISTRATION INSTRUCTIONS

# CATEGORY: POSITIONS MAINTENANCE

# CATEGORY: COLLATERAL MANAGEMENT

# VOLUME 6 - FIX DATA DICTIONARY

# FIELD DEFINITIONS

# APPENDIX 6-A - VALID CURRENCY CODES

# APPENDIX 6-B - FIX FIELDS BASED UPON OTHER STANDARDS

# APPENDIX 6-C - EXCHANGE CODES - ISO 10383 MARKET IDENTIFIER CODE (MIC)

# APPENDIX 6-D - CFICODE USAGE - ISO 10962 CLASSIFICATION OF FINANCIAL INSTRUMENTS (CFI CODE)

# APPENDIX 6-E - DEPRECATED (PHASED-OUT) FEATURES AND SUPPORTED APPROACH

# APPENDIX 6-F - REPLACED FEATURES AND SUPPORTED APPROACH

# APPENDIX 6-G - USE OF &#x3C;PARTIES> COMPONENT BLOCK

# APPENDIX 6-H - USE OF &#x3C;SETTLINSTRUCTIONS> COMPONENT BLOCK

# VOLUME 7 - FIX USAGE BY PRODUCT

# PRODUCT: COLLECTIVE INVESTMENT VEHICLES (CIV)

# PRODUCT: DERIVATIVES (FUTURES &#x26; OPTIONS)

# PRODUCT: EQUITIES

# PRODUCT: FIXED INCOME

# PRODUCT: FOREIGN EXCHANGE


---

# Contents – Volume 1



# PREFACE

3

# About FIX Protocol Limited

4

# VOLUME 1 - INTRODUCTION

8

# VOLUME INDEX

8

# INTRODUCTION

8

# DOCUMENT NAVIGATION

8

# FIX PROTOCOL SYNTAX

8

# COMMON COMPONENTS OF APPLICATION MESSAGES

8

# COMMON APPLICATION MESSAGES

8

# GLOSSARY

8

# VOLUME 2 - FIX SESSION PROTOCOL

8

# TRANSMITTING FIXML OR OTHER XML-BASED CONTENT

8

# FIX MESSAGE DELIVERY

8

# SESSION PROTOCOL

8

# ADMINISTRATIVE MESSAGES

8

# CHECKSUM CALCULATION

8

# FIX SESSION USING A MULTICAST TRANSPORT

8

# FIX SESSION-LEVEL TEST CASES AND EXPECTED BEHAVIORS

8

# VOLUME 3 - FIX APPLICATION MESSAGES: PRE-TRADE

8

# CATEGORY: INDICATION

8

# CATEGORY: EVENT COMMUNICATION

8

# CATEGORY: QUOTATION

8

# CATEGORY: MARKET DATA

8

# CATEGORY: SECURITY AND TRADING SESSION DEFINITION/STATUS

8

# VOLUME 4 - FIX APPLICATION MESSAGES: ORDERS AND EXECUTIONS (TRADE)

8

# CATEGORY: SINGLE/GENERAL ORDER HANDLING

8

# CATEGORY: CROSS ORDERS

8

# CATEGORY: MULTILEG ORDERS (SWAPS, OPTION STRATEGIES, ETC)

8

# CATEGORY: LIST/PROGRAM/BASKET TRADING

8

# VOLUME 5 - FIX APPLICATION MESSAGES: POST-TRADE

9

# CATEGORY: ALLOCATION AND READY-TO-BOOK

9

# CATEGORY: SETTLEMENT INSTRUCTIONS

9

# CATEGORY: TRADE CAPTURE ("STREETSIDE") REPORTING

9

# CATEGORY: REGISTRATION INSTRUCTIONS

9

# CATEGORY: POSITIONS MAINTENANCE

9

# CATEGORY: COLLATERAL MANAGEMENT

9

# VOLUME 6 - FIX DATA DICTIONARY

9

# FIELD DEFINITIONS

9

# APPENDIX 6-A - VALID CURRENCY CODES

9

# APPENDIX 6-B - FIX FIELDS BASED UPON OTHER STANDARDS

9

# APPENDIX 6-C - EXCHANGE CODES - ISO 10383 MARKET IDENTIFIER CODE (MIC)

9

# APPENDIX 6-D - CFICODE USAGE - ISO 10962 CLASSIFICATION OF FINANCIAL INSTRUMENTS (CFI CODE)

9

# APPENDIX 6-E - DEPRECATED (PHASED-OUT) FEATURES AND SUPPORTED APPROACH

9

# APPENDIX 6-F - REPLACED FEATURES AND SUPPORTED APPROACH

9

# APPENDIX 6-G - USE OF &#x3C;PARTIES> COMPONENT BLOCK

9

# APPENDIX 6-H - USE OF &#x3C;SETTLINSTRUCTIONS> COMPONENT BLOCK

9

# VOLUME 7 - FIX USAGE BY PRODUCT

9

# PRODUCT: COLLECTIVE INVESTMENT VEHICLES (CIV)

9



---
PRODUCT: DERIVATIVES (FUTURES &#x26; OPTIONS) 9
<page_header>PRODUCT: EQUITIES 9</page_header>
<page_header>PRODUCT: FIXED INCOME 9</page_header>
<page_header>PRODUCT: FOREIGN EXCHANGE 9</page_header>

# INTRODUCTION

13~~15~~

# DOCUMENT NAVIGATION

13~~15~~

# FIX PROTOCOL SYNTAX

14~~16~~

# COMMON FIX SYNTAX RULES

14~~16~~

- Data Types: 14~~16~~
- Required Fields: 17~~19~~

# FIX “Tag=Value” SYNTAX

18~~20~~

- Message Format 18~~20~~
- Field Delimiter: 18~~20~~
- Repeating Groups: 18~~20~~
- User Defined Fields: 20~~22~~

# Example Usage of Encoded Fields For Japanese Language Support

22~~24~~

# FIXML SYNTAX

23~~25~~

- Background 23~~25~~
- FIXML Highlights 23~~25~~

# FIXML Design Rules

24~~26~~

- General 24~~26~~
- ComponentType - Field 25~~27~~
- ComponentType - Block 26~~28~~
- ComponentType - RepeatingGroup 27~~29~~
- ComponentType - BlockRepeating 29~~31~~
- ComponentType - Message 30~~32~~

# COMMON COMPONENTS OF APPLICATION MESSAGES - Component Blocks (Included in pre-trade, trade, and post-trade messages)

37~~39~~

# Instrument (symbology) component block -

37~~39~~

- Examples using Alternative Security Ids 41~~43~~
- Specifying an FpML product specification from within the FIX Instrument Block 41~~43~~

# UnderlyingInstrument (underlying instrument) component block -

43~~45~~

# Instrument Leg (symbology) component block -

46~~48~~

# InstrumentExtension component block -

48~~50~~

# OrderQtyData component block -

49~~51~~

# CommissionData component block -

50~~52~~

# Parties component block -

51~~53~~

# NestedParties component block -

52~~54~~

# NestedParties2 (second instance of nesting) component block -

53~~55~~

# NestedParties3 (third instance of nesting) component block -

54~~56~~

# SettlParties (settlement parties) component block -

55~~57~~

# SpreadOrBenchmarkCurveData component block -

55~~57~~

# LegBenchmarkCurveData component block -

57~~59~~

# Stipulations component block -

58~~60~~

# UnderlyingStipulations component block -

59~~61~~

# LegStipulations component block -

60~~62~~

# YieldData component block -

61~~63~~

# PositionQty Component Block

62~~64~~

# PositionAmountData Component Block

63~~65~~

# TrdRegTimestamps component block -

64~~66~~

# SettlInstructionsData component block -

65~~67~~


---

# PegInstructions component block -

66 ~~68~~

# DiscretionInstructions component block -

67 ~~69~~

# FinancingDetails component block -

68 ~~70~~

# COMMON APPLICATION MESSAGES (Apply to pre-trade, trade, and post-trade)

69 ~~71~~

# Business Message Reject -

69 ~~71~~

# Network Status Messages

74 ~~76~~

# Network (Counterparty System) Status Request Message

74 ~~76~~

# Network (Counterparty System) Status Response Message

76 ~~78~~

# User Administration Messages

77 ~~79~~

# User Request Message

77 ~~79~~

# User Response Message

78 ~~80~~

# Glossary

79 ~~81~~

# Business Terms

79 ~~81~~

# Appendix 1-A

91 ~~93~~

# Abbreviations used within FIXML

91 ~~93~~


---

FINANCIAL INFORMATION EXCHANGE PROTOCOL


# INTRODUCTION

The Financial Information Exchange (FIX) Protocol is a message standard developed to facilitate the electronic exchange of information related to securities transactions. It is intended for use between trading partners wishing to automate communications.

The message protocol, as defined, will support a variety of business functions. FIX was originally defined for use in supporting US domestic equity trading with message traffic flowing directly between principals. As the protocol evolved, a number of fields were added to support cross-border trading, derivatives, fixed income, and other products. Similarly, the protocol was expanded to allow third parties to participate in the delivery of messages between trading partners. As subsequent versions of FIX are released, it is expected that functionality will continue to expand.

The protocol is defined at two levels: session and application. The session level is concerned with the delivery of data while the application level defines business related data content. This document is divided into volumes and organized to reflect the distinction.

# DOCUMENT NAVIGATION

One useful tip when navigating within a volume is to take advantage of the fact that each document contains “bookmarks” to its main sections. You can use the word processor’s “Goto” function (e.g. Ctrl-G) to quickly navigate from one key section or appendix to another.

Third parties or volunteers have historically built useful utilities “generated” using the specification document as their basis which provide cross-reference and lookup capabilities. Such free utilities are available via the FIX website.



---
FIX PROTOCOL SYNTAX

The FIX Protocol currently exists in two syntaxes:

1. “Tag=Value” syntax
2. FIXML syntax

The same business message flow applies to either syntax. A specific syntax is simply a slightly different way to represent the same thing in much the same way that “3” and “three” represent the same thing.

# COMMON FIX SYNTAX RULES

The following section summarizes general specifications for constructing FIX messages which are applicable to both “Tag=Value” and FIXML syntaxes.

# Data Types:

Data types (with the exception of those of type "data") are mapped to ASCII strings as follows:

- int: Sequence of digits without commas or decimals and optional sign character (ASCII characters "-" and "0" - "9"). The sign character utilizes one byte (i.e. positive int is "99999" while negative int is "-99999"). Note that int values may contain leading zeros (e.g. “00023” = “23”).
- - Examples: 723 in field 21 would be mapped int as |21=723|.
- -723 in field 12 would be mapped int as |12=-723|.

Length: int field (see definition of “int” above) representing the length in bytes. Value must be positive.
- NumInGroup: int field (see definition of “int” above) representing the number of entries in a repeating group. Value must be positive.
- SeqNum: int field (see definition of “int” above) representing a message sequence number. Value must be positive.
- TagNum: int field (see definition of “int” above) representing a field's tag number when using FIX "Tag=Value" syntax. Value must be positive and may not contain leading zeros.
- DayOfMonth: int field (see definition of “int” above) representing a day during a particular month (values 1-31).
- float: Sequence of digits with optional decimal point and sign character (ASCII characters "-", "0" - "9" and "."); the absence of the decimal point within the string will be interpreted as the float representation of an integer value. All float fields must accommodate up to fifteen significant digits. The number of decimal places used should be a factor of business/market needs and mutual agreement between counterparties. Note that float values may contain leading zeros (e.g. “00023.23” = “23.23”) and may contain or omit trailing zeros after the decimal point (e.g. “23.0” = “23.0000” = “23” = "23."). Note that fields which are derived from float may contain negative values unless explicitly specified otherwise.
- Qty: float field (see definition of “float” above) capable of storing either a whole number (no decimal places) of “shares” (securities denominated in whole units) or a
---
decimal value containing decimal places for non-share quantity asset classes (securities denominated in fractional units).

- Price: float field (see definition of “float” above) representing a price. Note the number of decimal places may vary. For certain asset classes prices may be negative values. For example, prices for options strategies can be negative under certain market conditions. Refer to Volume 7: FIX Usage by Product for asset classes that support negative price values.
- PriceOffset: float field (see definition of “float” above) representing a price offset, which can be mathematically added to a "Price". Note the number of decimal places may vary and some fields such as LastForwardPoints may be negative.
- Amt: float field (see definition of “float” above) typically representing a Price times a Qty.
- Percentage: float field (see definition of “float” above) representing a percentage (e.g. .05 represents 5% and .9525 represents 95.25%). Note the number of decimal places may vary.

char: Single character value, can include any alphanumeric character or punctuation except the delimiter. All char fields are case sensitive (i.e. m ≠ M).

- Boolean: a char field (see definition of “char” above) containing one of two values:
- 'Y' = True/Yes
- 'N' = False/No
- String: Alpha-numeric free format strings, can include any character or punctuation except the delimiter. All char fields are case sensitive (i.e. morstatt ≠ Morstatt).
- MultipleValueString: String field (see definition of “String” above) containing one or more space delimited values.
- Country: String field (see definition of “String” above) representing a country using ISO 3166 Country code (2 character) values. Valid values:
- See "Appendix 6-B - FIX Fields Based Upon Other Standards"
- Currency: String field (see definition of “String” above) representing a currency type using ISO 4217 Currency code (3 character) values. Valid values:
- See "Appendix 6-A - Currency Codes - ISO 4217 Currency codes"
- Exchange: String field (see definition of “String” above) representing a market or exchange. Valid values:
- See "Appendix 6-C - Exchange Codes - ISO 10383 Market Identifier Code (MIC)"
- month-year: String field representing month of a year. An optional day of the month can be appended or an optional week code.
---
# Valid formats:

- YYYYMM
- YYYYMMDD
- YYYYMMWW

# Valid values:

- YYYY = 0000-9999, MM = 01-12, DD = 01-31, WW = w1, w2, w3, w4, w5.

# UTCTimestamp:

Time/date combination represented in UTC (Universal Time Coordinated, also known as “GMT”) in either YYYYMMDD-HH:MM:SS (whole seconds) or YYYYMMDD-HH:MM:SS.sss (milliseconds) format, colons, dash, and period required.

# Valid values:

- YYYY = 0000-9999, MM = 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC leap second) (without milliseconds).
- YYYY = 0000-9999, MM = 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC leap second), sss=000-999 (indicating milliseconds).

# Leap Seconds:

Note that UTC includes corrections for leap seconds, which are inserted to account for slowing of the rotation of the earth. Leap second insertion is declared by the International Earth Rotation Service (IERS) and has, since 1972, only occurred on the night of Dec. 31 or Jun 30. The IERS considers March 31 and September 30 as secondary dates for leap second insertion, but has never utilized these dates. During a leap second insertion, a UTCTimestamp field may read "19981231-23:59:59", "19981231-23:59:60", "19990101-00:00:00". (see http://tycho.usno.navy.mil/leapsec.html)

# UTCTimeOnly:

Time-only represented in UTC (Universal Time Coordinated, also known as “GMT”) in either HH:MM:SS (whole seconds) or HH:MM:SS.sss (milliseconds) format, colons, and period required. This special-purpose field is paired with UTCDateOnly to form a proper UTCTimestamp for bandwidth-sensitive messages.

# Valid values:

- HH = 00-23, MM = 00-60 (60 only if UTC leap second), SS = 00-59. (without milliseconds)
- HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC leap second), sss=000-999 (indicating milliseconds).

# UTCDateOnly:

Date represented in UTC (Universal Time Coordinated, also known as “GMT”) in YYYYMMDD format. This special-purpose field is paired with UTCTimeOnly to form a proper UTCTimestamp for bandwidth-sensitive messages.

# Valid values:

- YYYY = 0000-9999, MM = 01-12, DD = 01-31.

# LocalMktDate:

Date of Local Market (vs. UTC) in YYYYMMDD format. This is the “normal” date field used by the FIX protocol.

# Valid values:

- YYYY = 0000-9999, MM = 01-12, DD = 01-31.

# data:

Raw data with no format or content restrictions. Data fields are always immediately preceded by a length field. The length field should specify the number of bytes of the value of the data field (up to but not including the terminating SOH). Caution: the value of one of these fields may contain the delimiter (SOH) character. Note that the value specified for this
---
field should be followed by the delimiter (SOH) character as all fields are terminated with an “SOH”.

# Required Fields:

Each message within the protocol is comprised of required, optional and conditionally required (fields which are required based on the presence or value of other fields) fields. Systems should be designed to operate when only the required and conditionally required fields are present.
---

# FIX “Tag=Value” SYNTAX

The following section summarizes general specifications for constructing FIX messages in “Tag=Value” syntax.

# Message Format

The general format of a FIX message is a standard header followed by the message body fields and terminated with a standard trailer.

Each message is constructed of a stream of <tag>=<value> fields with a field delimiter between fields in the stream. Tags are of data type TagNum. All tags must have a value specified. Optional fields without values should simply not be specified in the FIX message. A Reject message is the appropriate response to a tag with no value.</value></tag>

Except where noted, fields within a message can be defined in any sequence (Relative position of a field within a message is inconsequential.) The exceptions to this rule are:

1. General message format is composed of the standard header followed by the body followed by the standard trailer.
2. The first three fields in the standard header are BeginString (tag #8) followed by BodyLength (tag #9) followed by MsgType (tag #35).
3. The last field in the standard trailer is the CheckSum (tag #10).
4. Fields within repeating data groups must be specified in the order that the fields are specified in the message definition within the FIX specification document. The NoXXX field where XXX is the field being counted specifies the number of repeating group instances that must immediately precede the repeating group contents.
5. A tag number (field) should only appear in a message once. If it appears more than once in the message it should be considered an error with the specification document. The error should be pointed out to the FIX Global Technical Committee.

In addition, certain fields of the data type MultipleValueString can contain multiple individual values separated by a space within the "value" portion of that field followed by a single "SOH" character (e.g. "18=2 9 C<soh>" represents 3 individual values: '2', '9', and 'C').</soh>

It is also possible for a field to be contained in both the clear text portion and the encrypted data sections of the same message. This is normally used for validation and verification. For example, sending the SenderCompID in the encrypted data section can be used as a rudimentary validation technique. In the cases where the clear text data differs from the encrypted data, the encrypted data should be considered more reliable. (A security warning should be generated).

# Field Delimiter:

All fields (including those of data type data i.e. SecureData, RawData, SignatureData, XmlData, etc.) in a FIX message are terminated by a delimiter character. The non-printing, ASCII "SOH" (#001, hex: 0x01, referred to in this document as <soh>), is used for field termination. Messages are delimited by the “SOH” character following the CheckSum field. All messages begin with the “8=FIX.x.y<soh>” string and terminate with “10=nnn<soh>“.</soh></soh></soh>

There shall be no embedded delimiter characters within fields except for data type data.

# Repeating Groups:

It is permissible for fields to be repeated within a repeating group (e.g. "384=2<soh>372=6<soh>385=R<soh>372=7<soh>385=R<soh>" represents a</soh></soh></soh></soh></soh>


---

# Repeating Groups

Repeating group with two repeating instances “delimited” by tag 372 (first field in the repeating group).

- If the repeating group is used, the first field of the repeating group is required. This allows implementations of the protocol to use the first field as a "delimiter" indicating a new repeating group entry. The first field listed after the NoXXX, then becomes conditionally required if the NoXXX field is greater than zero.
- The NoXXX field (for example: NoTradingSessions, NoAllocs) which specifies the number of repeating group instances occurs once for a repeating group and must immediately precede the repeating group contents.
- The NoXXX field is required if one of the fields in the repeating group is required. If all members of a repeating group are optional, then the NoXXX field should also be optional.
- If a repeating group field is listed as required, then it must appear in every repeated instance of that repeating group.
- Repeating groups are designated within the message definition via indentation and the → symbol.

Some repeating groups are nested within another repeating group (potentially more than one level of nesting).

- Nested repeating groups are designated within the message definition via indentation and the → symbol followed by another → symbol.
- If a nested repeating group is used, then the outer repeating group must be specified.

# Example of a Repeating Group:

Part of message
215    NoRoutingIDs                       N      Required if any RoutingType and RoutingIDs are
specified.  Indicates the number within repeating
group.
→  216     RoutingType                N      Indicates   type  of  RoutingID.          Required   if
NoRoutingIDs is > 0.
→  217     RoutingID                  N      Identifies  routing    destination.       Required   if
NoRoutingIDs is > 0.
Rest of the message not shown


---

# Example of nested repeating group

Portion of New Order - List message showing a nested repeating group for allocations for each order. Note the NoAllocs repeating group is nested within the NoOrders repeating group and as such each instance of the orders repeating group may contain a repeating group of allocations.

| 73 | NoOrders        |                      | Y                 | Number of orders in this message (number of repeating groups to follow)                                              |
| -- | --------------- | -------------------- | ----------------- | -------------------------------------------------------------------------------------------------------------------- |
| →  | 11              | ClOrdID              | Y                 | Must be the first field in the repeating group.                                                                      |
| →  | 526             | SecondaryClOrdID     | N                 |                                                                                                                      |
| →  | 67              | ListSeqNo            | Y                 | Order number within the list                                                                                         |
| →  | 583             | ClOrdLinkID          | N                 |                                                                                                                      |
| →  | 160             | SettlInstMode        | N                 |                                                                                                                      |
| →  | component block | \<Parties>           | N                 | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |
| →  | 229             | TradeOriginationDate | N                 |                                                                                                                      |
| →  | 1               | Account              | N                 |                                                                                                                      |
| →  | 581             | AccountType          | N                 |                                                                                                                      |
| →  | 589             | DayBookingInst       | N                 |                                                                                                                      |
| →  | 590             | BookingUnit          | N                 |                                                                                                                      |
| →  | 591             | PreallocMethod       | N                 |                                                                                                                      |
| →  | 78              | NoAllocs             | N                 | Indicates number of pre-trade allocation accounts to follow                                                          |
| →  | →               | 79                   | AllocAccount      | N                                                                                                                    |
| →  | →               | 467                  | IndividualAllocID | N                                                                                                                    |
| →  | →               | component block      | \<NestedParties>  | N                                                                                                                    |
| →  | →               | 80                   | AllocQty          | N                                                                                                                    |
| →  | 63              | SettlmntTyp          | N                 |                                                                                                                      |
| →  | 64              | FutSettDate          | N                 | Takes precedence over SettlmntTyp value and conditionally required/omitted for specific SettlmntTyp values.          |

# User Defined Fields:

In order to provide maximum flexibility for its users, the FIX protocol accommodates User Defined Fields. These fields are intended to be implemented between consenting trading partners and should be used with caution to avoid conflicts, which will arise as multiple parties begin implementation of.


---
The protocol. It is suggested that if trading partners find that particular User Defined Fields add value, they should be recommended to the FIX Global Technical Committee for inclusion in a future FIX version.

The tag numbers 5000 to 9999 have been reserved for use with user defined fields, which are used as part of inter-firm communication. These tags can be registered/reserved via the FIX website.

The tag numbers greater than or equal to 10000 have been reserved for internal use (within a single firm) and do not need to be registered/reserved via the FIX website.
---
# Example Usage of Encoded Fields For Japanese Language Support

# Example 1 - Specify the ASCII/English value as Issuer plus Japanese character set as EncodedIssuer

| Tag                           | Field Name       | Value      |
| ----------------------------- | ---------------- | ---------- |
| …Other Standard Header fields |                  |            |
| 347                           | MessageEncoding  | Shift\_JIS |
| …Other Standard Header fields |                  |            |
| …Other Message Body fields    |                  |            |
| 106                           | Issuer           | HITACHI    |
| 348                           | EncodedIssuerLen | 10         |
| 349                           | EncodedIssuer    |            |
| …Other Message Body fields    |                  |            |

# Example 2 - Specify the ASCII/English value as Issuer plus Japanese character set as EncodedIssuer. Specify the ASCII/English value as Text plus Japanese character set as EncodedText.

| Tag                           | Field Name       | Value          |
| ----------------------------- | ---------------- | -------------- |
| …Other Standard Header fields |                  |                |
| 347                           | MessageEncoding  | Shift\_JIS     |
| …Other Standard Header fields |                  |                |
| …Other Message Body fields    |                  |                |
| 106                           | Issuer           | HITACHI        |
| 348                           | EncodedIssuerLen | 10             |
| 349                           | EncodedIssuer    |                |
| …Other Message Body fields    |                  |                |
| 58                            | Text             | This is a test |
| 356                           | EncodedTextLen   | 17             |
| 357                           | EncodedText      | chltzzkty      |
| …Other Message Body fields    |                  |                |

# Precautions when using UNICODE

There is the possibility that an SOH may be included in the character data when using UNICODE encoding. To avoid parsing problems, a FIX engine should use the EncodedLen value to extract the proper number of bytes.
---

# FIXML SYNTAX

# Background

The FPL FIXML Working Group began investigating the XML format in 1998 and published a White Paper supporting an evolutionary approach to migrating the FIX Protocol to an XML format. The working group released an initial version of the FIXML DTDs on January 15th, 1999. There are currently DTDs based on FIX Protocol versions 4.1, 4.2 and 4.3. A FIXML Schema based version of FIXML will be provided after the release of FIX 4.4. The FIXML Schema version will be able to provide reduced message size via the use of attributes and contextual abbreviations.

# FIXML Highlights

- FIXML is the XML vocabulary for creating FIX messages.
- Uses the same FIX data dictionary and business logic.
- Focuses primarily on the FIX Application Messages and does not provide a session layer.
- Can be encapsulated within the FIX Session Protocol or within another protocol like MQ Series, TIBCO, SOAP, etc.

This document incorporates FIXML in two distinct ways:

1. A corresponding DTD fragment supports each message definition. A reference to the FIXML element corresponding to the message name is provided for each message.
2. Each item in the data dictionary has a corresponding DTD equivalent provided in the data dictionary in Volume 6. This will be expanded to include the FIXML Schema definition for the field.

Note: while this document and the DTD are relatively in sync, the DTD will contain the full FIXML definitions. Note: The DTD, followed by the FIXML 4.4 Schema, shall be the official standard specification in the event of a discrepancy between the specification documents and the DTD and Schema documents.

FIXML 4.4 will also eventually be supplemented by an XML Schema version, which has recently been approved as a W3C Recommendation. The XML Schema version of FIXML will contain optimizations to optimize transport efficiency by decreasing message size. Increased usage of attributes and incorporation of contextual abbreviations within messages will accomplish the transport optimization. Contextual abbreviations are the removal of field prefixes that are obvious from within the context that the message is used.


---
# FIXML Design Rules

# General

Elements can contain other Elements, EMPTY content, or PCDATA (text) content and Attributes. Element names within FIXML start with the full names from the FIX specification and are abbreviated using the abbreviations specified as an appendix to this volume. FIXML uses camel case notation in which elements and attributes may be made up of multiple abbreviated words with each abbreviation word beginning with a capital letter.

FIXML requires content to be ordered. This differs from the traditional FIX approach (“tag=value” syntax) which allows fields to be in any order (other than the first couple and last). The FIXML is composed of ComponentTypes that correspond to the components that make up the FIX protocol specification. The following component types are defined for FIXML.

| **FIXML ComponentType**                                                                                                                                                                                                                                                                                                                                                                          | **Description**                                                                                                         |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------- |
| Message                                                                                                                                                                                                                                                                                                                                                                                          | Corresponds to a FIX message from the FIX specification                                                                 |
| Field                                                                                                                                                                                                                                                                                                                                                                                            | FIX Field                                                                                                               |
| Block                                                                                                                                                                                                                                                                                                                                                                                            | FIX Component Block - group of fields that are related and always appear together. Examples: Instrument Component Block |
| Component Blocks                                                                                                                                                                                                                                                                                                                                                                                 | for FIX and FIXML are specified within this volume.                                                                     |
| RepeatingGroup - A repeating group of fields within a FIX Message. The cardinality of the repeating group is defined with a field of type NumInGroup. FIXML does not require a separate data item to store the cardinality (number of items in the group). However, meta data for the NumInGroup field associated with a repeating group is provided to assist in mapping between FIX and FIXML. |                                                                                                                         |
| BlockRepeating                                                                                                                                                                                                                                                                                                                                                                                   | FIX Component Block that is also a repeating group, such as Parties Component Block.                                    |

FIXML provides metadata for each ComponentType that provides a mapping to the FIX document (such as Fullname of the field, tag value, FIX datatype). Metadata is provided as XML attributes within the DTD for FIXML. Metadata is provided as elements to the data type definitions for each component within the FIXML Schema version. Validation of these attributes must happen at the application level. The meta data items for each ComponentType are defined below.

1) Certain commonly used and well-known acronyms like IOI and DK are capitalized and separated from the rest of the tag by an underscore. (i.e. IOI_Qty).

# ComponentType - Field
---

# Meta data provided for FIXML Fields

| Metadata Item | Description                                                                            |
| ------------- | -------------------------------------------------------------------------------------- |
| FIXTag        | contains the FIX Protocol Field ID (Tag number)                                        |
| DataType      | One of the FIX datatypes defined in this specification (char, int, float, etc.).       |
| FullName      | Full name of the field within the FIX specification prior to abbreviation              |
| ComponentType | "Field"                                                                                |
| Value         | Enumerated values - if applicable                                                      |
| SDValue       | Self Describing values that correspond to the enumerated values in the Value attribute |

# Templates used to generate Fields in DTD

# Fields that use enums and values

&#x3C;!ELEMENT **XMLName** EMPTY>
&#x3C;!ATTLIST **XMLName**
FIXTag CDATA #FIXED '**Tag**'
DataType CDATA #FIXED '**Type**'
FullName CDATA #FIXED '**FieldName**'
ComponentType CDATA #FIXED 'Field'
Value **Value** #REQUIRED
SDValue **SDValue** #IMPLIED >

# Fields that use others standards or enums only

&#x3C;!ELEMENT **XMLName** EMPTY>
&#x3C;!ATTLIST **XMLName**
FIXTag CDATA #FIXED '**Tag**'
DataType CDATA #FIXED '**Type**'
FullName CDATA #FIXED '**FieldName**'
ComponentType CDATA #FIXED 'Field'
Value **Value** #REQUIRED >


---
# Other fields

XMLName (#PCDATA)>
XMLName
FIXTag CDATA #FIXED '**Tag**'
DataType CDATA #FIXED '**Type**'
FullName CDATA #FIXED '**FieldName**'
ComponentType CDATA #FIXED 'Field' >

# Field Example in DTD

FIXML allows for the XML parser to validate enumerations from the FIX Specification. These elements are defined with EMPTY content models and an attribute called Value. The acceptable values for FIXML attribute enumerations come from the FIX Specification. An optional attribute list named SDValue (SelfDescribingValue) contains the human-readable equivalent of the FIX specification values. The linkage between Value and SDValue cannot be validated using the DTD.

# ComponentType - Block

The FIX Component Blocks defined in Volume 1 that do not start off with a repeating group are identified as "Block" within FIXML.

# Meta data provided for FIXML Blocks

| Metadata Item        | Description |
| -------------------- | ----------- |
| FIXML Block Metadata |             |

---

# ComponentType

# FullName

Full name of the field within the FIX specification prior to abbreviation

# Category

Name of the category to which the element belongs

# ComponentType

"Block"

Template used to generate Component Blocks in DTD

# Block Example

# ComponentType - RepeatingGroup

Repeating groups from FIX messages have been identified. Many of the repeating groups are the same across multiple messages - even though they are not declared explicitly as component blocks. Repeating groups from within the spec that occur in multiple places have been identified in the repository as being implicit components. This permits the common naming and reuse of repeating group definitions across messages.

# Meta data provided for FIXML Repeating Groups

| Metadata Item   | Description                                                                                          |
| --------------- | ---------------------------------------------------------------------------------------------------- |
| NumInGrp\_FIELD | Name of the NumInGrp field that is used in the FIX tag=value version of FIX for the repeating group. |


---

# FIXTag

Tag # of the NumInGrp field

# Category

Name of the category to which the element belongs

# ComponentType

"RepeatingGroup"

Template used to generate RepeatingGroup component in DTD

&#x3C;!ENTITY % **ElementName**Custom "" >
&#x3C;!ENTITY % **ElementName**Content "**FieldList** %**ElementName**Custom;" >
&#x3C;!ELEMENT **ElementName** (%**ElementName**Content;)+>
&#x3C;!ATTLIST **ElementName**
NumInGrp_FIELD CDATA #FIXED '**CounterName**'
FIXTag CDATA #FIXED '**CounterTag**'
ComponentType CDATA #FIXED '**ComponentType**'
Category CDATA #FIXED '**Category**' >

# RepeatingGroup Example

&#x3C;!ENTITY % AllocGrpCustom "">
&#x3C;!ENTITY % AllocGrpContent "AllocAcct?, AllocAcctIDSrc?, MtchStat?, AllocPx?, AllocQty?,
IndAllocID?,    ProcessCode?,  NstPtys?,  NotifyBrkrOfCredit?,  AllocHandlInst?,       AllocText?,
EncAllocTextLen?,    EncAllocText?, CommData?, AllocAvgPx?,     AllocNetMny?,       SettlCurrAmt?,
AllocSettlCurrAmt?,   SettlCcy?,    AllocSettlCcy?,   SettlCurrFxRt?,   SettlCurrFxRtCalc?,
AllocAcrdIntAmt?,     AllocIntAtMat?,     MiscFeesGrp*,   ClrInstGrp*,    AllocSettlInstTyp?
%AllocGrpCustom;">
&#x3C;!ELEMENT AllocGrp (%AllocGrpContent;)+>
&#x3C;!ATTLIST AllocGrp
NumInGrp_FIELD CDATA #FIXED "NoAllocs"
FIXTag CDATA #FIXED "78"
ComponentType CDATA #FIXED "RepeatingGroup"
Category CDATA #FIXED "Allocation"
>

# ComponentType - BlockRepeating

Component Blocks that themselves are also repeating groups have been designated as BlockRepeating components within FIXML. This differentiation was done in order to minimize nesting of elements and to accommodate the additional meta data required for Repeating Groups (NumInGrp_FIELD, FIXTag).

# Meta data provided for FIXML BlockRepeating

| Metadata Item   | Description                                                                                          |
| --------------- | ---------------------------------------------------------------------------------------------------- |
| NumInGrp\_FIELD | Name of the NumInGrp field that is used in the FIX tag=value version of FIX for the repeating group. |
| FIXTag          | Tag # of the NumInGrp field                                                                          |


---

# DataType

One of the FIX datatypes defined in this specification (char, int, float, etc.).

# FullName

Full name of the field within the FIX specification prior to abbreviation

# Category

Name of the category to which the element belongs

# ComponentType

"BlockRepeating"

Template used to generate BlockRepeating Components in DTD (Same as RepeatingGroups template above)

# BlockRepeating Example

# ComponentType - Message

FIX Messages are represented in FIXML using the same name.

# Meta data provided for FIXML Messages

| Metadata Item | Description                                                               |
| ------------- | ------------------------------------------------------------------------- |
| FullName      | Full name of the field within the FIX specification prior to abbreviation |


---

# FIXML DTD Specification

# Category

Name of the category to which the element belongs

# ComponentType

"Message"

# FixSpecVolume

Number of the FIX volume where the message is documented

# FIXMsgType

The enumeration (Value) of the MsgType (Tag 35) field in the FIX specification for this message

# Template used to generate FIXML DTD

# Message Example

~~FIXML requires ordered content model.~~ ~~This differs from the traditional FIX approach (“tag=value” syntax) which allows fields to be in any order (other than the first couple and last).~~


---
# 5)

FIXML supports conditionally required content models. Options must contain a Strike Price.

FIXML permits the inclusion of custom (user defined fields) in order to be compliant with the FIX specification. ComponentTypes: Block, RepeatingGroup, BlockRepeating, and Message are all implemented using entities to permit this customization. Content models of business messages contain entities that allow for customization. For example, all application messages have a custom entity that can be redefined to extend the content model of the particular message. In the following example, the Position Maintenance Request message has two Entities - PosMntReqCustom and PosMntReqContent. PosMntReqCustom is defined as an empty string. PosMntReqContent is a string of all the components that make up the PosMntReq message. PosMntReqContent also includes the PosMntReqCustom entity. Custom fields are added to the PosMntReqCustom entity between the double quotes, each custom field must be preceded by a comma. Once added, the custom fields are automatically picked up as part of the PosMntReqContent entity. The following illustrates the ListExecute message:

To extend the content model of the ListExecute message, add the following to the internal subset of a FIXML message.
---

After entity reference resolution the Indication content model will look like:

&#x3C;!ELEMENT ListExecute (ListID,ClientBidID?,BidID?,TransactTime,Text?,EncodedTextGroup?,InternalTransNumber? )>

instead of

&#x3C;!ELEMENT ListExecute (ListID,ClientBidID?,BidID?,TransactTime,Text?,EncodedTextGroup? )>

FIXML elements have attributes, which contain referential information related to the FIX Field ID, Data type, and numeric constraints. Validation of these attributes must happen at the application level.

| FIXTag   | - | contains the FIX Protocol Field ID (Tag).                                                                |
| -------- | - | -------------------------------------------------------------------------------------------------------- |
| DataType | - | reflects data types (char, int, float, month-year, day-of-month, time, date) from the FIX specification. |

Example:

&#x3C;!ELEMENT ForexReq EMPTY>
&#x3C;!ATTLIST ForexReq FIXTag CDATA #FIXED '121'>
DataType CDATA #FIXED 'Boolean'>
Value (Y | N ) #REQUIRED>
SDValue (Yes | No ) #IMPLIED >

FIX defines message types with the MsgType field (tag "35"). Since the existence of a particular element indicates the message type (ie &#x3C;ExecutionReport>), MsgType is reflected as meta-data information. Each FIX message contains the attribute FIXTag with a fixed value equal to "35" and a Value attribute equal to the corresponding MsgType value.


---

FIXML allows for the XML parser to validate enumerations from the FIX Specification. These elements are defined with EMPTY content models and an attribute called Value. The acceptable values for FIXML attribute enumerations come from the FIX Specification. An optional attribute list called SDValue (SelfDescribingValue) contains the human-readable equivalent of the FIX specification values.

# 1. QuoteReq

&#x3C;!ELEMENT QuoteReq (%QuoteReqContent; )>
&#x3C;!ATTLIST QuoteReq
FIXTag CDATA #FIXED "35"
DataType CDATA #FIXED "char"
Value CDATA #FIXED "R">

# 2. ProcessCode

&#x3C;!ELEMENT ProcessCode EMPTY>
&#x3C;!ATTLIST ProcessCode
FIXTag CDATA #FIXED '81'
DataType CDATA #FIXED 'char'
Value (0 | 1 | 2 | 3 | 4 | 5 | 6 ) #REQUIRED
SDValue (Regular | SoftDollar | StepIn | StepOut | StepInSoft | StepOutSoft | PlanSponsor ) #IMPLIED >

The linkage between Value and SDValue cannot be validated.

# 3. Conditional Fields

When fields are conditionally required based on the value of other fields, the Tag=Value pair becomes an element. For example, ExecRefID is required when ExecTransType = Cancel. The attribute Value is added and contains the valid FIX Specification value.

# 4. ExecTransType

&#x3C;!ELEMENT ExecTransType (ExecNew | ExecCancel | ExecCorrect | ExecStatus)>

# 5. ExecCancel

&#x3C;!ELEMENT ExecCancel (ExecRefID, LastQty, LastPx)>
&#x3C;!ATTLIST ExecCancel
FIXTag CDATA #FIXED "20"
Value CDATA #FIXED "1">


---

20=1 (ExecTransType=Cancel)

becomes

<exectranstype><execnew fixtag="20" value="1"> … </execnew></exectranstype>

Applies to:

ExecNew, ExecCancel, ExecCorrect, ExecStatus, AllocStatusAccept, AllocStatusReject, AllocPartialAccept, AllocStatusReceived, AdvNew, AdvCancel, AdvReplace, IOINew, IOICancel, IOIReplace

7) FIXML has elements that serve as containers and do not map directly to FIX tag=value pairs.

8) Special containers are used when enumeration values of a FIX field must be split into two elements to handle conditionally required elements.

FIXTag CDATA #FIXED "59"

DataType CDATA #FIXED "char"

Value (0|1|2|3|4|5) #REQUIRED

SDValue (Day|GoodTillCancel|AtTheOpening|ImmediateOrCancel|FillOrKill| GoodTillCrossing) #IMPLIED

FIXTag CDATA #FIXED "59"

DataType CDATA #FIXED "char"

Value CDATA #FIXED "6"


---

# FIX Fields and Relationships

Certain FIX Fields are grouped into parent/child relationships. Referential information is contained in two places. The attribute FIXTags contains a list of valid tags in the content model and each field has its own attribute.

For example:

49=ssmb

becomes

<sender><compid senderfixtag="49">ssmb</compid></sender>

Applies to:

- Sender
- Target
- Location
- OnBehalfOf
- DeliverTo

~~9)~~ ~~FIX repeating groups are supported through the use of collection elements.~~ ~~To support conversions~~ ~~between FIX and FIXML, fields that identify the number of repeating elements are contained in the~~ ~~content model of the collection element.~~


---
COMMON COMPONENTS OF APPLICATION MESSAGES - Component Blocks

(Included in pre-trade, trade, and post-trade messages)

Many of the FIX Application Messages are composed of common "building blocks" or sets of data fields. For instance, almost every FIX Application Message has the set of symbology-related fields used to define the "Instrument": Symbol, SymbolSfx, SecurityIDSource, SecurityID….. EncodedSecurityDesc. Rather than replicate a common group of fields, the FIX specification specifies several key component blocks below which are simply referenced by component name within each Application Message which uses them. Thus when reviewing a specific message definition, the appropriate group of fields should be expanded and used whenever a component block is identified.

Note that some component blocks may be part of repeating groups thus if the component block is denoted as part of a repeating group, then the entire group of fields representing the component block are to be specified at the component block's repeating group "level" in the message definition and follow repeating group rules concerning field order. See "Repeating Groups" for more details.

# Instrument (symbology) component block -

| Tag | Field Name          | Req'd  | Comments                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| --- | ------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 55  | Symbol              | \*\*\* | Common, "human understood" representation of the security. SecurityID value can be specified if no symbol exists (e.g. non-exchange traded Collective Investment Vehicles) Use “\[N/A]” for products which do not have a symbol.                                                                                                                                                                                                                                                                                                                                                                                                               |
| 65  | SymbolSfx           | N      | Used in Fixed Income with a value of "WI" to indicate “When Issued” for a security to be reissued under an old CUSIP or ISIN or with a value of "CD" to indicate a EUCP with lump-sum interest rather than discount price.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| 48  | SecurityID          | N      | Takes precedence in identifying security to counterparty over SecurityAltID block. Requires SecurityIDSource if specified.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| 22  | SecurityIDSource    | N      | Required if SecurityID is specified.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| 454 | NoSecurityAltID     | N      | Number of alternate Security Identifiers                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|    |                     |        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 455 | SecurityAltID       | N      | Security Alternate identifier for this security First member of repeating group - must be specified if NoSecurityAltID > 0 The Security Alternative identifier block should not be populated unless SecurityID and SecurityIDSource are populated and should not duplicate the SecurityID and SecurityIDSource values contained in the SecurityID/SecurityIDSource tags. Use of SecurityAltID may be used if bilaterally agreed to assist in security identification, and does not imply an obligation on the receiver of the message to ensure validity or consistency with the SecurityID and SecurityIDSource fields which take precedence. |
| 456 | SecurityAltIDSource | N      | Source of SecurityAltID. Required if SecurityAltID is specified.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |


---

# Product Information

| 460 | Product                    | N | Indicates the type of product the security is associated with (high-level category)                                                                                                                                                                                                                                                                                                                                                                            |
| --- | -------------------------- | - | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 461 | CFICode                    | N | Indicates the type of security using ISO 10962 standard, Classification of Financial Instruments (CFI code) values. It is recommended that CFICode be used instead of SecurityType for non-Fixed Income instruments.                                                                                                                                                                                                                                           |
| 167 | SecurityType               | N | It is recommended that CFICode be used instead of SecurityType for non-Fixed Income instruments. Required for Fixed Income. Refer to Volume 7 – Fixed Income Futures and Options should be specified using the CFICode\[461] field instead of SecurityType\[167] (Refer to Volume 7 – Recommendations and Guidelines for Futures and Options Markets.)                                                                                                         |
| 762 | SecuritySubType            | N | Sub-type qualification/identification of the SecurityType (e.g. for SecurityType="MLEG"). If specified, SecurityType is required.                                                                                                                                                                                                                                                                                                                              |
| 200 | MaturityMonthYear          | N | Specifies the month and year of maturity. Applicable for standardized derivatives which are typically only referenced by month and year (e.g. S\&P futures). Note MaturityDate (a full date) can also be specified.                                                                                                                                                                                                                                            |
| 541 | MaturityDate               | N | Specifies date of maturity (a full date). Note that standardized derivatives which are typically only referenced by month and year (e.g. S\&P futures) may use MaturityMonthYear and/or this field. When using MaturityMonthYear, it is recommended that markets and sell sides report the MaturityDate on all outbound messages as a means of data enrichment.                                                                                                |
| 224 | CouponPaymentDate          | N | Date interest is to be paid. Used in identifying Corporate Bond issues.                                                                                                                                                                                                                                                                                                                                                                                        |
| 225 | IssueDate                  | N | Date instrument was issued. For Fixed Income IOIs for new issues, specifies the issue date.                                                                                                                                                                                                                                                                                                                                                                    |
| 239 | RepoCollateralSecurityType | N | (Deprecated, use UnderlyingSecurityType (310))                                                                                                                                                                                                                                                                                                                                                                                                                 |
| 226 | RepurchaseTerm             | N | (Deprecated, use TerminationType (788))                                                                                                                                                                                                                                                                                                                                                                                                                        |
| 227 | RepurchaseRate             | N | (Deprecated, use Price (44))                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| 228 | Factor                     | N | For Fixed Income: Amortization Factor for deriving Current face from Original face for ABS or MBS securities, note the fraction may be greater than, equal to or less than 1. In TIPS securities this is the Inflation index. Qty \* Factor \* Price = Gross Trade Amount For Derivatives: Contract Value Factor by which price must be adjusted to determine the true nominal value of one futures/options contract. (Qty \* Price) \* Factor = Nominal Value |
| 255 | CreditRating               | N |                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| 543 | InstrRegistry              | N | The location at which records of ownership are maintained for this instrument, and at which ownership changes must be                                                                                                                                                                                                                                                                                                                                          |


---

# Field Definitions

| Field Number | Field Name             | Type  | Description                                                                                                                                                                                                                                           |
| ------------ | ---------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 470          | CountryOfIssue         | N     | ISO Country code of instrument issue (e.g. the country portion typically used in ISIN). Can be used in conjunction with non-ISIN SecurityID (e.g. CUSIP for Municipal Bonds without ISIN) to provide uniqueness.                                      |
| 471          | StateOrProvinceOfIssue | N     | A two-character state or province abbreviation.                                                                                                                                                                                                       |
| 472          | LocaleOfIssue          | N     | The three-character IATA code for a locale (e.g. airport code for Municipal Bonds).                                                                                                                                                                   |
| 240          | RedemptionDate         | N     | (Deprecated, use YieldRedemptionDate (696) in \<YieldData> component block)                                                                                                                                                                           |
| 202          | StrikePrice            | N     | Used for derivatives, such as options and covered warrants.                                                                                                                                                                                           |
| 947          | StrikeCurrency         | N     | Used for derivatives.                                                                                                                                                                                                                                 |
| 206          | OptAttribute           | N     | Used for derivatives, such as options and covered warrants to indicate a versioning of the contract when required due to corporate actions to the underlying. Should not be used to indicate type of option – use the CFICode\[461] for this purpose. |
| 231          | ContractMultiplier     | N     | For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g. contracts vs. shares) amount.                                                                                             |
| 223          | CouponRate             | N     | For Fixed Income.                                                                                                                                                                                                                                     |
| 207          | SecurityExchange       | N     | Can be used to identify the security.                                                                                                                                                                                                                 |
| 106          | Issuer                 | N     |                                                                                                                                                                                                                                                       |
| 348          | EncodedIssuerLen       | N     | Must be set if EncodedIssuer field is specified and must immediately precede it.                                                                                                                                                                      |
| 349          | EncodedIssuer          | N     | Encoded (non-ASCII characters) representation of the Issuer field in the encoded format specified via the MessageEncoding field.                                                                                                                      |
| 107          | SecurityDesc           | N     |                                                                                                                                                                                                                                                       |
| 350          | EncodedSecurityDescLen | N     | Must be set if EncodedSecurityDesc field is specified and must immediately precede it.                                                                                                                                                                |
| 351          | EncodedSecurityDesc    | N     | Encoded (non-ASCII characters) representation of the SecurityDesc field in the encoded format specified via the MessageEncoding field.                                                                                                                |
| ~~668~~      | ~~DeliveryForm~~       | ~~N~~ | ~~i.e. Book Entry or Bearer~~                                                                                                                                                                                                                         |
| 691          | Pool                   | N     | Identifies MBS / ABS pool.                                                                                                                                                                                                                            |
| 667          | ContractSettlMonth     | N     | Must be present for MBS/TBA.                                                                                                                                                                                                                          |
| 875          | CPProgram              | N     | The program under which a commercial paper is issued.                                                                                                                                                                                                 |
| 876          | CPRegType              | N     | The registration type of a commercial paper issuance.                                                                                                                                                                                                 |
| 864          | NoEvents               | N     | Number of repeating EventType group entries.                                                                                                                                                                                                          |
| 865          | EventType              | N     | Put, Call, Tender, Sinking Fund Call, etc.                                                                                                                                                                                                            |


---

# Event Information

| 866 | EventDate           | N | Date of event                                        |
| --- | ------------------- | - | ---------------------------------------------------- |
| 867 | EventPx             | N | Predetermined price of issue at event, if applicable |
| 868 | EventText           | N | Comments                                             |
| 873 | DatedDate           | N | If different from IssueDate                          |
| 874 | InterestAccrualDate | N | If different from IssueDate and DatedDate            |

*** = Required status should match "Req'd" setting for &#x3C;Instrument> component block in the message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML Element Instrmt


---
Examples using Alternative Security Ids

The first example is from an order for shares in Daimler Chrysler, which has an ISIN DE0007100000, a CUSIP D1668R123, and a Sedol 5529027.

| Field (tag)               | Value        | Explanation                           |
| ------------------------- | ------------ | ------------------------------------- |
| Symbol (55)               | DCX          | Symbol = DCX (Daimler Chrysler)       |
| SecurityID (48)           | DE0007100000 |                                       |
| SecurityIDSource (22)     | 4            | ID Type is ISIN                       |
| NoSecurityAltID (454)     | 2            | Two additional security IDs specified |
| SecurityAltID (455)       | D1668R123    |                                       |
| SecurityAltIDSource (456) | 1            | SecurityID type is Cusip              |
| SecurityAltID (455)       | 5529027      |                                       |
| SecurityAltIDSource (456) | 2            | SecurityID type is Sedol              |

The second example is from an order for shares in IBM, which has an ISIN US4592001014, and a QUICK (Japanese) code of 000006680.

| Field (tag)               | Value        | Explanation                                    |
| ------------------------- | ------------ | ---------------------------------------------- |
| Symbol (55)               | IBM          | Symbol = IBM (International Business Machines) |
| SecurityID (48)           | US4592001014 |                                                |
| SecurityIDSource (22)     | 4            | ID Type is ISIN                                |
| NoSecurityAltID (454)     | 1            | One additional security ID specified           |
| SecurityAltID (455)       | 000006680    |                                                |
| SecurityAltIDSource (456) | 3            | SecurityID type is Quick                       |

Specifying an FpML product specification from within the FIX Instrument Block

| Field (tag)           | Value  | Explanation                                      |
| --------------------- | ------ | ------------------------------------------------ |
| Symbol (55)           | \[N/A] |                                                  |
| SecurityID (48)       | FpML   | Contains the FpML specification as an XML string |
| SecurityIDSource (22) | I      | ISDA/FpML Product Specification                  |

There are two alternative approaches to referencing the FpML specification.

SecurityID(48) Value Explanation

URL Specify a separate URL to reference a separate location for the FpML specification.

Example:
---

# FpML Specification


Local URL - the FpML specification is contained in the XMLDataLen (tag 212), XMLData (tag 213) of the FIX Session Layer

http://www.cme.com/product/irswap.jpg?id=122345



---

# Underlying Instrument

# (underlying instrument) component block

Refer to the Instrument component block comments as this component block mirrors Instrument.

| Tag | Field Name                           | Req'd  | Comments                                    |
| --- | ------------------------------------ | ------ | ------------------------------------------- |
| 311 | UnderlyingSymbol                     | \*\*\* |                                             |
| 312 | UnderlyingSymbolSfx                  | N      |                                             |
| 309 | UnderlyingSecurityID                 | N      |                                             |
| 305 | UnderlyingSecurityIDSource           | N      |                                             |
| 457 | NoUnderlyingSecurityAltID            | N      |                                             |
| 458 | UnderlyingSecurityAltID              | N      |                                             |
| 459 | UnderlyingSecurityAltIDSource        | N      |                                             |
| 462 | UnderlyingProduct                    | N      |                                             |
| 463 | UnderlyingCFICode                    | N      |                                             |
| 310 | UnderlyingSecurityType               | N      |                                             |
| 763 | UnderlyingSecuritySubType            | N      |                                             |
| 313 | UnderlyingMaturityMonthYear          | N      |                                             |
| 542 | UnderlyingMaturityDate               | N      |                                             |
| 241 | UnderlyingCouponPaymentDate          | N      |                                             |
| 242 | UnderlyingIssueDate                  | N      |                                             |
| 243 | UnderlyingRepoCollateralSecurityType | N      | (Deprecated, not applicable/used for Repos) |
| 244 | UnderlyingRepurchaseTerm             | N      | (Deprecated, not applicable/used for Repos) |
| 245 | UnderlyingRepurchaseRate             | N      | (Deprecated, not applicable/used for Repos) |
| 246 | UnderlyingFactor                     | N      |                                             |
| 256 | UnderlyingCreditRating               | N      |                                             |
| 595 | UnderlyingInstrRegistry              | N      |                                             |
| 592 | UnderlyingCountryOfIssue             | N      |                                             |
| 593 | UnderlyingStateOrProvinceOfIssue     | N      |                                             |
| 594 | UnderlyingLocaleOfIssue              | N      |                                             |


---

# 247

|                                      | UnderlyingRedemptionDate | N     | (Deprecated, use YieldRedemptionDate (696) in \<YieldData> component block)                                                                                                                            |
| ------------------------------------ | ------------------------ | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| ~~315~~ UnderlyingPutOrCall          |                          | ~~N~~ |                                                                                                                                                                                                        |
| 316 UnderlyingStrikePrice            |                          | N     |                                                                                                                                                                                                        |
| 941 UnderlyingStrikeCurrency         |                          | N     |                                                                                                                                                                                                        |
| 317 UnderlyingOptAttribute           |                          | N     |                                                                                                                                                                                                        |
| 436 UnderlyingContractMultiplier     |                          | N     |                                                                                                                                                                                                        |
| 435 UnderlyingCouponRate             |                          | N     |                                                                                                                                                                                                        |
| 308 UnderlyingSecurityExchange       |                          | N     |                                                                                                                                                                                                        |
| 306 UnderlyingIssuer                 |                          | N     |                                                                                                                                                                                                        |
| 362 EncodedUnderlyingIssuerLen       |                          | N     |                                                                                                                                                                                                        |
| 363 EncodedUnderlyingIssuer          |                          | N     |                                                                                                                                                                                                        |
| 307 UnderlyingSecurityDesc           |                          | N     |                                                                                                                                                                                                        |
| 364 EncodedUnderlyingSecurityDescLen |                          | N     |                                                                                                                                                                                                        |
| 365 EncodedUnderlyingSecurityDesc    |                          | N     |                                                                                                                                                                                                        |
| 877 UnderlyingCPProgram              |                          | N     |                                                                                                                                                                                                        |
| 878 UnderlyingCPRegType              |                          | N     |                                                                                                                                                                                                        |
| 318 UnderlyingCurrency               |                          | N     | Specific to the \<UnderlyingInstrument> (not in \<Instrument>)                                                                                                                                         |
| 879 UnderlyingQty                    |                          | N     | Specific to the \<UnderlyingInstrument> (not in \<Instrument>) Unit amount of the underlying security (par, shares, currency, etc.)                                                                    |
| 810 UnderlyingPx                     |                          | N     | Specific to the \<UnderlyingInstrument> (not in \<Instrument>) In a financing deal clean price (percent-of-par or per unit) of the underlying security or basket.                                      |
| 882 UnderlyingDirtyPrice             |                          | N     | Specific to the \<UnderlyingInstrument> (not in \<Instrument>) In a financing deal price (percent-of-par or per unit) of the underlying security or basket. “Dirty” means it includes accrued interest |
| 883 UnderlyingEndPrice               |                          | N     | Specific to the \<UnderlyingInstrument> (not in \<Instrument>) In a financing deal price (percent-of-par or per unit) of the underlying security or basket at the end of the agreement.                |
| 884 UnderlyingStartValue             |                          | N     | Specific to the \<UnderlyingInstrument> (not in \<Instrument>) Currency value attributed to this collateral at the start of the agreement                                                              |
| 885 UnderlyingCurrentValue           |                          | N     | Specific to the \<UnderlyingInstrument> (not in \<Instrument>)                                                                                                                                         |


---

# Currency value currently attributed to this collateral

| 886             | UnderlyingEndValue                                                       | N                         | Specific to the (not in ) |
| --------------- | ------------------------------------------------------------------------ | ------------------------- | ------------------------- |
|                 | Currency value attributed to this collateral at the end of the agreement |                           |                           |
|                 | N                                                                        | Specific to the (not in ) |                           |
| Component Block | Insert here the contents of the                                          |                           |                           |
| Component Block |                                                                          |                           |                           |
|                 |                                                                          |                           |                           |
|                 |                                                                          |                           |                           |

*** = Required status should match "Req'd" setting for <underlyinginstrument> component block in the message definition</underlyinginstrument>

# FIXML Definition for this Component Block

– see http://www.fixprotocol.org for details

Refer to FIXML element UndInstrmt


---

# Instrument Leg (symbology) component block

Refer to the Instrument component block comments as this component block mirrors Instrument. Several multileg-oriented messages specify an instrument leg component block. An instrument can have zero or more instrument legs. The fundamental business rule that applies to the multileg instrument is that the multileg instrument is defined as the combination of instrument legs. The multileg instrument must be able to be traded atomically – that all instrument legs are traded or none are traded.

The LegRatioQty[623] is used to define the quantity of the leg that makes up a single unit of the multileg instrument. An option butterfly strategy is made up of three option legs.

| Tag | Field Name                    | Req'd  | Comments                                    |
| --- | ----------------------------- | ------ | ------------------------------------------- |
| 600 | LegSymbol                     | \*\*\* |                                             |
| 601 | LegSymbolSfx                  | N      |                                             |
| 602 | LegSecurityID                 | N      |                                             |
| 603 | LegSecurityIDSource           | N      |                                             |
| 604 | NoLegSecurityAltID            | N      |                                             |
| 605 | LegSecurityAltID              | N      |                                             |
| 606 | LegSecurityAltIDSource        | N      |                                             |
| 607 | LegProduct                    | N      |                                             |
| 608 | LegCFICode                    | N      |                                             |
| 609 | LegSecurityType               | N      |                                             |
| 764 | LegSecuritySubType            | N      |                                             |
| 610 | LegMaturityMonthYear          | N      |                                             |
| 611 | LegMaturityDate               | N      |                                             |
| 248 | LegCouponPaymentDate          | N      |                                             |
| 249 | LegIssueDate                  | N      |                                             |
| 250 | LegRepoCollateralSecurityType | N      | (Deprecated, not applicable/used for Repos) |
| 251 | LegRepurchaseTerm             | N      | (Deprecated, not applicable/used for Repos) |
| 252 | LegRepurchaseRate             | N      | (Deprecated, not applicable/used for Repos) |
| 253 | LegFactor                     | N      |                                             |
| 257 | LegCreditRating               | N      |                                             |
| 599 | LegInstrRegistry              | N      |                                             |
| 596 | LegCountryOfIssue             | N      |                                             |


---

# FIXML Definition for this Component Block

– see http://www.fixprotocol.org for details

| Field                     | Required | Description                                                                 |
| ------------------------- | -------- | --------------------------------------------------------------------------- |
| LegStateOrProvinceOfIssue | N        |                                                                             |
| LegLocaleOfIssue          | N        |                                                                             |
| LegRedemptionDate         | N        | (Deprecated, use YieldRedemptionDate (696) in \<YieldData> component block) |
| LegStrikePrice            | N        |                                                                             |
| LegStrikeCurrency         | N        |                                                                             |
| LegOptAttribute           | N        |                                                                             |
| LegContractMultiplier     | N        |                                                                             |
| LegCouponRate             | N        |                                                                             |
| LegSecurityExchange       | N        |                                                                             |
| LegIssuer                 | N        |                                                                             |
| EncodedLegIssuerLen       | N        |                                                                             |
| EncodedLegIssuer          | N        |                                                                             |
| LegSecurityDesc           | N        |                                                                             |
| EncodedLegSecurityDescLen | N        |                                                                             |
| EncodedLegSecurityDesc    | N        |                                                                             |
| LegRatioQty               | N        | Specific to the \<InstrumentLeg> (not in \<Instrument>)                     |
| LegSide                   | N        | Specific to the \<InstrumentLeg> (not in \<Instrument>)                     |
| LegCurrency               | N        | Specific to the \<InstrumentLeg> (not in \<Instrument>)                     |
| LegDeliveryForm           | N        | i.e. Book Entry or Bearer                                                   |
| LegPool                   | N        | Identifies MBS / ABS pool                                                   |
| LegDatedDate              | N        |                                                                             |
| LegContractSettlMonth     | N        |                                                                             |
| LegInterestAccrualDate    | N        |                                                                             |

*** = Required status should match "Req'd" setting for &#x3C;OrderQtyData> component block in message definition


---


# InstrumentExtension component block

| Tag | Field Name       | Req'd           | Comments                                       |                              |
| --- | ---------------- | --------------- | ---------------------------------------------- | ---------------------------- |
| 668 | DeliveryForm     | N               | Identifies the form of delivery.               |                              |
| 869 | PctAtRisk        | N               | Percent at risk due to lowest possible call.   |                              |
| 870 | NoInstrAttrib    | N               | Number of repeating InstrAttrib group entries. |                              |
|    | 871              | InstrAttribType | N                                              | Type of instrument attribute |
| 872 | InstrAttribValue | N               | Value of instrument attribute, if applicable   |                              |

*** = Required status should match "Req'd" setting for &#x3C;InstrumentExtension> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element InstrmtExtension


---

OrderQtyData component block -


<orderqtydata>
| Tag | Field Name        | Req'd | Comments                                                                                                                                                                                                                                                                                                                                                                       |
| --- | ----------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 38  | OrderQty          | N     | One of CashOrderQty, OrderQty, or (for CIV only) OrderPercent is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or OrderPercent should be specified.                                                                                                                                                                                      |
| 152 | CashOrderQty      | N     | One of CashOrderQty, OrderQty, or (for CIV only) OrderPercent is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or OrderPercent should be specified. Specifies the approximate “monetary quantity” for the order. Broker is responsible for converting and calculating OrderQty in tradeable units (e.g. shares) for subsequent messages. |
| 516 | OrderPercent      | N     | For CIV - Optional. One of CashOrderQty, OrderQty or (for CIV only) OrderPercent is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or OrderPercent should be specified.                                                                                                                                                                   |
| 468 | RoundingDirection | N     | For CIV – Optional                                                                                                                                                                                                                                                                                                                                                             |
| 469 | RoundingModulus   | N     | For CIV – Optional                                                                                                                                                                                                                                                                                                                                                             |

</orderqtydata>

*** = Required status should match "Req'd" setting for <orderqtydata> component block in message definition</orderqtydata>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element OrdQtyData



---

# CommissionData component block

<commissiondata>
| Tag | Field Name    | Req'd | Comments           |
| --- | ------------- | ----- | ------------------ |
| 12  | Commission    | N     |                    |
| 13  | CommType      | N     |                    |
| 479 | CommCurrency  | N     | For CIV - Optional |
| 497 | FundRenewWaiv | N     | For CIV - Optional |

</commissiondata>
*** = Required status should match "Req'd" setting for &#x3C;CommissionData> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML Element CommData


---

# Parties component block -

See “Volume 6 - APPENDIX 6-G - USE OF &#x3C;PARTIES> COMPONENT BLOCK”.

| Tag | Field Name    | Req'd | Comments                                                                                                                 |
| --- | ------------- | ----- | ------------------------------------------------------------------------------------------------------------------------ |
| 453 | NoPartyIDs    | N     | Repeating group below should contain unique combinations of PartyID, PartyIDSource, and PartyRole                        |
| 448 | PartyID       | N     | Used to identify source of PartyID. Required if PartyIDSource is specified. Required if NoPartyIDs > 0.                  |
| 447 | PartyIDSource | N     | Used to identify class source of PartyID value (e.g. BIC). Required if PartyID is specified. Required if NoPartyIDs > 0. |
| 452 | PartyRole     | N     | Identifies the type of PartyID (e.g. Executing Broker). Required if NoPartyIDs > 0.                                      |
| 802 | NoPartySubIDs | N     | Repeating group of Party sub-identifiers.                                                                                |
|     | 523           | N     | Sub-identifier (e.g. Clearing Acct for PartyID=Clearing Firm) if applicable. Required if NoPartySubIDs > 0.              |
|     | 803           | N     | Type of Sub-identifier. Required if NoPartySubIDs > 0.                                                                   |

</parties>
*** = Required status should match "Req'd" setting for &#x3C;Parties> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Ptys


---

# NestedParties component block -

<nestedparties>
| Tag | Field Name       |     | Req'd                                                                                                                                      | Comments                                                                                                          |
| --- | ---------------- | --- | ------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------- |
| 539 | NoNestedPartyIDs | N   | Repeating group below should contain unique combinations of NestedPartyID, NestedPartyIDSource, and NestedPartyRole                        |                                                                                                                   |
|    | 524              | N   | Used to identify source of NestedPartyID. Required if NestedPartyIDSource is specified. Required if NoNestedPartyIDs > 0.                  |                                                                                                                   |
|    | 525              | N   | Used to identify class source of NestedPartyID value (e.g. BIC). Required if NestedPartyID is specified. Required if NoNestedPartyIDs > 0. |                                                                                                                   |
|    | 538              | N   | Identifies the type of NestedPartyID (e.g. Executing Broker). Required if NoNestedPartyIDs > 0.                                            |                                                                                                                   |
|    | 804              | N   |                                                                                                                                            | Repeating group of NestedParty sub-identifiers.                                                                   |
|    |                 | 545 | N                                                                                                                                          | Sub-identifier (e.g. Clearing Acct for PartyID=Clearing Firm) if applicable. Required if NoNestedPartySubIDs > 0. |
|    |                 | 805 | N                                                                                                                                          | Type of Sub-identifier. Required if NoNestedPartySubIDs > 0.                                                      |

</nestedparties>
*** = Required status should match "Req'd" setting for <nestedparties> component block in message definition</nestedparties>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element NstPtys


---

# NestedParties2 (second instance of nesting) component block

<nestedparties2>
| Tag | Field Name           | Req'd | Comments                                                                                                                                      |
| --- | -------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 756 | NoNested2PartyIDs    | N     | Repeating group below should contain unique combinations of Nested2PartyID, Nested2PartyIDSource, and Nested2PartyRole                        |
| 757 | Nested2PartyID       | N     | Used to identify source of Nested2PartyID. Required if Nested2PartyIDSource is specified. Required if NoNested2PartyIDs > 0.                  |
| 758 | Nested2PartyIDSource | N     | Used to identify class source of Nested2PartyID value (e.g. BIC). Required if Nested2PartyID is specified. Required if NoNested2PartyIDs > 0. |
| 759 | Nested2PartyRole     | N     | Identifies the type of Nested2PartyID (e.g. Executing Broker). Required if NoNested2PartyIDs > 0.                                             |
| 806 | NoNested2PartySubIDs | N     | Repeating group of Nested2Party sub-identifiers.                                                                                              |
| 760 | Nested2PartySubID    | N     | Sub-identifier (e.g. Clearing Acct for PartyID=Clearing Firm) if applicable. Required if NoNested2PartySubIDs > 0.                            |
| 807 | Nested2PartySubID    | N     | Type of Sub-identifier. Required if NoNested2PartySubIDs > 0.                                                                                 |

</nestedparties2>
*** = Required status should match "Req'd" setting for &#x3C;NestedParties2> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element NstPtys2


---

# NestedParties3 (third instance of nesting) component block -

<nestedparties3>
| Tag | Field Name        |                      | Req'd                                                                                                                  | Comments                                                                                                                                      |
| --- | ----------------- | -------------------- | ---------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 948 | NoNested3PartyIDs | N                    | Repeating group below should contain unique combinations of Nested3PartyID, Nested3PartyIDSource, and Nested3PartyRole |                                                                                                                                               |
|     | 949               | Nested3PartyID       | N                                                                                                                      | Used to identify source of Nested3PartyID. Required if Nested3PartyIDSource is specified. Required if NoNested3PartyIDs > 0.                  |
|     | 950               | Nested3PartyIDSource | N                                                                                                                      | Used to identify class source of Nested3PartyID value (e.g. BIC). Required if Nested3PartyID is specified. Required if NoNested3PartyIDs > 0. |
|     | 951               | Nested3PartyRole     | N                                                                                                                      | Identifies the type of Nested3PartyID (e.g. Executing Broker). Required if NoNested3PartyIDs > 0.                                             |
|     | 952               | NoNested3PartySubIDs | N                                                                                                                      | Repeating group of Nested3Party sub-identifiers.                                                                                              |
|     | 953               | Nested3PartySubID    | N                                                                                                                      | Sub-identifier (e.g. Clearing Acct for PartyID=Clearing Firm) if applicable. Required if NoNested3PartySubIDs > 0.                            |
|     |                   | Nested3PartySubID    | N                                                                                                                      | Type of Sub-identifier. Required if NoNested3PartySubIDs > 0.                                                                                 |

</nestedparties3>
*** = Required status should match "Req'd" setting for <nestedparties3> component block in message definition</nestedparties3>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element NstPtys3


---


# SettlParties (settlement parties) component block

<settlparties>
| Tag | Field Name          | Req'd | Comments                                                                                                                                |
| --- | ------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------- |
| 781 | NoSettlPartyIDs     | N     | Repeating group below should contain unique combinations of SettlPartyID, SettlPartyIDSource, and SettlPartyRole                        |
| 782 | SettlPartyID        | N     | Used to identify source of SettlPartyID. Required if SettlPartyIDSource is specified. Required if NoSettlPartyIDs > 0.                  |
| 783 | SettlPartyIDSource  | N     | Used to identify class source of SettlPartyID value (e.g. BIC). Required if SettlPartyID is specified. Required if NoSettlPartyIDs > 0. |
| 784 | SettlPartyRole      | N     | Identifies the type of SettlPartyID (e.g. Executing Broker). Required if NoSettlPartyIDs > 0.                                           |
| 801 | NoSettlPartySubIDs  | N     | Repeating group of SettlParty sub-identifiers.                                                                                          |
| 785 | SettlPartySubID     | N     | Sub-identifier (e.g. Clearing Acct for SettlPartyID=Clearing Firm) if applicable. Required if NoSettlPartySubIDs > 0.                   |
| 786 | SettlPartySubIDType | N     | Type of Sub-identifier. Required if NoSettlPartySubIDs > 0.                                                                             |

</settlparties>
*** = Required status should match "Req'd" setting for &#x3C;SettlParties> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SettlPtys

# SpreadOrBenchmarkCurveData component block

<spreadorbenchmarkcurvedata>
| Tag | Field Name             | Req'd | Comments         |
| --- | ---------------------- | ----- | ---------------- |
| 218 | Spread                 | N     | For Fixed Income |
| 220 | BenchmarkCurveCurrency | N     |                  |
| 221 | BenchmarkCurveName     | N     |                  |
| 222 | BenchmarkCurvePoint    | N     |                  |

</spreadorbenchmarkcurvedata>


---

# Benchmark Price Information

| 662 | BenchmarkPrice       | N |                                                                                                                               |
| --- | -------------------- | - | ----------------------------------------------------------------------------------------------------------------------------- |
| 663 | BenchmarkPriceType   | N | Must be present if BenchmarkPrice is used.                                                                                    |
| 699 | BenchmarkSecurityID  | N | The identifier of the benchmark security, e.g. Treasury against Corporate bond.                                               |
| 761 | BenchmarkSecurityIDS | N | Source of BenchmarkSecurityID. If not specified, then ID Source is understood to be the same as that in the Instrument block. |

*** = Required status should match "Req'd" setting for <spreadorbenchmarkcurvedata> component block in message definition</spreadorbenchmarkcurvedata>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SpreadOrBnchmkCrvData


---

# LegBenchmarkCurveData component block -

<legbenchmarkcurvedata>
| Tag | Field Name          | Req'd | Comments |
| --- | ------------------- | ----- | -------- |
| 676 | LegBenchmarkCurveC  | N     |          |
| 677 | LegBenchmarkCurveN  | N     |          |
| 678 | LegBenchmarkCurvePo | N     |          |
| 679 | LegBenchmarkPrice   | N     |          |
| 680 | LegBenchmarkPriceTy | N     |          |

</legbenchmarkcurvedata>
*** = Required status should match "Req'd" setting for &#x3C;LegBenchmarkCurveData> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element LegBnchmkCrvData


---


# Stipulations component block

| Tag | Field Name      | Req'd            | Comments                      |   |
| --- | --------------- | ---------------- | ----------------------------- | - |
| 232 | NoStipulations  | N                |                               |   |
| 233 | StipulationType | N                | Required if NoStipulations >0 |   |
|     | 234             | StipulationValue | N                             |   |

*** = Required status should match "Req'd" setting for &#x3C;Stipulations> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Stips


---

Underlying Stipulations Component Block


# Underlying Stipulations

| Tag | Field Name          | Req'd | Comments                          |
| --- | ------------------- | ----- | --------------------------------- |
| 887 | NoUnderlyingStips   | N     |                                   |
| 888 | UnderlyingStipType  | N     | Required if NoUnderlyingStips > 0 |
| 889 | UnderlyingStipValue | N     |                                   |

*** = Required status should match "Req'd" setting for &#x3C;UnderlyingStipulations> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element UndStips



---

# LegStipulations component block

| Tag                 | Field Name         | Req'd | Comments                          |
| ------------------- | ------------------ | ----- | --------------------------------- |
| 683                 | NoLegStipulations  | N     |                                   |
| 688                 | LegStipulationType | N     | Required if NoLegStipulations > 0 |
| LegStipulationValue | N                  |       |                                   |

*** = Required status should match "Req'd" setting for &#x3C;LegStipulations> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element LegStips


---

# YieldData component block

<yielddata>
| Tag | Field Name           | Req'd | Comments |
| --- | -------------------- | ----- | -------- |
| 235 | YieldType            | N     |          |
| 236 | Yield                | N     |          |
| 701 | YieldCalcDate        | N     |          |
| 696 | YieldRedemptionDate  | N     |          |
| 697 | YieldRedemptionPrice | N     |          |
| 698 | YieldRedemptionPrice | N     | Type     |

</yielddata>
*** = Required status should match "Req'd" setting for <yielddata> component block in message definition</yielddata>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element YldData


---

# PositionQty Component Block

{Need description here}

| Tag             | Field Name   | Req'd  | Comments                    |
| --------------- | ------------ | ------ | --------------------------- |
| 702             | NoPositions  | \*\*\* |                             |
| 703             | PosType      | N      | Required if NoPositions > 1 |
| 704             | LongQty      | N      |                             |
| 705             | ShortQty     | N      |                             |
| 706             | PosQtyStatus | N      |                             |
| Component Block |              |        |                             |
|                 |              |        |                             |

Optional repeating group - used to associate or distribute position to a specific party other than the party that currently owns the position.

{Need DTD}

{Need Examples Here in FIX and FIXML}

*** = Required status should match "Req'd" setting for  component block in message definition</positionqty>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element PosQty


---

# Position Amount Data Component Block

| Tag | Field Name | Req'd  | Comments                          |
| --- | ---------- | ------ | --------------------------------- |
| 753 | NoPosAmt   | \*\*\* | Number of Position Amount entries |
| 707 | PosAmtType | \*\*\* |                                   |
| 708 | PosAmt     | \*\*\* |                                   |

*** = Required status should match "Req'd" setting for &#x3C;PositionAmountData> component block in message definition

</positionamountdata>
FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element PosAmtData


---

# TrdRegTimestamps component block

<trdregtimestamps>
| Tag | Field Name         | Req'd  | Comments                           |
| --- | ------------------ | ------ | ---------------------------------- |
| 768 | NoTrdRegTimestamps | \*\*\* |                                    |
| 769 | TrdRegTimestamp    | N      | Required if NoTrdRegTimestamps > 1 |
| 770 | TrdRegTimestampTy  | N      | Required if NoTrdRegTimestamps > 1 |
|     | pe                 |        |                                    |
| 771 | TrdRegTimestampO   | N      | Optional                           |
|     | rigin              |        |                                    |

</trdregtimestamps>
*** = Required status should match "Req'd" setting for &#x3C;TrdRegTimestamps> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TrdRegTmstampsGrp


---

# SettlInstructionsData component block -

See “Volume 6 - APPENDIX 6-H - USE OF &#x3C;SETTLINSTRUCTIONS> COMPONENT BLOCK”.

| Tag             | Field Name        | Req'd | Comments                                                                                                              |
| --------------- | ----------------- | ----- | --------------------------------------------------------------------------------------------------------------------- |
| 172             | SettlDeliveryType | N     | Required if AllocSettlInstType = 1 or 2                                                                               |
| 169             | StandInstDbType   | N     | Required if AllocSettlInstType = 3 (should not be populated otherwise)                                                |
| 170             | StandInstDbName   | N     | Required if AllocSettlInstType = 3 (should not be populated otherwise)                                                |
| 171             | StandInstDbID     | N     | Identifier used within the StandInstDbType Required if AllocSettlInstType = 3 (should not be populated otherwise)     |
| 85              | NoDlvyInst        | N     | Required (and must be > 0) if AllocSettlInstType = 2 (should not be populated otherwise)                              |
| 165             | SettlInstSource   | N     | Used to identify whether these delivery instructions are for the buyside or the sellside. Required if NoDlvyInst > 0. |
| 787             | DlvyInstType      | N     | S – securities, C – cash, mandatory for each occurrence of this repeating group Required if NoDlvyInst > 0.           |
| Component Block |                   |       |                                                                                                                       |
| \<SettlParties> |                   |       |                                                                                                                       |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to the FIXML element SettlInstrctnsData


---

# PegInstructions component block

| Tag | Field Name        | Req'd | Comments                                                                                                                  |
| --- | ----------------- | ----- | ------------------------------------------------------------------------------------------------------------------------- |
| 211 | PegOffsetValue    | N     | Amount (signed) added to the peg for a pegged order in the context of the PegOffsetType                                   |
| 835 | PegMoveType       | N     | Describes whether peg is static/fixed or floats                                                                           |
| 836 | PegOffsetType     | N     | Type of Peg Offset (e.g. price offset, tick offset etc)                                                                   |
| 837 | PegLimitType      | N     | Specifies nature of resulting pegged price (e.g. or better limit, strict limit etc)                                       |
| 838 | PegRoundDirection | N     | If the calculated peg price is not a valid tick price, specifies how to round the price (e.g. be more or less aggressive) |
| 840 | PegScope          | N     | The scope of the “related to” price of the peg (e.g. local, global etc)                                                   |

</peginstructions>
Note that Pegged orders are specified by the use of OrdType (to denote that the order is a pegged order) and ExecInst (to specify what price the order is pegged to).

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to the FIXML element PegInstrctns


---

# DiscretionInstructions component block

<discretioninstructions>
| Tag | Field Name               | Req'd | Comments                                                                                                                            |
| --- | ------------------------ | ----- | ----------------------------------------------------------------------------------------------------------------------------------- |
| 388 | DiscretionInst           | N     | What the discretionary price is related to (e.g. primary price, display price etc)                                                  |
| 389 | DiscretionOffsetValue    | N     | Amount (signed) added to the “related to” price specified via DiscretionInst, in the context of DiscretionOffsetType                |
| 841 | DiscretionMoveType       | N     | Describes whether discretion price is static/fixed or floats                                                                        |
| 842 | DiscretionOffsetType     | N     | Type of Discretion Offset (e.g. price offset, tick offset etc)                                                                      |
| 843 | DiscretionLimitType      | N     | Specifies the nature of the resulting discretion price (e.g. or better limit, strict limit etc)                                     |
| 844 | DiscretionRoundDirection | N     | If the calculated discretion price is not a valid tick price, specifies how to round the price (e.g. to be more or less aggressive) |
| 846 | DiscretionScope          | N     | The scope of “related to” price of the discretion (e.g. local, global etc)                                                          |

</discretioninstructions>
FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to the FIXML element DsctnInstrctns


---

# FinancingDetails component block

Component block is optionally used only for financing deals to identify the legal agreement under which the deal was made and other unique characteristics of the transaction. The AgreementDesc field refers to base standard documents such as MRA 1996 Repurchase Agreement, GMRA 2000 Bills Transaction (U.K.), MSLA 1993 Securities Loan – Amended 1998, for example.

<financingdetails>
| Tag | Field Name        | Req'd | Comments                                                                                                                         |
| --- | ----------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------- |
| 913 | AgreementDesc     | N     | The full name of the base standard agreement, annexes and amendments in place between the principals and applicable to this deal |
| 914 | AgreementID       | N     | A common reference to the applicable standing agreement between the principals                                                   |
| 915 | AgreementDate     | N     | A reference to the date the underlying agreement was executed.                                                                   |
| 918 | AgreementCurrency | N     | Currency of the underlying agreement.                                                                                            |
| 788 | TerminationType   | N     | For Repos the timing or method for terminating the agreement.                                                                    |
| 916 | StartDate         | N     | Settlement date of the beginning of the deal                                                                                     |
| 917 | EndDate           | N     | Repayment / repurchase date                                                                                                      |
| 919 | DeliveryType      | N     | Delivery or custody arrangement for the underlying securities                                                                    |
| 898 | MarginRatio       | N     | Percentage of cash value that underlying security collateral must meet.                                                          |

</financingdetails>

*** = Required status should match "Req'd" setting for &#x3C;FinancingDetails> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to the FIXML element FinancingDetails


---

# COMMON APPLICATION MESSAGES (Apply to pre-trade, trade, and post-trade)

# Business Message Reject

The Business Message Reject message can reject an application-level message which fulfills session-level rules and cannot be rejected via any other means. Note if the message fails a session-level rule (e.g. body length is incorrect), a session-level Reject message should be issued. See the session-level Reject message.

It should NOT be used in the following situations:

| Situation                                                                      | Appropriate Response                             |
| ------------------------------------------------------------------------------ | ------------------------------------------------ |
| Session-level problem meeting the criteria of the session-level Reject message | Use the session-level Reject message (MsgType=3) |
| In response to:                                                                | Use the Quote Request Reject message             |
| · Quote Request                                                                |                                                  |
| In response to:                                                                | Use the Quote Status Report message              |
| · Quote                                                                        |                                                  |
| · Quote Cancel                                                                 |                                                  |
| · Quote Status Request                                                         |                                                  |
| · Quote Response                                                               |                                                  |
| In response to:                                                                | Use the Mass Quote Acknowledgment message        |
| · Mass Quote                                                                   |                                                  |
| In response to:                                                                | Use the Market Data Request Reject message       |
| · Market Data Request                                                          |                                                  |
| In response to:                                                                | Use the Security Definition message              |
| · Security Definition Request                                                  |                                                  |
| In response to:                                                                | Use the SecurityTypes message                    |
| · Security Type Request                                                        |                                                  |
| In response to:                                                                | Use the Security List message                    |
| · Security List Request                                                        |                                                  |
| In response to:                                                                | Use the Derivative Security List message         |
| · Derivative Security List Request                                             |                                                  |
| In response to:                                                                | Use the Security Status message                  |
| · Security Status Request                                                      |                                                  |
| In response to:                                                                | Use the Trading Session Status message           |
| · Trading Session Status Request                                               |                                                  |
| In response to                                                                 | Use the Execution Report message                 |
| · New Order - Single                                                           |                                                  |
| · Order Status Request                                                         |                                                  |


---

# Order Mass Status Request

- New Order – Cross
- New Order – Multileg
- New Order – List
- List Execute

In response to: Use the Order Cancel Reject message

- Order Cancel Request
- Order Cancel/Replace Request
- Cross Order Cancel Request
- Cross Order Cancel/Replace Request
- Multileg Order Cancel/Replace Request
- List Cancel Request

In response to: Use the Don’t Know Trade (DK) message

- Execution Report

In response to: Use the Order Mass Cancel Report message

- Order Mass Cancel Request

In response to: Use the List Status message

- List Status Request

In response to: Use the Allocation Instruction Ack message

- Allocation Instruction

In response to: Use the Allocation Report Ack message

- Allocation Report

In response to: Use the Confirmation Ack message

- Confirmation

In response to: Use the Registration Instructions Response message

- Registration Instructions

In response to: Use the Trade Capture Report message

- Trade Capture Report Request

In response to: Use the Bid Response message

- Bid Request

In response to: Use the Confirmation message

- Confirmation Request

In response to: Use the Settlement Instructions message

- Settlement Instruction Request

In response to: Use the Position Maintenance Report message

- Position Maintenance Request


---

# In response to:

- Use the Request for Positions Ack message
- Request for Positions
- Use the Collateral Assignment message
- Collateral Request
- Use the Collateral Response message
- Collateral Assignment
- Use the Collateral Inquiry Ack message
- Collateral Inquiry

Note the only exceptions to this rule are:

1. in the event a business message is received, fulfills session-level rules, however, the message cannot be communicated to the business-level processing system. In this situation a Business Message Reject with BusinessRejectReason = “Application not available at this time” can be issued if the system is unable to send the specific “reject” message listed above due to this condition.
2. in the event a valid business message is received, fulfills session-level rules, however, the message type is not supported by the recipient. In this situation a Business Message Reject with BusinessRejectReason = “Unsupported Message Type” can be issued if the system is unable to send the specific “reject” message listed above because the receiving system cannot generate the related “reject” message.
3. In the event a business message is received, fulfills session-level rules, but lacks a field conditionally required by the FIX specification. In this situation a Business Message Reject with BusinessRejectReason = “Conditionally Required Field Missing” can be issued if the system is unable to send the specific “reject” message listed above. One example of this would be a stop order missing StopPx. However, a Business Message Reject message MUST NOT be used to enforce proprietary rules more restrictive than those explicit in the FIX specification, such as a broker requiring an order to contain an Account, which the FIX specification considers an optional field.

Messages which can be referenced via the Business Message Reject message are:

- (the “ID” field BusinessRejectRefID refers to noted in [ ])
- Indication of Interest (IOI) [IOIid]
- Advertisement [AdvId]
- News [Headline]
- Email [EmailThreadID]
- Order Cancel Reject [ClOrdID]
- Allocation Instruction ACK [AllocID]
- Allocation Report ACK [AllocID]
- List Status [ListID]
- Don’t Know Trade (DK) – may respond with Order Cancel Reject if attempting to cancel order [ExecID]
- Settlement Instructions [SettlInstID]
- Market Data-Snapshot/Full Refresh [MDReqID]
- Market Data-Incremental Refresh [MDReqID]
- Market Data Request Reject [MDReqID]
- Security Definition [SecurityResponseID]


---

# Business Message Reject

- Security Status [SecurityStatusReqID]
- Trading Session Status [TradSesReqID]
- Order Mass Cancel Report [OrderID]
- Security Types [SecurityResponseID]
- Security List [SecurityResponseID]
- Derivative Security List [SecurityResponseID]
- Quote Request Reject [QuoteReqID]
- RFQ Request [RFQReqID]
- Quote Status Report [QuoteID]
- Registration Instructions Response [RegistID]
- Trade Capture Report [TradeReportID]
- Confirmation ACK [ConfirmID]
- Bid Response [BidID]
- List Strike Price [ListID]
- Settlement Instructions [SettInstMsgID]
- Trade Capture Report Request Ack [TradeRequestID]
- Trade Capture Report Ack [TradeReportID]
- Position Maintenance Report [PosMaintRptID]
- Request for Positions Ack [PosMaintRptID]
- Positions Report [PosMaintRptID]
- Assignment Report [AsgnRptID]
- Collateral Response [CollRespID]
- Collateral Inquiry Ack [CollInquiryID]

# Scenarios for Business Message Reject:

| BusinessRejectReason |                                                                     |
| -------------------- | ------------------------------------------------------------------- |
| 0                    | Other                                                               |
| 1                    | Unknown ID                                                          |
| 2                    | Unknown Security                                                    |
| 3                    | Unsupported Message Type (receive a valid, but unsupported MsgType) |
| 4                    | Application not available                                           |
| 5                    | Conditionally Required Field Missing                                |

Whenever possible, it is strongly recommended that the cause of the failure be described in the Text field (e.g. “UNKNOWN SYMBOL: XYZ”).

# Business Message Reject Format:

| Tag | Field Name      | Req'd | Comments                                         |
| --- | --------------- | ----- | ------------------------------------------------ |
|     | Standard Header | Y     | MsgType = j (lowercase)                          |
| 45  | RefSeqNum       | N     | MsgSeqNum of rejected message                    |
| 372 | RefMsgType      | Y     | The MsgType of the FIX message being referenced. |


---


# 379

BusinessRejectRefID

N

The value of the business-level “ID” field on the message being referenced. Required unless the corresponding ID field (see list above) was not specified.

# 380

BusinessRejectReason

Y

Code to identify reason for a Business Message Reject message.

# 58

Text

N

Where possible, message to explain reason for rejection.

# 354

EncodedTextLen

N

Must be set if EncodedText field is specified and must immediately precede it.

# 355

EncodedText

N

Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.

# Standard Trailer

Y

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element BizMsgRej


---
# Network Status Messages

It is envisaged these messages will be used in two scenarios

# Scenario A

Allow one counterparty using a “hub and spoke” FIX network to know whether another counterparty is currently connected to the hub.

# Scenario B

Allow a counterparty connecting to a global brokerage to know which regions within that brokerage are currently available as order routing destinations.

# Network (Counterparty System) Status Request Message

This message is send either immediately after logging on to inform a network (counterparty system) of the type of updates required or to at any other time in the FIX conversation to change the nature of the types of status updates required. It can also be used with a NetworkRequestType of Snapshot to request a one-off report of the status of a network (or counterparty) system. Finally this message can also be used to cancel a request to receive updates into the status of the counterparties on a network by sending a NetworkRequestStatusMessage with a NetworkRequestType of StopSubscribing.

# Network (Counterparty System) Status Request

| Tag                                                                                                                                                                                                                              | Field Name         | Req'd | Comments                                                                                            |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------ | ----- | --------------------------------------------------------------------------------------------------- |
|                                                                                                                                                                                                                                  | Standard Header    | Y     | MsgType = “BC”                                                                                      |
| 935                                                                                                                                                                                                                              | NetworkRequestType | Y     |                                                                                                     |
| 933                                                                                                                                                                                                                              | NetworkRequestID   | Y     |                                                                                                     |
| 936                                                                                                                                                                                                                              | NoCompIDs          | N     | Used to restrict updates/request to a list of specific CompID/SubID/LocationID/DeskID combinations. |
| If not present request applies to all applicable available counterparties. EG Unless one sell side broker was a customer of another you would not expect to see information about other brokers, similarly one fund manager etc. |                    |       |                                                                                                     |
| 930                                                                                                                                                                                                                              | RefCompID          | N     | Used to restrict updates/request to specific CompID                                                 |
| 931                                                                                                                                                                                                                              | RefSubID           | N     | Used to restrict updates/request to specific SubID                                                  |
| 283                                                                                                                                                                                                                              | LocationID         | N     | Used to restrict updates/request to specific LocationID                                             |
| 284                                                                                                                                                                                                                              | DeskID             | N     | Used to restrict updates/request to specific DeskID                                                 |
|                                                                                                                                                                                                                                  | Standard Trailer   | Y     |                                                                                                     |

FIXML Definition for this Message– see http://www.fixprotocol.org for details
---
# Refer to the FIXML element NtwkSysStatReq
---

# Network (Counterparty System) Status Response Message

This message is sent in response to a Network (Counterparty System) Status Request Message. If the network response payload is larger than the maximum permitted message size for that FIX conversation the response would be several Network Status Response Messages the first with a status of full and then as many messages, as updates to the first message, adding information as required.

# Network (Counterparty System) Status Response

| Tag | Field Name                | Req'd | Comments                                                             |
| --- | ------------------------- | ----- | -------------------------------------------------------------------- |
|     | Standard Header           | Y     | MsgType = “BD”                                                       |
| 937 | NetworkStatusResponseType | Y     |                                                                      |
| 933 | NetworkRequestID          | N     |                                                                      |
| 932 | NetworkResponseID         | Y     |                                                                      |
| 934 | LastNetworkResponseID     | N     | Required when NetworkStatusResponseType=2 ~~Required on~~ ~~Type 2~~ |
| 936 | NoCompIDs                 | Y     | Specifies the number of repeating CompId’s                           |
| 930 | RefCompID                 | N     | CompID that status is being report for. Required if NoCompIDs > 0,   |
| 931 | RefSubID                  | N     | SubID that status is being report for.                               |
| 283 | LocationID                | N     | LocationID that status is being report for.                          |
| 284 | DeskID                    | N     | DeskID that status is being report for.                              |
| 928 | StatusValue               | N     |                                                                      |
| 929 | StatusText                | N     | Additional Information, i.e. “National Holiday”                      |
|     | Standard Trailer          | Y     |                                                                      |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element NtwkSysStatRsp


---
User Administration Messages

The messages are provided in FIX to allow the passing of individual user information between two counterparties. The messages allow for the following function:

1. Individual User Logon
2. Individual User Status Enquiries
3. Individual User Logout
4. Individual User password change

NOTE WELL, it is not encouraged to transmit passwords in a FIX conversation unless you can guarantee the end to end security of both the FIX conversation and any intermediate routing hubs that are involved in the routing.

# User Request Message

This message is used to initiate a user action, logon, logout or password change. It can also be used to request a report on a user’s status.

| Tag | Field Name       | Req'd | Comments                                              |
| --- | ---------------- | ----- | ----------------------------------------------------- |
|     | Standard Header  | Y     | MsgType = “BE”                                        |
| 923 | UserRequestID    | Y     |                                                       |
| 924 | UserRequestType  | Y     |                                                       |
| 553 | Username         | Y     |                                                       |
| 554 | Password         | N     |                                                       |
| 925 | NewPassword      | N     |                                                       |
| 95  | RawDataLength    | N     |                                                       |
| 96  | RawData          | N     | Can be used to hand structures etc to other API’s etc |
|     | Standard Trailer | Y     |                                                       |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element UserReq


---

# User Response Message

This message is used to respond to a user request message, it reports the status of the user after the completion of any action requested in the user request message.

# User Response

| Tag | Field Name       | Req'd | Comments                             |
| --- | ---------------- | ----- | ------------------------------------ |
|     | Standard Header  | Y     | MsgType = “BF”                       |
| 923 | UserRequestID    | Y     |                                      |
| 553 | Usern~~N~~ame    | Y     |                                      |
| 926 | UserStatus       | N     |                                      |
| 927 | UserStatusText   | N     | Reason a request was not carried out |
|     | Standard Trailer | Y     |                                      |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element UserRsp


---
Glossary
# Business Terms

The following glossary is an attempt to identify business terms used in this document or related to implementing FIX globally. Requests for new terms and/or suggested definitions should be posted in the FIX Web Site’s Discussion section.

| Term                  | Definition                                                                                                                                                                                                                                                                                                                               | Field where used       |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| Accrued Interest Rate | The amount the buyer compensates the seller for the portion of the next coupon interest payment the seller has earned but will not receive from the issuer because the issuer will send the next coupon payment to the buyer. Accrued Interest Rate is the annualized Accrued Interest amount divided by the purchase price of the bond. |                        |
| After Tax Yield       | Municipals. The yield on the bond net of any tax consequences from holding the bond. The discount on municipal securities can be subject to both capital gains taxes and ordinary income taxes. Calculated from dollar price.                                                                                                            | \[YieldType]           |
| All or None           | A round-lot market or limit-price order that must be executed in its entirety or not at all; unlike Fill or Kill orders, AON orders are not treated as canceled if they are not executed as soon as represented in the Trading Crowd.                                                                                                    | \[ExecInst]            |
| Annual Yield          | The annual interest or dividend income an investment earns, expressed as a percentage of the investment’s total value.                                                                                                                                                                                                                   | \[YieldType]           |
| As defined            | Sides of the legs are the same as defined in the multileg instrument.                                                                                                                                                                                                                                                                    | \[Side]                |
| At the close          | Indicated price is to be around the closing price, however, not held to the closing price.                                                                                                                                                                                                                                               | \[IOIQualifier]        |
| At the Opening        | A market or limit-price order to be executed at the opening of the stock or not at all; all or part of any order not executed at the opening is treated as canceled.                                                                                                                                                                     | \[TimeInForce]         |
| Basis Price           | A price established by joint agreement of odd-lot dealers in 100-share-unit stocks when: - no round-lot has occurred during the trading session, - the spread between the closing bid and offer is two points or more, and - on odd-lot the dealer has been given a “basis-price” order.                                                 | \[OrdType]             |
| Book Yield            | The yield of a security calculated by using its book value instead of the current market price. This term is typically used in the US domestic market.                                                                                                                                                                                   | \[YieldType]           |
| Broker Execution      | According to US futures markets (CFTC): Time at which a broker executed the order for another broker.                                                                                                                                                                                                                                    | \[TrdRegTimestampType] |
| Broker of Credit      | Broker to receive trade credit.                                                                                                                                                                                                                                                                                                          | \[PartyRole]           |
| Broker Receipt        | According to US futures markets (CFTC):                                                                                                                                                                                                                                                                                                  | \[TrdRegTimestamp]     |


---

# Time at which broker received the order.

# Buy Minus

A round-lot market order to buy “minus” is an order to buy a stated amount of a stock provided that its price is:

- not higher than the last sale if the last sale was a “minus” or “zero minus” tick and
- not higher than the last sale minus the minimum fractional change in the stock if the last sale was a “plus” or “zero plus” tick.

A limit price order to buy “minus” also states the highest price at which it can be executed.

# Cabinet Trade

An off-market transaction to close out a nearly worthless out-of-the-money option contract.

# Call Date

The date on which the issuer of a security has the right but not the obligation to repurchase the security at a predetermined price.

# Call First

Refer to client before trading.

# Cancel if Not Best

Indicates that an order should be cancelled if it is no longer the best bid if buying, or the best offer if selling. If the order is cancelled due to this instruction, the message cancelling it must carry ExecRestatementReason="Canceled, Not Best".

# Cancel on System Failure

If a system failure interrupts trading or order routing, attempt to cancel this order. Note that depending on the type and severity of the failure, this might not be possible.

# Cancel on Trading Halt

If trading in this instrument is halted, cancel this order and do not reinstate it when/if trading resumes.

# CIV ("Collective Investment Vehicle")

Collective investment vehicle ("CIV") are set up for the purposes of collecting and pooling investor funds and issuing shares (or their equivalent). "Open-ended" CIVs entitle the holder to receive, on demand, an amount in value which is proportionate to the whole net asset value of the vehicle. Conversely "Closed-ended" CIVs do not grant this right to investors.

CIVs are more commonly known as Mutual Funds, Unit Trusts, OEICS (Open Ended Investment Companies), SICAVs etc.

A CIV may be legally constituted as a Limited Company with variable capital, a Trust or a Limited Partnership - depending on local legislation &#x26; tax regimes.

CIVs typically invest in equities, bonds, derivatives etc. - as described in their prospectus. Other CIVs are Umbrella Fund (made up of sub-funds investing in equities, gilts etc), Fund Of Funds (invests only in other funds), Master-Feeder Fund (marketed to a specific group for investment in a central master fund), Multi-Manager Fund (whose asset management is divided between several managers), Side By Side (onshore and offshore funds with the same investment strategy).

# Clearing Firm

Firm that will clear the trade. Used if different from the executing firm.

# Clearing Organization

Identifies the Clearing Organization where the position is maintained.


---

# Client ID

Firm identifier used in third party-transactions or for investor identification in intermediary transactions. (should not be a substitute for OnBehalfOfCompID/DeliverToCompID).

# Closing Yield

The yield of a bond based on the closing price.

# Closing Yield Most Recent Month

The yield of a bond based on the closing price as of the most recent month's end.

# Closing Yield Most Recent Quarter

The yield of a bond based on the closing price as of the most recent quarter’s end.

# Closing Yield Most Recent Year

The yield of a bond based on the closing price as of the most recent year’s end.

# Compound Yield

The yield of certain Japanese bonds based on its price. Certain Japanese bonds have irregular first or last coupons, and the yield is calculated compound for these irregular periods.

# Contra Firm

The broker or other firm which is the contra side of the trade.

# Contra Clearing Firm

Clearing firm of the broker or other firm which is the contra side of the trade.

# Contra Trader

Individual usually identified by a trading badge number or initials that takes the opposite side of a trade.

# Contract For Difference (CFD)

A single stock total return swap, combining financing and synthetic equity exposure in one transaction.

# Correspondent Broker

Identifies a correspondent broker.

# Correspondent Clearing Firm

ClearingFirm that is going to carry the position on their books at another clearing house (exchanges).

# Correspondent Clearing Organization

Identifies a correspondent clearing organization.

# Coupon Rate

The rate of interest that, when multiplied by the principal, par value, or face value of a bond, provides the currency amount of the periodic interest payment. The coupon is always cited, along with maturity, in any quotation of a bond's price.

# Cross

Client sends Broker a buy or sell order. Broker wishes to take the other side and cross with the client. Broker sends an order with Side=Cross to an exchange.

# Cross Short

Client wants to establish a short position, and so sends a Sell Short to Broker. Broker wants to cross with the Client, so Broker sends a Cross Short order to an exchange. Cross Short is crucial here because many exchanges have tick rules needing to be enforced, and the order getting converted from Sell Short to Cross (instead of Cross Short) could result in an illegal short sell.

# Cross Short Exempt

Client wants to establish a short position, and is exempt from the uptick restriction. So Client sends Sell Short Exempt to Broker. Broker wants to cross with the Client, so Broker needs a way to send "Cross Short Exempt" to the exchange so that an audit trail traces back indicating that the party selling short was exempt from the uptick rule.

# Current Yield

Annual interest on a bond divided by the market value. The actual yield type.


---

# Income Rate of Return

Income rate of return as opposed to the coupon rate expressed as a percentage.

# Customer Account

Identifies the customer account associated with the message. [PartyRole]

# Dated Date

The effective date of a new securities issue determined by its underwriters. Often but not always the same as the "Issue Date" and the "Interest Accrual Date".

# Day Order

A buy or sell order that, if not executed expires at the end of the trading day on which it was entered. [TimeInForce]

# Discount

When a bond sells below its par value, it is said to be selling at a discount. A price with a PriceType of "discount" is the difference between 100 and the bond's percent-of-par price. [PriceType]

# Do Not Increase

A limit order to buy, a stop order to sell, or a stop-limit order to sell which is not to be increased in shares on the ex-dividend date as a result of a stock dividend or distribution. [ExecInst]

# Do Not Reduce

A limit order to buy, a stop order to sell, or a stop-limit order to sell that is not to be reduced in price by the amount of an ordinary cash dividend on the ex-dividend date. A do-not-reduce order applies only to ordinary cash dividends; it should be reduced for other distributions - such as when a stock goes “ex” stock dividend or “ex” rights. [ExecInst]

# Dollar Price

See "Percent of Par" [PriceType]

# Entering Firm

Broker who has recorded or reported an execution. This field is particularly useful where the trade is entered into a trade recording system by a broker who is not a party to the trade, as it allows any inquiries or problem resolution to be directed to the appropriate source. [PartyRole]

# Entering Trader

Individual usually identified by a trading badge number or initials that actually enters an order to a market (especially in open outcry markets). Usually the Entering Trader is the same as the Executing Trader. However, under some circumstances the Entering Trader will have the trade executed by another trader who is then identified as the Executing Trader. [PartyRole]

# Exchange

Exchange associated with the position. [PartyRole]

# Execution Time

According to US futures markets (CFTC): Non-qualified reporting time of order execution. [TrdRegTimestampType]

# Executing Firm

Identifies executing / give-up broker. [PartyRole]

# Executing System

System Identifier where execution took place (e.g. some markets have multiple execution location such as an electronic book or automatic execution system). [PartyRole]

# Executing Trader

Trader or broker id associated with Executing Firm who actually executes the trade. [PartyRole]

# Fill or Kill

A market or limit-price order that is to be executed in its entirety as soon as it is represented in the Trading Crowd; if not so executed, the order is to be canceled. Not to be confused with Immediate or Cancel. [TimeInForce]


---

# FIX Connection

A FIX Connection is comprised of three parts: logon, message exchange, and logout.

# FIX Session

A FIX Session is comprised of one or more FIX Connections, meaning that a FIX Session spans multiple logins.

# Fixed Price Cabinet Trade

Cabinet Trade executed at a price equal to the minimum tick size (or smallest possible price). See "Cabinet Trade".

# Floating Price Cabinet Trade

Cabinet Trade executed at a price that can be different than the minimal price. See "Cabinet Trade".

# Forex - Swap

A "Swap" order for Foreign Exchange (currency trading).

# Funari

Japanese term for an order to buy or sell a stated amount of a security at the specified limit price with any unexecuted (leftover) quantity becoming a Market On Close order.

# Fund manager Client ID

For CIV: An identifier for an Investor or a broker or funds supermarket’s nominee/custodian company which is recognized by the Fund manager.

# Giveup Clearing Firm

Firm to which the trade is given up (carries the position that results from a trade).

# Good Till Canceled

An order to buy or sell that remains in effect until it is either executed or canceled; sometimes called an “open order”.

# Government Equivalent Yield

Ask yield based on semi-annual coupons compounding in all periods and actual/actual calendar.

# Held

The firm executing the order is held to best execution requirements, and may not make discretionary decisions. Opposite of Not Held.

# Ignore Price Validity Checks

Disables validity checking of price fields for an order or change request.

# Immediate or Cancel

A market or limit-price order that is to be executed in whole or in part as soon as it is represented in the Trading Crowd; any portion not so executed is to be canceled. Not to be confused with Fill or Kill.

# Initiator

An “initiator” may be one of the following:

- an institutional client
- a financial planner
- a retail broker representing a retail customer
- a broker/dealer
- an inter-dealer broker (or broker’s broker)
- an issuer

# Institutions Only

Broker is restricted to dealing with other buy side firms.

# Interest Accrual Date

The start date used for calculating accrued interest on debt instruments which are being sold between interest payment dates. Often but not always the same as the "Issue Date" and the "Dated Date".

# Introducing Firm

The broker or other intermediary with the closest association with the investor.

# Inverse Floater Bond

Inverse floater semi-annual bond equivalent rate.


---

# Yield

Investor ID

- For Equities: [PartyRole]
- Identifies beneficiary or broker acting on behalf of beneficiary.
- This field is mandatory for various exchanges either pre or post trade.
- Numerical entry containing no dashes.

- For CIV: [PartyRole]
- An Investor identifier such as a taxpayer reference (NINO, NPN, DSS, SSN number etc) for an individual investor or a registration number (EIN, etc.) for a company.
- May contain alphanumeric and dashes.

Issue Date

The date on which a bond or stock offering is issued. It may or may not be the same as the effective date ("Dated Date") or the date on which interest begins to accrue ("Interest Accrual Date").

Limit

An order to buy a security at or below a stated price, or to sell a security at or above a stated price. [OrdType]

Limit or Better

Indicates an order to [OrdType]

- buy a security at the indicated limit price or lower, or to
- sell a security at the indicated limit price or higher.

Limit With or Without

An order to be executed at a limit price, with or without round-lot sales; valid only for odd lot orders. [OrdType]

Liquidity Provider

Identifies the individual that provided liquidity, e.g. was the market maker (specialist) involved in a trade. Used to identify the liquidity provider involved in a block of EFP trade for listed futures markets. [PartyRole]

Locate/Lending Firm

Identity of the firm which is loaning the security in a short sale. [PartyRole]

Marked To Market Yield

An adjustment in the valuation of a securities portfolio to reflect [YieldType]

the current market values of the respective securities in the portfolio.

Market

Indicates an order to buy or sell a stated amount of a security at the [OrdType]

most advantageous price obtainable after the order is represented in the Trading Crowd.

Market If Touched

Indicates an order to buy or sell a stated amount of a security or [OrdType]

commodity as soon as a preset market price is reached, at which point it becomes a Market order.

Market On Close

Indicated price is held to the closing price ("firm" instruction). [IOIQualifier]

Market Or Better

Indicates an order to buy or sell a stated amount of a security at the [OrdType]

quoted market or better.

Market with Leftover as Limit

Indicates an order to buy or sell a stated amount of a security at the [OrdType]

prevailing market price with any unexecuted (leftover) quantity becoming a Limit order at the last executed price.

Most Recent Closing Yield

The last available yield stored in history, computed using price. [YieldType]


---

# Fund Valuation

For CIV orders - indicates that the Investor wishes the order to be dealt at the unit price determined at the next Valuation Point, a.k.a. a Forward price.

# No Cross

The broker executing this trade is forbidden from taking the other side of the trade. Opposite of OK to Cross.

# Not Held

The firm executing the order is not held to best execution requirements, and may be able to make some discretionary decisions. Opposite of Held.

# OK to Cross

The broker executing this trade is allowed to take the other side of the trade. Opposite of No Cross.

# Omnibus Account

An account where the positions for multiple entities (usually customers) are maintained. The omnibus accounting is usually done on a gross basis where long and short positions are not netted together.

# On Basis

An order to buy or sell at the basis price. The basis price is established by joint agreement of odd lot dealers in 100 share unit stocks when no round lot sale has occurred during the trading session, the spread between the closing bid and offer is two points or more, and an odd lot dealer has been given a basis price order. (e.g. NYSE order type)

# Opposite

Sides of the legs are the opposite of their definition in the multileg instrument.

# Order Origination Firm

Buyside firm associated with Order Origination Firm which originates/submits the order.

# Order Origination Trader

Buyside trader id associated with Order Origination Firm which originates/submits the order.

# Par

Equal to the face value (nominal) of a security, i.e. A bond selling at par is worth an equivalent to its original issue value, typically $1000/bond.

# Participate Don’t Initiate

An order that may participate in a transaction initiated by another party, but may not initiate a transaction. For example, on US ECNs / Exchanges, this may represent an order that will be quoted to the marketplace and will trade if another party initiates a trade (i.e. hits the posted quote), but cannot be routed to initiate a trade with another market or market maker.

# Per Unit

The currency price per unit, i.e. per equity share or per contract.

# Percent of Par

The ratio between the current price of a bond and its par value adjusted for amortization or indexing and expressed as a percent. For example if a EUR1,000 bond is trading at EUR1032.50 its price is expressed as 103.25 or 103¼. In the US this is usually referred to as the "dollar price" even in scholarly material and handbooks.

# Position Account

Account for positions resulting from derivatives trades. Each position account has a long and short quantity. Position quantities stored in the long and short quantity fields can be kept net or gross. Accounts that are kept gross are usually omnibus accounts.

# Percent of Volume

The sender does not want to be all of the volume on the floor.


---

# Premium

When a bond sells above its par value, it is said to be selling at a [PriceType] premium. A price with a PriceType of "premium" is the difference between the bond's percent-of-par price and 100.

# Previous Fund Valuation Point

For CIV orders - indicates that the Investor prefers that the order be dealt at the unit price determined at the immediately preceding Valuation Point, a.k.a. a Historic price. (This can be overridden by the constitution of the fund or, in certain circumstances, by the Fund Manager.)

# Open Average Yield

The average yield of the respective securities in the portfolio. [YieldType]

# Order Originator

ID of the party entering the trade into the system (data entry, userid, buy side trader, etc.). [PartyRole]

# Put Date

The date on which the buyer of a security has the right but not the obligation to sell the security back to the issuer at a predetermined price. [EventType]

# Previous Close Yield

The yield of a bond based on the closing price 1 day ago. [YieldType]

# Previously indicated

An order sent in response to an Indication of Interest message. [OrdType]

# Previously quoted

An order sent in response to a Quote message. [OrdType]

# Proceeds Yield

The CD equivalent yield when the remaining time to maturity is less than two years. [YieldType]

# Redeem

For CIV: [Side]

A “sell” order for CIV units which must be forwarded to the fund manager (or their transfer agent) rather than being matched / crossed with a “buy” order, e.g. by an intermediary, funds supermarket, broker/dealer etc. This would be used in markets where the originator requires specific tax treatment and/or dealing charges.

# Reinstate on System Failure

If a system failure interrupts trading or order routing, attempt to reinstate this order, subject to time in force limitations. Note that depending on the type and severity of the failure, this might not be possible. [ExecInst]

# Reinstate on Trading Halt

If trading in this instrument is halted, reinstate this order when/if trading resumes, subject to time in force limitations. [ExecInst]

# Request to Intermediary

Used in a model where an intermediary, i.e. clearing house is involved in communicating allocation details and actions between two parties [AllocType]

# Respondent

A “respondent” may be one of the following: Quoting and other messages

- a broker/dealer
- an inter-dealer broker (or broker’s broker)
- an electronic service
- bid or offer prices provided by one or more market makers
- bid or offer prices provided by an inter-dealer broker
- matching system with limit orders entered by customers (dealers or institutions)
- an issuer

# Riskless Principal

"Riskless" principal transactions are generally described as trades in which, after receiving an order to buy (or sell) from a customer, [OrderCapacity]


---

# Definitions

# Broker-Dealer Transactions

The broker-dealer purchases (or sells) the security from (or to) another person in a contemporaneous offsetting transaction.

Above from the SEC web-site http://www.sec.gov/rules/final/34-44291.htm

See Exchange Act Rule 10b-10(a)(2)(ii)(A) [17 CFR 240. 10b-10(a)(2)(ii)(A)]; Exchange Act Rel. No. 33743 (Mar. 9, 1994) at n.11.

# Order Types

# Sell Plus

A round-lot market order to sell “plus” is an order to sell a stated amount of a stock provided that its price is:

- not lower than the last sale if the last sale was a “plus” or “zero plus” tick and
- not lower than the last sale minus the minimum fractional change in the stock if the last sale was a “minus” or “zero minus” tick.

A limit-price order to sell “plus” also states the lowest price at which it can be executed.

# Sell Short

An order to sell a security that the seller does not own; a sale effected by delivering a security borrowed by, or for the account of, the seller. Can only be executed on a “plus” or “zero plus” tick.

# Sell Short Exempt

Short sale exempt from short-sale rules.

# Semi-annual Yield

The yield of a bond whose coupon payments are reinvested semi-annually.

# Settlement Location

Identifies Settlement Depository or, if local settlement, the ISO Country Code.

# Sponsoring Firm

A member of the exchange that is sponsoring an Entering Entity to send orders to the exchange. The Sponsoring Member Firm permits sponsorees (i.e. Entering Entities) to trade thereby allowing them to enter orders directly to the exchange via automated means. (e.g. NYSE allowing direct access via Anonymous DOT service).

# Spread

A "spread" price is one of four things all denominated in basis points:

1. For an outright security trade, the "spread" price is the difference in yield between the object security and its benchmark - implied or explicit.
2. For a swap (or switch) of two issued securities the "spread" price is the difference in yield between the security being sold and the one being bought.
3. For a roll of a futures contract with a contract in the same commodity but having a different contract settlement month the "spread" price is the price difference between the contract being sold and the one being bought.
4. For a floating-rate Financing transaction the “spread” is the difference in yield extended above or below the yield of the stated benchmark.

All four types are expressed in basis points (the price or yield difference times 100) and may be negative.


---

# Order Types and Definitions

# Stop

A stop order to buy which becomes a market order when the security trades at - or above - the stop price after the order is represented in the Trading Crowd. A stop order to sell which becomes a market order when the security trades at - or below - the stop price after the order is represented in the Trading Crowd.

# Stop Limit

A stop order to buy which becomes a limit order at the limit price when the security trades at - or above - the stop price after the order is represented in the Trading Crowd. A stop order to sell which becomes a limit order at the limit price when the security trades at - or below - the stop price after the order is represented in the Trading Crowd.

# Stopped

A trade is guaranteed for the order, usually at a stated price or better, but has not yet occurred. For example, a specialist on an exchange may "stop" an order while searching for a better price.

# Streetside Trade Capture Reporting

Reporting of completed trades for clearance and settlement or compliance purposes. Reports may be originated by Exchanges or by clearing firms and sent to clearing firms directly or via a clearing corporation or central counterparty such as DTCC in the US.

# Simple Yield

The yield of a bond assuming no reinvestment of coupon payments. (Act/360 day count)

# Strict Limit (No Price Improvement)

A limit order that must be traded at the exact limit price specified without any price improvement. Requires OrdType=Limit.

# Subscribe

For CIV: A “buy” order for CIV units which must be forwarded to the fund manager (or their transfer agent) rather than being matched / crossed with a “sell” order, e.g. by an intermediary funds supermarket, broker/dealer etc. This would be used in markets where the originator requires specific tax treatment and/or dealing charges.

# Suspended

The order is not eligible for trading. This usually happens as a result of a verbal or otherwise out of band request to suspend the order, or because the order was submitted, or modified via a Cancel/Replace Request, with ExecInst=Suspended.

# Tax Equivalent Yield

The after tax yield grossed up by the maximum federal tax rate of 39.6%. For comparison to taxable yields.

# TED Price

The price spread between the active 3 month treasury bill futures contract and the 3 month Eurodollar futures contract. Used as an indicator of investor confidence in the U.S. markets.

# TED Yield

The difference in basis points between the yield-to-maturity of the bond / note and the yield-to-maturity of a Hypothetical Euromarket bond with identical coupon and maturity.

# Time In

According to US futures markets (CFTC): Timestamp of when order was received on the trading floor (booth).

# Time Out

According to US futures markets (CFTC):


---

# Trade Terms and Definitions

Timestamp when the trade was received from the pit.

| **Trade Along**                       | Clients who specify "Trade Along" give brokers permission to handle and place their order in the market even if the broker already has its own proprietary orders for the same security placed in the market.                                                                                                                             | \[ExecInst]      |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| **Trailing Stop Peg**                 | A pegged order representing a stop order with a stop price pegged to trail a specified distance behind the last sale price. The price of a trailing stop to buy can never increase, and the price of a trailing stop to sell can never decrease.                                                                                          | \[ExecInst]      |
| **True Gross Yield**                  | Yield calculated using the price including accrued interest, where coupon dates are moved from holidays and weekends to the next trading day.                                                                                                                                                                                             | \[YieldType]     |
| **True Yield**                        | The yield calculated with coupon dates moved from a weekend or holiday to the next valid settlement date.                                                                                                                                                                                                                                 | \[YieldType]     |
| **Try to Stop**                       | Used in specialist-driven markets to direct the specialist to try and stop the order.                                                                                                                                                                                                                                                     | \[ExecInst]      |
| **Underlying Contra Firm**            | The broker or other firm which is the contra side of the trade for the underlying security.                                                                                                                                                                                                                                               | \[PartyRole]     |
| **URI (Uniform Resource Identifier)** | W3C standard defined as "the generic set of all names/addresses that are short strings that refer to resources". Note that "URL" (Uniform Resource Locator), commonly referred to by web browsers, is a subset of the URI standard. The W3C standards body considers URL an "informal term (no longer used in technical specifications)". | See Appendix 6-B |
| **With or Without**                   | An odd lot order filled on an effective round lot transaction, or on an effective bid or offer, whichever occurs first after the specialist receives the order. (e.g. NYSE order type)                                                                                                                                                    | \[OrdType]       |
| **Yield At Issue**                    | Municipals. The yield of the bond offered on the issue date.                                                                                                                                                                                                                                                                              | \[YieldType]     |
| **Yield Change Since Close**          | The change in the yield since the previous day's closing yield.                                                                                                                                                                                                                                                                           | \[YieldType]     |
| **Yield To Average Maturity**         | The yield achieved by substituting a bond's average maturity for the issue's final maturity date.                                                                                                                                                                                                                                         | \[YieldType]     |
| **Yield To Next Call**                | The yield of a bond to the next possible call date.                                                                                                                                                                                                                                                                                       | \[YieldType]     |
| **Yield To Longest Average Life**     | The yield assuming only mandatory sinks are taken. This results in a lower paydown of debt; the yield is then calculated to the final payment date.                                                                                                                                                                                       | \[YieldType]     |
| **Yield To Maturity**                 | The yield of a bond to its maturity date.                                                                                                                                                                                                                                                                                                 | \[YieldType]     |
| **Yield To Next Put**                 | The yield to the date at which the bond holder can next put the bond to the issuer.                                                                                                                                                                                                                                                       | \[YieldType]     |
| **Yield To Next Refund**              | Sinking Fund Bonds. Yield assuming all bonds are redeemed at the next refund date at the redemption price.                                                                                                                                                                                                                                | \[YieldType]     |
| **Yield To Shortest Average Life**    | The yield assuming that all sinks (mandatory and voluntary) are taken at par. This results in a faster paydown of debt; the yield is then calculated to the final payment date.                                                                                                                                                           | \[YieldType]     |
| **Yield To Tender Date**              | The yield on a Municipal bond to its mandatory tender date.                                                                                                                                                                                                                                                                               | \[YieldType]     |


---
# Yield To Worst

# Yield Value of 1/32

# Yield with Inflation Assumption

The lowest yield to all possible redemption date scenarios. [YieldType]

The amount that the yield will change for a 1/32ⁿᵈ change in price. [YieldType]

Based on price, the return an investor would require on a normal [YieldType] bond that would make the real return equal to that of the inflation-indexed bond, assuming a constant inflation rate.
---
Appendix 1-A

# Abbreviations used within FIXML

| Acrl    | Accrual               |
| ------- | --------------------- |
| Acct    | Account               |
| Ack     | Acknowledgement       |
| Acrd    | Accrued               |
| Actn    | Action                |
| Adj     | Adjust                |
| Adjmt   | Adjustment            |
| Adv     | Advertisement         |
| Alloc   | Allocation            |
| Amt     | Amount                |
| AOS     | AllowableOneSidedness |
| Asgn    | Assignment            |
| Avg     | Average               |
| Bhf     | Behalf                |
| Bkng    | Booking               |
| Bnchmk  | Benchmark             |
| Brkr    | Broker                |
| Brkrs   | Brokers               |
| Biz     | Business              |
| Calc    | Calculation           |
| Capt    | Capture               |
| Ccy     | Currency              |
| Cl      | Client                |
| Cls     | Close                 |
| Cmn     | Common                |
| Cnfm    | Confirmation          |
|         | Confirm               |
| Cntra   | Contra                |
| Coll    | Collateral            |
| Comm    | Commission            |
| Comp    | Company               |
| Corp    | Corporate             |
| Cpcty   | Capacity              |
| Cpn     | Coupon                |
| Crss    | Cross                 |
| Crv     | Curve                 |
| Csh     | Cash                  |
| Ctry    | Country               |
| Cum     | Cumulative            |
| Cxl     | Cancel                |
| Data    | Data                  |
| Db      | Database              |
| Del     | Delete                |
| Desc    | Description           |
| Dest    | Destination           |
| Dev     | Device                |
| Disc    | Discount              |
| DK      | Don't Know            |
| Dlvr    | Deliver               |
| Dsctn   | Discretion            |
| Dsctnry | Discretionary         |
| Dt      | Date                  |
| Dup     | Duplicate             |
| Efctv   | Effective             |
| EFP     | ExchangeForPhysical   |
| Enc     | Encoded               |
| Err     | Error                 |
| Exct    | Execute               |
| Exch    | Exchange              |
| Exctn   | Execution             |
| Exr     | Exercise              |
| Fctr    | Factor                |
| Fut     | Future                |
| Fwd     | Forward               |


---

# Foreign Currency Abbreviations

| Abbreviation | Meaning                           |
| ------------ | --------------------------------- |
| FX           | Foreign Currency                  |
| Grp          | Group                             |
| GTD          | Good Till Date                    |
| Hndl         | Handling                          |
| ID           | Identifier                        |
| Implct       | Implicit                          |
| Ind          | Indicator                         |
| Info         | Information                       |
| Inpt         | Input                             |
| Inq          | Inquiry                           |
| Instrctn     | Instruction                       |
| Instn        | Institution                       |
| Instrmt      | Instrument                        |
| Int          | Interest                          |
| IOI          | Indication of Interest            |
| Iss          | Issue                             |
| Issr         | Issuer                            |
| Lctn         | Location                          |
| Loc          | Locate                            |
| Lqdty        | Liquidity                         |
| Mat          | Maturity                          |
| Max          | Maximum                           |
| Mgn          | Margin                            |
| Min          | Minimum                           |
| Mkt          | Market                            |
| Mleg         | Multileg                          |
| Mnt          | Maintenance                       |
| Mny          | Money                             |
| Mo           | Month                             |
| Mod          | Modification                      |
| Misc         | Miscellaneous                     |
| Msg          | Message                           |
| Mtch         | Match                             |
| Ndx          | Index                             |
| No           | Number - NumInGroup fields        |
| Nst          | Nested                            |
| Ntwk         | Network                           |
| Num          | Number - multiple reports, counts |
| Ofr          | Offer                             |
| Opt          | Option                            |
| Ord          | Order                             |
| Orig         | Original                          |
| Oth          | Other                             |
| Pct          | Percent                           |
| Pctg         | Percentage                        |
| Pmt          | Payment                           |
| Pos          | Position                          |
| Prod         | Product                           |
| Pri          | Priority                          |
| Prlm         | Preliminary                       |
| Prtztn       | Prioritization                    |
| Prev         | Previous                          |
| Psbl         | Possible                          |
| Pty          | Party                             |
| Pub          | Publish                           |
| Px           | Price                             |
| Qlty         | Quality                           |
| Qty          | Quantity                          |
| Qual         | Qualifier                         |
| Quot         | Quote                             |
| Red          | Redemption                        |
| Ref          | Reference                         |
| Rej          | Reject                            |
| Reltd        | Related                           |
| Repo         | Repurchase                        |
| Req          | Request                           |
| Rgst         | Registration                      |
| Rgstry       | Registry                          |
| Rnd          | Round                             |


---

# Report

| Abbreviation | Full Term      |
| ------------ | -------------- |
| Rpt          | Report         |
| Rpts         | Reports        |
| Rslt         | Result         |
| Rsn          | Reason         |
| Rsp          | Response       |
| Rstct        | Restrict       |
| Rstctn       | Restriction    |
| Rstctns      | Restrictions   |
| Rstmt        | Restatement    |
| Rt           | Rate           |
| Rtng         | Rating         |
| Scnd         | Secondary      |
| Sec          | Security       |
| Seq          | Sequence       |
| Sess         | Session        |
| Settl        | Settlement     |
| Sfx          | Suffix         |
| Shrt         | Short          |
| Snd          | Sender         |
| Sndg         | Sending        |
| Src          | Source         |
| St           | State          |
| Stand        | Standing       |
| Stat         | Status         |
| Stip         | Stipulation    |
| Strk         | Strike         |
| Sub          | Subscription   |
| Subsid       | Subsidiary     |
| Svc          | Service        |
| Sym          | Symbol         |
| Sys          | System         |
| Sz           | Size           |
| Tgt          | Target         |
| Tkt          | Ticket         |
| Tm           | Time           |
| Tot          | Total          |
| Typ          | Type           |
| Trd          | Trade          |
| TrdSes       | TradingSession |
| Trkng        | Tracking       |
| Trm          | Term           |
| Txn          | Transaction    |
| Valu         | Value          |
| Vol          | Volume         |
| Yld          | Yield          |
| Yr           | Year           |
| TS           | Timestamp      |


---
NO_CONTENT_HERE