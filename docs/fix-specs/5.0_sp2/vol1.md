
FINANCIAL INFORMATION
EXCHANGE PROTOCOL
(FIX)


# Version 5.0 Service Pack 2 - Errata

# VOLUME 1 – INTRODUCTION TO THE FIX PROTOCOL

August 18, 2011

© Copyright, 2008-2009 2011, FIX Protocol, Limited



---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 1

# August 18, 2011



# DISCLAIMER

THE INFORMATION CONTAINED HEREIN AND THE FINANCIAL INFORMATION EXCHANGE PROTOCOL (COLLECTIVELY, THE "FIX PROTOCOL") ARE PROVIDED "AS IS" AND NO PERSON OR ENTITY ASSOCIATED WITH THE FIX PROTOCOL MAKES ANY REPRESENTATION OR WARRANTY, EXPRESS OR IMPLIED, AS TO THE FIX PROTOCOL (OR THE RESULTS TO BE OBTAINED BY THE USE THEREOF) OR ANY OTHER MATTER AND EACH SUCH PERSON AND ENTITY SPECIFICALLY DISCLAIMS ANY WARRANTY OF ORIGINALITY, ACCURACY, COMPLETENESS, MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. SUCH PERSONS AND ENTITIES DO NOT WARRANT THAT THE FIX PROTOCOL WILL CONFORM TO ANY DESCRIPTION THEREOF OR BE FREE OF ERRORS. THE ENTIRE RISK OF ANY USE OF THE FIX PROTOCOL IS ASSUMED BY THE USER.

NO PERSON OR ENTITY ASSOCIATED WITH THE FIX PROTOCOL SHALL HAVE ANY LIABILITY FOR DAMAGES OF ANY KIND ARISING IN ANY MANNER OUT OF OR IN CONNECTION WITH ANY USER'S USE OF (OR ANY INABILITY TO USE) THE FIX PROTOCOL, WHETHER DIRECT, INDIRECT, INCIDENTAL, SPECIAL OR CONSEQUENTIAL (INCLUDING, WITHOUT LIMITATION, LOSS OF DATA, LOSS OF USE, CLAIMS OF THIRD PARTIES OR LOST PROFITS OR REVENUES OR OTHER ECONOMIC LOSS), WHETHER IN TORT (INCLUDING NEGLIGENCE AND STRICT LIABILITY), CONTRACT OR OTHERWISE, WHETHER OR NOT ANY SUCH PERSON OR ENTITY HAS BEEN ADVISED OF, OR OTHERWISE MIGHT HAVE ANTICIPATED THE POSSIBILITY OF, SUCH DAMAGES.

No proprietary or ownership interest of any kind is granted with respect to the FIX Protocol (or any rights therein), except as expressly set out in FIX Protocol Limited's Copyright and Acceptable Use Policy.

© Copyright 2003-2011 FIX Protocol Limited, all rights reserved

# REPRODUCTION

FIX Protocol Limited grants permission to print in hard copy form or reproduce the FIX Protocol specification in its entirety provided that the duplicated pages retain the “Copyright FIX Protocol Limited” statement at the bottom of the page.

Portions of the FIX Protocol specification may be extracted or cited in other documents (such as a document which describes one’s implementation of the FIX Protocol) provided that one reference the origin of the FIX Protocol specification (HTUhttp://www.fixprotocol.orgUTH) and that the specification itself is “Copyright FIX Protocol Limited”.

FIX Protocol Limited claims no intellectual property over one’s implementation (programming code) of an application which implements the behavior and details from the FIX Protocol specification.

© Copyright, 2008-2011, FIX Protocol, Limited


Page 2 of 158

---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# PREFACE

The Financial Information eXchange (FIX) effort was initiated in 1992 by a group of institutions and brokers interested in streamlining their trading processes. These firms felt that they, and the industry as a whole, could benefit from efficiencies derived through the electronic communication of indications, orders and executions. The result is FIX, an open message standard controlled by no single entity, that can be structured to match the business requirements of each firm. The benefits are:

- From the business flow perspective, FIX provides institutions, brokers, and other market participants a means of reducing the clutter of unnecessary telephone calls and scraps of paper, and facilitates targeting high quality information to specific individuals.
- For technologists, FIX provides an open standard that leverages the development effort so that they can efficiently create links with a wide range of counter-parties.
- For vendors, FIX provides ready access to the industry, with the incumbent reduction in marketing effort and increase in potential client base.

Openness has been the key to FIX's success. For that reason, while encouraging vendors to participate with the standard, FIX has remained vendor neutral. Similarly, FIX avoids over-standardization. It does not demand a single type of carrier (e.g., it will work with leased lines, frame relay, Internet, etc.), nor a single security protocol. It leaves many of these decisions to the individual firms that are using it. We do expect that, over time, the rules of engagement in these non-standardized areas will converge as technologies mature.

FIX is now used by a variety of firms and vendors. It has clearly emerged as the inter-firm messaging protocol of choice. FIX has grown from its original buyside-to-sellside equity trading roots. It is now used by markets (exchanges, “ECNs”, etc) and other market participants. In addition to equities, FIX currently supports four other products: Collective Investment Vehicles (CIVs), Derivatives, Fixed Income, and Foreign Exchange. The process for modifications to the specification is very open with input and feedback encouraged from the community. Those interested in providing input to the protocol are encouraged to use the FIX website Discussion section or contact the FIX Global Technical Committee Chairpersons, HH Kevin Houstoun, ~~HSBC~~ Rapid Addition, Ltd., or Hanno Klein, Deutsche Boerse ~~Matt~~ ~~Simpson,~~ ~~Chicago~~ ~~Mercantile~~ ~~Exchange~~. The FIX website at http://www.fixprotocol.org is the main source of information, discussion, and notification of FIX-related events.

We look forward to your participation.

FIX Protocol Ltd ~~March 2009~~ August 2011

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 3 of 158
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 1

# August 18, 2011



# About FIX Protocol Limited

FIX Protocol Limited (FPL) (http://www.fixprotocol.org) oversees and manages the development of the FIX Protocol specification and encourages its use throughout the industry. FPL is open to due paying members representing business and technology professionals interested in guiding the growth and adoption of the FIX Protocol that work for: Buy-side Firms, Sell-side Firms, Exchanges, ECNs/ATSs, Utilities, Vendors, and Other Associations. For more information about membership please visit http://www.fixprotocol.org/join/.

# FIX Protocol Limited is represented by the following high-level organization structure:

| Global Steering Committee (GSC) |                     |                              |                |
| ------------------------------- | ------------------- | ---------------------------- | -------------- |
| Global Technical                |                     | Global Education & Marketing |                |
| Americas                        | Asia/Pac Region     | EMEA                         | Japan          |
| Global Derivatives              | Global Fixed Income | Global Foreign Exchange      | Global Markets |

Global Steering Committee comprised of the FPL Committee Chairs. Global Technical and Global Education &#x26; Marketing comprised of Product/Region Committee Representatives.

For a current list of FPL Member firms, visit: http://www.fixprotocol.org/members/

For a current list of active FPL Working Groups, visit: http://www.fixprotocol.org/working_groups/

Links to Product and Regional Committees' web pages are at: http://www.fixprotocol.org/committees/


© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 4 of 158


---
Version 5.0 Service Pack 2 - Errata   VOLUME 1               August 18, 2011

# VOLUME INDEX

# VOLUME 1 - INTRODUCTION

- VOLUME INDEX
- INTRODUCTION
- DOCUMENT NAVIGATION
- FIX PROTOCOL SYNTAX
- COMMON COMPONENTS OF APPLICATION MESSAGES
- COMMON APPLICATION MESSAGES
- GLOSSARY

# VOLUME 2 - TRANSPORT PROTOCOLS

- INTRODUCTION
- TRANSPORT INDEPENDENCE (TI) FRAMEWORK
- TRANSPORT PROTOCOLS

# VOLUME 3 - FIX APPLICATION MESSAGES: PRE-TRADE

- CATEGORY: INDICATION
- CATEGORY: EVENT COMMUNICATION
- CATEGORY: QUOTATION / NEGOTIATION
- CATEGORY: MARKET DATA
- CATEGORY: MARKET STRUCTURE REFERENCE DATA
- CATEGORY: SECURITIES REFERENCE DATA
- ~~CATEGORY: PARTIES REFERENCE DATA~~

# VOLUME 4 - FIX APPLICATION MESSAGES: ORDERS AND EXECUTIONS (TRADE)

- CATEGORY: SINGLE/GENERAL ORDER HANDLING
- CATEGORY: ORDER MASS HANDLING
- CATEGORY: CROSS ORDERS
- CATEGORY: MULTILEG ORDERS (SWAPS, OPTION STRATEGIES, ETC)
- CATEGORY: LIST/PROGRAM/BASKET TRADING

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited               Page 5 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                    August 18, 2011

# VOLUME 5 - FIX APPLICATION MESSAGES: POST-TRADE

- CATEGORY: ALLOCATION AND READY-TO-BOOK
- CATEGORY: CONFIRMATION
- CATEGORY: SETTLEMENT INSTRUCTIONS
- CATEGORY: TRADE CAPTURE ("STREETSIDE") REPORTING
- CATEGORY: REGISTRATION INSTRUCTIONS
- CATEGORY: POSITIONS MAINTENANCE
- CATEGORY: COLLATERAL MANAGEMENT

# VOLUME 6 - FIX DATA DICTIONARY

- FIELD DEFINITIONS
- APPENDIX 6-A - VALID CURRENCY CODES
- APPENDIX 6-B - FIX FIELDS BASED UPON OTHER STANDARDS
- APPENDIX 6-C - EXCHANGE CODES - ISO 10383 MARKET IDENTIFIER CODE (MIC)
- APPENDIX 6-D - CFICODE USAGE - ISO 10962 CLASSIFICATION OF FINANCIAL INSTRUMENTS (CFI CODE)
- APPENDIX 6-E - DEPRECATED (PHASED-OUT) FEATURES AND SUPPORTED APPROACH
- APPENDIX 6-F - REPLACED FEATURES AND SUPPORTED APPROACH
- APPENDIX 6-G - USE OF &#x3C;PARTIES> COMPONENT BLOCK
- APPENDIX 6-H - USE OF &#x3C;SETTLINSTRUCTIONS> COMPONENT BLOCK

# VOLUME 7 - FIX USAGE NOTES

- PRODUCT: COLLECTIVE INVESTMENT VEHICLES (CIV)
- PRODUCT: LISTED DERIVATIVES (FUTURES &#x26; OPTIONS)
- PRODUCT: EQUITIES
- PRODUCT: FIXED INCOME
- PRODUCT: FOREIGN EXCHANGE
- USER GROUP: EXCHANGES AND MARKETS

© Copyright, 2008-2011, FIX Protocol, Limited               Page 6 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                       August 18, 2011

# Contents – Volume 1

DISCLAIMER.............................................................................................................................................................. 2

REPRODUCTION....................................................................................................................................................... 2

PREFACE..................................................................................................................................................................... 3

ABOUT FIX PROTOCOL LIMITED....................................................................................................................... 4

VOLUME INDEX........................................................................................................................................................ 5

INTRODUCTION...................................................................................................................................................... 10

ORGANIZATION OF SPECIFICATION...............................................................................................................10

Message and Component Blocks Definitions.......................................................................................................10

DOCUMENT NAVIGATION................................................................................................................................. 11

OVERVIEW OF MAJOR CHANGES IN FIX 5.0.................................................................................................11

TRANSPORT INDEPENDENCE (TI) FRAMEWORK..........................................................................................11

APPLICATION VERSIONING........................................................................................................................................ 12

SERVICE PACK MANGEMENT.....................................................................................................................................13

EXTENSION PACK MANGEMENT................................................................................................................................ 13

FLEXIBILITY PROVIDED BY FIX 5.0..........................................................................................................................13

FIX PROTOCOL SYNTAX...................................................................................................................................... 15

COMMON FIX SYNTAX RULES.........................................................................................................................15

Data Types:...........................................................................................................................................................15

Required Fields:................................................................................................................................................... 18

FIX “Tag=Value” SYNTAX................................................................................................................................. 20

Message Format...................................................................................................................................................................20

Field Delimiter:....................................................................................................................................................................20

Repeating Groups:............................................................................................................................................................... 21

User Defined Fields:............................................................................................................................................................ 22

Example Usage of Encoded Fields For non-ASCII Language Support...............................................................24

FIXML SYNTAX....................................................................................................................................................25

FIXML Highlights............................................................................................................................................................... 25

Background...........................................................................................................................................................25

FIX and FIXML Version and Comparison using New Order Single Message..................................................25

FIXML Transition to Schema..............................................................................................................................................27

FIXML 4.4 Schema Version Design Objectives.................................................................................................................28

FIXML Design Rules............................................................................................................................................ 29

FIXML Schema Root Element............................................................................................................................................ 29

FIXML Schema File Structure............................................................................................................................................ 33

Customization...................................................................................................................................................................... 44

FIXML Schema Version Datatypes........................................................................................................................................

FIXML Schema File Summary...........................................................................................................................................49

COMMON COMPONENTS OF APPLICATION MESSAGES - COMPONENT BLOCKS (INCLUDED IN
PRE-TRADE, TRADE, AND POST-TRADE MESSAGES).................................................................................54

INSTRUMENT (SYMBOLOGY) COMPONENT BLOCK......................................................................................................54

Examples using Alternative Security IDs............................................................................................................. 59

Specifying an FpML product specification from within the FIX Instrument Block.............................................59

UNDERLYINGINSTRUMENT (UNDERLYING INSTRUMENT) COMPONENT BLOCK..........................................................61

INSTRUMENTLEG (SYMBOLOGY) COMPONENT BLOCK...............................................................................................65

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                 Page 7 of 158
---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                            August 18, 2011


# INSTRUMENTEXTENSION COMPONENT BLOCK

........................................................................................................... 68

# ORDERQTYDATA COMPONENT BLOCK

...................................................................................................................... 69

# COMMISSIONDATA COMPONENT BLOCK

....................................................................................................................70

# PARTIES COMPONENT BLOCK

.....................................................................................................................................71

# NESTEDPARTIES COMPONENT BLOCK

........................................................................................................................ 72

# NESTEDPARTIES2 (SECOND INSTANCE OF NESTING) COMPONENT BLOCK

.................................................................................................73

# NESTEDPARTIES3 (THIRD INSTANCE OF NESTING) COMPONENT BLOCK

.................................................................... 74

# NESTEDPARTIES4 (FOURTH INSTANCE OF NESTING) COMPONENT BLOCK

....................................................................75

# SPREADORBENCHMARKCURVEDATA COMPONENT BLOCK

.......................................................................................76

# STIPULATIONS COMPONENT BLOCK

...........................................................................................................................77

# UNDERLYINGSTIPULATIONS COMPONENT BLOCK

...................................................................................................... 78

# LEGSTIPULATIONS COMPONENT BLOCK

.....................................................................................................................79

# YIELDDATA COMPONENT BLOCK

...............................................................................................................................80

# TRDREGTIMESTAMPS COMPONENT BLOCK

................................................................................................................81

# FINANCINGDETAILS COMPONENT BLOCK

.................................................................................................................. 82

# FINANCINGDETAILS COMPONENT BLOCK

.................................................................................................................. 82

# INSTRUMENTPARTIES COMPONENT BLOCK

................................................................................................................ 84

# INSTRUMENTPARTIES COMPONENT BLOCK

................................................................................................................ 84

# DISPLAYINSTRUCTION COMPONENT BLOCK

...............................................................................................................85

# DISPLAYINSTRUCTION COMPONENT BLOCK

...............................................................................................................85

# ROOTPARTIES COMPONENT BLOCK

............................................................................................................................86

# ROOTPARTIES COMPONENT BLOCK

............................................................................................................................86

# UNDLYINSTRUMENTPARTIES COMPONENT BLOCK

.................................................................................................... 87

# APPLICATIONSEQUENCECONTROL COMPONENT BLOCK

............................................................................................88

# SECURITYXML COMPONENT BLOCK

......................................................................................................................... 89

# RATESOURCE COMPONENT BLOCK

............................................................................................................................ 89

# TARGETPARTIES COMPONENT BLOCK

........................................................................................................................90

# INSTRMTGRP COMPONENT BLOCK

............................................................................................................................. 90

# INSTRMTLEGGRP COMPONENT BLOCK

...................................................................................................................... 91

# UNDINSTRMTGRP COMPONENT BLOCK

......................................................................................................................91

# SECALTIDGRP COMPONENT BLOCK

.......................................................................................................................... 91

# LEGSECALTIDGRP COMPONENT BLOCK

...................................................................................................................92

# UNDSECALTIDGRP COMPONENT BLOCK

...................................................................................................................92

# EVNTGRP COMPONENT BLOCK

.................................................................................................................................. 93

# INSTRUMENTPTYSSUBGRP COMPONENT BLOCK

........................................................................................................93

# UNDLYINSTRUMENTPTYSSUBGRP COMPONENT BLOCK

............................................................................................94

# PTYSSUBGRP COMPONENT BLOCK

............................................................................................................................ 94

# NSTDPTYSSUBGRP COMPONENT BLOCK

....................................................................................................................95

# NSTDPTYS2SUBGRP COMPONENT BLOCK

..................................................................................................................95

# NSTDPTYS3SUBGRP COMPONENT BLOCK

..................................................................................................................96

# NSTDPTYS4SUBGRP COMPONENT BLOCK

..................................................................................................................96

# ROOTSUBPARTIES COMPONENT BLOCK

.....................................................................................................................97

# ATTRBGRP COMPONENT BLOCK

................................................................................................................................ 97

# CONTAMTGRP COMPONENT BLOCK

.......................................................................................................................... 98

# MISCFEESGRP COMPONENT BLOCK

...........................................................................................................................98

# TRDGSESGRP COMPONENT BLOCK

............................................................................................................................99

# COMPLEXEVENTS COMPONENT BLOCK

....................................................................................................................100

# COMPLEXEVENTDATES COMPONENT BLOCK

...........................................................................................................101

# COMPLEXEVENTTIMES COMPONENT BLOCK

...........................................................................................................101

# COMMON INFRASTRUCTURE MESSAGES (APPLY TO PRE-TRADE, TRADE, AND POST-TRADE)

.................................................................................................................................................................................... 102

# BUSINESS MESSAGE REJECT

....................................................................................................................................102

# NETWORK STATUS MESSAGES

................................................................................................................................ 108

# Network Status Component Blocks

.....................................................................................................................108

# CompIDReqGrp component block

....................................................................................................................................108


© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                                                                                                   Page 8 of 158

---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011


# Table of Contents

# CompIDStatGrp component block

.................................................................................................................................... 108

# Network (Counterparty System) Status Request Message

.................................................................................109

# Network (Counterparty System) Status Response Message

...............................................................................110

# USER ADMINISTRATION MESSAGES

........................................................................................................................ 111

# User Management Component Blocks

...............................................................................................................111

# UsernameGrp component block

........................................................................................................................................ 111

# User Request Message

........................................................................................................................................111

# User Response Message

..................................................................................................................................... 113

# User Notification

................................................................................................................................................ 113

# APPLICATION SEQUENCING MESSAGES

......................................................................................................................... 115

# Introduction

........................................................................................................................................................ 115

# Background

.........................................................................................................................................................115

# Extends control over resent data

........................................................................................................................................115

# Support for secondary data distribution

............................................................................................................................................................ 115

# Transaction usage is not recommended

................................................................................................................................. 116

# Using Application Sequencing and Session Sequencing for Gap Detection

.....................................................................................116

# Applicaton Sequencing Component Blocks

...............................................................................................................................117

# ApplIDRequestGrp component block

............................................................................................................................................................... 117

# ApplIDRequestAckGrp component block

........................................................................................................................................................117

# ApplIDReportGrp component block

................................................................................................................................................................. 118

# Application Message Request

.............................................................................................................................................................119

# Application Message Request Ack

..............................................................................................................................................................120

# Application Message Report

...............................................................................................................................................................121

# Using Application Message Report to reset ApplSeqNum

...............................................................................................................................................................121

# Using Application Message Report to indicate last message sent

............................................................................................................................................................121

# Using Application Message Report as keep-alive mechanism

.................................................................................................................................................................121

# Using Application Message Report to indicate completion of resent messages

..............................................................................................................................................................121

# Application Sequencing Message flows

.............................................................................................................................................................123

# Application recovery over a FIX session

..........................................................................................................................................................123

# Application recovery independent of FIX session

............................................................................................................................................................ 124

# GLOSSARY

.............................................................................................................................................................. 126

# APPENDIX 1-A: ABBREVIATIONS USED WITHIN FIXML

........................................................................153


© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                        Page 9 of 158

---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                   August 18, 2011

# FINANCIAL INFORMATION EXCHANGE PROTOCOL

# INTRODUCTION

The Financial Information Exchange (FIX) Protocol is a message standard developed to facilitate the electronic exchange of information related to securities transactions. It is intended for use between trading partners wishing to automate communications.

The message protocol, as defined, will support a variety of business functions. FIX was originally defined for use in supporting US domestic equity trading with message traffic flowing directly between principals. As the protocol evolved, a number of fields were added to support cross-border trading, derivatives, fixed income, and other products. Similarly, the protocol was expanded to allow third parties to participate in the delivery of messages between trading partners. As subsequent versions of FIX are released, it is expected that functionality will continue to expand.

The protocol is defined at two levels: session and application. The session level is concerned with the delivery of data while the application level defines business related data content. This document is divided into volumes and organized to reflect the distinction.

# ORGANIZATION OF SPECIFICATION

The FIX Protocol Specification is organized into 7 Volumes, with each volume covering specific topics areas:

- Volume 1: Introduction (this volume)
- Volume 2: Transport Protocols
- Volume 3: FIX Application Messages for Pre-trade
- Volume 4: FIX Application Messages for Orders and Executions (Trade)
- Volume 5: FIX Application Messages for Post-trade
- Volume 6: FIX Data Dictionary
- Volume 7: FIX Usage Notes

# Message and Component Blocks Definitions

Volumes 1, 3, 4, and 5 contains definitions of FIX component blocks and application message types. Component blocks are sets of related data fields grouped together and are referenced by the component block name in messages that they are used in. FIX component blocks are organized as follows:

- Common Components - are components commonly used by many messages defined across all the volumes in the FIX specification. These are the most commonly used components. Their definitions are found in Volume 1.
- Volume or section specific components - these are component blocks commonly used only by the FIX messages found in that volume or section (e.g. pre-trade, trade, post-trade sections). Their definitions are found in a section at the beginning of the respective volume.
- Message category specific components - these are component blocks that are used only by the FIX messages in a specific message category in a given volume (e.g. Securities Reference Data message category). Their definitions are found in a section at the beginning of their respective message category.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                         Page 10 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# DOCUMENT NAVIGATION

One useful tip when navigating within a volume is to take advantage of the fact that each document contains “bookmarks” to its main sections. You can use the word processor’s “Goto” function (i.e. Ctrl-G) to quickly navigate from one key section or appendix to another.

Third parties or volunteers have historically built useful utilities “generated” using the specification document as their basis which provide cross-reference and lookup capabilities. Such free utilities are available via the FIX website.

# OVERVIEW OF MAJOR CHANGES IN FIX 5.0

With the release of FIX 5.0 in October 2006, the FPL Global Technical Committee (GTC) introduced a new framework, the transport independence (TI) framework, which separated the FIX Session Protocol from the FIX Application Protocol. Under the TI framework the application protocol messages can be sent over any suitable session transport technology (e.g. WS-RX, MQ, publish/subscribe message bus), where the FIX Session Protocol is one of the available options as a session transport for FIX application messages. From this release forward the FIX Application layer and the FIX Session layer will have their own versioning moniker. The FIX Application layer will retain the traditional version moniker of "FIX x.y" while the FIX Session layer will utilize a new version moniker of "FIXT x.y" (note that the version numbers will be independent of each other). The diagram below illustrates how previously the FIX Session layer was tightly coupled to the Application layer. With the advent of Application Versioning and Transport Independence, the FIX Session and Application layers have been decoupled and are now independent.

# FIX 5.0 Unlocks the Application Layer From the Session Layer

| FIX 4.0,4.1,4.2,4.3,4.4 | FIX 5.0                |
| ----------------------- | ---------------------- |
| FIX Application Layer   | Application Versioning |
| FIX Session Layer       | Transport Independence |
|                         | FIX Session Layer      |

# Transport Independence (TI) Framework

The transport independence (TI) framework separates the previously coupled FIX Session layer from the FIX Application layer. Under this framework the FIX Application Protocol can use any transport technology in addition to the FIX Session Protocol. The diagram below illustrates how various transport mechanisms, including the FIX Session layer, can be used to carry the full suite of FIX Application versions.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                      Page 11 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# Transport Independence Framework

| FIX Session          |                      | Web Services         | Transport            | Multicast            | Other Transport      |
| -------------------- | -------------------- | -------------------- | -------------------- | -------------------- | -------------------- |
| Layer                |                      | HTTP                 | MQSeries             | UDP                  | Transport            |
| FKXLL                | 4ransport            |                      |                      |                      |                      |
| FIX 4.0              | FIX 4.1              | FIX 4.2              | FIX 4.3              | FIX 4.4              | FIX 5.0              |
| Application Messages | Application Messages | Application Messages | Application Messages | Application Messages | Application Messages |

To support this framework a key new field has been added called ApplVerID (application version ID, tag 1128). Depending on the use case ApplVerID may be optional or required. Additionally, the FIX field BeginString will no longer identify the FIX application version, but identifies the FIX Session Protocol version. The sections below discusses the four main uses cases supported by the TI framework.

# Application Versioning

Application Versioning allows extensions to the current base application version to be applied using a formal release process. Extension Packs represent the individual gap analysis proposals submitted to the GTC for review and approval. Extension Packs are grouped into Service Packs and are applied to the base application version, usually the most current FIX application version. A new application version is formed when a new Service Pack is applied to a base version. In the diagram below, FIX 4.4 has been extended via Service Pack 0, forming a new application version called FIX 5.0. As new Extension Packs are approved they will be grouped into Service Pack 1 which is then released to form the next application version identified as FIX 5.0 SP1. These application versions are expressed using the new tag ApplVerID.

# Service Pack Release Process

| Service Pack0  | Service Pack1        |
| -------------- | -------------------- |
| Extension Pack | Extension Pack       |
| FIX 4.4        | Base Release FIX 5.0 |
| Extension Pack |                      |
| Extension Pack |                      |
| Extension Pack |                      |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                       Page 12 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Service Pack Management

ApplVerID is an enumerated field. These enumerations are used to express prior versions of FIX inclusive of FIX 4.0, 4.1, 4.2, 4.3 and 4.4 as well as the most recent version, FIX 5.0. Going forward, service packs will be applied to the base version, in this case FIX 5.0, and will be identified as FIX Version + Service Pack. This means that FIX 5.0 will be represented as an enumeration (7) rather than as an actual value in the ApplVerID field. Service Pack identifiers will consist of the base FIX version, the service pack number for that version, and the date the service pack was released. For example, the assigned value for service pack 1 may be “FIX 5.0 SP1 June 30, 2007”.

# Extension Pack Management

Extension Packs are the building blocks of a Service Pack and represent specific functional proposals that have been presented to the GTC. Prior to the release of a Service Pack, Extension Packs are applied to the most recent version of the repository so that they can be used at the point they become available. Extension Packs are applied to the repository in a cumulative manner and will at some point culminate in a Service Pack release. Extension Packs management will be conducted as follows:

1. Extension Packs will be assigned a unique, sequential number at the point they are approved by the GTC
2. Extension Packs are applied to the most recent version of the repository and may be inclusive of prior Extension Packs
3. At the point an Extension Pack has been applied, the updated repository, schema, and message tables will be available
4. When implementing a specific Extension Pack, the field ApplExtID (1156) will be used to specify the Extension Pack Identifier
5. Users of an Extension Pack need not implement other Extension Packs present in the repository. Rules of engagement need to be bilaterally agreed on.

# Flexibility Provided by FIX 5.0

This is the ‘GTC approved’ approach which separates the FIX session layer from the application layer, provides support for application versioning, and creates a platform for transport independence. This approach will treat the FIX session like ‘any other’ transport and allow the unambiguous use of any application version via the ApplVerID field. A value of FIXT.1.1 in the BeginString of the FIX Session will indicate that application versioning is in effect and the version should be determined either through the Logon's NoMsgType repeating group or the AppVerID field. Future extensions to the session layer or application layer will be supported independent of each other as point releases to BeginString and ApplVerID, respectively. Major Tags describing the session and application versions are: BeginString=FIXT.1.1 (or later versions) and ApplVerID=FIX.5.0T₁T (or later versions). A BeginString=FIX.5.0 (or later versions) will not be valid.

The diagram below illustrates how the new FIXT.1.1 Session layer can be used to transport makes use of the ApplVerID in the Application layer in order to support a broad set of application versions. T₂T

1TT The value FIX.5.0 will be represented using enumeration 7

2TT FIX.4.0, FIX.4.1, FIX.4.2, FIX.4.3, FIX.4.4, FIX.5.0 are represented using enumerations

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 13 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

# Buy Side

# FIX Application Layer

# FIX.4.0

ApplVerID = FIX.4.0

New Order

Single

# FIX.4.1

ApplVerID = FIX.4.1

Quote

# FIX Session Layer

# ApplVerID = FIX.4.2

# FIX.4.2

Market Data

# FIX.4.4

ApplVerID = FIX.4.4

Allocation

Instruction

# FIX.5.0

ApplVerID = FIX.5.0

TradeCapture

Report

# FIX Application Layer

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited

Page 14 of 158


---
Version 5.0 Service Pack 2 - Errata    VOLUME 1                                                    August 18, 2011

# FIX PROTOCOL SYNTAX

The FIX Protocol currently exists in two syntaxes:

1. “Tag=Value” syntax
2. FIXML syntax

The same business message flow applies to either syntax. A specific syntax is simply a slightly different way to represent the same thing in much the same way that “3” and “three” represent the same thing.

# COMMON FIX SYNTAX RULES

The following section summarizes general specifications for constructing FIX messages which are applicable to both “Tag=Value” and FIXML syntaxes.

# Data Types:

Data types (with the exception of those of type "data") are mapped to ASCII strings as follows:

# int

Sequence of digits without commas or decimals and optional sign character (ASCII characters "-" and "0" - "9"). The sign character utilizes one byte (i.e. positive int is "99999" while negative int is "-99999"). Note that int values may contain leading zeros (e.g. "00023" = "23").

Examples:

- 723 in field 21 would be mapped int as |21=723|.
- -723 in field 12 would be mapped int as |12=-723|.

The following data types are based on int:

- Length: int field representing the length in bytes. Value must be positive.
- TagNum: int field representing a field's tag number when using FIX "Tag=Value" syntax. Value must be positive and may not contain leading zeros.
- SeqNum: int field representing a message sequence number. Value must be positive.
- NumInGroup: int field representing the number of entries in a repeating group. Value must be positive.
- DayOfMonth: int field representing a day during a particular month (values 1 to 31).

# float

Sequence of digits with optional decimal point and sign character (ASCII characters "-", "0" - "9" and "."); the absence of the decimal point within the string will be interpreted as the float representation of an integer value. All float fields must accommodate up to fifteen significant digits. The number of decimal places used should be a factor of business/market needs and mutual agreement between counterparties. Note that float values may contain leading zeros (e.g. "00023.23" = "23.23") and may contain or omit trailing zeros after the decimal point (e.g. "23.0" = "23.0000" = "23" = "23."). Note that fields which are derived from float may contain negative values unless explicitly specified otherwise. The following data types are based on float:

- Qty: float field capable of storing either a whole number (no decimal places) of "shares" (securities denominated in whole units) or a decimal value containing decimal places for non-share quantity asset classes (securities denominated in fractional units).
- Price: float field representing a price. Note the number of decimal places may vary.

© Copyright, 2008-       ~~2009~~2011, FIX Protocol, Limited                                        Page 15 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                  August 18, 2011

# Errata

For certain asset classes prices may be negative values. For example, prices for options strategies can be negative under certain market conditions. Refer to Volume 7: FIX Usage by Product for asset classes that support negative price values.

| PriceOffset        | float field representing a price offset, which can be mathematically added to a "Price". Note the number of decimal places may vary and some fields such as LastForwardPoints may be negative.                                                                                                                                  | | |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |---|---|
| Amt                | float field typically representing a Price times a Qty                                                                                                                                                                                                                                                                          | | |
| Percentage         | float field representing a percentage (e.g. 0.05 represents 5% and 0.9525 represents 95.25%). Note the number of decimal places may vary.                                                                                                                                                                                       | | |
| char               | Single character value, can include any alphanumeric character or punctuation except the delimiter. All char fields are case sensitive (i.e. m != M).                                                                                                                                                                           | | |
| Boolean            | char field containing one of two values: 'Y' = True/Yes, 'N' = False/No                                                                                                                                                                                                                                                         | | |
| String             | Alpha-numeric free format strings, can include any character or punctuation except the delimiter. All String fields are case sensitive (i.e. morstatt != Morstatt).                                                                                                                                                             | | |
| MultipleCharValue  | string field containing one or more space delimited single character values (e.g. \|18=2 A F\|).                                                                                                                                                                                                                                |
| MultipleStringValu | string field containing one or more space delimited multiple character values (e.g. \|277=AV AN A\|).                                                                                                                                                                                                                           |
| Country            | string field representing a country using ISO 3166 Country code (2 character) values (see Appendix 6-B).                                                                                                                                                                                                                        | | |
| Currency           | string field representing a currency type using ISO 4217 Currency code (3 character) values (see Appendix 6-A).                                                                                                                                                                                                                 | | |
| Exchange           | string field representing a market or exchange using ISO 10383 Market Identifier Code (MIC) values (see Appendix 6-C).                                                                                                                                                                                                          | | |
| MonthYear          | string field representing month of a year. An optional day of the month can be appended or an optional week code. Valid formats: YYYYMM, YYYYMMDD, YYYYMMWW. Valid values: YYYY = 0000-9999; MM = 01-12; DD = 01-31; WW = w1, w2, w3, w4, w5.                                                                                   | | |
| UTCTimestamp       | string field representing Time/date combination represented in UTC (Universal Time Coordinated, also known as "GMT") in either YYYYMMDD-HH:MM:SS (whole seconds) or YYYYMMDD-HH:MM:SS.sss (milliseconds) format, colons, dash, and period required. Valid values: \* YYYY = 0000-9999, MM = 01-12, DD = 01-31, HH = 00-23, MM = | | |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                         Page 16 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# 00-59, SS = 00-60 (60 only if UTC leap second) (without milliseconds).

* YYYY = 0000-9999, MM = 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC leap second), sss=000-999 (indicating milliseconds).

# Leap Seconds:

Note that UTC includes corrections for leap seconds, which are inserted to account for slowing of the rotation of the earth. Leap second insertion is declared by the International Earth Rotation Service (IERS) and has, since 1972, only occurred on the night of Dec. 31 or Jun 30. The IERS considers March 31 and September 30 as secondary dates for leap second insertion, but has never utilized these dates. During a leap second insertion, a UTCTimestamp field may read "19981231-23:59:59", "19981231-23:59:60", "19990101-00:00:00". (see http://tycho.usno.navy.mil/leapsec.html)

# UTCTimeOnly

string field representing Time-only represented in UTC (Universal Time Coordinated, also known as "GMT") in either HH:MM:SS (whole seconds) or HH:MM:SS.sss (milliseconds) format, colons, and period required. This special-purpose field is paired with UTCDateOnly to form a proper UTCTimestamp for bandwidth-sensitive messages.

Valid values:

- HH = 00-23, MM = 00-60 (60 only if UTC leap second), SS = 00-59. (without milliseconds)
- HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC leap second), sss=000-999 (indicating milliseconds).

# UTCDateOnly

string field representing Date represented in UTC (Universal Time Coordinated, also known as "GMT") in YYYYMMDD format. This special-purpose field is paired with UTCTimeOnly to form a proper UTCTimestamp for bandwidth-sensitive messages.

Valid values:

- YYYY = 0000-9999, MM = 01-12, DD = 01-31.

# LocalMktDate

string field representing a Date of Local Market (as opposed to UTC) in YYYYMMDD format. This is the "normal" date field used by the FIX Protocol.

Valid values:

- YYYY = 0000-9999, MM = 01-12, DD = 01-31.

# TZTimeOnly

string field representing the time represented based on ISO 8601. This is the time with a UTC offset to allow identification of local time and timezone of that time.

Format is HH:MM[:SS][Z | [ + | - hh[:mm]]] where HH = 00-23 hours, MM = 00-59 minutes, SS = 00-59 seconds, hh = 01-12 offset hours, mm = 00-59 offset minutes.

Example: 07:39Z is 07:39 UTC

Example: 02:39-05 is five hours behind UTC, thus Eastern Time

Example: 15:39+08 is eight hours ahead of UTC, Hong Kong/Singapore time

Example: 13:09+05:30 is 5.5 hours ahead of UTC, India time

# TZTimestamp

string field representing a time/date combination representing local time with

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                       Page 17 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                  August 18, 2011

an offset to UTC to allow identification of local time and timezone offset of that time. The representation is based on ISO 8601. Format is YYYYMMDD-HH:MM:SS[Z | [ + | - hh[:mm]]] where YYYY = 0000 to 9999, MM = 01-12, DD = 01-31 HH = 00-23 hours, MM = 00-59 minutes, SS = 00-59 seconds, hh = 01-12 offset hours, mm = 00-59 offset minutes

Example: 20060901-07:39Z is 07:39 UTC on 1st of September 2006

Example: 20060901-02:39-05 is five hours behind UTC, thus Eastern Time on 1st of September 2006

Example: 20060901-15:39+08 is eight hours ahead of UTC, Hong Kong/Singapore time on 1st of September 2006

Example: 20060901-13:09+05:30 is 5.5 hours ahead of UTC, India time on 1st of September 2006

data string field containing raw data with no format or content restrictions. Data fields are always immediately preceded by a length field. The length field should specify the number of bytes of the value of the data field (up to but not including the terminating SOH).

Caution: the value of one of these fields may contain the delimiter (SOH) character. Note that the value specified for this field should be followed by the delimiter (SOH) character as all fields are terminated with an "SOH".

XMLData Contains an XML document raw data with no format or content restrictions. XMLData fields are always immediately preceded by a length field. The length field should specify the number of bytes of the value of the data field (up to but not including the terminating SOH).

Language Identifier for a national language - uses ISO 639-1 standard

# Pattern

Used to build on and provide some restrictions on what is allowed as valid values in fields that uses a base FIX data type and a pattern data type. The universe of allowable valid values for the field would then be the union of the base set of valid values and what is defined by the pattern data type. The pattern data type used by the field will retain its base FIX data type (e.g. String, int, char).

# Tenor

used to allow the expression of FX standard tenors in addition to the base valid enumerations defined for the field that uses this pattern data type. This pattern data type is defined as follows:

- Dx = tenor expression for "days", e.g. "D5", where "x" is any integer > 0
- Mx = tenor expression for "months", e.g. "M3", where "x" is any integer > 0
- Wx = tenor expression for "weeks", e.g. "W13", where "x" is any integer > 0
- Yx = tenor expression for "years", e.g. "Y1", where "x" is any integer > 0

# Reserved Fields:

- Reserved100Plus: Values "100" and above are reserved for bilaterally agreed upon user defined enumerations.
- Reserved1000Plus: Values "1000" and above are reserved for bilaterally agreed upon user defined enumerations.
- Reserved4000Plus: Values "4000" and above are reserved for bilaterally agreed upon user defined enumerations.

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                                 Page 18 of 158


---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                  August 18, 2011

Each message within the protocol is comprised of required, optional and conditionally required (fields which are required based on the presence or value of other fields) fields. Systems should be designed to operate when only the required and conditionally required fields are present.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                         Page 19 of 158


---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                August 18, 2011

# FIX “Tag=Value” SYNTAX

The following section summarizes general specifications for constructing FIX messages in “Tag=Value” syntax.

# Message Format

The general format of a FIX message is a standard header followed by the message body fields and terminated with a standard trailer.

Each message is constructed of a stream of  <tag>=<value> fields with a field delimiter between fields in the stream.  Tags are of data type   TagNum.   All tags must have a value specified.   Optional fields without values should simply not be specified in the FIX message. A Reject message is the appropriate response to a tag with no value.</value></tag>

Except where noted, fields within a message can be defined in any sequence (Relative position of a field within a message is inconsequential.) The exceptions to this rule are:

1. General message format is composed of the standard header followed by the body followed by the standard trailer.
2. The first three fields in the standard header are BeginString (tag #8) followed by BodyLength (tag #9) followed by MsgType (tag #35).
3. The last field in the standard trailer is the CheckSum (tag #10).
4. Fields within repeating data groups must be specified in the order that the fields are specified in the message definition within the FIX specification document. The NoXXX field where XXX is the field being counted specifies the number of repeating group instances that must immediately precede the repeating group contents.
5. A tag number (field) should only appear in a message once. If it appears more than once in the message it should be considered an error with the specification document. The error should be pointed out to the FIX Global Technical Committee.

In addition, certain fields of the data type MultipleCharValue can contain multiple individual values separated by a space within the "value" portion of that field followed by a single "SOH" character (e.g. "18=2 9 C<soh>" represents 3 individual values: '2', '9', and 'C' ). Fields of the data type MultipleStringValue can contain multiple values that consists of string values separated by a space within the "value" portion of that field followed by a single "SOH" character (e.g. "277=AA I AJ<soh>" represents 3 values: 'AA', 'I', 'AJ').</soh></soh>

It is also possible for a field to be contained in both the clear text portion and the encrypted data sections of the same message. This is normally used for validation and verification. For example, sending the SenderCompID in the encrypted data section can be used as a rudimentary validation technique.   In the cases where the clear text data differs from the encrypted data, the encrypted data should be considered more reliable. (A security warning should be generated).

# Field Delimiter:

All fields (including those of data type data e.g. SecureData, RawData, SignatureData, XmlData, etc.) in a FIX message are terminated by a delimiter character. The non-printing, ASCII "SOH" (#001, hex:    0x01, referred to in this document as <soh>), is used for field termination. Messages are delimited by the “SOH” character following the CheckSum field. All messages begin with the “8=FIX.x.y<soh>” string and terminate with “10=nnn<soh>“.</soh></soh></soh>

There shall be no embedded delimiter characters within fields except for data type data.

# Repeating Groups:

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                        Page 20 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# 1. Repeating Groups

It is permissible for fields to be repeated within a repeating group (e.g. "384=2<soh>372=6<soh>385=R<soh>372=7<soh>385=R<soh>" represents a repeating group with two repeating instances “delimited” by tag 372 (first field in the repeating group.)). If the repeating group is used, the first field of the repeating group is required. This allows implementations of the protocol to use the first field as a "delimiter" indicating a new repeating group entry. The first field listed after the NoXXX, then becomes conditionally required if the NoXXX field is greater than zero.</soh></soh></soh></soh></soh>

The NoXXX field (for example: NoTradingSessions, NoAllocs) which specifies the number of repeating group instances occurs once for a repeating group and must immediately precede the repeating group contents. The NoXXX field is required if one of the fields in the repeating group is required. If all members of a repeating group are optional, then the NoXXX field should also be optional.

If a repeating group field is listed as required, then it must appear in every repeated instance of that repeating group. For optional repeating group, there is no requirement to specify NoXXX=0 (e.g. NoPartyIDs=0) when there is no data to send. The absence of the repeating group means the same thing. Sending NoXXX=0 (e.g. NoPartyIDs=0) for optional repeating group is valid but not recommended. Recipients should be able to accept NoXXX=0, but Recipients should not require this. Senders should never send NoXXX=0.

For repeating groups that are marked as required, sending NoXXX=0 is not FIX compliant. Repeating groups are designated within the message definition via indentation and the symbol. The ordering of repeating group instances must be preserved and processed in the order provided by the message sender.

Some repeating groups are nested within another repeating group (potentially more than one level of nesting). Nested repeating groups are designated within the message definition via indentation and the symbol followed by another symbol. If a nested repeating group is used, then the outer repeating group must be specified.

# Example of a repeating group:

| 215 | NoRoutingIDs | N | Required if any RoutingType and RoutingIDs are specified. Indicates the number within repeating group. |
| --- | ------------ | - | ------------------------------------------------------------------------------------------------------ |
| 216 | RoutingType  | N | Indicates type of RoutingID. Required if NoRoutingIDs is > 0.                                          |
| 217 | RoutingID    | N | Identifies routing destination. Required if NoRoutingIDs is > 0.                                       |

Rest of the message not shown

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 21 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Example of nested repeating group

Portion of New Order - List message showing a nested repeating group for allocations for each order. Note the NoAllocs repeating group is nested within the NoOrders repeating group and as such each instance of the orders repeating group may contain a repeating group of allocations.

| 73                         | NoOrders             | Y | Number of orders in this message (number of repeating groups to follow)                                                                                                |
| -------------------------- | -------------------- | - | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 11                         | ClOrdID              | Y | Must be the first field in the repeating group.                                                                                                                        |
| 526                        | SecondaryClOrdID     | N |                                                                                                                                                                        |
| 67                         | ListSeqNo            | Y | Order number within the list                                                                                                                                           |
| 583                        | ClOrdLinkID          | N |                                                                                                                                                                        |
| 160                        | SettlInstMode        | N |                                                                                                                                                                        |
| component block \<Parties> |                      | N | Insert here the set of "Parties" (firm identification) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES"                                                   |
| 229                        | TradeOriginationDate | N |                                                                                                                                                                        |
| 1                          | Account              | N |                                                                                                                                                                        |
| 581                        | AccountType          | N |                                                                                                                                                                        |
| 589                        | DayBookingInst       | N |                                                                                                                                                                        |
| 590                        | BookingUnit          | N |                                                                                                                                                                        |
| 591                        | PreallocMethod       | N |                                                                                                                                                                        |
| 78                         | NoAllocs             | N | Indicates number of pre-trade allocation accounts to follow                                                                                                            |
| 79                         | AllocAccount         | N | Required if NoAllocs > 0. Must be the first field in the repeating group.                                                                                              |
| 467                        | IndividualAllocID    | N |                                                                                                                                                                        |
| component block            |                      | N | Insert here the set of "Nested Parties" (firm identification "nested" within additional repeating group) fields defined in "COMMON COMPONENTS OF APPLICATION MESSAGES" |
| 80                         | AllocQty             | N |                                                                                                                                                                        |
| 63                         | SettlmntTyp          | N |                                                                                                                                                                        |
| 64                         | FutSettDate          | N | Takes precedence over SettlmntTyp value and conditionally required/omitted for specific SettlmntTyp values.                                                            |

Rest of the message not shown

User Defined Fields:

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 22 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                 August 18, 2011

# 1. User Defined Fields

In order to provide maximum flexibility for its users, the FIX protocol accommodates User Defined Fields. These fields are intended to be implemented between consenting trading partners and should be used with caution to avoid conflicts, which will arise as multiple parties begin implementation of the protocol. It is suggested that if trading partners find that particular User Defined Fields add value, they should be recommended to the FIX Global Technical Committee for inclusion in a future FIX version.

The tag numbers 5000 to 9999 have been reserved for use with user defined fields, which are used as part of inter-firm communication. These tags can be registered/reserved via the FIX website. The tag numbers greater than or equal to 10000 have been reserved for internal use (within a single firm) and do not need to be registered/reserved via the FIX website.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                         Page 23 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                           August 18, 2011

# Example Usage of Encoded Fields For non-ASCII Language Support

The examples below illustrate how the MessageEncoding (347) field is used in conjunction with the various available encoded fields in FIX.

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

© Copyright, 2008-2011, FIX Protocol, Limited                                    Page 24 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# FIXML SYNTAX

# FIXML Highlights

- FIXML is the XML vocabulary for creating FIX messages.
- Uses the same FIX data dictionary and business logic.
- Focuses primarily on the FIX Application Messages and does not provide a session layer.
- Can be encapsulated within the FIX Session Protocol or within another protocol like, MQ Series, TIBCO, SOAP, etc.

# Background

The FPL FIXML Working Group began investigating the XML format in 1998 and published a White Paper supporting an evolutionary approach to migrating the FIX Protocol to an XML format. The working group released an initial version of the FIXML DTDs on January 15th, 1999. There are currently DTDs based on FIX Protocol versions 4.1, 4.2 and 4.3. A FIXML Schema based version of FIXML was released following the release of FIX 4.4.

The FIXML language is in a state of transition. It has been four years since the initial release of FIXML. XML technology has advanced considerably in those four years. FPL committed to deliver an XML Schema representation for FIXML starting with FIX 4.3. Issues confronting FIXML users in the derivatives post trade area preempted release of the FIXML Schema for FIX 4.3. Instead the effort shifted to attempts to exploit the capabilities available in XML Schema to define a version of FIXML that was optimized to reduce message size. This version of FIXML was referred to as Transport Optimized FIXML during its development. The Global Technical Committee chose to release the transport optimizations in two phases.

The FIX 4.4 DTD Version was released with FIX 4.4 and introduced standardized abbreviations for field names and removal of container elements used to represent repeating groups and component blocks. This version has been replaced by the FIX 4.4 Schema Version and should no longer be used.

The FIX 4.4 Schema Version was released as part of FIX 4.4 Errata release. The FIX 4.4 Schema Version exploits the enhanced capabilities of XML Schema to further optimize FIXML message size by introducing the use of attributes to represent fields.

FIXML for FIX 5.0 is defined by an XML Schema based upon the work done for FIX 4.4.

# FIX and FIXML Version and Comparison using New Order Single Message

The following section compares the implementation of the same FIX new order single message in FIX 4.2 tag=value format, FIXML 4.2 DTD version, and FIXML Schema Version.

# FIX tag=value Version

The following is a FIX 4.2 New Order Single message in classic tag-value pair format:

8=FIX.4.2^9=251^35=D^49=AFUNDMGR^56=ABROKER^34=2^52=20030615-01:14:49^11=12345^1=111111^63=0^64=20030621^21=3^110=1000^111=50000^55=IBM^48=459200101^22=1^54=1^60=2003061501:14:49^38=5000^40=1^44=15.75^15=USD^59=0^10=127

NOTE: ^ represents the SOH separator.

The message is 195 bytes in length.

© Copyright, 2008-2009, FIX Protocol, Limited                                        Page 25 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                          August 18, 2011

# FIXML 4.2 Version

The following is a roughly equivalent FIXML 4.2 DTD-based message:

<fixml>
<fixmlmessage>

<sendingtime>20020103-12:00:01</sendingtime>
<sender>
<compid>AFUNDMGR</compid>
</sender>
<target>
<compid>ABROKER</compid>
</target>
</possresend></possdupflag>
<applicationmessage>
<order>
<clordid>1968</clordid>
<account>4130287</account>
#
<exdestination value="L">
<instrument>
<symbol>IBM</symbol>
<securityid>459200101</securityid>
<securityidsource value="1">
</securityidsource></instrument>
<side value="2">
<transacttime>20021120-12:13:12</transacttime>
<orderqtydata>
<orderqty>1000</orderqty>
</orderqtydata>
<ordtype value="2">
93.25</price>
<currency value="USD">
</currency></ordtype></side></exdestination></handlinst></order>
</applicationmessage>
</fixmlmessage>
</fixml>

This message is 684 bytes; over three times the message size of the raw FIX tag=value message. In practice, FIXML messages could be 3-5 times their FIX tag=value equivalents.

# FIXML 4.4 Schema Version

The following is a New Order Single message based on the FIXML 4.4 Schema.

<fixml>
<order clordid="123456" side="2" transacttm="2001-09-11T09:30:47-05:00" ordtyp="2" px="93.25" acct="26522154">
#
</hdr></order></fixml>

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                  Page 26 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                August 18, 2011

PosRsnd="N"

SeqNum="521">

&#x3C;Sndr ID="AFUNDMGR"/>

&#x3C;Tgt  ID="ABROKER"/>

&#x3C;/Hdr>

&#x3C;Instrmt         Sym="IBM" ID="459200101" IDSrc="1"/>

&#x3C;OrdQty         Qty="1000"/>

&#x3C;/Order>

&#x3C;/FIXML>

NOTE: The XML attributes in the message have been placed on separate lines to aid readability.

This message is 348 bytes in length; approximately 70% larger than the raw FIX tag=value message, but roughly half the size of the previous FIXML format without significant loss in readability.

# Sample Message Content

The following table is included to help clarify the message content shown above:

| Tag/Attribute                          | Meaning            |
| -------------------------------------- | ------------------ |
| \<FIXML>                               | Root element       |
| \<Order                                | New order          |
| ClOrdID="123456"                       | Client’s order ID  |
| Side="2"                               | Sell order         |
| TransactTm="2001-09-11T09:30:47-05:00" | Transaction time   |
| OrdTyp="2"                             | Limit order        |
| Px="93.25"                             | Limit price        |
| Acct="26522154">                       | Customer’s account |
| \<Instrmt Sym="IBM"                    | Stock symbol       |
| ID="459200101"                         | Stock CUSIP        |
| IDSrc="1"/>                            | (ID source=CUSIP)  |
| \<OrdQty Qty="1000"/>                  | Order quantity     |
| \</Order>                              | Close of order     |
| \</FIXML>                              | Close root element |

# FIXML Transition to Schema

FIXML was initiated at a time when the only mechanism available to define and validate an XML syntax was the Document Type Definition (DTD) originally created as part of the Standardized General Markup Language (SGML). The DTD provided only minimal ability to define XML syntax.

Since then, the World Wide Web Consortium (http://www.w3c.org) adopted XML Schema as a way of representing the format of XML messages using XML syntax. Document Type Definitions (DTDs), which were originally part of XML, have limited syntax and capabilities for defining XML syntax. XML Schema was designed to address many of the deficiencies of DTDs. The FPL Global Technical Committee has received numerous requests from FIX users for an XML Schema representation of the FIX Protocol and believes that a version of FIXML defined using XML Schema will provide a more robust, optimized message format and provide a better environment for users implementing FIXML applications.

The following limitations of DTDs determined much of the FIXML implementation:

- Meta data could not be included in the DTD - so attributes were used for meta-data.
- Attributes could not be "typed" so this restricted datatyping to elements. Many XML syntaxes then relied heavily on elements for data, attributes for meta-data. This is the approach taken for FIXML up through the FIX 4.4 Errata 20030618 release.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited

Page 27 of 158
---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                 August 18, 2011


# FIXML 4.4 Schema Version Enhancements

The Schema version introduces the following enhancements:

- Incorporated further transport optimizations
- - Adoption of attributes
- Contextual Abbreviations – further reducing field names

Addressed component blocks built around limitations of FIX tag=value by using consistent field names across component blocks
- - InstrumentLeg
- NestedParties
- Nested2Parties
- UnderlyingInstrument

Develop XML Schema Design Approach

# FIXML 4.4 Schema Version Design Objectives

Design objectives for FIXML messages (instance documents):

These design objectives refer to the FIXML instance documents. Instance documents are the actual FIXML messages.

- FIXML implementation shall adhere to XML technology standards as specified by the W3C.
- FIXML implementation shall be suitable implementation for use in high volume transaction scenarios.
- - Target applications:

FIXML implementation shall minimize bandwidth consumption (reduced message size). The goal is to have FIXML messages be less than 1.5 X the size of an equivalent FIX tag=value message.
- FIXML implementation shall maintain human readability of FIXML message, while still adhering to performance goals.
- FIXML implementation shall support integration of FpML product specifications within the FIXML message in an equivalent manner to FIX 4.4 tag=value. This integration should use commonly agreed upon, de facto standard XML design patterns.
- FIXML implementation shall support a ready translation to and from FIX tag=value messages.
- FIXML implementation shall provide a cross-reference to ISO 15022 repository for each message, element, and component.


© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                         Page 28 of 158

---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# FIXML implementation shall maintain the extensibility and customization available via the FIX tag=value message format, including:

- Ability to add custom messages,
- Ability to add custom fields to messages, component blocks, and repeating groups.

FIXML Implementation shall provide full transport level independence.

FIXML Implementation shall support version identification.

# Design Objectives for the Schema Document

- FIXML Schema shall be implemented using the current de facto industry best practices for XML Schema usage.
- FIXML Schema shall be implemented in such a way as to fully support the FIXML 4.4 "Schema Version" Instance Requirements defined above.
- FIXML Schema shall support version identification.
- FIXML Schema shall provide meta-data sufficient to identify the FIX field name, component type, tag number, ISO 15022 repository cross-reference.
- FIXML Schema shall be interoperable and compatible with the FpML schema.
- The FIXML Schema shall be based upon and be compatible with the current version of XML schema: http://www.w3.org/2001/XMLSchema

# FIXML Design Rules

The following design guidelines were created to meet the design objectives for the FIXML Schema and the FIXML instance documents defined above.

1. Use meaningful abbreviations for element and attribute names wherever possible. Use standard abbreviations for common words (e.g., Price = Px, Currency = Ccy, etc.).
2. FIX Messages shall be implemented as XML Elements.
3. Individual, non-repeating fields shall be implemented as attributes of FIX Message elements.
4. FIX Component Blocks shall be implemented as an XML element.
5. Component blocks that were duplicated within FIX to circumvent tag=value requirements for uniqueness across fields and tag numbers, such as the Parties, NestedParties, NestedParties2 component blocks, shall use common naming in FIXML. The datatypes for each of the ComponentTypes will provide the mapping back to FIX tag=value format.
6. Non-repeating fields belonging to a FIX component block shall be implemented as attributes.
7. Repeating groups shall be implemented as XML elements.
8. Non-repeating fields belonging to a repeating group shall be implemented as attributes.
9. Identical repeating groups that occur across FIX messages will be identified as implicit components and reused across messages.
10. Field name prefixes that were used in FIX tag=value format for uniqueness shall be removed – thus creating a contextual abbreviation.
11. FIX datatypes will be mapped to the closest XML Schema datatype whenever possible, thus making FIXML more compatible with standard XML toolsets.

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 29 of 158
---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                                     August 18, 2011


# FIXML Schema Root Element

The FIXML Schema root element has been expanded to include the ability to include a batch of FIXML
application messages. Batch capability was provided to deliver groups of messages, such as post trade confirms
or position reports at the end of a trading session. Single message capability is still supported. Note that the
headers are optional.

| 0..1                   | Batch        | 0..1 | Batch Header |         |      |     |
| ---------------------- | ------------ | ---- | ------------ | ------- | ---- | --- |
| FIXML Element          | (HeaderType) |      |              |         |      |     |
|                        |              | 0..1 | 0..n         | Message | 0..1 | Hdr |
| (MessageType abstract) |              |      |              |         |      |     |
| Batch                  |              |      |              |         |      |     |
| Single Message Usage   |              |      |              |         |      |     |

&#x3C;FIXML>
&#x3C;Order>
&#x3C;Hdr/>
&#x3C;/Order>
&#x3C;/FIXML>

# An Example FIXML Single Message

The following is a New Order Single FIXML Schema message sent individually.

&#x3C;FIXML v="4.4"  r="20030618" s="20040109">
&#x3C;Order ClOrdID="123456" Side="2" TransactTm="2001-09-11T09:30:47-05:00"
OrdTyp="2" Px="93.25" Acct="26522154">
&#x3C;Instrmt Sym="IBM" ID="459200101" IDSrc="1"/>
&#x3C;OrdQty Qty="1000"/>
&#x3C;/Order>
&#x3C;/FIXML>

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                             Page 30 of 158



---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                          August 18, 2011

# An Example FIXML Batch Message

The following example shows a batch of position reports. Note that the header is provided for the entire batch of messages.

&#x3C;FIXML v="4.4" r="20030618" s="20031030">
&#x3C;Batch>
&#x3C;Hdr Snt="2001-12-17T09:30:47-05:00">
&#x3C;Sndr ID="OCC"/>
&#x3C;Tgt ID="Firm"/>
&#x3C;/Hdr>
&#x3C;PosRpt      RptID="541386431"                 Rslt="0" BizDt="2003-09-10T00:00:00" Acct="1"
AcctTyp="1"   SetPx="0.00" SetPxTyp="1" PriSetPx="0.00" ReqTyp="0" Ccy="USD">
&#x3C;Pty ID="OCC" Role="21"/>
&#x3C;Pty ID="99999" Role="4"/>
&#x3C;Pty ID="C" Role="38">
&#x3C;PtySub    SubID="ZZZ"    SubIDTyp="2"/>
&#x3C;/Pty>
&#x3C;Qty Typ="SOD"  Long="35"      Short="0"/>
&#x3C;Qty Typ="FIN"  Long="20"      Short="10"/>
&#x3C;Qty Typ="IAS"  Long="10"/>
&#x3C;Amt Typ="FMTM" Amt="0.00"/>
&#x3C;Instrmt Sym="AOL"                ID="KW"  IDSrc="J" CFI="OCASPS" MMY="20031122"
Mat="2003-11-22T00:00:00" Strk="47.50" StrkCcy="USD"              Mult="100"/>
&#x3C;/PosRpt>
&#x3C;PosRpt      RptID="541386536"                 Rslt="0" BizDt="2003-09-10T00:00:00" Acct="1"
AcctTyp="1"   SetPx="0.00" SetPxTyp="1" PriSetPx="0.00" ReqTyp="0" Ccy="USD">
&#x3C;Pty ID="OCC" Role="21"/>
&#x3C;Pty ID="99999" Role="4"/>
&#x3C;Pty ID="C" Role="38">
&#x3C;PtySub    SubID="ZZZ"    SubIDTyp="2"/>
&#x3C;/Pty>
&#x3C;Qty Typ="SOD"  Long="35"      Short="0"/>
&#x3C;Qty Typ="FIN"  Long="20"      Short="10"/>
&#x3C;Qty Typ="IAS"  Long="10"/>
&#x3C;Amt Typ="FMTM" Amt="0.00"/>
&#x3C;Instrmt Sym="AOL"                ID="KW"  IDSrc="J" CFI="OCASPS" MMY="20031122"
Mat="2003-11-22T00:00:00" Strk="47.50" StrkCcy="USD"              Mult="100"/>
&#x3C;/PosRpt>
&#x3C;PosRpt      RptID="541386678"                 Rslt="0" BizDt="2003-09-10T00:00:00" Acct="1"
AcctTyp="1"   SetPx="0.00" SetPxTyp="1" PriSetPx="0.00" ReqTyp="0" Ccy="USD">
&#x3C;Pty ID="OCC" Role="21"/>
&#x3C;Pty ID="99999" Role="4"/>
&#x3C;Pty ID="C" Role="38">
&#x3C;PtySub    SubID="ZZZ"    SubIDTyp="2"/>
&#x3C;/Pty>
&#x3C;Qty Typ="SOD"  Long="35"      Short="0"/>
&#x3C;Qty Typ="FIN"  Long="20"      Short="10"/>
&#x3C;Qty Typ="IAS"  Long="10"/>
&#x3C;Amt Typ="FMTM" Amt="0.00"/>
&#x3C;Instrmt Sym="AOL"                ID="KW"  IDSrc="J" CFI="OCASPS" MMY="20031122"
Mat="2003-11-22T00:00:00" Strk="47.50" StrkCcy="USD"              Mult="100"/>
&#x3C;/PosRpt>
&#x3C;/Batch>
&#x3C;/FIXML>

# Version Identification

FIXML versions are identified explicitly in the schema file names and also with constant attribute values defined in the fixml-component-base schema file.

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                          Page 31 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# FIXML Schema File Versioning

FIXML Schema employed the file naming convention developed for FpML. The major and minor version numbers of the FIX version represented by the schema are appended to all FIXML schema file names. This approach was taken to explicitly force users to recognize when counterparties have changed their version of the schema.

# FIXML Message Versioning

The FIXML root element &#x3C;FIXML> contains three attributes that define the version of the message. The FIXML root element is defined in the fixml-components-base schema file.

| Attribute | Description                                                                       | Format   | Example  |
| --------- | --------------------------------------------------------------------------------- | -------- | -------- |
| v         | FIX Version                                                                       | N.N      | 4.4      |
| r         | FIX Version release date (used to designate errata releases between FIX versions) | YYYYMMDD | 20030618 |
| s         | Schema Release (used to designate schema releases between errata releases)        | YYYYMMDD | 20031030 |
| xv        | FIX Extension Pack number                                                         | EPN      | EP79     |
| xc        | Custom functionality, support of which required bilateral agreement.              |          |          |

Example:

&#x3C;FIXML v=”5.0” r=”20061024” s=”20061026”> &#x3C;/FIXML>

# Version

| Version | FIXML Field       | Abbreviation | FIX Tag | FIX Field Name | Discussion                 |
| ------- | ----------------- | ------------ | ------- | -------------- | -------------------------- |
| FIX.4.4 | Version           | v            | 8       | BeginString    | Version of FIX             |
| FIX.4.4 | Release           | r            |         |                | Release date of FIX        |
| FIX.4.4 | SchemaRelease     | s            |         |                | Release date of the Schema |
| FIX.4.4 | Extension Version | xv           |         |                | Extension version          |
| FIX.4.4 | Extension Release | xr           |         |                | Extension release date     |

# New fields in the standard header

| FIXT.1.1 | v  | 1128 | ApplVerID  | Indicates application version using a service pack identifier. The ApplVerID applies to a specific message |
| -------- | -- | ---- | ---------- | ---------------------------------------------------------------------------------------------------------- |
| FIXT.1.1 | r  |      | deprecated | can be used to provide the version release date                                                            |
| FIXT.1.1 | xv | 1156 | ApplExtID  | Indicates the Extension Pack number being applied.                                                         |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 32 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# FIXT.1.1

|   | xc | 1129 | CstmApplVerID | Used to support bilaterally agreed custom functionality        |
| - | -- | ---- | ------------- | -------------------------------------------------------------- |
|   | xr |      | deprecated    | can be used to provide a release date for the extended version |

# FIXML Schema File Structure

Organization of files was driven largely by the requirement to support customization of the FIXML Schema per the requirements set forth by the FIXML Schema Working Group. The basic organization of the schema has the datatypes used by the fields maintained in a separate file. FIX fields are defined in the shared file. Components and the FIXML root element are defined in the component files. FIXML messages are defined within separate category files.

# Layers of the FIXML Schema

| datatypes  | Defines FIXML Datatypes                                                  | fixml\~5-0-datatypes xsd    |
| ---------- | ------------------------------------------------------------------------ | --------------------------- |
| fields     | Defines FIXML Fields and enumerations                                    | fixml-5-O-fields-\* xsd     |
| components | Defines FIXML root elements and global component blocks                  | fixml-5-0-components-\* xsd |
| categories | Defines FIXML messages and components for each message category in FIXML | fixml-5-0-categoryName xsd  |

# Extensibility Design Pattern

Much of the design work that went into the FIXML Schema was done to permit counterparties to further refine the FIXML language either by restriction or extension. A possible scenario for restriction would be a market place that only supports a subset of the enumerations available for OrdType (tag=39). The exchange can override the OrdType_t FIXML datatype in the fixml-shared-impl-M-N.xsd file to restrict the set of possible values to only those supported by the market place. An example of extension would be counterparties that require an additional custom field to be added to a new message.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 33 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                 August 18, 2011

In order to provide a uniform method in defining customizations that could be readily absorbed by counterparties an extensibility design pattern was developed that defines how the FIXML definition was partitioned and organized within separate schema files.

Each level of schema file (with the exception of datatypes) provides a base definition file that defines the standard (default) FIXML language. Redefining this base file an implementation file (“impl”) is provided that by default simply references the base definition.

# Extensibility Pattern

| type-base | Contains the base definition for part of FIXML            | Base files should be treated as read only                     |
| --------- | --------------------------------------------------------- | ------------------------------------------------------------- |
| type-impl | Can be modified to restrict or extend the base definition | \~Modification by agreement between counterparties or markets |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                         Page 34 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                  August 18, 2011

# Schema File Hierarchy

| datatypes         | Contains the base definition for fields                            |
| ----------------- | ------------------------------------------------------------------ |
| fields-base       | Used to redefine existing fields, add fields; and add enumerations |
| components-base   | Contains the base definition for components                        |
| components-impl   | Used to extend, restrict components                                |
| categoryName-base |                                                                    |
| categoryName-impl |                                                                    |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                         Page 35 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Subsequent levels of the schema reference the impl from the previous level – thus providing a customization entry point at the field level, component level, and message level.

# Customizable and read-only schema files

| fixml-datatypes-m-n       | Defines basic FIXML data types design                 | read only                                                      | decision to not permit redefinition          |
| ------------------------- | ----------------------------------------------------- | -------------------------------------------------------------- | -------------------------------------------- |
| fixml-fields-base-m-n     | Defines the standard fields and enumerations for IXML | read Only                                                      |                                              |
| ffixml-fields-impl-m-n    | customizable                                          | Redefines shared-base                                          | Adds types for fields with enumerations      |
| fixml-components-base-m-n | New fields are added here                             | read Only                                                      | Enumerations are extended or restricted here |
| fixml-components-impl-mtr | customizable                                          | Defines component blocks and FIXML root element, header fields |                                              |

# Category Files

| fixml-order-base-m-n      | read Only    | |
| ------------------------- | ------------ |---|
| fixml-allocation-base-n-= | read Only    | |
| fixml-marketdata-base-m-n | read Only    | |
| fixml-category-base-m-I   | read Only    | |
| fixml-order-impl-m-n      | customizable | |
| xml-allocation-impl-m-\|  | customizable |
| kml-marketdata-impl-m-    | customizable | |
| xml-category-impl-m-F     | customizable | |

# FIXML Schema file naming conventions

FIXML file naming conventions are shown in the following illustration.

- All filenames begin with lowercase “fixml-”
- “-” is used to separate portions of the filename
- The type of the schema file is identified in the second component of the file name. The datatypes file contains the basic datatypes used within FIXML. The shared files contain the definitions for FIX fields. The components file contains definitions for FIXML components (as defined in Volume 1 of the specification, additional components identified while defining the FIXML schema, and the outer elements for FIX.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 36 of 158
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 1

# August 18, 2011



Files are either a base file or an implementation (impl). Base files define the standard FIXML language. Impl files are used to extend or restrict the base FIXML language.

# Schema File Naming Conventions

fixml-Type-{base impl}-m-n.xsd

- Type is one of
- - datatypes
- fields
- components
- category - where category is one of the FTX message categories, such as confirmation, listorder; order; settlement, etc.

m is the FIX Major Version number; such as &#x3C;5
- n is the FIX Minor Version number; such as *0

# Example File Names

- Fields base file for FIX Version 4.4: fixml-fields-base-5-Oxsd
- Order Category base file for FIX Version 4.4: fixml-order_base-5-Oxsd
- Component implementation file for FIX Version 4.4: fixml-components-impl-5-=Qxsd

Refer to the FIXML Schema File Summary section for a complete list of schema files used in FIXML as of FIX release 4.4.

# Datatypes schema file

A decision was made to use native XML Schema datatypes wherever possible. Many of the XML Schema standards are based upon ISO standard datatypes. This means that the FIX representation of UTCTimestamp is different from the FIXML representation. The FIXML Schema working group felt it more important to be compatible with XML and as a result XML toolsets. The requirement for conversion between FIX tag=value datatypes and XML is left to implementors.

The fixml-datatypes schema file contains definitions for the FIXML datatypes.

FIX 5.0 introduces pattern datatypes that are used to appropriately support customization of enumerations and also to support types that require both enumerations and specific patterns, such as the SettlementType field. The &#x3C;xs:union> element is used to combine an enumerated type with a pattern type in the fixml-fields-impl-M-N.xsd file.


© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 37 of 158


---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                August 18, 2011

# Errata

The following patterns have been created to support validation of user defined enumeration values and extended patterns.

| Tenor            | Pattern | Description                                                                                                                 |
| ---------------- | ------- | --------------------------------------------------------------------------------------------------------------------------- |
| Tenor            |         | Currently used to support the SettlementType which can be either an enumeration or a tenor pattern, such as M6 (six month). |
| Reserved100Plus  |         | Used for enumerated fields that permit user defined values of 100 and greater.                                              |
| Reserved1000Plus |         | Used for enumerated fields that permit user defined values of 1000 and greater.                                             |
| Reserved4000Plus |         | Used for enumerated fields that permit user defined values of 4000 and greater.                                             |

Example union types from fixml-fields-impl-M-N.xsd:

<xs:simpletype name="SettlType_t">The Settlement type is a union of the settlement type enumerations and the Tenor type described above</xs:simpletype>

<xs:simpletype name="OrdRejReason_t">The OrderRejectReason field is a union of the OrderReject Reason enumerations and can also be extended with user defined values of 100 or greater.</xs:simpletype>

# Fields schema files

- Fields schema file (fixml-fields-*-M-N.xsd)
- Fields base file (fixml-fields-base-M-N.xsd)

The fixml-fields-base file contains simple type definitions for all FIX application level fields and session level fields that are used as part of the FIXML header. All fields are defined as simple types. The simple type name is derived from the full FIX field name appended with a “_t”. All fields with enumerations are defined as simple types. The enumeration simple type name is derived from the full FIX field name appended with a “enum_t”.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                     Page 38 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Field definition examples

An example of a field definition for the AvgPx (tag=6) field:

<xs:simpletype name="AvgPx_t">
<xs:annotation>
<xs:documentation xml:lang="en">Calculated average price of all fills on this order For Fixed Income trades AvgPx is always expressed as percent of par regardless of the PriceType 423 of LastPx 3 I e AvgPx will contain an average of percent of par values see LastParPx 669 for issues traded in Yield Spread or Discount</xs:documentation>
<xs:appinfo xmlns:x="http://www.fixprotocol.org/fixml/metadata.xsd">
<xs:xref protocol="FIX" name="AvgPx" tag="6" data-type="Price" componenttype="Field">
<xs:xref protocol="ISO_15022_XML">
</xs:xref></xs:xref></xs:appinfo>
</xs:annotation>
<xs:restriction base="Price">
</xs:restriction></xs:simpletype>

An example of an enumerated field:

<xs:simpletype name="CommType_enum_t">
<xs:annotation>
<xs:documentation xml:lang="en">Commission type Valid values: = per unit implying shares par currency etc 2 = percentage 3 = absolute total monetary amount 4 = for CIV buy orders percentage waived cash discount 5 = for CIV buy orders percentage waived enhanced units 6 = points per bond or or contract Supply ContractMultiplier 23 in the Instrument component block if the object security is denominated in a size other than the industry default 000 par for bonds</xs:documentation>
<xs:appinfo xmlns:x="http://www.fixprotocol.org/fixml/metadata.xsd">
<xs:xref protocol="FIX" name="CommType" tag="13" data-type="char" componenttype="Field">
<xs:xref protocol="ISO_15022_XML">
</xs:xref></xs:xref></xs:appinfo>
<xs:appinfo xmlns:x="http://www.fixprotocol.org/fixml/metadata.xsd">
<x:enumdoc value="1" desc="PerShare">
<x:enumdoc value="2" desc="Percent">
<x:enumdoc value="3" desc="Absolute">
<x:enumdoc value="4" desc="PctWaivedCshDisc">
<x:enumdoc value="5" desc="PctWaivedEnUnits">
<x:enumdoc value="6" desc="PerBond">
</x:enumdoc></x:enumdoc></x:enumdoc></x:enumdoc></x:enumdoc></x:enumdoc></xs:appinfo>
</xs:annotation>
<xs:restriction base="xs:string">
<xs:enumeration value="1">
<xs:enumeration value="2">
<xs:enumeration value="3">
<xs:enumeration value="4">
<xs:enumeration value="5">
<xs:enumeration value="6">
</xs:enumeration></xs:enumeration></xs:enumeration></xs:enumeration></xs:enumeration></xs:enumeration></xs:restriction>
</xs:simpletype>

© Copyright, 2008-2009, FIX Protocol, Limited Page 39 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# Fields implementation file (fixml-fields-impl-M-N.xsd)

One of the more convoluted constructs used was the need to place the field level definitions for enumerated types in the fixml-fields-impl file. As shown above, the fixml-fields-base file defines each enumerated field as a simple type named fieldname_enum_t. This enumerated type is then used to define a corresponding field type in the fixml-fields-impl schema file named fieldname_t. It is this fieldname_t type that is referenced in subsequent schema files (fixml-components and the message category schema files). This construct was required to provide a mechanism to extend enumerations. The fieldname_t can be modified in the fixml-fields-impl file to include additional enumerations. The fieldname_t can be restricted by redefining the fieldname_enum_t simple type within the fixed-shared-impl file.

# Components (fixml-components-*-M-N.xsd)

Component files are used to define the reusable components that are used across FIX messages. The FIXML root element and headers are defined in the components file, as well.

# Components base file (fixml-components-base-M-N.xsd)

The fixml-components-base file contains the definitions for all FIX component blocks defined in volume 1 of the FIX specification. The FIXML root element, FIXML headers, the batch element, and the abstract message type are also defined within this file. Components (and messages) are defined using element groups and attribute groups. The advantage of these groups is that you can redefine the groups (using either restriction or extension) to change the overall structure of the component (or message). These groups are defined for each component and message.

| componentOrMessageNameElements   | Contains a list of elements contained in the component.   |
| -------------------------------- | --------------------------------------------------------- |
| componentOrMessageNameAttributes | Contains a list of Attributes contained in the component. |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                       Page 40 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                 August 18, 2011

# Parties Component

The Parties Component block is shown below. Notice the overall definition pattern. This pattern is followed for all component blocks and message definitions.

# Groups

<xs:group name="PartiesElementsRequired">
<xs:sequence>
</xs:sequence></xs:group>
<xs:group name="PartiesElementsOptional">
<xs:sequence>
<xs:element name="PtySub" type="PtysSubGrp_Block_t" minoccurs="0" maxoccurs="unbounded">
</xs:element></xs:sequence>
</xs:group>
<xs:group name="PartiesElementsCustom">
<xs:sequence>
</xs:sequence></xs:group>

# Attribute Groups

<xs:attributegroup name="PartiesAttributesRequired">
</xs:attributegroup>
<xs:attributegroup name="PartiesAttributesOptional">
<xs:attribute name="ID" type="PartyID_t" use="optional">
<xs:attribute name="IDSrc" type="PartyIDSource_t" use="optional">
<xs:attribute name="Role" type="PartyRole_t" use="optional">
</xs:attribute></xs:attribute></xs:attribute></xs:attributegroup>
<xs:attributegroup name="PartiesAttributesCustom">
</xs:attributegroup>

# Complex Type

<xs:complextype name="Parties_Block_t" final="#all">
<xs:annotation>
<xs:documentation xml:lang="en">**Desc**</xs:documentation>
<xs:appinfo>
<fm:xref protocol="FIX" name="Parties" componenttype="BlockRepeating">
<xs:xref protocol="ISO_15022_XML">
</xs:xref></fm:xref></xs:appinfo>
</xs:annotation>
<xs:sequence>
<xs:group ref="PartiesElementsRequired">
<xs:group ref="PartiesElementsOptional">
<xs:group ref="PartiesElementsCustom">
</xs:group></xs:group></xs:group></xs:sequence>
<xs:attributegroup ref="PartiesAttributesRequired">
<xs:attributegroup ref="PartiesAttributesOptional">
<xs:attributegroup ref="PartiesAttributesCustom">
</xs:attributegroup></xs:attributegroup></xs:attributegroup></xs:complextype>

# Components Implementation File

The default version fixml-components-impl file simply redefines the components-base file. This is the file where modifications (restrictions or extensions) would be made to component blocks used in the FIX protocol.

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                          Page 41 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                         August 18, 2011

# Categories (fixml-categoryName-base-M-N.xsd)

Each message category defined within the FIX specification has its own schema file. This provides a granular level of usage for applications only requiring access to one message category. The message category schema files contain the component and message definitions that belong to a specific message category defined within the FIX Protocol. Examples of message categories include: Indications, Market Data, Positions, Allocation. A complete list of the category files for FIXML is provided in the FIXML Schema File Summary section.

Category messages and components are defined following the same pattern defined above for components. The following defines the New Order Single message from the fixml-categoryOrder-5-0.xsd:

# New Order Single Elements Required

<xs:group name="NewOrderSingleElementsRequired">
<xs:sequence>
<xs:element name="Instrmt" type="Instrument_Block_t" minoccurs="1" maxoccurs="1">
<xs:element name="OrdQty" type="OrderQtyData_Block_t" minoccurs="1" maxoccurs="1">
</xs:element></xs:element></xs:sequence>
</xs:group>

# New Order Single Elements Optional

<xs:group name="NewOrderSingleElementsOptional">
<xs:sequence>
<xs:element name="Pty" type="Parties_Block_t" minoccurs="0" maxoccurs="unbounded">
<xs:element name="Stip" type="Stipulations_Block_t" minoccurs="0" maxoccurs="unbounded">
<xs:element name="FinDetls" type="FinancingDetails_Block_t" minoccurs="0" maxoccurs="1">
<xs:element name="SprdBnchmkCurve" type="SpreadOrBenchmarkCurveData_Block_t" minoccurs="0" maxoccurs="1">
<xs:element name="Yield" type="YieldData_Block_t" minoccurs="0" maxoccurs="1">
<xs:element name="Comm" type="CommissionData_Block_t" minoccurs="0" maxoccurs="1">
<xs:element name="PegInstr" type="PegInstructions_Block_t" minoccurs="0" maxoccurs="1">
<xs:element name="DiscInstr" type="DiscretionInstructions_Block_t" minoccurs="0" maxoccurs="1">
<xs:element name="PreAll" type="PreAllocGrp_Block_t" minoccurs="0" maxoccurs="unbounded">
<xs:element name="TrdSes" type="TrdgSesGrp_Block_t" minoccurs="0" maxoccurs="unbounded">
<xs:element name="Undl" type="UndInstrmtGrp_Block_t" minoccurs="0" maxoccurs="unbounded">
</xs:element></xs:element></xs:element></xs:element></xs:element></xs:element></xs:element></xs:element></xs:element></xs:element></xs:element></xs:sequence>
</xs:group>

# New Order Single Elements Custom

<xs:group name="NewOrderSingleElementsCustom">
<xs:sequence>
</xs:sequence></xs:group>

# New Order Single Attributes Required

<xs:attributegroup name="NewOrderSingleAttributesRequired">
<xs:attribute name="ClOrdID" type="ClOrdID_t" use="required">
<xs:attribute name="Side" type="Side_t" use="required">
<xs:attribute name="TransactTm" type="TransactTime_t" use="required">
<xs:attribute name="OrdTyp" type="OrdType_t" use="required">
</xs:attribute></xs:attribute></xs:attribute></xs:attribute></xs:attributegroup>

# New Order Single Attributes Optional

<xs:attributegroup name="NewOrderSingleAttributesOptional">
<xs:attribute name="ScndClOrdID" type="SecondaryClOrdID_t" use="optional">
<xs:attribute name="ClOrdLinkID" type="ClOrdLinkID_t" use="optional">
<xs:attribute name="TrdOrigntnDt" type="TradeOriginationDate_t" use="optional">
<xs:attribute name="TrdDt" type="TradeDate_t" use="optional">
<xs:attribute name="Acct" type="Account_t" use="optional">
</xs:attribute></xs:attribute></xs:attribute></xs:attribute></xs:attribute></xs:attributegroup>

© Copyright, 2008-2009, 2011, FIX Protocol, Limited                                     Page 42 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

# Attributes

<xs:attribute name="AcctIDSrc" type="AcctIDSource_t" use="optional"></xs:attribute>
<xs:attribute name="AcctTyp" type="AccountType_t" use="optional"></xs:attribute>
<xs:attribute name="DayBkngInst" type="DayBookingInst_t" use="optional"></xs:attribute>
<xs:attribute name="BkngUnit" type="BookingUnit_t" use="optional"></xs:attribute>
<xs:attribute name="PreallocMethod" type="PreallocMethod_t" use="optional"></xs:attribute>
<xs:attribute name="AllocID" type="AllocID_t" use="optional"></xs:attribute>
<xs:attribute name="SettlTyp" type="SettlType_t" use="optional"></xs:attribute>
<xs:attribute name="SettlDt" type="SettlDate_t" use="optional"></xs:attribute>
<xs:attribute name="CshMgn" type="CashMargin_t" use="optional"></xs:attribute>
<xs:attribute name="ClrngFeeInd" type="ClearingFeeIndicator_t" use="optional"></xs:attribute>
<xs:attribute name="HandlInst" type="HandlInst_t" use="optional"></xs:attribute>
<xs:attribute name="ExecInst" type="ExecInst_t" use="optional"></xs:attribute>
<xs:attribute name="MinQty" type="MinQty_t" use="optional"></xs:attribute>
<xs:attribute name="MaxFloor" type="MaxFloor_t" use="optional"></xs:attribute>
<xs:attribute name="ExDest" type="ExDestination_t" use="optional"></xs:attribute>
<xs:attribute name="ProcCode" type="ProcessCode_t" use="optional"></xs:attribute>
<xs:attribute name="PrevClsPx" type="PrevClosePx_t" use="optional"></xs:attribute>
<xs:attribute name="LocReqd" type="LocateReqd_t" use="optional"></xs:attribute>
<xs:attribute name="QtyTyp" type="QtyType_t" use="optional"></xs:attribute>
<xs:attribute name="PxTyp" type="PriceType_t" use="optional"></xs:attribute>
<xs:attribute name="Px" type="Price_t" use="optional"></xs:attribute>
<xs:attribute name="StopPx" type="StopPx_t" use="optional"></xs:attribute>
<xs:attribute name="Ccy" type="Currency_t" use="optional"></xs:attribute>
<xs:attribute name="ComplianceID" type="ComplianceID_t" use="optional"></xs:attribute>
<xs:attribute name="SolFlag" type="SolicitedFlag_t" use="optional"></xs:attribute>
<xs:attribute name="IOIID" type="IOIID_t" use="optional"></xs:attribute>
<xs:attribute name="QID" type="QuoteID_t" use="optional"></xs:attribute>
<xs:attribute name="TmInForce" type="TimeInForce_t" use="optional"></xs:attribute>
<xs:attribute name="EfctvTm" type="EffectiveTime_t" use="optional"></xs:attribute>
<xs:attribute name="ExpireDt" type="ExpireDate_t" use="optional"></xs:attribute>
<xs:attribute name="ExpireTm" type="ExpireTime_t" use="optional"></xs:attribute>
<xs:attribute name="GTBkngInst" type="GTBookingInst_t" use="optional"></xs:attribute>
<xs:attribute name="Cpcty" type="OrderCapacity_t" use="optional"></xs:attribute>
<xs:attribute name="Rstctions" type="OrderRestrictions_t" use="optional"></xs:attribute>
<xs:attribute name="CustOrdCpcty" type="CustOrderCapacity_t" use="optional"></xs:attribute>
<xs:attribute name="ForexReq" type="ForexReq_t" use="optional"></xs:attribute>
<xs:attribute name="SettlCcy" type="SettlCurrency_t" use="optional"></xs:attribute>
<xs:attribute name="BkngTyp" type="BookingType_t" use="optional"></xs:attribute>
<xs:attribute name="Txt" type="Text_t" use="optional"></xs:attribute>
<xs:attribute name="EncTxtLen" type="EncodedTextLen_t" use="optional"></xs:attribute>
<xs:attribute name="EncTxt" type="EncodedText_t" use="optional"></xs:attribute>
<xs:attribute name="SettlDt2" type="SettlDate2_t" use="optional"></xs:attribute>
<xs:attribute name="Qty2" type="OrderQty2_t" use="optional"></xs:attribute>
<xs:attribute name="Px2" type="Price2_t" use="optional"></xs:attribute>
<xs:attribute name="PosEfct" type="PositionEffect_t" use="optional"></xs:attribute>
<xs:attribute name="CoveredOrUncovered" type="CoveredOrUncovered_t" use="optional"></xs:attribute>
<xs:attribute name="MaxShow" type="MaxShow_t" use="optional"></xs:attribute>
<xs:attribute name="TgtStrategy" type="TargetStrategy_t" use="optional"></xs:attribute>
<xs:attribute name="TgtStrategyParameters" type="TargetStrategyParameters_t" use="optional"></xs:attribute>
<xs:attribute name="ParticipationRt" type="ParticipationRate_t" use="optional"></xs:attribute>
<xs:attribute name="CxllationRights" type="CancellationRights_t" use="optional"></xs:attribute>
<xs:attribute name="MnyLaunderingStat" type="MoneyLaunderingStatus_t" use="optional"></xs:attribute>
<xs:attribute name="RegistID" type="RegistID_t" use="optional"></xs:attribute>
<xs:attribute name="Designation" type="Designation_t" use="optional">

© Copyright, 2008-2009, FIX Protocol, Limited                                     Page 43 of 158
</xs:attribute>
---

Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011


# Categories (fixml-categoryName-impl-M-N.xsd)

Each message category defined within the FIX specification has its own schema file. This provides a granular level of usage for applications only requiring access to one message category. A complete list of the category files for FIXML is provided below in the FIXML File Summary table.

# Trading Life Cycle files

Convenience files are provided with the FIXML schema version that includes the message categories for each of the trade life cycles (pre-trade, trade, post-trade) used by FIX. These files are provided to make it easier for applications that require access to multiple message categories within one of the trading life cycles.

- Pretrade file (fixml-pretrade-M-N.xsd)
Includes the pre-trade message category implementation files.
- Trade file (fixml-trade-M-N.xsd)
Includes the trade message category implementation files.
- Post trade file (fixml-trade-M-N.xsd)
Includes the post trade message category implementation files.

# Main (fixml-main-M-N.xsd)

A main schema file is included that pulls in the pretrade, trade, and post trade schema files. This is provided for applications that require access to the full suite of FIX messages.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited


Page 44 of 158

---
Version 5.0 Service Pack 2 - Errata    VOLUME 1                                                 August 18, 2011

The FIXML Schema files have been organized to permit extensibility. Implementation versions of each schema file (with the exception of the datatypes file) are provided to permit users to redefine the base FIXML Schema version, as defined in the base files. This section provides guidelines for customizing the FIXML syntax. Even though a considerable amount of work has gone into making FIXML extensible, users are strongly encouraged to minimize modifications, in order to promote more consistent usage of the FIXML syntax within the industry. Obviously, the less customization, the easier it is to connect to counterparties. If customization is required, you are encouraged to communicate your requirements that are not being met by FIX to the FPL Global Technical Committee. There you may find out that there is a technique to meet your business requirement. Or, you may find that the Technical Committee has already addressed the issue for a planned future release. At a minimum you will receive coaching and assistance in how to extend FIXML in such a way as to make the new feature a part of a future version of FIX.

# Defining a custom field

New fields are defined as an XML SimpleType in the fixml-shared-impl-N-N.xsd file. You are recommended to add the file to the end of the schema document. You also are strongly encouraged to include XML comments to define the reason for the field. The field should then be added to the component or message where it will be used, once the field is defined in the fixml-shared-impl schema file. If the field will be added to a component contained in fixml-components-base-N-N.xsd, you must now redefine that component in the fixml-components-impl-N-N.xsd file. Adding a field to a component or message contained in one of the message categories is done in the same way you modify the components schema file. You need to redefine the portion of the message in the implementation version of the file. You are encouraged to follow the same procedure for procuring new custom field names as is done for the FIX tag=value version of FIX. The FIX website provides a web page of custom fields and a form to submit requests for additional custom fields.

# Restricting enumeration values for a FIX field

Restricting enumeration values is done by modifying the type definition in the fixml-shared-impl schema file.

# Extending enumeration values for a FIX field

Extending enumeration values is done by creating a union of the original enumeration type definition with new enumeration values.

# Making an optional field required

Making an optional field required is done by redefining the optional attribute group, modifying the usage of the field from “optional” to “required”. This redefinition is done within the implementation file for either the components or a particular message category.

# Making a required field optional

It is not possible to make a required field optional without modifying the original required element or attribute group. Making required fields optional does go against the standard base definition of FIX and should be avoided.

# Adding a custom message

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                         Page 45 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

Custom messages are added by creating a message structure within the category to which the custom message belongs. Required and optional element and attribute groups should be created for the custom message.

# FIXML Schema Version Datatypes

| Type        | BaseType | FIXML Implementation                     | Example       |
| ----------- | -------- | ---------------------------------------- | ------------- |
| int         |          | Use builtin type: xs:integer             |               |
| Length      | int      | ~~~~~~~~~~~~                             |               |
| TagNum      | int      | NOT REQUIRED IN FIXML                    |               |
| SeqNum      | int      | ~~~~~~~~~~~~                             |               |
| NumInGroup  | int      | NOT REQUIRED IN FIXML                    |               |
| DayOfMonth  | int      | NOT REQUIRED IN FIXML                    |               |
| float       |          | Use builtin type: xs:decimal             |               |
| Qty         | float    | ~~~~~~~~~~~~Use builtin type: xs:decimal |               |
| Price       | float    | ~~~~~~~~~~~~Use builtin type: xs:decimal | Strk="47.50"  |
| PriceOffset | float    | ~~~~~~~~~~~~Use builtin type: xs:decimal |               |
| Amt         | float    | ~~~~~~~~~~~~Use builtin type: xs:decimal | Amt="6847.00" |
| Percentage  | float    | ~~~~~~~~~~~~Use builtin type: xs:decimal |               |
| char        |          | ~~~~~~~~~~~~Use builtin type: xs:string  |               |
| Boolean     | char     | ~~~~~~~~                                 |               |

© Copyright, 2008-2011, FIX Protocol, Limited                                    Page 46 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                    August 18, 2011

# Errata

| Type                | BaseT                       | FIXML Implementation | Example                                                        |
| ------------------- | --------------------------- | -------------------- | -------------------------------------------------------------- |
| String              | Use builtin type: xs:string | ~~~~                 | ~~~~                                                           |
| MultipleCharValue   | String                      | ~~~~~~~~~~~~~~~~~~~~ | Use builtin type: xs:string                                    |
| MultipleStringValue | String                      | ~~~~~~~~~~~~~~~~~~~~ | Use builtin type: xs:string                                    |
| Country             | String                      | ~~~~~~~~~~~~~~~~~~~~ | Use builtin type: xs:string                                    |
| Currency            | String                      | ~~~~~~~~~~~~~~~~~~~~ | StrkCcy="USD"                                                  |
| Exchange            | String                      | ~~~~~~~~~~~~~~~~~~~~ | Use builtin type: xs:string                                    |
| MonthYear           | String                      | ~~~~~~~~~~~~~~~~~~~~ | MonthYear="200303", MonthYear="20030320", MonthYear="200303w2" |
| UTCTimestamp        | String                      | ~~~~~~~~             | TransactTm="2001-12-17T09:30:47-05:00"                         |
| UTCTimeOnly         | String                      | ~~~~~~~~~~~~~~~~     | MDEntryTime="13:20:00.000-05:00"                               |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited                                             Page 47 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

# Type

| Base Type        | FIXML Implementation                                                      | Example                          |
| ---------------- | ------------------------------------------------------------------------- | -------------------------------- |
| UTCDateOnly      | ~~~~~~~~~~~~Use builtin type: xs:date                                     | MDEntryDate="2003-09-10"         |
| LocalMktDate     | ~~~~~~~~~~~~Use builtin type: xs:date                                     | BizDate="2003-09-10"             |
| TZTimeOnly       | ~~~~~~~~~~~~Use builtin type: xs:time                                     |                                  |
| TZTimestamp      | ~~~~~~~~~~~~Use builtin type: xs:dateTime                                 |                                  |
| data             | ~~~~~~~~~~~~Use builtin type: xs:string                                   |                                  |
| XMLData          | ~~~~~~~~~~~~Use builtin type: xs:string                                   |                                  |
| Language         | ~~~~~~~~~~~~Use builtin type: xs:language                                 | en (English), es (spanish), etc. |
| Pattern          | NOT REQUIRED IN FIXML                                                     |                                  |
| Tenor            | ~~~~~~~~~~~~~~~~Use builtin type: xs:string                               |                                  |
| Reserved100Plus  | ~~~~name="Reserved100Plus">~~ ~~~~~~~~~~~~~~Use builtin type: xs:integer  |                                  |
| Reserved1000Plus | ~~~~name="Reserved1000Plus">~~ ~~~~~~~~~~~~~~Use builtin type: xs:integer |                                  |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                    Page 48 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# FIXML Schema File Summary

| Type             | Base Type | FIXML Implementation                                                                                                                                                   | Example                      |
| ---------------- | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| Reserved4000Plus | Pattern   | ~~\<xs:simpleType~~ ~~name="Reserved4000Plus">~~ ~~\<xs:restriction base="xs:integer">~~ ~~\<xs:minInclusive value="4000"/>~~ ~~\</xs:restriction> \</xs:simpleType>~~ | Use builtin type: xs:integer |

# File Name

# Description

| Fixml-datatypes-5-0-SP2.xsd            | Defines the base data types that are to be used in other fixml schema files. These fixml base data types are based on simple types built into XML Schema.                                                                                               |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Fixml-session-base-5-0-SP2.xsd         | Includes Fixml--base-5-0-SP2.xsd. Defines Session messages: Heartbeat, Logon, Logout, Reject, ResendRequest, SequenceReset, TestRequest, ~~XML\_non\_FIX~~ XMLnonFIX                                                                                    |
| Fixml-indications-base-5-0-SP2.xsd     | Includes Fixml-components-base-5-0-SP2.xsd. Defines Indication messages: Advertisement, IOI                                                                                                                                                             |
| ~~Fixml-indications-impl-5-0-SP2.xsd~~ | ~~Includes FIX50-components-impl-5-0-SP2.xsd. Used to customise the Indication message category.~~                                                                                                                                                      |
| Fixml-order-base-5-0-SP2.xsd           | Includes Fixml-components-base-5-0-SP2.xsd. Defines SingleGeneralOrderHandling messages: ~~DontKnowTradeDK~~ DontKnowTrade, ExecutionAcknowledgement, ExecutionReport, NewOrderSingle, OrderCancelReject, OrderCancelReplaceRequest, OrderCancelRequest |

© Copyright, 2008-2011, FIX Protocol, Limited Page 49 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# OrderStatusRequest

~~Fixml-order-impl-5-0-SP2.xsd~~ ~~Includes FIX50-components-impl-5-0-SP2.xsd.~~ ~~Used to customise the SingleGeneralOrderHandling message category.~~

# Fixml-newsevents-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines EventCommunication messages:

- Email
- News

~~Fixml-newsevents-impl-5-0-SP2.xsd~~ ~~Includes FIX50-components-impl-5-0-SP2.xsd.~~ ~~Used to customise the EventCommunication message category.~~

# Fixml-listorders-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines ProgramTrading messages:

- BidRequest
- BidResponse
- ListCancelRequest
- ListExecute
- ListStatus
- ListStatusRequest
- ListStrikePrice
- NewOrderList

~~Fixml-listorders-impl-5-0-SP2.xsd~~ ~~Includes FIX50-components-impl-5-0-SP2.xsd.~~ ~~Used to customise the ProgramTrading message category.~~

# Fixml-ordermasshandling-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines OrderMassHandling messages:

- OrderMassActionReport
- OrderMassActionRequest
- OrderMassCancelReport
- OrderMassCancelRequest
- OrderMassStatusRequest

~~Fixml-ordermasshandling-impl-5-0-SP2.xsd~~ ~~Includes FIX50-components-impl-5-0-SP2.xsd.~~ ~~Used to customise the OrderMassHandling message category.~~

# Fixml-allocation-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines Allocation messages:

- AllocationInstruction
- AllocationInstructionAck
- AllocationInstructionAlert
- AllocationReport
- AllocationReportAck

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 50 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Version 5.0 Service Pack 2 - Errata

# Fixml-allocation-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd.

Used to customise the Allocation message category.

# Fixml-quotation-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines QuotationNegotiation messages:

- MassQuote
- MassQuoteAcknowledgement
- Quote
- QuoteCancel
- QuoteRequest
- QuoteRequestReject
- QuoteResponse
- QuoteStatusReport
- QuoteStatusRequest
- RFQRequest

# Fixml-settlement-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines SettlementInstruction messages:

- SettlementInstructionRequest
- SettlementInstructions
- SettlementObligationReport

# Fixml-marketdata-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines MarketData messages:

- MarketDataIncrementalRefresh
- MarketDataRequest
- MarketDataRequestReject
- MarketDataSnapshotFullRefresh
- StreamAssignmentReport
- StreamAssignmentReportACK
- StreamAssignmentRequest

# Fixml-components-base-5-0-SP2.xsd

Includes Fixml-fields-base-5-0-SP2.xsd. Defines Common messages:

# Fixml-components-impl-5-0-SP2.xsd

Includes FIX50-fields-impl-5-0-SP2.xsd.

Used to customise the Common message category.

© Copyright, 2008-2011, FIX Protocol, Limited Page 51 of 158


---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Fixml-registration-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines RegistrationInstruction messages:

- RegistrationInstructions
- RegistrationInstructionsResponse

# Fixml-crossorders-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines CrossOrders messages:

- CrossOrderCancelReplaceRequest
- CrossOrderCancelRequest
- NewOrderCross

# Fixml-multilegorders-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines MultilegOrders messages:

- MultilegOrderCancelReplace
- NewOrderMultileg

# Fixml-tradecapture-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines TradeCapture messages:

- TradeCaptureReport
- TradeCaptureReportAck
- TradeCaptureReportRequest
- TradeCaptureReportRequestAck

# Fixml-confirmation-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines Confirmation messages:

- Confirmation
- ConfirmationAck
- ConfirmationRequest

© Copyright, 2008-2011, FIX Protocol, Limited Page 52 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Fixml-positions-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines PositionMaintenance messages:

- AdjustedPositionReport
- AssignmentReport
- ContraryIntentionReport
- PositionMaintenanceReport
- PositionMaintenanceRequest
- PositionReport
- RequestForPositions
- RequestForPositionsAck

# Fixml-positions-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd. Used to customise the PositionMaintenance message category.

# Fixml-collateral-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines CollateralManagement messages:

- CollateralAssignment
- CollateralInquiry
- CollateralInquiryAck
- CollateralReport
- CollateralRequest
- CollateralResponse

# Fixml-collateral-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd. Used to customise the CollateralManagement message category.

# Fixml-application-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines Application messages:

- ApplicationMessageReport
- ApplicationMessageRequest
- ApplicationMessageRequestAck

# Fixml-application-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd. Used to customise the Application message category.

# Fixml-businessreject-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines BusinessReject messages:

- BusinessMessageReject

# Fixml-businessreject-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd. Used to customise the BusinessReject message category.

# Fixml-network-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines Network messages:

- NetworkCounterpartySystemStatusRequest
- NetworkCounterpartySystemStatusResponse

© Copyright, 2008- 2009 2011, FIX Protocol, Limited Page 53 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Fixml-network-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd.

Used to customise the Network message category.

# Fixml-usermanagement-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines UserManagement messages:

- UserNotification
- UserRequest
- UserResponse

# Fixml-usermanagement-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd.

Used to customise the UserManagement message category.

# Fixml-fields-base-5-0-SP2.xsd

Includes Fixml--base-5-0-SP2.xsd. Defines Fields messages:

# Fixml-fields-base-5-0-SP2.xsd

Includes Fixml--base-5-0-SP2.xsd. Defines ~~Impl~~ ~~Fields~~ImplFields messages:

# Fixml-marketstructure-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines MarketStructureReferenceData messages:

- MarketDefinition
- MarketDefinitionRequest
- MarketDefinitionUpdateReport
- TradingSessionList
- TradingSessionListRequest
- TradingSessionListUpdateReport
- TradingSessionStatus
- TradingSessionStatusRequest

# Fixml-marketstructure-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd.

Used to customise the MarketStructureReferenceData message category.

# Fixml-securitiesreference-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd. Defines SecuritiesReferenceData messages:

- DerivativeSecurityList
- DerivativeSecurityListRequest
- DerivativeSecurityListUpdateReport
- SecurityDefinition
- SecurityDefinitionRequest
- SecurityDefinitionUpdateReport
- SecurityList
- SecurityListRequest
- SecurityListUpdateReport
- SecurityStatus

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 54 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# SecurityStatusRequest

# SecurityTypeRequest

# SecurityTypes

# Fixml-securitiesreference-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd.

Used to customise the SecuritiesReferenceData message category.

# Fixml-partiesreference-base-5-0-SP2.xsd

Includes Fixml-components-base-5-0-SP2.xsd.

Defines PartiesReferenceData messages:

- PartyDetailsListReport
- PartyDetailsListRequest

# Fixml-partiesreference-impl-5-0-SP2.xsd

Includes FIX50-components-impl-5-0-SP2.xsd.

Used to customise the PartiesReferenceData message category.

# Fixml-session-base-5-0-SP2.xsd

Session level messages to establish and control a FIX session

# Fixml-pretrade-base-5-0-SP2.xsd

Pre trade messages including reference data, market data, quoting, news and email, indication of interest

# Fixml-trade-base-5-0-SP2.xsd

Order handling and execution messages

# Fixml-posttrade-base-5-0-SP2.xsd

Post trade messages including trade reporting, allocation, collateral, confirmation, position maintenance, registration instruction, and settlement instructions

# Fixml-infrastructure-base-5-0-SP2.xsd

Infrastructure messages for application sequencing, business reject, network and user management

# Fixml-main-5-0-SP2.xsd

Includes the session, pretrade, trade, posttrade and infrastructure schema files

© Copyright, 2008-2011, FIX Protocol, Limited Page 55 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# COMMON COMPONENTS OF APPLICATION MESSAGES - Component Blocks

(Included in pre-trade, trade, and post-trade messages)

Many of the FIX Application Messages are composed of common "building blocks" or sets of data fields. For instance, almost every FIX Application Message has the set of symbology-related fields used to define the "Instrument": Symbol, SymbolSfx, SecurityIDSource, SecurityID….. EncodedSecurityDesc. Rather than replicate a common group of fields, the FIX specification specifies component blocks which are simply referenced by component name within each Application Message which uses them. Thus when reviewing a specific message definition, the appropriate group of fields should be expanded and used whenever a component block is identified. Note that some component blocks may be part of repeating groups thus if the component block is denoted as part of a repeating group, then the entire group of fields representing the component block are to be specified at the component block's repeating group "level" in the message definition and follow repeating group rules concerning field order. See "Repeating Groups" for more details.

The component blocks identified within this section of Volume 1 are referred to as "Common Components". They are component blocks that are commonly used across the various messages defined in Volumes 3, 4 and 5.

# Instrument (symbology) component block

The Instrument component block contains all the fields commonly used to describe a security or instrument. Typically the data elements in this component block are considered the static data of a security, data that may be commonly found in a security master database. The Instrument component block can be used to describe any asset type supported by FIX.

| Tag                                                                       | FieldName        | Req'd | Comments                                                                                                                                                                                                                          |
| ------------------------------------------------------------------------- | ---------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 55                                                                        | Symbol           | N     | Common, "human understood" representation of the security. SecurityID value can be specified if no symbol exists (e.g. non-exchange traded Collective Investment Vehicles). Use "\[N/A]" for products which do not have a symbol. |
| 65                                                                        | SymbolSfx        | N     | Used in Fixed Income with a value of "WI" to indicate "When Issued" for a security to be reissued under an old CUSIP or ISIN or with a value of "CD" to indicate a EUCP with lump-sum interest rather than discount price.        |
| 48                                                                        | SecurityID       | N     | Takes precedence in identifying security to counterparty over SecurityAltID block. Requires SecurityIDSource if specified.                                                                                                        |
| 22                                                                        | SecurityIDSource | N     | Required if SecurityID is specified.                                                                                                                                                                                              |
| component block \<SecAltIDGrp> N Number of alternate Security Identifiers |                  |       |                                                                                                                                                                                                                                   |
| 460                                                                       | Product          | N     | Indicates the type of product the security is associated with (high-level category)                                                                                                                                               |
| 1227                                                                      | ProductComplex   | N     | Identifies an entire suite of products for a given market. In Futures this may be "interest rates", "agricultural", "equity indexes", etc                                                                                         |
| 1151                                                                      | SecurityGroup    | N     | An exchange specific name assigned to a group of                                                                                                                                                                                  |

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                         Page 56 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Field Number | Field Name                            | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                               |
| ------------ | ------------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 461          | CFICode                               | N        | Indicates the type of security using ISO 10962 standard, Classification of Financial Instruments (CFI code) values. It is recommended that CFICode be used instead of SecurityType for non-Fixed Income instruments.                                                                                                                                                                                                      |
| 167          | SecurityType                          | N        | It is recommended that CFICode be used instead of SecurityType for non-Fixed Income instruments. Required for Fixed Income. Refer to Volume 7 - Fixed Income Futures and Options should be specified using the CFICode\[461] field instead of SecurityType\[167] (Refer to Volume 7 - Recommendations and Guidelines for Futures and Options Markets.)                                                                    |
| 762          | SecuritySubType                       | N        | Sub-type qualification/identification of the SecurityType (e.g. for SecurityType="MLEG"). If specified, SecurityType is required.                                                                                                                                                                                                                                                                                         |
| 200          | MaturityMonthYear                     | N        | Specifies the month and year of maturity. Applicable for standardized derivatives which are typically only referenced by month and year (e.g. S\&P futures). Note MaturityDate (a full date) can also be specified.                                                                                                                                                                                                       |
| 541          | MaturityDate                          | N        | Specifies date of maturity (a full date). Note that standardized derivatives which are typically only referenced by month and year (e.g. S\&P futures) may use MaturityMonthYear and/or this field. When using MaturityMonthYear, it is recommended that markets and sell sides report the MaturityDate on all outbound messages as a means of data enrichment. For NDFs this represents the fixing date of the contract. |
| 1079         | MaturityTime                          | N        | For NDFs this represents the fixing time of the contract. It is optional to specify the fixing time.                                                                                                                                                                                                                                                                                                                      |
| 966          | SettleOnOpenFlag                      | N        | Indicator to determine if Instrument is Settle on Open.                                                                                                                                                                                                                                                                                                                                                                   |
| 1049         | InstrmtAssignmentMethod               | N        |                                                                                                                                                                                                                                                                                                                                                                                                                           |
| 965          | SecurityStatus                        | N        | Gives the current state of the instrument.                                                                                                                                                                                                                                                                                                                                                                                |
| 224          | CouponPaymentDate                     | N        | Date interest is to be paid. Used in identifying Corporate Bond issues.                                                                                                                                                                                                                                                                                                                                                   |
| 1449         | RestructuringType                     | N        |                                                                                                                                                                                                                                                                                                                                                                                                                           |
| 1450         | Seniority                             | N        |                                                                                                                                                                                                                                                                                                                                                                                                                           |
| 1451         | NotionalPercentageOutstanding         | N        |                                                                                                                                                                                                                                                                                                                                                                                                                           |
| 1452         | OriginalNotionalPercentageOutstanding | N        |                                                                                                                                                                                                                                                                                                                                                                                                                           |
| 1457         | AttachmentPoint                       | N        |                                                                                                                                                                                                                                                                                                                                                                                                                           |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 57 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Tag  | Field Name                         | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ---- | ---------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1458 | DetachmentPoint                    | N        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 225  | IssueDate                          | N        | Date instrument was issued. For Fixed Income IOIs for new issues, specifies the issue date.                                                                                                                                                                                                                                                                                                                                                                      |
| 239  | RepoCollateralSecurityType         | N        | (Deprecated in FIX.4.4)                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 226  | RepurchaseTerm                     | N        | (Deprecated in FIX.4.4)                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 227  | RepurchaseRate                     | N        | (Deprecated in FIX.4.4)                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 228  | Factor                             | N        | For Fixed Income: Amortization Factor for deriving Current face from Original face for ABS or MBS securities, note the fraction may be greater than, equal to or less than 1. In TIPS securities this is the Inflation index. Qty \* Factor \* Price = Gross Trade Amount. For Derivatives: Contract Value Factor by which price must be adjusted to determine the true nominal value of one futures/options contract. (Qty \* Price) \* Factor = Nominal Value. |
| 255  | CreditRating                       | N        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 543  | InstrRegistry                      | N        | The location at which records of ownership are maintained for this instrument, and at which ownership changes must be recorded. Can be used in conjunction with ISIN to address ISIN uniqueness issues.                                                                                                                                                                                                                                                          |
| 470  | CountryOfIssue                     | N        | ISO Country code of instrument issue (e.g. the country portion typically used in ISIN). Can be used in conjunction with non-ISIN SecurityID (e.g. CUSIP for Municipal Bonds without ISIN) to provide uniqueness.                                                                                                                                                                                                                                                 |
| 471  | StateOrProvinceOfIssue             | N        | A two-character state or province abbreviation.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 472  | LocaleOfIssue                      | N        | The three-character IATA code for a locale (e.g. airport code for Municipal Bonds).                                                                                                                                                                                                                                                                                                                                                                              |
| 240  | RedemptionDate                     | N        | (Deprecated in FIX.4.4)                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| 202  | StrikePrice                        | N        | Used for derivatives, such as options and covered warrants.                                                                                                                                                                                                                                                                                                                                                                                                      |
| 947  | StrikeCurrency                     | N        | Used for derivatives.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| 967  | StrikeMultiplier                   | N        | Used for derivatives. Multiplier applied to the strike price for the purpose of calculating the settlement value.                                                                                                                                                                                                                                                                                                                                                |
| 968  | StrikeValue                        | N        | Used for derivatives. The number of shares/units for the financial instrument involved in the option trade.                                                                                                                                                                                                                                                                                                                                                      |
| 1478 | StrikePriceDeterminationMethod     | N        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 1479 | StrikePriceBoundaryMethod          | N        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 1480 | StrikePriceBoundaryPrecision       | N        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| 1481 | UnderlyingPriceDeterminationMethod | N        |                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 58 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                    August 18, 2011

# Errata

| Code | Description                     | Notes                                                                                                                                                                                                                                                 |
| ---- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 206  | OptAttribute                    | Used for derivatives, such as options and covered warrants to indicate a versioning of the contract when required due to corporate actions to the underlying. Should not be used to indicate type of option - use the CFICode\[461] for this purpose. |
| 231  | ContractMultiplier              | For Fixed Income, Convertible Bonds, Derivatives, etc. Note: If used, quantities should be expressed in the "nominal" (e.g. contracts vs. shares) amount.                                                                                             |
| 1435 | ContractMultiplierUnit          | N                                                                                                                                                                                                                                                     |
| 1439 | FlowScheduleType                | N                                                                                                                                                                                                                                                     |
| 969  | MinPriceIncrement               | Minimum price increment for the instrument. Could also be used to represent tick value.                                                                                                                                                               |
| 1146 | MinPriceIncrementAmount         | Minimum price increment amount associated with the MinPriceIncrement \[969]. For listed derivatives, the value can be calculated by multiplying MinPriceIncrement by ContractValueFactor \[231]                                                       |
| 996  | UnitOfMeasure                   | 0                                                                                                                                                                                                                                                     |
| 1147 | UnitOfMeasureQty                | N                                                                                                                                                                                                                                                     |
| 1191 | PriceUnitOfMeasure              | N                                                                                                                                                                                                                                                     |
| 1192 | PriceUnitOfMeasureQty           | N                                                                                                                                                                                                                                                     |
| 1193 | SettlMethod                     | Settlement method for a contract. Can be used as an alternative to CFI Code value                                                                                                                                                                     |
| 1194 | ExerciseStyle                   | Type of exercise of a derivatives security                                                                                                                                                                                                            |
| 1482 | OptPayoutType                   | N                                                                                                                                                                                                                                                     |
| 1195 | OptPayoutAmount                 | Cash amount indicating the pay out associated with an option. For binary options this is a fixed amount                                                                                                                                               |
| 1196 | PriceQuoteMethod                | N                                                                                                                                                                                                                                                     |
| 1197 | ValuationMethod                 | Indicates type of valuation method used.                                                                                                                                                                                                              |
| 1198 | ListMethod                      | Indicates whether the instruments are pre-listed only or can also be defined via user request                                                                                                                                                         |
| 1199 | CapPrice                        | Used to express the ceiling price of a capped call                                                                                                                                                                                                    |
| 1200 | FloorPrice                      | Used to express the floor price of a capped put                                                                                                                                                                                                       |
| 201  | PutOrCall                       | Used to express option right                                                                                                                                                                                                                          |
| 1244 | FlexibleIndicator               | Used to indicate if a security has been defined as flexible according to "non-standard" means. Analog to CFICode Standard/Non-standard indicator                                                                                                      |
| 1242 | FlexProductEligibilityIndicator | Used to indicate if a product or group of product supports the creation of flexible securities                                                                                                                                                        |
| 997  | TimeUnit                        | Used to indicate a time unit for the contract (e.g., days, weeks, months, etc.)                                                                                                                                                                       |
| 223  | CouponRate                      | For Fixed Income.                                                                                                                                                                                                                                     |

© Copyright, 2008-      ~~2009~~2011, FIX Protocol, Limited                                             Page 59 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                August 18, 2011

# 207

SecurityExchange

N

Can be used to identify the security.

# 970

PositionLimit

N

Position Limit for the instrument.

# 971

NTPositionLimit

N

Near-term Position Limit for the instrument.

# 106

Issuer

N

# 348

EncodedIssuerLen

N

Must be set if EncodedIssuer field is specified and must immediately precede it.

# 349

EncodedIssuer

N

Encoded (non-ASCII characters) representation of the Issuer field in the encoded format specified via the MessageEncoding field.

# 107

SecurityDesc

N

# 350

EncodedSecurityDescLen

N

Must be set if EncodedSecurityDesc field is specified and must immediately precede it.

# 351

EncodedSecurityDesc

N

Encoded (non-ASCII characters) representation of the SecurityDesc field in the encoded format specified via the MessageEncoding field.

# component block &#x3C;SecurityXML>

N

Embedded XML document describing security.

# 691

Pool

N

Identifies MBS / ABS pool

# 667

ContractSettlMonth

N

Must be present for MBS/TBA

# 875

CPProgram

N

The program under which a commercial paper is issued

# 876

CPRegType

N

The registration type of a commercial paper issuance

# component block &#x3C;EvntGrp>

N

Number of repeating EventType group entries.

# 873

DatedDate

N

If different from IssueDate

# 874

InterestAccrualDate

N

If different from IssueDate and DatedDate

# component block &#x3C;InstrumentParties>

N

Used to identify the parties listing a specific instrument

# component block &#x3C;ComplexEvents>

N

*** = Required status should match "Req'd" setting for &#x3C;Instrument> component block in the message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML Element Instrmt

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                   Page 60 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                  August 18, 2011

# Examples using Alternative Security IDs

The SecurityAltID repeating group is used to carry additional security identifiers for the same security. Note that this repeating group can only be used in conjunction with the information in SecurityID and SecurityIDSource fields. In other words, it may not be used instead of the SecurityID and SecurityIDSource fields.

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

# Specifying an FpML product specification from within the FIX Instrument Block

There are two methods in which a FpML product specification or document can be referenced from the FIX Instrument component block. The first method allows the full FpML product document to be embedded within the Instrument component block's SecurityXML (1185) field, found in the SecurityXML component block. The second method allows the FpML production document to be referenced as a URL in the Instrument component block. The tables below illustrate these two methods.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                        Page 61 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# Option 1 – Include the FpML product specification as an XML String within SecurityXML

| Field (tag)           | Value             | Explanation                                                                                            |
| --------------------- | ----------------- | ------------------------------------------------------------------------------------------------------ |
| Symbol (55)           | \[N/A]            |                                                                                                        |
| SecurityID (48)       | \[FpML]           | Refer to EncodedSecurityDesc for the FpML product description,                                         |
| SecurityIDSource (22) | I                 | ISDA/FpML Product Specification                                                                        |
| SecurityXMLLen (1184) | 1234              | The length of the FpML product specification contained within EncodedSecurityDesc                      |
| SecurityXML (1185)    | \<FpML>….\</FpML> | Contains the FpML product specification as an XML string                                               |
| SecuityXMLSchema      | fpml.org/...      | Contains the URI or URL for the schema that is used to interpret the XML payload in SecurityXML (1185) |

Note that prior to FIX 5.0 SP1 the FpML product specification was recommended to be transmitted in the EncodedSecurityDesc (351) field. By using the SecurityXML (1185) field to transmit the FpML product specification the EncodedSecurityDesc (351) field can be used in its intended manner to provide security descriptions using non-ASCII character encoding. This prior approach may still be used in FIX 5.0 and prior versions.

# Option 2 – Reference the FpML product specification from another source via a URL in SecurityID

| Field (tag)           | Value                   | Explanation                                                                                                                                                   |
| --------------------- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Symbol (55)           | \[N/A]                  |                                                                                                                                                               |
| SecurityID (48)       | (a valid URL reference) | Specify a URL to reference a separate or external location for the FpML product description. Example: HTUhttp\://www\.cme.com/product/irswap.jpg?id=122345UTH |
| SecurityIDSource (22) | K                       | ISDA/FpML Product URL                                                                                                                                         |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                        Page 62 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# UnderlyingInstrument (underlying instrument) component block

The UnderlyingInstrument component block, like the Instrument component block, contains all the fields
commonly used to describe a security or instrument. In the case of the UnderlyingInstrument component block
it describes an instrument which underlies the primary instrument. Refer to the Instrument component block
comments as this component block mirrors Instrument, except for the noted fields.

| Tag                                 | FieldName                    | Req'd | Comments                |
| ----------------------------------- | ---------------------------- | ----- | ----------------------- |
| 311                                 | UnderlyingSymbol             | N     |                         |
| 312                                 | UnderlyingSymbolSfx          | N     |                         |
| 309                                 | UnderlyingSecurityID         | N     |                         |
| 305                                 | UnderlyingSecurityIDSource   | N     |                         |
| component block \<UndSecAltIDGrp> N |                              |       |                         |
| 462                                 | UnderlyingProduct            | N     |                         |
| 463                                 | UnderlyingCFICode            | N     |                         |
| 310                                 | UnderlyingSecurityType       | N     |                         |
| 763                                 | UnderlyingSecuritySubType    | N     |                         |
| 313                                 | UnderlyingMaturityMonthYea   | N     | r                       |
| 542                                 | UnderlyingMaturityDate       | N     |                         |
| 1213                                | UnderlyingMaturityTime       | N     |                         |
| 241                                 | UnderlyingCouponPaymentDa    | N     | te                      |
| 1453                                | UnderlyingRestructuringType  | N     |                         |
| 1454                                | UnderlyingSeniority          | N     |                         |
| 1455                                | UnderlyingNotionalPercentage | N     | Outstanding             |
| 1456                                | UnderlyingOriginalNotionalPe | N     | rcentageOutstanding     |
| 1459                                | UnderlyingAttachmentPoint    | N     |                         |
| 1460                                | UnderlyingDetachmentPoint    | N     |                         |
| 242                                 | UnderlyingIssueDate          | N     |                         |
| 243                                 | UnderlyingRepoCollateralSecu | N     | (Deprecated in FIX.4.4) |
| 244                                 | UnderlyingRepurchaseTerm     | N     | (Deprecated in FIX.4.4) |
| 245                                 | UnderlyingRepurchaseRate     | N     | (Deprecated in FIX.4.4) |
| 246                                 | UnderlyingFactor             | N     |                         |
| 256                                 | UnderlyingCreditRating       | N     |                         |
| 595                                 | UnderlyingInstrRegistry      | N     |                         |

© Copyright, 2008-2009, FIX Protocol, Limited Page 63 of 158
---
Version 5.0 Service Pack 2 - Errata      VOLUME 1                                     August 18, 2011

# Errata

| 592  | UnderlyingCountryOfIssue         | N |                                                                                                                                                                                        |   |
| ---- | -------------------------------- | - | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | - |
| 593  | UnderlyingStateOrProvinceOfIssue | N |                                                                                                                                                                                        |   |
| 594  | UnderlyingLocaleOfIssue          | N |                                                                                                                                                                                        |   |
| 247  | UnderlyingRedemptionDate         | N | (Deprecated in FIX.4.4)                                                                                                                                                                |   |
| 316  | UnderlyingStrikePrice            | N |                                                                                                                                                                                        |   |
| 941  | UnderlyingStrikeCurrency         | N |                                                                                                                                                                                        |   |
| 317  | UnderlyingOptAttribute           | N |                                                                                                                                                                                        |   |
| 436  | UnderlyingContractMultiplier     | N |                                                                                                                                                                                        |   |
| 1437 | UnderlyingContractMultiplier     | N | Unit                                                                                                                                                                                   |   |
| 1441 | UnderlyingFlowScheduleType       | N |                                                                                                                                                                                        |   |
| 998  | UnderlyingUnitOfMeasure          | N |                                                                                                                                                                                        |   |
| 1423 | UnderlyingUnitOfMeasureQty       | N |                                                                                                                                                                                        |   |
| 1424 | UnderlyingPriceUnitOfMeasure     | N |                                                                                                                                                                                        |   |
| 1425 | UnderlyingPriceUnitOfMeasureQty  | N |                                                                                                                                                                                        |   |
| 1000 | UnderlyingTimeUnit               | N | Used to indicate a time unit for the contract (e.g., days, weeks, months, etc.)                                                                                                        |   |
| 1419 | UnderlyingExerciseStyle          | N |                                                                                                                                                                                        |   |
| 435  | UnderlyingCouponRate             | N |                                                                                                                                                                                        |   |
| 308  | UnderlyingSecurityExchange       | N |                                                                                                                                                                                        |   |
| 306  | UnderlyingIssuer                 | N |                                                                                                                                                                                        |   |
| 362  | EncodedUnderlyingIssuerLen       | N |                                                                                                                                                                                        |   |
| 363  | EncodedUnderlyingIssuer          | N |                                                                                                                                                                                        |   |
| 307  | UnderlyingSecurityDesc           | N |                                                                                                                                                                                        |   |
| 364  | EncodedUnderlyingSecurityDescLen | N |                                                                                                                                                                                        |   |
| 365  | EncodedUnderlyingSecurityDesc    | N |                                                                                                                                                                                        |   |
| 877  | UnderlyingCPProgram              | N |                                                                                                                                                                                        |   |
| 878  | UnderlyingCPRegType              | N |                                                                                                                                                                                        |   |
| 972  | UnderlyingAllocationPercent      | N | Specific to the < UnderlyingInstrument > Percent of the Strike Price that this underlying represents. Necessary for derivatives that deliver into more than one underlying instrument. |   |
| 318  | UnderlyingCurrency               | N | Specific to the \<UnderlyingInstrument> (not in \<Instrument>)                                                                                                                         |   |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited                                            Page 64 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                      August 18, 2011

# Errata

| 879             | UnderlyingQty             | N | Specific to the (not in ) | Unit amount of the underlying security (par, shares, currency, etc.)                                                                                                                     |
| --------------- | ------------------------- | - | ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 975             | UnderlyingSettlementType  | N | Specific to the           | Indicates order settlement period for the underlying deliverable component.                                                                                                              |
| 973             | UnderlyingCashAmount      | N | Specific to the           | Cash amount associated with the underlying component. Necessary for derivatives that deliver into more than one underlying instrument and one of the underlying's is a fixed cash value. |
| 974             | UnderlyingCashType        | N | Specific to the           | Used for derivatives that deliver into cash underlying. Indicates that the cash is either fixed or difference value (difference between strike and current underlying price)             |
| 810             | UnderlyingPx              | N | Specific to the (not in ) | In a financing deal clean price (percent-of-par or per unit) of the underlying security or basket.                                                                                       |
| 882             | UnderlyingDirtyPrice      | N | Specific to the (not in ) | In a financing deal price (percent-of-par or per unit) of the underlying security or basket. "Dirty" means it includes accrued interest                                                  |
| 883             | UnderlyingEndPrice        | N | Specific to the (not in ) | In a financing deal price (percent-of-par or per unit) of the underlying security or basket at the end of the agreement.                                                                 |
| 884             | UnderlyingStartValue      | N | Specific to the (not in ) | Currency value attributed to this collateral at the start of the agreement                                                                                                               |
| 885             | UnderlyingCurrentValue    | N | Specific to the (not in ) | Currency value currently attributed to this collateral                                                                                                                                   |
| 886             | UnderlyingEndValue        | N | Specific to the (not in ) | Currency value attributed to this collateral at the end of the agreement                                                                                                                 |
| component block |                           |   |                           |                                                                                                                                                                                          |
|                 | Specific to the (not in ) |   |                           | Insert here the contents of the Component Block                                                                                                                                          |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                            Page 65 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# 1044 UnderlyingAdjustedQuantity

N Specific to the &#x3C;UnderlyingInstrument> (not in &#x3C;Instrument>). For listed derivatives margin management, this is the number of shares adjusted for upcoming corporate action. Used only for securities which are optionable and are between ex-date and settlement date (4 days).

# 1045 UnderlyingFXRate

N Specific to the &#x3C;UnderlyingInstrument> (not in &#x3C;Instrument>). Foreign exchange rate used to compute UnderlyingCurrentValue (885) (or market value) from UnderlyingCurrency (318) to Currency (15).

# 1046 UnderlyingFXRateCalc

N Specific to the &#x3C;UnderlyingInstrument> (not in &#x3C;Instrument>). Specified whether UnderlyingFxRate (1045) should be multiplied or divided to derive UnderlyingCurrentValue (885).

# 1038 UnderlyingCapValue

N component block &#x3C;UndlyInstrumentParties>

# 1039 UnderlyingSettlMethod

N

# 315 UnderlyingPutOrCall

N Used to express option right

*** = Required status should match "Req'd" setting for &#x3C;UnderlyingInstrument> component block in the message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element UndInstrmt

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 66 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# InstrumentLeg (symbology) component block

The InstrumentLeg component block, like the Instrument component block, contains all the fields commonly used to describe a security or instrument. In the case of the InstrumentLeg component block it describes a security used in multileg-oriented messages.

Refer to the Instrument component block comments as this component block mirrors Instrument, except for the noted fields.

Several multileg-oriented messages specify an Instrument Leg component block. An instrument can have zero or more instrument legs. The fundamental business rule that applies to the multileg instrument is that the multileg instrument is defined as the combination of instrument legs. The multileg instrument must be able to be traded atomically – that all instrument legs are traded or none are traded.

The LegRatioQty[623] is used to define the quantity of the leg that makes up a single unit of the multileg instrument. An option butterfly strategy is made up of three option legs.

| Tag                               | FieldName                    | Req'd | Comments                |
| --------------------------------- | ---------------------------- | ----- | ----------------------- |
| 600                               | LegSymbol                    | N     |                         |
| 601                               | LegSymbolSfx                 | N     |                         |
| 602                               | LegSecurityID                | N     |                         |
| 603                               | LegSecurityIDSource          | N     |                         |
| component block \<LegSecAltIDGrp> |                              |       | N                       |
| 607                               | LegProduct                   | N     |                         |
| 608                               | LegCFICode                   | N     |                         |
| 609                               | LegSecurityType              | N     |                         |
| 764                               | LegSecuritySubType           | N     |                         |
| 610                               | LegMaturityMonthYear         | N     |                         |
| 611                               | LegMaturityDate              | N     |                         |
| 1212                              | LegMaturityTime              | N     |                         |
| 248                               | LegCouponPaymentDate         | N     |                         |
| 249                               | LegIssueDate                 | N     |                         |
| 250                               | LegRepoCollateralSecurityTyp | N     | (Deprecated in FIX.4.4) |
| 251                               | LegRepurchaseTerm            | N     | (Deprecated in FIX.4.4) |
| 252                               | LegRepurchaseRate            | N     | (Deprecated in FIX.4.4) |
| 253                               | LegFactor                    | N     |                         |
| 257                               | LegCreditRating              | N     |                         |
| 599                               | LegInstrRegistry             | N     |                         |
| 596                               | LegCountryOfIssue            | N     |                         |
| 597                               | LegStateOrProvinceOfIssue    | N     |                         |
| 598                               | LegLocaleOfIssue             | N     |                         |
| 254                               | LegRedemptionDate            | N     | (Deprecated in FIX.4.4) |

© Copyright, 2008- 2009 2011, FIX Protocol, Limited Page 67 of 158
---
Version 5.0 Service Pack 2 - Errata    VOLUME 1                                         August 18, 2011

# Errata

| Field Number | Field Name                | Required | Description                                                                                                                                                                                               |
| ------------ | ------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 612          | LegStrikePrice            | N        |                                                                                                                                                                                                           |
| 942          | LegStrikeCurrency         | N        |                                                                                                                                                                                                           |
| 613          | LegOptAttribute           | N        |                                                                                                                                                                                                           |
| 614          | LegContractMultiplier     | N        |                                                                                                                                                                                                           |
| 1436         | LegContractMultiplierUnit | N        |                                                                                                                                                                                                           |
| 1440         | LegFlowScheduleType       | N        |                                                                                                                                                                                                           |
| 999          | LegUnitOfMeasure          | N        |                                                                                                                                                                                                           |
| 1224         | LegUnitOfMeasureQty       | N        |                                                                                                                                                                                                           |
| 1421         | LegPriceUnitOfMeasure     | N        |                                                                                                                                                                                                           |
| 1422         | LegPriceUnitOfMeasureQty  | N        |                                                                                                                                                                                                           |
| 1001         | LegTimeUnit               | N        | Used to indicate a time unit for the contract (e.g., days, weeks, months, etc.)                                                                                                                           |
| 1420         | LegExerciseStyle          | N        |                                                                                                                                                                                                           |
| 615          | LegCouponRate             | N        |                                                                                                                                                                                                           |
| 616          | LegSecurityExchange       | N        |                                                                                                                                                                                                           |
| 617          | LegIssuer                 | N        |                                                                                                                                                                                                           |
| 618          | EncodedLegIssuerLen       | N        |                                                                                                                                                                                                           |
| 619          | EncodedLegIssuer          | N        |                                                                                                                                                                                                           |
| 620          | LegSecurityDesc           | N        |                                                                                                                                                                                                           |
| 621          | EncodedLegSecurityDescLen | N        |                                                                                                                                                                                                           |
| 622          | EncodedLegSecurityDesc    | N        |                                                                                                                                                                                                           |
| 623          | LegRatioQty               | N        | Specific to the \<InstrumentLeg> (not in \<Instrument>)                                                                                                                                                   |
| 624          | LegSide                   | N        | Specific to the \<InstrumentLeg> (not in \<Instrument>)                                                                                                                                                   |
| 556          | LegCurrency               | N        | Specific to the \<InstrumentLeg> (not in \<Instrument>)                                                                                                                                                   |
| 740          | LegPool                   | N        | Identifies MBS / ABS pool                                                                                                                                                                                 |
| 739          | LegDatedDate              | N        |                                                                                                                                                                                                           |
| 955          | LegContractSettlMonth     | N        |                                                                                                                                                                                                           |
| 956          | LegInterestAccrualDate    | N        |                                                                                                                                                                                                           |
| 1358         | LegPutOrCall              | N        | Used to express option right                                                                                                                                                                              |
| 1017         | LegOptionRatio            | N        | LegOptionRatio is provided on covering leg to create a delta neutral spread. In Listed Derivatives, the delta of the leg is multiplied by LegOptionRatio and OrderQty to determine the covering quantity. |
| 566          | LegPrice                  | N        | Used to specify an anchor price for a leg as part of the definition or creation of the strategy - not used for execution price.                                                                           |

*** = Required status should match "Req'd" setting for &#x3C;InstrumentLeg> component block in message definition

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                               Page 68 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                        August 18, 2011

# FIXML Definition for this Component Block

Refer to FIXML element InstrmtLeg

© Copyright, 2008-2011, FIX Protocol, Limited

Page 69 of 158


---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                  August 18, 2011

# InstrumentExtension component block

The InstrumentExtension component block identifies additional security attributes that are more commonly found for Fixed Income securities.

| Tag                                                                          | FieldName    | Req'd | Comments                                     |
| ---------------------------------------------------------------------------- | ------------ | ----- | -------------------------------------------- |
| 668                                                                          | DeliveryForm | N     | Identifies the form of delivery.             |
| 869                                                                          | PctAtRisk    | N     | Percent at risk due to lowest possible call. |
| component block \<AttrbGrp> N Number of repeating InstrAttrib group entries. |              |       |                                              |

*** = Required status should match "Req'd" setting for &#x3C;InstrumentExtension> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element InstrmtExtension

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited

Page 70 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

# OrderQtyData component block

The OrderQtyData component block contains the fields commonly used for indicating the amount or quantity of an order. Note that when this component block is marked as "required" in a message either one of these three fields must be used to identify the amount: OrderQty, CashOrderQty or OrderPercent (in the case of CIV).

| Tag | FieldName         | Req'd | Comments                                                                                                                                                                                                                                                                                                                                                                       |
| --- | ----------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 38  | OrderQty          | N     | One of CashOrderQty, OrderQty, or (for CIV only) OrderPercent is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or OrderPercent should be specified.                                                                                                                                                                                      |
| 152 | CashOrderQty      | N     | One of CashOrderQty, OrderQty, or (for CIV only) OrderPercent is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or OrderPercent should be specified. Specifies the approximate "monetary quantity" for the order. Broker is responsible for converting and calculating OrderQty in tradeable units (e.g. shares) for subsequent messages. |
| 516 | OrderPercent      | N     | For CIV - Optional. One of CashOrderQty, OrderQty or (for CIV only) OrderPercent is required. Note that unless otherwise specified, only one of CashOrderQty, OrderQty, or OrderPercent should be specified.                                                                                                                                                                   |
| 468 | RoundingDirection | N     | For CIV - Optional                                                                                                                                                                                                                                                                                                                                                             |
| 469 | RoundingModulus   | N     | For CIV - Optional                                                                                                                                                                                                                                                                                                                                                             |

*** = Required status should match "Req'd" setting for <orderqtydata> component block in message definition</orderqtydata>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element OrdQtyData

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited

Page 71 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                        August 18, 2011

# CommissionData component block

The CommissionDate component block is used to carry commission information such as the type of commission and the rate.

| Tag | FieldName     | Req'd | Comments |
| --- | ------------- | ----- | -------- |
| 12  | Commission    | N     |          |
| 13  | CommType      | N     |          |
| 479 | CommCurrency  | N     |          |
| 497 | FundRenewWaiv | N     |          |

*** = Required status should match "Req'd" setting for <commissiondata> component block in message definition</commissiondata>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML Element CommData

© Copyright, 2008-2009, FIX Protocol, Limited                                Page 72 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# Parties component block

The Parties component block is used to identify and convey information on the entities both central and peripheral to the financial transaction represented by the FIX message containing the Parties Block. The Parties block allows many different types of entities to be expressed through use of the PartyRole field and identifies the source of the PartyID through the PartyIDSource.

See “Volume 6 - APPENDIX 6-G - USE OF &#x3C;PARTIES> COMPONENT BLOCK” for additional usage information.

| Tag             | FieldName     | Req'd | Comments                                                                                                                 |
| --------------- | ------------- | ----- | ------------------------------------------------------------------------------------------------------------------------ |
| 453             | NoPartyIDs    | N     | Repeating group below should contain unique combinations of PartyID, PartyIDSource, and PartyRole                        |
| 448             | PartyID       | N     | Used to identify source of PartyID. Required if PartyIDSource is specified. Required if NoPartyIDs > 0.                  |
| 447             | PartyIDSource | N     | Used to identify class source of PartyID value (e.g. BIC). Required if PartyID is specified. Required if NoPartyIDs > 0. |
| 452             | PartyRole     | N     | Identifies the type of PartyID (e.g. Executing Broker). Required if NoPartyIDs > 0.                                      |
| component block |               | N     | Repeating group of Party sub-identifiers.                                                                                |

&#x3C;PtysSubGrp>

*** = Required status should match "Req'd" setting for &#x3C;Parties> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Ptys

© Copyright, 2008-2009, FIX Protocol, Limited

Page 73 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# NestedParties component block

The NestedParties component block is identical to the Parties Block. It is used in other component blocks and repeating groups when nesting will take place resulting in multiple occurrences of the Parties block within a single FIX message. Use of NestedParties under these conditions avoids multiple references to the Parties block within the same message which is not allowed in FIX tag/value syntax.

| Tag             | FieldName           | Req'd | Comments                                                                                                                                   |
| --------------- | ------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| 539             | NoNestedPartyIDs    | N     | Repeating group below should contain unique combinations of NestedPartyID, NestedPartyIDSource, and NestedPartyRole                        |
| 524             | NestedPartyID       | N     | Used to identify source of NestedPartyID. Required if NestedPartyIDSource is specified. Required if NoNestedPartyIDs > 0.                  |
| 525             | NestedPartyIDSource | N     | Used to identify class source of NestedPartyID value (e.g. BIC). Required if NestedPartyID is specified. Required if NoNestedPartyIDs > 0. |
| 538             | NestedPartyRole     | N     | Identifies the type of NestedPartyID (e.g. Executing Broker). Required if NoNestedPartyIDs > 0.                                            |
| component block |                     |       |                                                                                                                                            |
|                 |                     |       |                                                                                                                                            |

*** = Required status should match "Req'd" setting for <nestedparties> component block in message definition</nestedparties>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element NstPtys

© Copyright, 2008-2009, FIX Protocol, Limited

Page 74 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# NestedParties2 (second instance of nesting) component block

The NestedParties2 component block is identical to the Parties Block. It is used in other component blocks and repeating groups when nesting will take place resulting in multiple occurrences of the Parties block within a single FIX message. Use of NestedParties2 under these conditions avoids multiple references to the Parties block within the same message which is not allowed in FIX tag/value syntax.

| Tag                | FieldName            | Req'd | Comments                                                                                                                                      |
| ------------------ | -------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 756                | NoNested2PartyIDs    | N     | Repeating group below should contain unique combinations of Nested2PartyID, Nested2PartyIDSource, and Nested2PartyRole                        |
| 757                | Nested2PartyID       | N     | Used to identify source of Nested2PartyID. Required if Nested2PartyIDSource is specified. Required if NoNested2PartyIDs > 0.                  |
| 758                | Nested2PartyIDSource | N     | Used to identify class source of Nested2PartyID value (e.g. BIC). Required if Nested2PartyID is specified. Required if NoNested2PartyIDs > 0. |
| 759                | Nested2PartyRole     | N     | Identifies the type of Nested2PartyID (e.g. Executing Broker). Required if NoNested2PartyIDs > 0.                                             |
| component block    |                      |       | N                                                                                                                                             |
| \<NstdPtys2SubGrp> |                      |       |                                                                                                                                               |

*** = Required status should match "Req'd" setting for &#x3C;NestedParties2> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element NstPtys2

© Copyright, 2008-2009, FIX Protocol, Limited Page 75 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# NestedParties3 (third instance of nesting) component block

The NestedParties3 component block is identical to the Parties Block. It is used in other component blocks and repeating groups when nesting will take place resulting in multiple occurrences of the Parties block within a single FIX message. Use of NestedParties3 under these conditions avoids multiple references to the Parties block within the same message which is not allowed in FIX tag/value syntax.

| Tag                | FieldName            | Req'd | Comments                                                                                                                                      |
| ------------------ | -------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 948                | NoNested3PartyIDs    | N     | Repeating group below should contain unique combinations of Nested3PartyID, Nested3PartyIDSource, and Nested3PartyRole                        |
| 949                | Nested3PartyID       | N     | Used to identify source of Nested3PartyID. Required if Nested3PartyIDSource is specified. Required if NoNested3PartyIDs > 0.                  |
| 950                | Nested3PartyIDSource | N     | Used to identify class source of Nested3PartyID value (e.g. BIC). Required if Nested3PartyID is specified. Required if NoNested3PartyIDs > 0. |
| 951                | Nested3PartyRole     | N     | Identifies the type of Nested3PartyID (e.g. Executing Broker). Required if NoNested3PartyIDs > 0.                                             |
| component block    |                      |       | N                                                                                                                                             |
|                    |                      |       | Repeating group of Nested3Party sub-identifiers.                                                                                              |
| \<NstdPtys3SubGrp> |                      |       |                                                                                                                                               |

*** = Required status should match "Req'd" setting for &#x3C;NestedParties3> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element NstPtys3

© Copyright, 2008-2009, FIX Protocol, Limited

Page 76 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# NestedParties4 (fourth instance of nesting) component block

The NestedParties4 component block is identical to the Parties Block. It is used in other component blocks and repeating groups when nesting will take place resulting in multiple occurrences of the Parties block within a single FIX message. Use of NestedParties4 under these conditions avoids multiple references to the Parties block within the same message which is not allowed in FIX tag/value syntax.

| Tag             | FieldName            | Req'd | Comments                                                                                                                                      |
| --------------- | -------------------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| 1414            | NoNested4PartyIDs    | N     | Repeating group below should contain unique combinations of Nested4PartyID, Nested4PartyIDSource, and Nested4PartyRole.                       |
| 1415            | Nested4PartyID       | N     | Used to identify source of Nested4PartyID. Required if Nested4PartyIDSource is specified. Required if NoNested4PartyIDs > 0.                  |
| 1416            | Nested4PartyIDSource | N     | Used to identify class source of Nested4PartyID value (e.g. BIC). Required if Nested4PartyID is specified. Required if NoNested4PartyIDs > 0. |
| 1417            | Nested4PartyRole     | N     | Identifies the type of Nested4PartyID (e.g. Executing Broker). Required if NoNested4PartyIDs > 0.                                             |
| component block |                      | N     | \<NstdPtys4SubGrp>                                                                                                                            |

*** = Required status should match "Req'd" setting for &#x3C;NestedParties3> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element NstPtys4

© Copyright, 2008- 2009, 2011, FIX Protocol, Limited Page 77 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

# SpreadOrBenchmarkCurveData component block

The SpreadOrBenchmarkCurveData component block is primarily used for Fixed Income to convey spread to a benchmark security or curve.

| Tag | FieldName                 | Req'd | Comments                                                                                                                      |
| --- | ------------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------- |
| 218 | Spread                    | N     | For Fixed Income                                                                                                              |
| 220 | BenchmarkCurveCurrency    | N     |                                                                                                                               |
| 221 | BenchmarkCurveName        | N     |                                                                                                                               |
| 222 | BenchmarkCurvePoint       | N     |                                                                                                                               |
| 662 | BenchmarkPrice            | N     |                                                                                                                               |
| 663 | BenchmarkPriceType        | N     | Must be present if BenchmarkPrice is used.                                                                                    |
| 699 | BenchmarkSecurityID       | N     | The identifier of the benchmark security, e.g. Treasury against Corporate bond.                                               |
| 761 | BenchmarkSecurityIDSource | N     | Source of BenchmarkSecurityID. If not specified, then ID Source is understood to be the same as that in the Instrument block. |

*** = Required status should match "Req'd" setting for <spreadorbenchmarkcurvedata> component block in message definition</spreadorbenchmarkcurvedata>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SpreadOrBnchmkCrvData

© Copyright, 2008-2009, FIX Protocol, Limited                                    Page 78 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

# Stipulations component block

The Stipulations component block is used in Fixed Income to provide additional information on a given security. These additional information are usually not considered static data information.

| Tag | FieldName        | Req'd | Comments                      |
| --- | ---------------- | ----- | ----------------------------- |
| 232 | NoStipulations   | N     |                               |
| 233 | StipulationType  | N     | Required if NoStipulations >0 |
| 234 | StipulationValue | N     |                               |

*** = Required status should match "Req'd" setting for <stipulations> component block in message definition</stipulations>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Stips

© Copyright, 2008-2009, FIX Protocol, Limited                                      Page 79 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# UnderlyingStipulations component block

The UnderlyingStipulations component block has the same usage as the Stipulations component block, but for an underlying security.

| Tag | FieldName           | Req'd | Comments                         |
| --- | ------------------- | ----- | -------------------------------- |
| 887 | NoUnderlyingStips   | N     |                                  |
| 888 | UnderlyingStipType  | N     | Required if NoUnderlyingStips >0 |
| 889 | UnderlyingStipValue | N     |                                  |

*** = Required status should match "Req'd" setting for <underlyingstipulations> component block in message definition</underlyingstipulations>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element UndStips

© Copyright, 2008-2009, FIX Protocol, Limited Page 80 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# LegStipulations component block

The LegStipulations component block has the same usage as the Stipulations component block, but for a leg instrument in a multi-legged security.

| Tag | FieldName           | Req'd | Comments                         |
| --- | ------------------- | ----- | -------------------------------- |
| 683 | NoLegStipulations   | N     |                                  |
| 688 | LegStipulationType  | N     | Required if NoLegStipulations >0 |
| 689 | LegStipulationValue | N     |                                  |

*** = Required status should match "Req'd" setting for &#x3C;LegStipulations> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element LegStips

© Copyright, 2008-2009, FIX Protocol, Limited Page 81 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                            August 18, 2011

# YieldData component block

The YieldData component block conveys yield information for a given Fixed Income security.

| Tag | FieldName                | Req'd | Comments |
| --- | ------------------------ | ----- | -------- |
| 235 | YieldType                | N     |          |
| 236 | Yield                    | N     |          |
| 701 | YieldCalcDate            | N     |          |
| 696 | YieldRedemptionDate      | N     |          |
| 697 | YieldRedemptionPrice     | N     |          |
| 698 | YieldRedemptionPriceType | N     |          |

*** = Required status should match "Req'd" setting for <yielddata> component block in message definition</yielddata>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element YldData

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited

Page 82 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                         August 18, 2011

# TrdRegTimestamps component block

The TrdRegTimestamps component block is used to express timestamps for an order or trade that are required by regulatory agencies. These timestamps are used to identify the timeframes for when an order or trade is received on the floor, received and executed by the broker, etc.

| Tag  | FieldName             | Req'd | Comments                           |
| ---- | --------------------- | ----- | ---------------------------------- |
| 768  | NoTrdRegTimestamps    | N     |                                    |
| 769  | TrdRegTimestamp       | N     | Required if NoTrdRegTimestamps > 1 |
| 770  | TrdRegTimestampTyp    | N     | Required if NoTrdRegTimestamps > 1 |
| 771  | TrdRegTimestampOrig   | N     |                                    |
| 1033 | DeskType              | N     | Type of Trading desk               |
| 1034 | DeskTypeSource        | N     |                                    |
| 1035 | DeskOrderHandlingInst | N     |                                    |

*** = Required status should match "Req'd" setting for <trdregtimestamps> component block in message definition</trdregtimestamps>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TrdRegTmstampsGrp

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                Page 83 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                August 18, 2011

# FinancingDetails component block

Component block is optionally used only for financing deals to identify the legal agreement under which the deal was made and other unique characteristics of the transaction. The AgreementDesc field refers to base standard documents such as MRA 1996 Repurchase Agreement, GMRA 2000 Bills Transaction (U.K.), MSLA 1993 Securities Loan – Amended 1998, for example.

| Tag | FieldName         | Req'd | Comments                                                                                                                         |
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

*** = Required status should match "Req'd" setting for &#x3C;FinancingDetails> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to the FIXML element FinancingDetails

© Copyright, 2008-2011, FIX Protocol, Limited

Page 84 of 158
---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                                         August 18, 2011


© Copyright, 2008-2011, FIX Protocol, Limited

Page 85 of 158



---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# InstrumentParties component block

The use of this component block is restricted to instrument definition only and is not permitted to contain transactional information. Only a specified subset of party roles will be supported within the InstrumentParty block. Possible uses of this block include identifying Listing Source information; Clearing Org information; Parent and Capital Structure information for F/I and derivative instruments.

| Tag                                                 | FieldName            | Req'd | Comments                                                                                                                        |
| --------------------------------------------------- | -------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------- |
| 1018                                                | NoInstrumentParties  | N     | Repeating group below should contain unique combinations of InstrumentPartyID, InstrumentPartyIDSource, and InstrumentPartyRole |
| 1019                                                | InstrumentPartyID    | N     | Used to identify party id related to instrument                                                                                 |
| 1050                                                | InstrumentPartyIDSou | N     | Used to identify source of instrument party id                                                                                  |
| 1051                                                | InstrumentPartyRole  | N     | Used to identify the role of instrument party id                                                                                |
| component block                                     |                      |       | N                                                                                                                               |
| Repeating group of InstrumentParty sub-identifiers. |                      |       |                                                                                                                                 |

<instrumentptyssubgrp></instrumentptyssubgrp>

*** = Required status should match "Req'd" setting for <instrumentparties> component block in message definition</instrumentparties>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element InstrumentParties

© Copyright, 2008-2011, FIX Protocol, Limited Page 86 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                         August 18, 2011

# DisplayInstruction component block

The DisplayInstruction component block is used to convey instructions on how a reserved order is to be handled in terms of when and how much of the order quantity is to be displayed to the market.

| Tag  | FieldName           | Req'd | Comments                                                                                                                            |
| ---- | ------------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------- |
| 1138 | DisplayQty          | N     |                                                                                                                                     |
| 1082 | SecondaryDisplayQty | N     |                                                                                                                                     |
| 1083 | DisplayWhen         | N     |                                                                                                                                     |
| 1084 | DisplayMethod       | N     |                                                                                                                                     |
| 1085 | DisplayLowQty       | N     | Required when DisplayMethod = 3                                                                                                     |
| 1086 | DisplayHighQty      | N     | Required when DisplayMethod = 3                                                                                                     |
| 1087 | DisplayMinIncr      | N     | Can be used to specify larger increments than the standard increment provided by the market. Optionally used when DisplayMethod = 3 |
| 1088 | RefreshQty          | N     | Required when DisplayMethod = 2                                                                                                     |

*** = Required status should match "Req'd" setting for <displayinstruction> component block in message definition</displayinstruction>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element DisplayInstruction Grp

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                         Page 87 of 158
---

Version 5.0 Service Pack 2 - Errata        VOLUME 1                                                 August 18, 2011


# RootParties component block

The RootParties component block is a version of the Parties component block used to provide root information regarding the owning and entering parties of a transaction.

| Tag             | FieldName         | Req'd | Comments                                                                                                                             |
| --------------- | ----------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------ |
| 1116            | NoRootPartyIDs    | N     | Repeating group below should contain unique combinations of RootPartyID, RootPartyIDSource, and RootPartyRole                        |
| 1117            | RootPartyID       | N     | Used to identify source of RootPartyID. Required if RootPartyIDSource is specified. Required if NoRootPartyIDs > 0.                  |
| 1118            | RootPartyIDSource | N     | Used to identify class source of RootPartyID value (e.g. BIC). Required if RootPartyID is specified. Required if NoRootPartyIDs > 0. |
| 1119            | RootPartyRole     | N     | Identifies the type of RootPartyID (e.g. Executing Broker). Required if NoRootPartyIDs > 0.                                          |
| component block |                   |       |                                                                                                                                      |
|                 |                   |       |                                                                                                                                      |

*** = Required status should match "Req'd" setting for <rootparties> component block in message definition</rootparties>

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element RootParties Grp

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited


Page 88 of 158

---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# UndlyInstrumentParties component block

The use of this component block is restricted to instrument definition only and is not permitted to contain transactional information. Only a specified subset of party roles will be supported within the InstrumentParty block. Possible uses of this block include identifying Listing Source information; Clearing Org information; Parent and Capital Structure information for Fixed Income and derivative instruments.

| Tag             | FieldName                          | Req'd | Comments                                                                                                                        |
| --------------- | ---------------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------- |
| 1058            | NoUndlyInstrumentParties           | N     | Repeating group below should contain unique combinations of InstrumentPartyID, InstrumentPartyIDSource, and InstrumentPartyRole |
| 1059            | UnderlyingInstrument PartyID       | N     | Used to identify party id related to instrument                                                                                 |
| 1060            | UnderlyingInstrument PartyIDSource | N     | Used to identify source of instrument party id                                                                                  |
| 1061            | UnderlyingInstrument PartyRole     | N     | Used to identify the role of instrument party id                                                                                |
| component block |                                    | N     | Repeating group of InstrumentParty sub-identifiers.                                                                             |

&#x3C;UndlyInstrumentPtysSubGrp>

*** = Required status should match "Req'd" setting for &#x3C;UndlyInstrumentParties> component block in message definition

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element UndlyInstrumentParties Grp

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 89 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                   August 18, 2011

# ApplicationSequenceControl component block

The ApplicationSequenceControl is used for application sequencing and recovery. Consisting of ApplSeqNum (1181), ApplID (1180), ApplLastSeqNum (1350), and ApplResendFlag (1352), FIX application messages that carries this component block will be able to use application level sequencing. ApplID, ApplSeqNum and ApplLastSeqNum fields identify the application id, application sequence number and the previous application sequence number (in case of intentional gaps) on each application message that carries this block. The ApplResendFlag (1352) is used to indicate that messages are being retransmitted as a result of an Application Message Request. See Application Sequencing Message section for further details on usage and restrictions.

| Tag  | FieldName      | Req'd | Comments                                                                                                                                                                                                                                                                                                                                                   |
| ---- | -------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1180 | ApplID         | N     | Identifies the application with which a message is associated. Used only if application sequencing is in effect.                                                                                                                                                                                                                                           |
| 1181 | ApplSeqNum     | N     | Application sequence number assigned to the message by the application generating the message. Used only if application sequencing is in effect. Conditionally required if ApplID has been specified.                                                                                                                                                      |
| 1350 | ApplLastSeqNum | N     | The previous sequence number in the application sequence stream. Permits an application to publish messages with sequence gaps where it cannot be avoided. Used only if application sequencing is in effect. Conditionally required if ApplID has been specified.                                                                                          |
| 1352 | ApplResendFlag | N     | Used to indicate that a message is being sent in response to an Application Message Request. Used only if application sequencing is in effect. It is possible for both ApplResendFlag and PossDupFlag to be set on the same message if the Sender's cache size is greater than zero and the message is being resent due to a session level resend request. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element ApplSeqGrp

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                            Page 90 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# SecurityXML component block

The SecurityXML component is used for carrying security description or definition in an XML format. See "Specifying an FpML product specification from within the FIX Instrument Block" for more information on using this component block with FpML as a guideline.

| Tag  | FieldName         | Req'd | Comments                                                                       |
| ---- | ----------------- | ----- | ------------------------------------------------------------------------------ |
| 1184 | SecurityXMLLen    | N     | Must be set if SecurityXML field is specified and must immediately precede it. |
| 1185 | SecurityXML       | N     | XML payload or content describing the Security information.                    |
| 1186 | SecurityXMLSchema | N     | XML Schema used to validate the XML used to describe the Security.             |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element SecXML

# RateSource component block

| Tag  | FieldName      | Req'd | Comments                            |
| ---- | -------------- | ----- | ----------------------------------- |
| 1445 | NoRateSources  | N     |                                     |
| 1446 | RateSource     | N     | Required if NoRateSource(1445) > 0  |
| 1447 | RateSourceType | N     | Required if NoRateSources(1445) > 0 |
| 1448 | ReferencePage  | N     | Required if RateSource(1446)=other  |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element RtSrc

© Copyright, 2008-2011, FIX Protocol, Limited

Page 91 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# TargetParties component block

| Tag  | FieldName           | Req'd | Comments                                                                                                             |
| ---- | ------------------- | ----- | -------------------------------------------------------------------------------------------------------------------- |
| 1461 | NoTargetPartyIDs    | N     | Repeating group below should contain unique combinations of TargetPartyID, TargetPartyIDSource, and TargetPartyRole. |
| 1462 | TargetPartyID       | N     | Required if NoTargetPartyIDs > 0. Used to identify PartyID targeted for the action specified in the message.         |
| 1463 | TargetPartyIDSource | N     | Used to identify source of target party id.                                                                          |
| 1464 | TargetPartyRole     | N     | Used to identify the role of target party id.                                                                        |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TgtPty

# InstrmtGrp component block

| Tag             | FieldName    | Req'd | Comments                                                                                                      |
| --------------- | ------------ | ----- | ------------------------------------------------------------------------------------------------------------- |
| 146             | NoRelatedSym | N     | Specifies the number of repeating symbols (instruments) specified                                             |
| component block |              | N     | Insert here the set of "Instrument" (symbology) fields defined in "Common Components of Application Messages" |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Inst

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited Page 92 of 158
---
Version 5.0 Service Pack 2 - Errata      VOLUME 1                                          August 18, 2011

# InstrmtLegGrp component block

| Tag              | FieldName       | Req'd | Comments                               |
| ---------------- | --------------- | ----- | -------------------------------------- |
| 555              | NoLegs          | N     | Number of legs                         |
|                 | component block | N     | Must be provided if Number of legs > 0 |
| \<InstrumentLeg> |                 |       |                                        |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Leg

# UndInstrmtGrp component block

| Tag                     | FieldName       | Req'd | Comments                                      |
| ----------------------- | --------------- | ----- | --------------------------------------------- |
| 711                     | NoUnderlyings   | N     | Number of underlyings                         |
|                        | component block | N     | Must be provided if Number of underlyings > 0 |
| \<UnderlyingInstrument> |                 |       |                                               |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Undly

# SecAltIDGrp component block

| Tag | FieldName               | Req'd | Comments |
| --- | ----------------------- | ----- | -------- |
| 454 | NoSecurityAltID         | N     |          |
|    | 455 SecurityAltID       | N     |          |
|    | 456 SecurityAltIDSource | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element AID

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                          Page 93 of 158
---
Version 5.0 Service Pack 2 - Errata     VOLUME 1                               August 18, 2011

# LegSecAltIDGrp component block

| Tag | FieldName            | Req'd | Comments |
| --- | -------------------- | ----- | -------- |
| 604 | NoLegSecurityAltID   | N     |          |
| 605 | LegSecurityAltID     | N     |          |
| 606 | LegSecurityAltIDSour | N     | ce       |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element LegAID

# UndSecAltIDGrp component block

| Tag | FieldName                 | Req'd | Comments |
| --- | ------------------------- | ----- | -------- |
| 457 | NoUnderlyingSecurityAltID | N     |          |
| 458 | UnderlyingSecurityAlt     | N     | ID       |
| 459 | UnderlyingSecurityAlt     | N     | IDSource |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element UndAID

© Copyright, 2008-2009, FIX Protocol, Limited                          Page 94 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# EvntGrp component block

| Tag  | FieldName |
| ---- | --------- |
| 864  | NoEvents  |
| 865  | EventType |
| 866  | EventDate |
| 1145 | EventTime |
| 867  | EventPx   |
| 868  | EventText |

# Req'd

| Req'd | Comments                                                                |
| ----- | ----------------------------------------------------------------------- |
| N     |                                                                         |
| N     |                                                                         |
| N     |                                                                         |
| N     | Specific time of event. To be used in combination with EventDate \[866] |
| N     |                                                                         |
| N     |                                                                         |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Evnt

# InstrumentPtysSubGrp component block

| Tag  | FieldName               | Req'd | Comments |
| ---- | ----------------------- | ----- | -------- |
| 1052 | NoInstrumentPartySubIDs | N     |          |
| 1053 | InstrumentPartySubID    | N     |          |
| 1054 | InstrumentPartySubID    | N     | Type     |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 95 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# UndlyInstrumentPtysSubGrp component block

| Tag  | FieldName                          | Req'd | Comments |
| ---- | ---------------------------------- | ----- | -------- |
| 1062 | NoUndlyInstrumentPartySubIDs       | N     |          |
| 1063 | UnderlyingInstrumentPartySubID     | N     |          |
| 1064 | UnderlyingInstrumentPartySubIDType | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

# PtysSubGrp component block

| Tag | FieldName      | Req'd | Comments |
| --- | -------------- | ----- | -------- |
| 802 | NoPartySubIDs  | N     |          |
| 523 | PartySubID     | N     |          |
| 803 | PartySubIDType | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 96 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                 August 18, 2011

# NstdPtysSubGrp component block

| Tag | FieldName               | Req'd | Comments |
| --- | ----------------------- | ----- | -------- |
| 804 | NoNestedPartySubIDs     | N     |          |
|     | 545 NestedPartySubID    | N     |          |
|     | 805 NestedPartySubIDTyp | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

# NstdPtys2SubGrp component block

| Tag | FieldName               | Req'd | Comments |
| --- | ----------------------- | ----- | -------- |
| 806 | NoNested2PartySubIDs    | N     |          |
|     | 760 Nested2PartySubID   | N     |          |
|     | 807 Nested2PartySubIDTy | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                          Page 97 of 158
---
Version 5.0 Service Pack 2 - Errata    VOLUME 1                                August 18, 2011

# NstdPtys3SubGrp component block

| Tag | FieldName            | Req'd | Comments |
| --- | -------------------- | ----- | -------- |
| 952 | NoNested3PartySubIDs | N     |          |
| 953 | Nested3PartySubID    | N     |          |
| 954 | Nested3PartySubIDTy  | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

# NstdPtys4SubGrp component block

| Tag  | FieldName            | Req'd | Comments |
| ---- | -------------------- | ----- | -------- |
| 1413 | NoNested4PartySubIDs | N     |          |
| 1412 | Nested4PartySubID    | N     |          |
| 1411 | Nested4PartySubIDTy  | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                         Page 98 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                 August 18, 2011

# RootSubParties component block

| Tag  | FieldName          | Req'd | Comments                                                                                                        |
| ---- | ------------------ | ----- | --------------------------------------------------------------------------------------------------------------- |
| 1120 | NoRootPartySubIDs  | N     | Repeating group of RootParty sub-identifiers.                                                                   |
| 1121 | RootPartySubID     | N     | Sub-identifier (e.g. Clearing Acct for PartyID=Clearing Firm) if applicable. Required if NoRootPartySubIDs > 0. |
| 1122 | RootPartySubIDType | N     | Type of Sub-identifier. Required if NoRootPartySubIDs > 0.                                                      |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Sub

# AttrbGrp component block

| Tag | FieldName        | Req'd | Comments |
| --- | ---------------- | ----- | -------- |
| 870 | NoInstrAttrib    | N     |          |
| 871 | InstrAttribType  | N     |          |
| 872 | InstrAttribValue | N     |          |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element Attrb

© Copyright, 2008-2011, FIX Protocol, Limited                                        Page 99 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# ContAmtGrp component block

| Tag | FieldName    | Req'd | Comments                                                                          |
| --- | ------------ | ----- | --------------------------------------------------------------------------------- |
| 518 | NoContAmts   | N     | Number of contract details in this message (number of repeating groups to follow) |
| 519 | ContAmtType  | N     | Must be first field in the repeating group.                                       |
| 520 | ContAmtValue | N     |                                                                                   |
| 521 | ContAmtCurr  | N     |                                                                                   |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element ContAmt

# MiscFeesGrp component block

| Tag | FieldName    | Req'd | Comments                                                                                |
| --- | ------------ | ----- | --------------------------------------------------------------------------------------- |
| 136 | NoMiscFees   | N     | Required if any miscellaneous fees are reported. Indicates number of repeating entries. |
| 137 | MiscFeeAmt   | N     | Required if NoMiscFees > 0                                                              |
| 138 | MiscFeeCurr  | N     |                                                                                         |
| 139 | MiscFeeType  | N     | Required if NoMiscFees > 0                                                              |
| 891 | MiscFeeBasis | N     |                                                                                         |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element MiscFees

© Copyright, 2008-2009, FIX Protocol, Limited                                           Page 100 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                         August 18, 2011

# TrdgSesGrp component block

| Tag | FieldName           | Req'd | Comments                                            |
| --- | ------------------- | ----- | --------------------------------------------------- |
| 386 | NoTradingSessions   | N     | Specifies the number of repeating TradingSessionIDs |
| 336 | TradingSessionID    | N     | Required if NoTradingSessions is > 0.               |
| 625 | TradingSessionSubID | N     |                                                     |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element TrdSes

© Copyright, 2008-2009, FIX Protocol, Limited

Page 101 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# ComplexEvents component block

The ComplexEvent Group is a repeating block which allows an unlimited number and types of events in the lifetime of an option to be specified.

| Tag                                                                          | FieldName                          | Req'd | Comments                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------------------------------------------------------------- | ---------------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1483                                                                         | NoComplexEvents                    | N     | Number of complex events                                                                                                                                                                                                                                                                                                                                                                    |
| 1484                                                                         | ComplexEventType                   | N     | Identifies the type of complex event. Required if NoComplexEvents > 0.                                                                                                                                                                                                                                                                                                                      |
| 1485                                                                         | ComplexOptPayoutAmount             | N     |                                                                                                                                                                                                                                                                                                                                                                                             |
| 1486                                                                         | ComplexEventPrice                  | N     |                                                                                                                                                                                                                                                                                                                                                                                             |
| 1487                                                                         | ComplexEventPriceBoundaryMethod    | N     |                                                                                                                                                                                                                                                                                                                                                                                             |
| 1488                                                                         | ComplexEventPriceBoundaryPrecision | N     |                                                                                                                                                                                                                                                                                                                                                                                             |
| 1489                                                                         | ComplexEventPriceTimeType          | N     |                                                                                                                                                                                                                                                                                                                                                                                             |
| 1490                                                                         | ComplexEventCondition              | N     | ComplexEventCondition is conditionally required when there are more than one ComplexEvent occurrences. A chain of ComplexEvents must be linked together through use of the ComplexEventCondition in which the relationship between any two events is described. For any two ComplexEvents the first occurrence will specify the ComplexEventCondition which links it with the second event. |
| component block                                                              |                                    |       |                                                                                                                                                                                                                                                                                                                                                                                             |
| N                                                                            |                                    |       |                                                                                                                                                                                                                                                                                                                                                                                             |
| Used to specify the dates and time ranges when a complex event is in effect. |                                    |       |                                                                                                                                                                                                                                                                                                                                                                                             |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element CmplxEvnt

© Copyright, 2008-2011, FIX Protocol, Limited Page 102 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# ComplexEventDates component block

The ComplexEventDate and ComplexEventTime components are used to constrain a complex event to a specific date range or time range. If specified the event is only effective on or within the specified dates and times.

| Tag                  | FieldName            | Req'd | Comments                                                            |
| -------------------- | -------------------- | ----- | ------------------------------------------------------------------- |
| 1491                 | NoComplexEventDates  | N     | Number of complex event date occurrences for a given complex event. |
| 1492                 | ComplexEventStartDat | N     | Required if NoComplexEventDates(1491) > 0.                          |
| 1493                 | ComplexEventEndDat   | N     | Required if NoComplexEventDates(1491) > 0.                          |
| component block      |                      |       |                                                                     |
| \<ComplexEventTimes> |                      |       |                                                                     |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element EvntDts

# ComplexEventTimes component block

The ComplexEventTime component is nested within the ComplexEventDate in order to further qualify any dates placed on the event and is used to specify time ranges for which a complex event is effective. It is always provided within the context of start and end dates. The time range is assumed to be in effect for the entirety of the date or date range specified.

| Tag  | FieldName           | Req'd | Comments                                   |
| ---- | ------------------- | ----- | ------------------------------------------ |
| 1494 | NoComplexEventTimes | N     |                                            |
| 1495 | ComplexEventStartTi | N     | Required if NoComplexEventTimes(1494) > 0. |
| 1496 | ComplexEventEndTim  | N     | Required if NoComplexEventTimes(1494) > 0. |

FIXML Definition for this Component Block– see http://www.fixprotocol.org for details

Refer to FIXML element EvntTms

© Copyright, 2008-2011, FIX Protocol, Limited Page 103 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# COMMON INFRASTRUCTURE MESSAGES (Apply to pre-trade, trade, and post-trade)

# Business Message Reject

The Business Message Reject message can reject an application-level message which fulfills session-level
rules and cannot be rejected via any other means. Note if the message fails a session-level rule (e.g. body
length is incorrect), a session-level Reject message should be issued. The only exception to this rule is when
a transport other than the FIX session protocol is being used (transport independence). An appropriate reject
message of the given session protocol or the Business Message Reject message should be used instead.

See the session-level Reject message

It should NOT be used in the following situations:

| Situation                                                                      | Appropriate Response                                                                                                                                                                                                                               |
| ------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Session-level problem meeting the criteria of the session-level Reject message | Use the session-level Reject message (MsgType=3) if the FIX session protocol is being used. If the FIX session protocol is not being used, use an appropriate reject message of the given session protocol or the Business Message Reject message. |
| In response to: Quote Request                                                  | Use the Quote Request Reject message                                                                                                                                                                                                               |
| In response to: Quote                                                          | Use the Quote Status Report message                                                                                                                                                                                                                |
| In response to: Quote Cancel                                                   | Use the Quote Status Report message                                                                                                                                                                                                                |
| In response to: Quote Status Request                                           | Use the Quote Status Report message                                                                                                                                                                                                                |
| In response to: Quote Response                                                 | Use the Quote Status Report message                                                                                                                                                                                                                |
| In response to: Mass Quote                                                     | Use the Mass Quote Acknowledgment message                                                                                                                                                                                                          |
| In response to: Market Data Request                                            | Use the Market Data Request Reject message                                                                                                                                                                                                         |
| In response to: Stream Assignment Request                                      | Use the Stream Assignment Report message                                                                                                                                                                                                           |
| In response to: Stream Assignment Report                                       | Use the Stream Assignment Report Ack message                                                                                                                                                                                                       |
| In response to: Security Definition Request                                    | Use the Security Definition message                                                                                                                                                                                                                |
| In response to: Security Type Request                                          | Use the SecurityTypes message                                                                                                                                                                                                                      |
| In response to: Security List Request                                          | Use the Security List message                                                                                                                                                                                                                      |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 104 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                       August 18, 2011

# Security List Request

In response to: Use the Derivative Security List message

# Derivative Security List Request

In response to: Use the Security Status message

# Security Status Request

In response to: Use the Trading Session Status message

# Trading Session Status Request

In response to: Use the Trading Session List message

# Trading Session List Request

In response to: Use the Party Details List Report message

# Party Details List Request

In response to Use the Execution Report message

# New Order - Single

# Order Status Request

# Order Mass Status Request

# New Order – Cross

# New Order – Multileg

# New Order – List

# List Execute

In response to: Use the Order Cancel Reject message

# Order Cancel Request

# Order Cancel/Replace Request

# Cross Order Cancel Request

# Cross Order Cancel/Replace Request

# Multileg Order Cancel/Replace Request

# List Cancel Request

In response to: Use the Don’t Know Trade (DK) message or the Execution Report Acknowledgement message

In response to: Use the Order Mass Cancel Report message

# Order Mass Cancel Request

In response to: Use the Order Mass Action Report message

# Order Mass Action Request

In response to: Use the List Status message

# List Status Request

In response to: Use the Bid Response message

# Bid Request

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                             Page 105 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                           August 18, 2011

# In response to:

| Allocation Instruction         | Use the Allocation Instruction Ack message         |
| ------------------------------ | -------------------------------------------------- |
| Allocation Report              | Use the Allocation Report Ack message              |
| Confirmation                   | Use the Confirmation Ack message                   |
| Registration Instructions      | Use the Registration Instructions Response message |
| Trade Capture Report Request   | Use the Trade Capture Report message               |
| Confirmation Request           | Use the Confirmation message                       |
| Settlement Instruction Request | Use the Settlement Instructions message            |
| Position Maintenance Request   | Use the Position Maintenance Report message        |
| Request for Positions          | Use the Request for Positions Ack message          |
| Collateral Request             | Use the Collateral Assignment message              |
| Collateral Assignment          | Use the Collateral Response message                |
| Collateral Inquiry             | Use the Collateral Inquiry Ack message             |

# Note the only exceptions to this rule are:

1. In the event a business message is received, fulfills session-level rules, however, the message cannot be communicated to the business-level processing system. In this situation a Business Message Reject with BusinessRejectReason = “Application not available at this time” can be issued if the system is unable to send the specific “reject” message listed above due to this condition.
2. In the event a valid business message is received, fulfills session-level rules, however, the message type is not supported by the recipient. In this situation a Business Message Reject with BusinessRejectReason = “Unsupported Message Type” can be issued if the system is unable to send the specific “reject” message listed above because the receiving system cannot generate the related “reject” message.
3. In the event a business message is received, fulfills session-level rules, but lacks a field conditionally required by the FIX specification. In this situation a Business Message Reject with BusinessRejectReason = “Conditionally Required Field Missing” can be issued if the system is unable to send the specific “reject” message listed above. One example of this would be a stop order missing StopPx. However, a Business Message Reject message MUST NOT be used to enforce proprietary rules.

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                              Page 106 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                               August 18, 2011

# Messages which can be referenced via the Business Message Reject message are:

- (the “ID” field BusinessRejectRefID refers to noted in [ ])
- Indication of Interest (IOI) [IOIid]
- Advertisement [AdvId]
- News [Headline]
- Email [EmailThreadID]
- Market Data-Snapshot/Full Refresh [MDReqID]
- Market Data-Incremental Refresh [MDReqID]
- Market Data Request Reject [MDReqID]
- Market Definition [MarketReportID]
- Market Definition Request [MarketReqID]
- Market Definition Update Report [MarketReportID]
- Stream Assignment Ack [StramAsgnRptID]
- Security Definition [SecurityResponseID or SecurityReportID]
- Security Definition Update Report [SecurityResponseID or SecurityReportID]
- Security Status [SecurityStatusReqID]
- Security Types [SecurityResponseID]
- Security List [SecurityResponseID]
- Security List Update Report [SecurityResponseID or SecurityReportID]
- Derivative Security List [SecurityResponseID]
- Derivative Security List Update Report [SecurityRespondID]
- Trading Session Status [TradSesReqID]
- Trading Session List [TradSesReqID]
- Trading Session List Update Report [TradSesReqID]
- Party Details List Report [PartyDetailsListReportID]
- Mass Quote Acknowledgement [QuoteReqID or QuoteID]
- Quote Request Reject [QuoteReqID]
- RFQ Request [RFQReqID]
- Quote Status Report [QuoteStatusReqID or QuoteRespID or QuoteID or QuoteMsgID]
- Quote Status Report [QuoteID]
- Order Cancel Reject [ClOrdID]
- List Status [ListID]
- List Strike Price [ListID]
- Bid Response [BidID]
- Order Mass Cancel Report [OrderID]
- Order Mass Action Report [MassActionReportID]
- Order Mass Status Request [MassStatusReqID] [tbd]
- Don’t Know Trade (DK) – may respond with Order Cancel Reject if attempting to cancel order [ExecID]
- Execution Report Acknowledgement [ExecID]
- Allocation Instruction ACK [AllocID]
- Allocation Report ACK [AllocID]
- Allocation Alert [AllocID]
- Confirmation ACK [ConfirmID]
- Trade Capture Report [TradeReportID]
- Trade Capture Report Request Ack [TradeRequestID]
- Trade Capture Report Ack [TradeReportID]
- Position Maintenance Report [PosMaintRptID]
- Request for Positions Ack [PosMaintRptID]

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                      Page 107 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Adjusted Position Report [PosMaintRptID]

# Positions Report [PosMaintRptID]

# Assignment Report [AsgnRptID]

# Contrary Intention Report [ContIntRptID]

# Settlement Instructions [SettInstMsgID]

# Settlement Obligation Report [SettlObligMsgID]

# Registration Instructions Response [RegistID]

# Collateral Response [CollRespID]

# Collateral Inquiry Ack [CollInquiryID]

# Collateral Report [CollRptID]

# Scenarios for Business Message Reject:

BusinessRejectReason
0 = Other
1 = Unkown ID
2 = Unknown Security
3 = Unsupported Message Type (receive a valid, but unsupported MsgType)
4 = Application not available
5 = Conditionally Required Field Missing
6 = Not Authorised
7 = DeliverTo firm not available at this time
18 = Invalid price increment

Whenever possible, it is strongly recommended that the cause of the failure be described in the Text field (e.g. “UNKNOWN SYMBOL: XYZ”).

# The business message reject format is as follows:

| Tag            | FieldName    | Req'd | Comments                                                                                                                                                                                                                                                                                            |
| -------------- | ------------ | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader |              | Y     | MsgType = j (lowercase)                                                                                                                                                                                                                                                                             |
| 45             | RefSeqNum    | N     | MsgSeqNum of rejected message                                                                                                                                                                                                                                                                       |
| 372            | RefMsgType   | Y     | The MsgType of the FIX message being referenced.                                                                                                                                                                                                                                                    |
| 1130           | RefApplVerID | N     | Recommended when rejecting an application message that does not explicitly provide ApplVerID (1128) on the message being rejected. In this case the value from the DefaultApplVerID(1137) or the default value specified in the NoMsgTypes repeating group on the logon message should be provided. |
| 1406           | RefApplExtID | N     | Recommended when rejecting an application message that does not explicitly provide ApplExtID(1156) on the rejected message. In this case the value from the                                                                                                                                         |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 108 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

DefaultApplExtID(1407) or the default value specified in the NoMsgTypes repeating group on the logon message should be provided.

| 1131 | RefCstmApplVerID     | N | Recommended when rejecting an application message that does not explicitly provide CstmApplVerID(1129) on the message being rejected. In this case the value from the DefaultCstmApplVerID(1408) or the default value specified in the NoMsgTypes repeating group on the logon message should be provided. |
| ---- | -------------------- | - | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 379  | BusinessRejectRefID  | N | The value of the business-level "ID" field on the message being referenced. Required unless the corresponding ID field (see list above) was not specified.                                                                                                                                                 |
| 380  | BusinessRejectReason | Y | Code to identify reason for a Business Message Reject message.                                                                                                                                                                                                                                             |
| 58   | Text                 | N | Where possible, message to explain reason for rejection.                                                                                                                                                                                                                                                   |
| 354  | EncodedTextLen       | N | Must be set if EncodedText field is specified and must immediately precede it.                                                                                                                                                                                                                             |
| 355  | EncodedText          | N | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field.                                                                                                                                                                             |

StandardTrailer Y

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element BizMsgRej

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 109 of 158
---

Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011


# Network Status Messages

It is envisaged these messages will be used in two scenarios:

# Scenario A

Allow one counterparty using a “hub and spoke” FIX network to know whether another counterparty is currently connected to the hub (i.e. whether the counterparty's session to the hub is up or not).

# Scenario B

Allow a counterparty connecting to a global brokerage to know which regions within that brokerage are currently available as order routing destinations.

# Network Status Component Blocks

This section lists the component blocks used exclusively by the messages defined for Network Status.

# CompIDReqGrp component block

| Tag | FieldName  | Req'd | Comments                                                                                                                                                                                                                                                                                                                             |
| --- | ---------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 936 | NoCompIDs  | N     | Used to restrict updates/request to a list of specific CompID/SubID/LocationID/DeskID combinations. If not present request applies to all applicable available counterparties. EG Unless one sell side broker was a customer of another you would not expect to see information about other brokers, similarly one fund manager etc. |
| 930 | RefCompID  | N     | Used to restrict updates/request to specific CompID                                                                                                                                                                                                                                                                                  |
| 931 | RefSubID   | N     | Used to restrict updates/request to specific SubID                                                                                                                                                                                                                                                                                   |
| 283 | LocationID | N     | Used to restrict updates/request to specific LocationID                                                                                                                                                                                                                                                                              |
| 284 | DeskID     | N     | Used to restrict updates/request to specific DeskID                                                                                                                                                                                                                                                                                  |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element CIDReq

# CompIDStatGrp component block

| Tag | FieldName  | Req'd | Comments                                                                                               |
| --- | ---------- | ----- | ------------------------------------------------------------------------------------------------------ |
| 936 | NoCompIDs  | Y     | Specifies the number of repeating CompId's that status is being report for. Required if NoCompIDs > 0, |
| 930 | RefCompID  | Y     | CompID that status is being report for.                                                                |
| 931 | RefSubID   | N     | SubID that status is being report for.                                                                 |
| 283 | LocationID | N     | LocationID that status is being report for.                                                            |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited


Page 110 of 158

---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                August 18, 2011

# Network (Counterparty System) Status Request Message

This message is send either immediately after logging on to inform a network (counterparty system) of the type of updates required or to at any other time in the FIX conversation to change the nature of the types of status updates required. It can also be used with a NetworkRequestType of Snapshot to request a one-off report of the status of a network (or counterparty) system. Finally this message can also be used to cancel a request to receive updates into the status of the counterparties on a network by sending a NetworkRequestStatusMessage with a NetworkRequestType of StopSubscribing.

# Network (Counterparty System) Status Request

| Tag                             | FieldName          | Req'd | Comments                                                                                                                                                                                                                                                                                                                             |
| ------------------------------- | ------------------ | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader                  |                    | Y     | MsgType = "BC"                                                                                                                                                                                                                                                                                                                       |
| 935                             | NetworkRequestType | Y     |                                                                                                                                                                                                                                                                                                                                      |
| 933                             | NetworkRequestID   | Y     |                                                                                                                                                                                                                                                                                                                                      |
| component block \<CompIDReqGrp> |                    | N     | Used to restrict updates/request to a list of specific CompID/SubID/LocationID/DeskID combinations. If not present request applies to all applicable available counterparties. EG Unless one sell side broker was a customer of another you would not expect to see information about other brokers, similarly one fund manager etc. |
| StandardTrailer                 |                    | Y     |                                                                                                                                                                                                                                                                                                                                      |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element NtwkSysStatReq

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                            Page 111 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                              August 18, 2011

# Network (Counterparty System) Status Response Message

This message is sent in response to a Network (Counterparty System) Status Request Message. If the network response payload is larger than the maximum permitted message size for that FIX conversation the response would be several Network Status Response Messages the first with a status of full and then as many messages, as updates to the first message, adding information as required.

# Network (Counterparty System) Status Response

| Tag                              | FieldName                 | Req'd | Comments                                   |
| -------------------------------- | ------------------------- | ----- | ------------------------------------------ |
| StandardHeader                   |                           | Y     | MsgType = "BD"                             |
| 937                              | NetworkStatusResponseType | Y     |                                            |
| 933                              | NetworkRequestID          | N     |                                            |
| 932                              | NetworkResponseID         | Y     |                                            |
| 934                              | LastNetworkResponseID     | N     | Required when NetworkStatusResponseType=2  |
| component block \<CompIDStatGrp> |                           | Y     | Specifies the number of repeating CompId's |
| StandardTrailer                  |                           | Y     |                                            |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element NtwkSysStatRsp

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                    Page 112 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                            August 18, 2011

# User Administration Messages

These messages are provided in FIX to allow the passing of individual user information between two counterparties. The messages allow for the following function:

1. Individual User Logon
2. Individual User Status Enquiries
3. Individual User Logout
4. Individual User password change

NOTE: While it is not encouraged to transmit passwords in a FIX conversation unless you can guarantee the end to end security of both the FIX conversation and any intermediate routing hubs that are involved in the routing.

# User Management Component Blocks

This section lists the component blocks used exclusively by the messages defined for Network Status.

# UsernameGrp component block

| Tag | FieldName   | Req'd    |                     | Comments                      |
| --- | ----------- | -------- | ------------------- | ----------------------------- |
| 809 | NoUsernames | N        | Number of usernames |                               |
| →   | 553         | Username | N                   | Recipient of the notification |

FIXML Definition for this Message– see http://www.fixprotocol.org for details. Refer to the FIXML element UserGrp.

# User Request Message

This message is used to initiate a user action, logon, logout or password change. It can also be used to request a report on a user’s status.

# User Request

| Tag            | FieldName       | Req'd | Comments       |
| -------------- | --------------- | ----- | -------------- |
| StandardHeader |                 | Y     | MsgType = "BE" |
| 923            | UserRequestID   | Y     |                |
| 924            | UserRequestType | Y     |                |
| 553            | Username        | Y     |                |
| 554            | Password        | N     |                |

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                   Page 113 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                     August 18, 2011

| 925  | NewPassword             | N |                                                       |
| ---- | ----------------------- | - | ----------------------------------------------------- |
| 1400 | EncryptedPasswordMethod | N |                                                       |
| 1401 | EncryptedPasswordLen    | N |                                                       |
| 1402 | EncryptedPassword       | N |                                                       |
| 1403 | EncryptedNewPasswordLen | N |                                                       |
| 1404 | EncryptedNewPassword    | N |                                                       |
| 95   | RawDataLength           | N |                                                       |
| 96   | RawData                 | N | Can be used to hand structures etc to other API's etc |

StandardTrailer                        Y

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element UserReq

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited

Page 114 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                            August 18, 2011

# User Response Message

This message is used to respond to a user request message, it reports the status of the user after the completion of any action requested in the user request message.

# User Response

| Tag             | FieldName      | Req'd | Comments                             |
| --------------- | -------------- | ----- | ------------------------------------ |
| StandardHeader  |                | Y     | MsgType = "BF"                       |
| 923             | UserRequestID  | Y     |                                      |
| 553             | Username       | Y     |                                      |
| 926             | UserStatus     | N     |                                      |
| 927             | UserStatusText | N     | Reason a request was not carried out |
| StandardTrailer |                | Y     |                                      |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element UserRsp

# User Notification

The User Notification message is used to notify one or more users of an event or information from the sender of the message. This message is usually sent unsolicited from a marketplace (e.g. Exchange, ECN) to a market participant.

# User Notification

| Tag                            | FieldName      | Req'd | Comments                                                                                                                       |
| ------------------------------ | -------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------ |
| StandardHeader                 |                | Y     | MsgType = CB                                                                                                                   |
| component block \<UsernameGrp> |                | N     | List of users to which the notification is directed                                                                            |
| 926                            | UserStatus     | Y     | Reason for notification - when possible provide an explanation.                                                                |
| 58                             | Text           | N     | Explanation for user notification.                                                                                             |
| 354                            | EncodedTextLen | N     | Must be set if EncodedText field is specified and must immediately precede it.                                                 |
| 355                            | EncodedText    | N     | Encoded (non-ASCII characters) representation of the Text field in the encoded format specified via the MessageEncoding field. |
| StandardTrailer                |                | Y     |                                                                                                                                |

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                      Page 115 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                          August 18, 2011

# FIXML Definition for this Message

– see http://www.fixprotocol.org for details

Refer to the FIXML element UserNotification

© Copyright, 2008-2011, FIX Protocol, Limited

Page 116 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                  August 18, 2011

# Application Sequencing Messages

# Introduction

FIX has a growing need to support application-level sequencing of messages in order to segregate the transmission of data over a session. The ability to identify and retransmit a subset of data by application and application sequence number range is an important feature in support of secondary distribution of data (see definition below). The current retransmission capabilities of the FIX session require that all messages on that session between the specified starting and ending message sequence number are resent rather than just those that have been produced by a specific upstream business process or application. This can pose capacity and performance problems for systems that need only a small set of messages related to an application. Secondary data distribution consists of a diverse set of data sourced from different applications; drop copy data, credit limit information, metrics, etc. It is not recommended that application sequencing is used over a conventional order routing or a transaction flow oriented connection. Standard FIX session capabilities should be used in this case.

Application Sequencing greatly enhances the usefulness of FIX messages that are transmitted apart from the FIX session layer by making it possible for the receiver to detect and request missed messages on a specified feed. Market Data sent over a broadcast or multicast transport is often in need of sequencing and retransmission. Application Sequencing provides a means by which to sequence each message that is part of a broadcast stream such that the receiver can verify ordered delivery of the data. Application Resends can then be requested when gaps are detected in the application sequence.

# Background

The purpose of Application-level Sequencing is to allow messages being sent over a FIX session to be distinguished by the sending application that is upstream from the FIX engine. In the case that a session-level resend would result in an unnecessarily large number of messages being resent, Application Sequencing and Recovery makes provision for the desired messages - and only the desired messages - to be seamlessly requested and resent while retaining the standard behaviors of the session protocol. It also provides the receiver with the flexibility to put off recovery of application level messages until a slow period or after the market has closed.

Extends control over resent data The primary intent of Application Sequencing and Recovery is to allow receivers to avoid a retransmission of large quantities of unusable data which may result in receivers needing to glean the retransmission for the data they actually need - such as critical drop copy information that is used in risk management applications. Application Sequencing allows the channeling of different types of data across a single FIX session. For example, Application Sequencing can allow drop copy data to be sent over the same FIX session with order flow data. While this may not be practical from a trading standpoint the flexibility that it introduces is compelling. This allows data which has a higher importance and priority to be identified by application ID thereby allowing requests for retransmission to be issued promptly and precisely.

Support for secondary data distribution Another goal of the proposal is to provide support for “secondary data” distribution. Application Sequencing extends the capabilities of FIX such that secondary data can be distributed using a single channel. This data may be less time critical with less demanding latency requirements than order entry and market data, although this is not necessarily the case as drop copies are used for time sensitive risk management tasks. Secondary data may consist of drop copy fills, credit limit information, statistical data, trade confirmations, and best bids and offers for vendor consumption, etc. These are just a few of the possibilities. Application Sequencing benefits data providers and their users by providing a common protocol which can be used to perform secondary data distribution. New applications transmitting data can be quickly introduced over an existing channel with minimal effort simply by introducing a new ApplID (application ID).

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                        Page 117 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                 August 18, 2011

Transaction usage is not recommended. Application Sequencing is not something that will be used in a normal order routing scenario. It has more relevance in large volume one-way connections in which the receiver would like to have some ability to control the data that is resent after a disconnect or data loss. There is no obvious advantage in using Application Sequencing with a regular trading connection since all data transmitted between Sender and Receiver is of equal importance in maintaining a viable trading session. Application Sequencing should not be used to track broker connections that are in place for trading purposes. It should only be used for managing the flow of data when a FIX connection is used to deliver data in bulk and where there is a stated need to create classes of data. For additional usage guidance on using these messages, see Volume 7 – USER GROUP: EXCHANGES AND MARKETS.

# Using Application Sequencing and Session Sequencing for Gap Detection

The use of ApplResendFlag on the new Application Sequence Group Component is used to indicate that messages are being retransmitted as a result of an Application Message Request. It is possible for both ApplResendFlag and PossDupFlag to be set on the same message if the Sender’s cache size is greater than zero and the message is being resent due to a session level resend request.

The Sender and Receiver may agree to use a limited cache in order to benefit from the convenience of session-level retransmission. In this case, a message that is dropped in response to an Application Message Request may have both fields present. This scenario depends on whether (1) the Sender is maintaining a cache and (2) the Sender and Receiver have agreed to fill any gaps to the extent possible using the session-level.

In this scenario, a combination of Application and Session level sequencing will be used to recover missed messages. A limited cache of session level messages may be retained by the Sender in order to recover messages that have been dropped within a pre-stated window defined by time or number of messages. When a FIX session Resend Request message is issued within this window the Sender’s session will resend the messages. Once the window has been exceeded an Application Message Request must be issued in order to recover dropped messages. The application level will not be aware that a gap has occurred until the session level has recovered what is available. Beyond this, the application will detect the gap according to the logic as described and issue a resend request at the application level using the Application Message Request.

Gap detection and recovery with respect to the Application Message Request message and response messages (e.g. Application Message Request Ack and resent application messages using ApplicationSequenceGrp component block) may also need to take place at the application level since session level recovery may have been suspended.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                        Page 118 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Applicaton Sequencing Component Blocks

This section lists the component blocks used exclusively by the messages defined for Application Sequencing.

# ApplIDRequestGrp component block

| Tag  | FieldName       | Req'd | Comments                                                                                                                                                                                                                                        |
| ---- | --------------- | ----- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1351 | NoApplIDs       | N     | Specifies number of application id occurrences                                                                                                                                                                                                  |
| 1355 | RefApplID       | N     |                                                                                                                                                                                                                                                 |
| 1433 | RefApplReqID    | N     |                                                                                                                                                                                                                                                 |
| 1182 | ApplBegSeqNum   | N     | Message sequence number of first message in range to be resent                                                                                                                                                                                  |
| 1183 | ApplEndSeqNum   | N     | Message sequence number of last message in range to be resent. If request is for a single message ApplBeginSeqNo = ApplEndSeqNo. If request is for all messages subsequent to a particular message, ApplEndSeqNo = "0" (representing infinity). |
|      | component block | N     | \<NestedParties>                                                                                                                                                                                                                                |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element ApplIDReqGrp

# ApplIDRequestAckGrp component block

| Tag  | FieldName         | Req'd | Comments               |
| ---- | ----------------- | ----- | ---------------------- |
| 1351 | NoApplIDs         | N     | Number of applications |
| 1355 | RefApplID         | N     |                        |
| 1433 | RefApplReqID      | N     |                        |
| 1182 | ApplBegSeqNum     | N     |                        |
| 1183 | ApplEndSeqNum     | N     |                        |
| 1357 | RefApplLastSeqNum | N     |                        |
| 1354 | ApplResponseError | N     |                        |
|      | component block   | N     | \<NestedParties>       |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 119 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                 August 18, 2011

# Refer to the FIXML element ApplIDReqAckGrp

# ApplIDReportGrp component block

| Tag  | FieldName         | Req'd | Comments               |
| ---- | ----------------- | ----- | ---------------------- |
| 1351 | NoApplIDs         | N     | Number of applications |
| 1355 | RefApplID         | N     |                        |
| 1399 | ApplNewSeqNum     | N     |                        |
| 1357 | RefApplLastSeqNum | N     |                        |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

# Refer to the FIXML element ApplIDRptGrp

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited

Page 120 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                 August 18, 2011

# Application Message Request

This message is used to request a retransmission of a set of one or more messages generated by the application specified in RefApplID (1355). The message can be used for five types of transmission requests:

- 0 - retransmission of application messages for a specified application and sequence number range,
- 1 - subscription to an application in order to receive, for example, drop copy services,
- 2 - request for the last application sequence number sent by an application,
- 3 - request the valid set of application identifiers for which a user is authorized,
- 4 - unsubscribe to one or more applications.

The Request message specifies the sequence number range using ApplBegSeqNum (1182) and ApplEndSeqNum (1183) for a given RefApplID (1355) to request messages for retransmission.

| Tag             | FieldName           |
| --------------- | ------------------- |
| StandardHeader  |                     |
| 1346            | ApplReqID           |
| 1347            | ApplReqType         |
| component block | \<ApplIDRequestGrp> |
| component block | \<Parties>          |
| 58              | Text                |
| 354             | EncodedTextLen      |
| 355             | EncodedText         |
| StandardTrailer |                     |

# Application Message Request

| Req'd | Comments                                       |
| ----- | ---------------------------------------------- |
| Y     | MsgType = BW                                   |
| Y     | Unique identifier for request                  |
| Y     | Type of Application Message Request being made |
| N     |                                                |
| N     |                                                |
| N     | Allows user to provide reason for request      |
| N     |                                                |
| N     |                                                |
| Y     |                                                |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element ApplMsgReq

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited
Page 121 of 158
---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                                        August 18, 2011


# Application Message Request Ack

This message is used to acknowledge an Application Message Request providing a status on the request (i.e. whether successful or not). This message does not provide the actual content of the messages to be resent.

| Tag             | FieldName              |
| --------------- | ---------------------- |
| StandardHeader  |                        |
| 1353            | ApplResponseID         |
| 1346            | ApplReqID              |
| 1347            | ApplReqType            |
| 1348            | ApplResponseType       |
| 1349            | ApplTotalMessageCount  |
| component block | \<ApplIDRequestAckGrp> |
| component block | \<Parties>             |
| 58              | Text                   |
| 354             | EncodedTextLen         |
| 355             | EncodedText            |
| StandardTrailer |                        |

# Application Message Request Ack

| Req'd | Comments                                                   |
| ----- | ---------------------------------------------------------- |
| Y     | MsgType = BX                                               |
| Y     | Identifier for the Application Message Request Ack         |
| N     | Identifier of the request associated with this ACK message |
| N     |                                                            |
| N     |                                                            |
| N     | Total number of messages included in transmission          |
| N     |                                                            |
| N     |                                                            |
| N     |                                                            |
| Y     |                                                            |

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element ApplMsgReqAck

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited


Page 122 of 158

---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                   August 18, 2011

# Application Message Report

This message is used for three different purposes: to reset the ApplSeqNum (1181) of a specified ApplID (1180), to indicate that the last message has been sent for a particular ApplID, or as a keep-alive mechanism for ApplIDs with infrequent message traffic. The purpose of the Application Message Report is indicated by ApplReportType (1426).

# Using Application Message Report to reset ApplSeqNum

The Application Message Report (Reset) is sent by the ApplID sender to alert the receiver that the ApplSeqNum is being reset, for one or more ApplID, to the specified value(s). The next application message received will then conform to this value. In other words, ApplSeqNum in this message represents the next expected application sequence number the receiver will receive from the sender for the corresponding ApplID. An Application Message Report (Reset) has no effect on, and is independent of, the FIX session sequence number in MsgSeqNum (34).

# Using Application Message Report to indicate last message sent

The ApplID sender can use the Application Message Report to indicate that the last message has been sent for one or more ApplIDs. Reception of this message means the recipient can safely assume that no more messages will be sent for that/or those ApplIDs. RefApplLastSeqNum should be set to the last ApplSeqNum sent for this ApplID.

# Using Application Message Report as keep-alive mechanism

For recipients of ApplIDs with infrequent message traffic it is a problem to detect a gap in the message flow. The gap cannot be detected until reception of the next message for that ApplID. To mitigate this problem the Application Message Report message can be issued by the ApplID sender at regular intervals. RefApplLastSeqNum should be set to the last ApplSeqNum sent for this ApplID.

# Using Application Message Report to indicate completion of resent messages

As part of a recovery scenario, the receiver (or consumer) may request all of the messages for one or more ApplIDs. Because of the potentially lengthy re-send situation, the request can be acknowledged with an ApplicationMessageRequestAck prior to beginning the re-send of messages. In this case, the receiver or consumer will begin seeing re-sent messages until the re-send is complete. However, once the re-send is complete, the receiver or consumer will only know that the re-send has completed when they receive a new copied message from that ApplID that no longer has tag 1352 ApplResendFlag=Y. If the specified ApplID is only “heartbeating” and there are no new messages to send, the consumer will still not know the Application Message re-send has actually finished. It is in this case that an Application Message Report can be generated, which signals that the Application Message re-send has completed by setting ApplReportType (1426) = 3 (application message re-send completed).

# Application Message Report

| Tag                | FieldName      | Req'd | Comments                                                                                                                                                            |
| ------------------ | -------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| StandardHeader     |                | Y     | MsgType = BY                                                                                                                                                        |
| 1356               | ApplReportID   | Y     | Identifier for the Application Message Report                                                                                                                       |
| 1346               | ApplReqID      | N     | If the application message report is generated in response to an ApplicationMessageRequest(MsgType=BW), then this tag contains the ApplReqID(1346) of that request. |
| 1426               | ApplReportType | Y     | Type of report                                                                                                                                                      |
| component block    |                | N     |                                                                                                                                                                     |
| \<ApplIDReportGrp> |                |       |                                                                                                                                                                     |
| 58                 | Text           | N     |                                                                                                                                                                     |

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                        Page 123 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                        August 18, 2011

# Errata

| 354 | EncodedTextLen | N |
| --- | -------------- | - |
| 355 | EncodedText    | N |

StandardTrailer                        Y

FIXML Definition for this Message– see http://www.fixprotocol.org for details

Refer to the FIXML element ApplSeqReset

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 124 of 158


---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                  August 18, 2011

# Application Sequencing Message flows

# Application recovery over a FIX session

The message flow shows a use case in which the Receiver disconnects from the Sender and generates an Application Message Request in order to fill the sequence gap. The workflow occurs as follows (see Figure 1):

1. The Sender and receiver establish a connection using a standard FIX session.
2. The Sender forwards messages to a receiver from Application 1 and Application 2 over the FIX session. The Receiver is checking Application Sequence Numbers at the application level.
3. The Receiver then experiences a disconnection.
4. While disconnected, the Sender continues to send messages for Application 1 and Application 2.
5. Upon reconnecting, the Receiver’s session sends a Resend Request to recover missed messages (due to Logon with higher than expected MsgSeqNum).
6. The Sender responds with SequenceReset-GapFill in order to suppress the session level retransmission of messages. This has been pre-arranged between parties based on the nature of the connection.
7. A message for Application 1, ApplSeqNum is 11 is received causing the Receiver to detect an application level gap.
8. The Receiver sends an Application Message Request to specifically request any messages from Application 1 that may have been missed starting at the last application sequence number received.
9. Messages from Application 2 are not requested and are therefore not retransmitted.
10. The Sender retransmits the requested messages for Application 1.

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                        Page 125 of 158
---
Version 5.0 Service Pack 2 - Errata        VOLUME 1                                                                        August 18, 2011

# Application Sequencing Recovery over a FIX session

|                 | Receiver                         | Receiver                        | Sender                         | Sender                         |                                  |
| --------------- | -------------------------------- | ------------------------------- | ------------------------------ | ------------------------------ | -------------------------------- |
|                 |                                  |                                 | Application1                   | Application2                   |                                  |
| Receiving       | Application checks               | ApplFeedID=A1, ApplSeqNum=1     | ApplFeedID=A1, ApplSeqNum=1    |                                |                                  |
|                 | ApplFeedID=A2, ApplSeqNum=1      | ApplFeedID=A2, ApplSeqNum=1     | Application 2 Messages         |                                |                                  |
| Disconnect      |                                  |                                 | Application 1 Messages         | ApplFeedID=A1, ApplSeqNum=2-10 | ApplFeedID=A2, ApplSeqNum=2-9999 |
|                 | Reconnect                        |                                 | Sender responds                | using GapFill for              | ALL MESSAGES                     |
|                 | Next message for                 | App A1 is sent                  | Logon MsgSeqNum=10009          | Resend Request (35=4)          |                                  |
| Receiving App   | ApplFeedID=A1, ApplSeqNum=11     | ApplFeedID=A1, ApplSeqNum=11    | Application 1 Messages         |                                |                                  |
| detects gap and | generates App                    | Appl Message Request – A1, 2-10 | Messages for                   | App A1 are                     |                                  |
| Resend Request  | Appl Message Response – A1, 2-10 | ApplFeedID=A1, ApplSeqNum=2     | ApplFeedID=A1, ApplSeqNum=2-10 |                                |                                  |

# Application recovery independent of FIX session

The message flow shows a use case in which the Receiver is using Application Sequencing and Recovery to recover data over that has been lost over a multicast-broadcast transport. In this scenario the following recovery takes place (see Figure 2):

1. The Sender sequences the messages in Application 1 and 2 using ApplSeqNum so that the Receiver/s are able to detect gaps and perform Application Message Requests based on the ApplSeqNum.
2. The Receiver disconnects from the feed while the Sender continues to generate new messages.
3. Once reconnected the Receiver detects the gap and generates an Application Message Request for Application 1 in order to fill the sequence gap. The missed messages for Application 2 are not recovered.
4. The Request is sent over the back-channel which has been separately established in order to support the Resend Request.
5. The Sender uses the back-channel to respond with an Application Resend Response and delivers the requested messages with the original ApplSeqNum.

© Copyright, 2008-          ~~2009~~2011, FIX Protocol, Limited                                                         Page 126 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Application Sequencing Recovery Outside of the FIX Session

| Receiver                                                           | Receiver                                                                                                          | Sender                       | Sender                 | Sender                 |
| ------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------- | ---------------------------- | ---------------------- | ---------------------- |
| Application                                                        |                                                                                                                   | Application1                 | Application2           |                        |
| # Multicast # One-way # Multicast                                  |                                                                                                                   |                              |                        |                        |
| Receiving Application checks                                       | ApplFeedID=A1, ApplSeqNum=1                                                                                       | ApplFeedID=A1, ApplSeqNum=1  | Application 1 Messages | Application 2 Messages |
| ApplFeedID=A2, ApplSeqNum=1                                        | ApplFeedID=A2, ApplSeqNum=1                                                                                       |                              |                        |                        |
| Disconnect                                                         | # Application 1 Messages ApplFeedID=A1, ApplSeqNum=2-10 ApplFeedID=A2, ApplSeqNum=2-9999 # Application 2 Messages |                              |                        |                        |
| Reconnect                                                          | # Next message for App A1 is sent                                                                                 |                              |                        |                        |
| Receiving App detects gap and generates App Resend Request         | ApplFeedID=A1, ApplSeqNum=11                                                                                      | ApplFeedID=A1, ApplSeqNum=11 | Application 1 Messages |                        |
| # Appl Message Request – A1, 2-10 # Messages for App A1 are Resent |                                                                                                                   |                              |                        |                        |
| ApplFeedID=A1, ApplSeqNum=2                                        | ApplFeedID=A1, ApplSeqNum=2-10                                                                                    |                              |                        |                        |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 127 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Glossary

# Business Terms

The following glossary is an attempt to identify business terms used in this document or related to implementing FIX globally. Requests for new terms and/or suggested definitions should be posted in the FIX Web Site’s Discussion section.

| Term                    | Definition                                                                                                                                                                                                                                                                                                                                                                                                                              | Field where used  |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------- |
| Acceptable Counterparty | A counterparty eligible for trading with the order or quote Initiator.                                                                                                                                                                                                                                                                                                                                                                  | \[PartyRole]      |
| Accrued Interest Rate   | The amount the buyer compensates the seller for the portion of the next coupon interest payment the seller has earned but will not receive from the issuer because the issuer will send the next coupon payment to the buyer. Accrued Interest Rate is the annualized Accrued Interest amount divided by the purchase price of the bond.                                                                                                |                   |
| ACPN                    | Accrued Coupon (ACPN) is a pro-rated amount from the prior coupon date to the current business date which is collateralized by the clearing house \[from EP83]                                                                                                                                                                                                                                                                          | \[PosAmtTyp]      |
| After Tax Yield         | Municipals. The yield on the bond net of any tax consequences from holding the bond. The discount on municipal securities can be subject to both capital gains taxes and ordinary income taxes. Calculated from dollar price.                                                                                                                                                                                                           | \[YieldType]      |
| All or None             | A round-lot market or limit-price order that must be executed in its entirety or not at all; unlike Fill or Kill orders, AON orders are not treated as canceled if they are not executed as soon as represented in the Trading Crowd.                                                                                                                                                                                                   | \[ExecInst]       |
| Allowances              | Under an emissions cap and trade program, each allowance entitles the holder to emit some amount of gas such as carbon. Sources that emit less than their emissions cap can sell allowances to those sources needing to purchase additional allowances to comply with the cap. Emission sources can then decide whether to control emissions through control technology or through allowance surrender to meet compliance. \[from EP89] | \[UnitOfMeasur e] |
| American style option   | An option that can be exercised at anytime before its expiration date.                                                                                                                                                                                                                                                                                                                                                                  | \[ExerciseStyle]  |
| Annual Yield            | The annual interest or dividend income an investment earns, expressed as a percentage of the investment’s total value.                                                                                                                                                                                                                                                                                                                  | \[YieldType]      |
| As defined              | Sides of the legs are the same as defined in the multileg instrument.                                                                                                                                                                                                                                                                                                                                                                   | \[Side]           |
| At Crossing             | An order that is valid only during crossing (auction) phases. The order is valid during the day or up to and including a specified.                                                                                                                                                                                                                                                                                                     | \[TimeInForce]    |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 128 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Term

# Definition

# Field where used

| At the close                  | Indicated price is to be around the closing price, however, not held                                                                                                                                                                    | \[IOIQualifier]        |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| At the Opening                | A market or limit-price order to be executed at the opening of the stock or not at all; all or part of any order not executed at the opening is treated as canceled.                                                                    | \[TimeInForce]         |
| Auto-match                    | Specifies that the source matching mechanism for a trade is automatic matching (of orders and / or quotes)                                                                                                                              | \[MatchType]           |
| Automated Floor Order Routing | The use of electronic devices or systems to capture orders and route the resulting trades to downstream system for matching and post-trade activities.                                                                                  | \[TradeHandling Instr] |
| Average Price Guarantee       | A limit order instruction that order allows fills against worse prices if this is compensated with higher prices so the volume weighted average is at the limit price or better. Applies to each execution round in automatic matching. | \[DiscretionInst]      |
| Average Price (Asian) Option  | The underlying price is an average of the daily settlement prices over a specified period                                                                                                                                               | \[from EP92]           |
| Average Strike                | The strike price is an average of the daily settlement prices over a specified period                                                                                                                                                   | \[from EP92]           |
| BANK                          | Total Banked Amount (BANK) represents the summation of all banked amounts (ICPN+TVAR+IACPN+ICMTM+CPN+DLV)                                                                                                                               | \[PosAmtTyp]           |
| Barrier Option                | The option becomes active (knock-in) or inactive (knock-out) based on a predetermined price level                                                                                                                                       | \[from EP92]           |
| Basis Price                   | A price established by joint agreement of odd-lot dealers in 100-share-unit stocks when:                                                                                                                                                | \[OrdType]             |
|                               | - no round-lot has occurred during the trading session,                                                                                                                                                                                 |                        |
|                               | - the spread between the closing bid and offer is two points or more, and                                                                                                                                                               |                        |
|                               | - on odd-lot the dealer has been given a “basis-price” order.                                                                                                                                                                           |                        |
| Bermuda style option          | A type of option that can only be exercised on predetermined dates, usually every month.                                                                                                                                                | \[ExerciseStyle]       |
|                               | Source: www\.investopedia.com                                                                                                                                                                                                           |                        |
| Binary All or None Option     | Fixed pay out if the underlying settles on a predefined trigger price.                                                                                                                                                                  | \[from EP92]           |
| Binary Barrier Option         | A digital option which becomes active or inactive based on the                                                                                                                                                                          |                        |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 129 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Definitions

| Term                    | Definition                                                                                                                                                                                                                                                                                                                                                                                                                                  | Field where used       |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| crossing of a barrier.  | Events are linked through “And” condition.                                                                                                                                                                                                                                                                                                                                                                                                  | \[from EP92]           |
| Binary One Touch Option | Immediate fixed pay out if the underlying reaches the predefined trigger price at any point during the life of the instrument.                                                                                                                                                                                                                                                                                                              | \[from EP92]           |
| Binary Option           | A binary option is a cash settled option that has a discontinuous payoff. Binary options come in many forms, but the two most basic are: cash-or-nothing and asset-or-nothing. Each can be European or American style and can be structured as a put or call. Also called a "digital option" or "all-or-nothing option". Source: www\.riskglossary.com                                                                                      |                        |
| Binary Range Option     | Fixed pay out if the underlying settles between an upper and lower trigger price.                                                                                                                                                                                                                                                                                                                                                           | \[from EP92]           |
| Block Lot               | A lot size that is larger than the Round Lot and associated with special block trading rules as bilaterally agreed between parties.                                                                                                                                                                                                                                                                                                         | \[LotType]             |
| Block Trade             | A Block Trade is a privately negotiated futures transaction executed apart from the public auction market, either on or off the exchange trading floor.                                                                                                                                                                                                                                                                                     | \[TradeCondition]      |
| Book Yield              | The yield of a security calculated by using its book value instead of the current market price. This term is typically used in the US domestic market.                                                                                                                                                                                                                                                                                      | \[YieldType]           |
| Broker Execution        | According to US futures markets (CFTC): Time at which a broker executed the order for another broker.                                                                                                                                                                                                                                                                                                                                       | \[TrdRegTimestampType] |
| Broker of Credit        | Broker to receive trade credit.                                                                                                                                                                                                                                                                                                                                                                                                             | \[PartyRole]           |
| Broker Receipt          | According to US futures markets (CFTC): Time at which broker received the order.                                                                                                                                                                                                                                                                                                                                                            | \[TrdRegTimestampType] |
| Buy Minus               | A round-lot market order to buy “minus” is an order to buy a stated amount of a stock provided that its price is: - not higher than the last sale if the last sale was a “minus” or “zero minus” tick and - not higher than the last sale minus the minimum fractional change in the stock if the last sale was a “plus” or “zero plus” tick. A limit price order to buy “minus” also states the highest price at which it can be executed. | \[Side]                |
| Cabinet Trade           | An off-market transaction to close out a nearly worthless out-of-the-money option contract.                                                                                                                                                                                                                                                                                                                                                 |                        |
| Call Date               | The date on which the issuer of a security has the right but not the obligation to repurchase the security at a predetermined price.                                                                                                                                                                                                                                                                                                        | \[EventType]           |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 130 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Term                                  | Definition                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Field where used |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| Call First                            | Refer to client before trading.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | \[ExecInst]      |
| Cancel if Not Best                    | Indicates that an order should be cancelled if it is no longer the best bid if buying, or the best offer if selling. If the order is cancelled due to this instruction, the message cancelling it must carry ExecRestatementReason="Canceled, Not Best".                                                                                                                                                                                                                                                                                                | \[ExecInst]      |
| Cancel on System Failure              | If a system failure interrupts trading or order routing, attempt to cancel this order. Note that depending on the type and severity of the failure, this might not be possible.                                                                                                                                                                                                                                                                                                                                                                         | \[ExecInst]      |
| Cancel on Trading Halt                | If trading in this instrument is halted, cancel this order and do not reinstate it when/if trading resumes.                                                                                                                                                                                                                                                                                                                                                                                                                                             | \[ExecInst]      |
| Capped Asian Option                   | A capped option which pays out based on the average price of the underlying.                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | \[from EP92]     |
| Capped Barrier Option                 | A capped option which becomes active or inactive based on the crossing of a barrier.                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | \[from EP92]     |
| Capped Call Option                    | The option has a linear payout (like a vanilla) up to a capped amount as specified by a cap price.                                                                                                                                                                                                                                                                                                                                                                                                                                                      | \[from EP92]     |
| Capped Payout Option                  | The payout amount is capped based on the difference between the strike and a cap price for call options and a floor price for put options.                                                                                                                                                                                                                                                                                                                                                                                                              | \[from EP92]     |
| CEA                                   | Credit Event Adjustment quantity (CEA) is used to represent the position movement associated with a credit event on processing date.                                                                                                                                                                                                                                                                                                                                                                                                                    | \[PosType]       |
| Central Registration Depository (CRD) | “The Central Registration Depository is a computerized database that contains information about most brokers, their representatives, and the firms they work for.” From SEC website: www\.sec.gov/investor/brokers.htm                                                                                                                                                                                                                                                                                                                                  | \[from EP79]     |
| CIV ("Collective Investment Vehicle") | Collective investment vehicle ("CIV") are set up for the purposes of collecting and pooling investor funds and issuing shares (or their equivalent). "Open-ended" CIVs entitle the holder to receive, on demand, an amount in value which is proportionate to the whole net asset value of the vehicle. Conversely "Closed-ended" CIVs do not grant this right to investors. CIVs are more commonly known as Mutual Funds, Unit Trusts, OEICS (Open Ended Investment Companies), SICAVs etc. A CIV may be legally constituted as a Limited Company with | \[CFICode]       |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 131 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Term                                                                                               | Definition                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Field where used  |
| -------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------- |
| variable capital, a Trust or a Limited Partnership - depending on local legislation & tax regimes. | CIVs typically invest in equities, bonds, derivatives etc. - as described in their prospectus. Other CIVs are Umbrella Fund (made up of sub-funds investing in equities, gilts etc), Fund Of Funds (invests only in other funds), Master-Feeder Fund (marketed to a specific group for investment in a central master fund), Multi-Manager Fund (whose asset management is divied between several managers), Side By Side (onshore and offshore funds with the same investment strategy) |                   |
| Clearing Firm                                                                                      | Firm that will clear the trade. Used if different from the executing firm.                                                                                                                                                                                                                                                                                                                                                                                                               | \[PartyRole]      |
| Clearing Organization                                                                              | Identifies the Clearing Organization where the position is maintained.                                                                                                                                                                                                                                                                                                                                                                                                                   | \[PartyRole]      |
| Client ID                                                                                          | Firm identifier used in third party-transactions or for investor identification in intermediary transactions. (should not be a substitute for OnBehalfOfCompID/DeliverToCompID).                                                                                                                                                                                                                                                                                                         | \[PartyRole]      |
| Close                                                                                              | An instruction to position keeping that the trade should bring the position towards zero, i.e. close as much as possible of any existing position and open an opposite position for any remainder.                                                                                                                                                                                                                                                                                       | \[PositionEffect] |
| Close but Notify on Open                                                                           | An instruction to position keeping that the trade should close an existing position. If the position is closed out and an opposite position is opened a notification is sent out.                                                                                                                                                                                                                                                                                                        | \[PositionEffect] |
| Closing Yield                                                                                      | The yield of a bond based on the closing price.                                                                                                                                                                                                                                                                                                                                                                                                                                          | \[YieldType]      |
| Closing Yield Most Recent Month                                                                    | The yield of a bond based on the closing price as of the most recent month's end.                                                                                                                                                                                                                                                                                                                                                                                                        | \[YieldType]      |
| Closing Yield Most Recent Quarter                                                                  | The yield of a bond based on the closing price as of the most recent quarter’s end.                                                                                                                                                                                                                                                                                                                                                                                                      | \[YieldType]      |
| Closing Yield Most Recent Year                                                                     | The yield of a bond based on the closing price as of the most recent year’s end.                                                                                                                                                                                                                                                                                                                                                                                                         | \[YieldType]      |
| CMTM                                                                                               | Collateralized mark-to-market (CMTM) is determined by marking from coupon rate to settlement price. The resulting amount is collateralized meaning the holder of the position must post acceptable collateral to cover the obligation.                                                                                                                                                                                                                                                   | \[PosAmtTyp]      |
| COLAT                                                                                              | Total Collateralized Amount (COLAT) represents the summation of all collateralized amounts (ACPN+CMTM).                                                                                                                                                                                                                                                                                                                                                                                  | \[PosAmtTyp]      |
| Compound Yield                                                                                     | The yield of certain Japanese bonds based on its price. Certain Japanese bonds have irregular first or last coupons, and the yield is calculated compound for these irregular periods.                                                                                                                                                                                                                                                                                                   | \[YieldType]      |
| Confirmed Trade Report                                                                             | A trade that is completed (both sides are reported) and matched.                                                                                                                                                                                                                                                                                                                                                                                                                         | \[MatchType]      |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited Page 132 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

| Term                                | Definition                                                                                                                                                                                                                                           | Field where used        |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| Contra Firm                         | The broker or other firm which is the contra side of the trade.                                                                                                                                                                                      | \[PartyRole]            |
| Contra Clearing Firm                | Clearing firm of the broker or other firm which is the contra side of the trade.                                                                                                                                                                     | \[PartyRole]            |
| Contra Trader                       | Individual usually identified by a trading badge number or initials that takes the opposite side of a trade.                                                                                                                                         | \[PartyRole]            |
| Contract For Difference (CFD)       | A single stock total return swap, combining financing and synthetic equity exposure in one transaction.                                                                                                                                              | \[BookingType]          |
| Contract Weighted Average Price     | Relevant for multileg orders (Energy Specific). The price of the strategy is given as an average price of all legs in the multileg, including adjustment for differences in contract sizes between the legs.                                         | \[MultilegPrice Method] |
| Correspondent Broker                | Identifies a correspondent broker.                                                                                                                                                                                                                   | \[PartyRole]            |
| Correspondent Clearing Firm         | Clearing Firm that is going to carry the position on their books at another clearing house (exchanges).                                                                                                                                              | \[PartyRole]            |
| Correspondent Clearing Organization | Identifies a correspondent clearing organization.                                                                                                                                                                                                    | \[PartyRole]            |
| Counter-Order Selection             | A model where the user selects which order to hit by providing the order ID in the order being submitted against it (also known as Hit/Take orders). Specifies that the source matching mechanism for a trade is a Hit / Take.                       | \[OrdType]              |
| Coupon Rate                         | The rate of interest that, when multiplied by the principal, par value, or face value of a bond, provides the currency amount of the periodic interest payment. The coupon is always cited, along with maturity, in any quotation of a bond's price. |                         |
| CPN                                 | Coupon (CPN) is the payment as determined by coupon rate paid on coupon date.                                                                                                                                                                        | \[PosAmtTyp]            |
|                                     | \[from EP83]                                                                                                                                                                                                                                         |                         |
| CRES                                | Cash Residual Amount (CRES) is used to represent a residual amount associated with migrated trades and succession events.                                                                                                                            | \[PosAmtTyp]            |
|                                     | \[from EP83]                                                                                                                                                                                                                                         |                         |
| Cross                               | Client sends Broker a buy or sell order. Broker wishes to take the other side and cross with the client. Broker sends an order with Side=Cross to an exchange.                                                                                       | \[OrdType]              |
| Crossed                             | Generally indicates that the current offer price is better than the bid one - a situation where a trade normally should have occurred. The rules for crossed book situations are bilaterally agreed between counterparties.                          | \[QuoteCondition]       |
|                                     | \[See Vol. 3 Market Data for usage notes]                                                                                                                                                                                                            |                         |
| Cross Auction                       | Crossing session, for example to open or close a market. Rules.                                                                                                                                                                                      | \[MatchType]            |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 133 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Term

# Definition

# Field where used

governing this type of session usually differ from those effective during continuous trading. For example, price determination could be subject to the principle of most executable volume. Visibility of the order book is typically restricted (e.g. top-of-book only, indicative crossing price) compared to continuous trading. The session itself might be separated into different phases governed by different rules, all of which are defined by the marketplace offering this type of matching mechanism.

# Cross Short

Client wants to establish a short position, and so sends a Sell Short to Broker. Broker wants to cross with the Client, so Broker sends a Cross Short order to an exchange. Cross Short is crucial here because many exchanges have tick rules needing to be enforced, and the order getting converted from Sell Short to Cross (instead of Cross Short) could result in an illegal short sell.

# Cross Short Exempt

Client wants to establish a short position, and is exempt from the uptick restriction. So Client sends Sell Short Exempt to Broker. Broker wants to cross with the Client, so Broker needs a way to send "Cross Short Exempt" to the exchange so that an audit trail traces back indicating that the party selling short was exempt from the uptick rule.

# Currency swap

"An agreement to exchange future cash flows. There are two fundamental types: the cross-currency swap and the interest rate (single currency) swap."

Source: UA Foreign Exchange Primer by Shani Shamah

# Current Yield

Annual interest on a bond divided by the market value. The actual income rate of return as opposed to the coupon rate expressed as a percentage.

# Customer Account

Identifies the customer account associated with the message.

# Dated Date

The effective date of a new securities issue determined by its underwriters. Often but not always the same as the "Issue Date" and the "Interest Accrual Date".

# Day Order

A buy or sell order that, if not executed expires at the end of the trading day on which it was entered.

# Default Position Effect

An instruction to use the default position keeping rules assigned to the account. For Options and Futures the default is normally "net" position, while for Forwards the default is usually "open" position.

# Dealt currency

In a foreign exchange transaction, 'dealt currency' indicates which currency was originally specified. For example, An investment manager may 'buy 100M USD against EUR'. This has USD as the 'dealt currency' and EUR as the 'counter currency'. Note that when viewed from the sell-side's (or broker's) perspective, this is a 'Sell 100M USD against EUR' the 'dealt currency' remains the same.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 134 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

| Term                   | Definition                                                                                                                                                                                                                                                                                                                                                   | Field where used              |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------- |
| Derivative Transaction | Related Exercise or expiration of options, forwards or futures contracts that imply an exchange of securities or a trade that relates to a derivatives trade and that forms an unconditional part of a combination together with a derivative trade.                                                                                                         | \[TrdType]                    |
| Discount               | When a bond sells below its par value, it is said to be selling at a discount. A price with a PriceType of "discount" is the difference between 100 and the bond's percent-of-par price.                                                                                                                                                                     | \[PriceType]                  |
| DLV                    | 1) Delivery Amount (DLV) represents the amount paid or collected in association with a credit event. \[from EP83] 2) Delivery quantity (DLV) is used on the target CDS position to represent the delivery of the underlying bonds. \[from EP83]                                                                                                              | 1) \[PosAmtTyp] 2) \[PosType] |
| Do Not Increase (DNI)  | A limit order to buy, a stop order to sell, or a stop-limit order to sell which is not to be increased in shares on the ex-dividend date as a result of a stock dividend or distribution.                                                                                                                                                                    | \[ExecInst]                   |
| Do Not Reduce (DNR)    | A limit order to buy, a stop order to sell, or a stop-limit order to sell that is not to be reduced in price by the amount of an ordinary cash dividend on the ex-dividend date. A do-not-reduce order applies only to ordinary cash dividends; it should be reduced for other distributions - such as when a stock goes “ex” stock dividend or “ex” rights. | \[ExecInst]                   |
| Dollar Price           | See "Percent of Par"                                                                                                                                                                                                                                                                                                                                         | \[PriceType]                  |
| Double Barrier Option  | An option that has a combination of activation/inactivation conditions. Specifies multiple price levels at which the option becomes active (knock-in) or inactive (knock-out). \[from EP92]                                                                                                                                                                  |                               |
| Down and In Option     | the option becomes active if the underlying price drops below the specified barrier. \[from EP92]                                                                                                                                                                                                                                                            |                               |
| Down and Out Option    | The option becomes inactive if the underlying price drops below the specified barrier. \[from EP92]                                                                                                                                                                                                                                                          |                               |
| Entering Firm          | Broker who has recorded or reported an execution. This field is particularly useful where the trade is entered into a trade recording system by a broker who is not a party to the trade, as it allows any inquiries or problem resolution to be directed to the appropriate source.                                                                         | \[PartyRole]                  |
| Entering Trader        | Individual usually identified by a trading badge number or initials that actually enters an order to a market (especially in open outcry markets). Usually the Entering Trader is the same as the Executing Trader. However, under some circumstances the Entering Trader will have the trade executed by another trader who is then                         | \[PartyRole]                  |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 135 of 158
---
Version 5.0 Service Pack 2 - Errata        VOLUME 1                                                    August 18, 2011

# Term Definitions

| Term                        | Definition                                                                                                                                                                                                                                                                                                                                                              | Field where used   |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------ |
| Entering Unit               | Individual unit within an Entering Firm, e.g. trading desk, branch office or similar, that actually enters an order to a market. Usually the Entering Unit is the same as the Executing Unit that is responsible for the transaction. The Executing Unit can delegate the entry but not the responsibility to another unit who is then identified as the Entering Unit. | \[PartyRole]       |
| European style option       | An option that can only be exercised at the end of its life or for a short, specified period of time just prior to expiration (usually on a single day).                                                                                                                                                                                                                |                    |
| Even swap                   | An FX Swap where the given amount to be bought and sold is the same on the near and far legs. See "uneven swap".                                                                                                                                                                                                                                                        |                    |
| Exchange                    | Exchange associated with the position.                                                                                                                                                                                                                                                                                                                                  | \[PartyRole]       |
| Exchange for Physical       | EFP trades involve a futures contract and a cash position. The parties involved agree privately upon a price for a simultaneous exchange or transfer "cash for futures" and then report the terms of this agreement to the clearing house.                                                                                                                              | \[TradeCondition]  |
| Exchange for Risk           | EFR trades involve a futures contract and a spot commodity. The parties involved agree privately upon a price for a simultaneous exchange or transfer of "spot for futures" and then report the terms of this agreement to the clearing house.                                                                                                                          | \[TradeCondition]  |
| Exchange granted trade      | A trade done according to an individual or general authorisation, obtained prior to the consummation of the trade, from the marketplace for the specific case and/or kind of trade.                                                                                                                                                                                     | \[TrdType]         |
| Execute as delta neutral    | Indicates that the quantity of a strategy leg is to be (or was) calculated as delta neutral using provided volatility parameters.                                                                                                                                                                                                                                       | \[ExecInst]        |
| Execute as duration neutral | Indicates that the quantity of a strategy leg is to be (or was) calculated as duration neutral.                                                                                                                                                                                                                                                                         | \[ExecInst]        |
| Execute as FX neutral       | Indicates that the quantity of a strategy FX leg is to be (or was) calculated as neutral with regards to the security traded for the other legs of the strategy.                                                                                                                                                                                                        | \[ExecInst]        |
| Execution Time              | According to US futures markets (CFTC): Non-qualified reporting time of order execution.                                                                                                                                                                                                                                                                                | \[TrdRegTimestamp] |
| Executing Firm              | Identifies executing / give-up broker.                                                                                                                                                                                                                                                                                                                                  | \[PartyRole]       |
| Executing System            | System Identifier where execution took place (some markets have multiple execution locations such as an electronic book or automatic execution system).                                                                                                                                                                                                                 | \[PartyRole]       |
| Executing Trader            | Trader or broker id associated with Executing Firm who actually identified as the Executing Trader.                                                                                                                                                                                                                                                                     | \[PartyRole]       |

Source: www.investopedia.com and www.investorwords.com

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                             Page 136 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

| Term                         | Definition                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Field where used |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| Executing Unit               | Identifies executing unit within an Executing Firm, e.g. trading desk, branch office or similar.                                                                                                                                                                                                                                                                                                                                                                                    | \[PartyRole]     |
| Exhaust                      | A reserve order refresh method where the displayed quantity is not refreshed until exhausted (filled).                                                                                                                                                                                                                                                                                                                                                                              | \[DisplayWhen]   |
| External Routing Allowed     | Indicates that an order sent to one market may be routed by that market to other external markets, especially in cases where the order locks or crosses the market and it can be executed against another market’s superior price. Note: The absence of this instruction does not imply that an order should not be routed externally; rather, the order receiver’s default will apply.                                                                                             | \[ExecInst]      |
| External Routing Not Allowed | Indicates that an order sent to one market may never be routed by that market to other external markets. Should the order lock or cross the market but be unable to execute due to price protection reasons, a market may have to take alternate action, which might include rejecting the order, depending on the market’s rules. Note: The absence of this instruction does not imply that an order should be routed externally; rather, the order receiver’s default will apply. | \[ExecInst]      |
| Fill or Kill                 | A market or limit-price order that is to be executed in its entirety as soon as it is represented in the Trading Crowd; if not so executed, the order is to be canceled. Not to be confused with Immediate or Cancel.                                                                                                                                                                                                                                                               | \[TimeInForce]   |
| Final Inventory Due Date     | Specifies the last date on which purchase dates for a contract must be provided to the service provider.                                                                                                                                                                                                                                                                                                                                                                            | \[EventType]     |
| First Delivery Date          | Specifies the first delivery date of the delivery period for a physically delivered instrument.                                                                                                                                                                                                                                                                                                                                                                                     | \[EventType]     |
| First Intent Date            | Specifies the first date of the delivery period on which intents may be submitted for a physically delivered instrument.                                                                                                                                                                                                                                                                                                                                                            | \[EventType]     |
| FIX Connection               | A FIX Connection is comprised of three parts: logon, message exchange, and logout.                                                                                                                                                                                                                                                                                                                                                                                                  |                  |
| FIX Session                  | A FIX Session is comprised of one or more FIX Connections, meaning that a FIX Session spans multiple logins.                                                                                                                                                                                                                                                                                                                                                                        |                  |
| Fixed Payout Option          | The payout amount is specified at inception. Associated with Binary options (Yes, it pays or No, it doesn’t pay).                                                                                                                                                                                                                                                                                                                                                                   | \[from EP92]     |
| Fixed Price Cabinet Trade    | Cabinet Trade executed at a price equal to the minimum tick size (or smallest possible price). See "Cabinet Trade".                                                                                                                                                                                                                                                                                                                                                                 | \[PriceType]     |
| Fixed Tick Rule              | A fixed cabinet trade price set to a minimum tick amount.                                                                                                                                                                                                                                                                                                                                                                                                                           | \[TickRuleType]  |
| Fixing Date                  | The date on which the rate is used for which the settlement amount is calculated. Every NDF has a fixing date. The fixing date is the day the comparison between the NDF rate and the prevailing spot rate is made. Depending on the currencies dealt, there are variations. For some currencies the fixing date is one good business day before the settlement date and for other currencies the                                                                                   |                  |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 137 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

| Term                   | Definition                                                                                                                                                                                                                                           | Field where used   |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------ |
| Fixing Date            | The fixing date is two good business days before the settlement date.                                                                                                                                                                                | \[from EP82]       |
| Fixing Price           | Imputed price based on VWAP/TWAP algorithm. Used especially to price FX futures.                                                                                                                                                                     | \[MDEntryType]     |
| Fixing Time            | The time on the fixing date on which the rate is used for which the settlement amount is calculated for NDFs. See "Fixing Date". Fixing time for NDFs typically follows market conventions and is set by the central bank of the currency's country. | \[from EP82]       |
| Flat Default Curve     | Price calculation method that uses a flat credit curve to establish future probability of defaults for pricing a specific CDS contract.                                                                                                              | \[Quote Condition] |
| Flexible Instrument    | An exchange-listed instrument for which a set of pre-defined attributes such as strike price, expiration date and underlying instrument may be provided by the user at the time of creation (creating a new instrument). Can be an option or future. |                    |
| Floating Price Trade   | Cabinet Trade executed at a price that can be different than the minimal price. See "Cabinet Trade".                                                                                                                                                 | \[PriceType]       |
| Floored Put Option     | The option has a linear payout (like a vanilla) up to a capped amount as specified by a floor price.                                                                                                                                                 | \[from EP92]       |
| Forex - Swap           | A "Swap" order for Foreign Exchange (currency trading).                                                                                                                                                                                              | \[OrdType]         |
| Foreign exchange swap  | The transaction of exchanging two currencies at an agreed upon rate at an agreed upon time. The transaction is reversed at a future rate and time, with no payment streams between the points in time.                                               |                    |
|                        | Source: A paraphrase of definition from http\://joxo.co.uk/SummaryGuideToFXForFinancialandITProfessionals.html                                                                                                                                       |                    |
| Full Default Curve     | Price calculation method that uses a complete credit curve to establish future probability of defaults for pricing a specific CDS contract.                                                                                                          | \[Quote Condition] |
| Funari                 | Japanese term for an order to buy or sell a stated amount of a security at the specified limit price with any unexecuted (leftover) quantity becoming a Market On Close order.                                                                       | \[OrdType]         |
| Fund manager Client ID | For CIV: An identifier for an Investor or a broker or funds supermarket’s nominee/custodian company which is recognized by the Fund manager.                                                                                                         | \[PartyRole]       |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 138 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Term

# Definition

# Field where used

| Giveup Clearing Firm         | Firm to which the trade is given up (carries the position that results from a trade).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | \[PartyRole]   |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| Good Till Canceled           | An order to buy or sell that remains in effect until it is either executed or canceled; sometimes called an “open order”.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | \[TimeInForce] |
| Good Till Crossing (GTX)     | An order to buy or sell that is canceled prior to the market entering into an auction, or crossing phase. Typically, markets that support continuous trading will have an auction phase at the beginning and sometimes also at the end of trading to match up orders that have been entered into the exchange's order book during the pre- or post-trading phase (i.e. where no continuous trading was available). A GTX order automatically expires immediately prior to the commencement of a crossing session, i.e. the party originating the order wants to make sure it gets filled during the current continuous auction, and any remaining open quantity should be discarded at the end of the current continuous auction period. | \[TimeInForce] |
| Good Through Crossing        | An order that is valid up till and including a crossing phase. Also see Good Till Crossing (GTX), At Crossing, At the Opening and At the Close.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | \[TimeInForce] |
| Government Equivalent Yield  | Ask yield based on semi-annual coupons compounding in all periods and actual/actual calendar.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | \[YieldType]   |
| Held                         | The firm executing the order is held to best execution requirements, and may not make discretionary decisions. Opposite of Not Held.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | \[ExecInst]    |
| IACPN                        | Incremental Accrued Coupon (IACPN) represents the incremental accrued coupon which is banked each day.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | \[PosAmtTyp]   |
| ICMTM                        | Incremental Accrued Coupon (IACPN) represents the incremental accrued coupon which is banked each day.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | \[PosAmtTyp]   |
| ICPN                         | Initial coupon (ICPN) is an amount paid to the buyer as a pro-rated portion of the coupon from the prior coupon through trade date. The buyer will be responsible for the full coupon on the next coupon date.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | \[PosAmtTyp]   |
| Ignore Price Validity Checks | Disables validity checking of price fields for an order or change request.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | \[ExecInst]    |
| Imbalance Only               | Indicates than order is an “Imbalance” order. Exchanges often use this type of (often unpriced) order to allow certain trading participants to remove imbalances in call auctions.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | \[ExecInst]    |
| Immediate                    | A reserve order refresh method where the displayed quantity stays the same until the remaining executable quantity of the order goes below.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | \[DisplayWhen] |
| Immediate or Cancel          | A market or limit-price order that is to be executed in whole or in.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | \[TimeInForce] |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 139 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Term

# Definition

# Field where used

| part as soon as it is represented in the Trading Crowd; any portion not so executed is to be canceled. Not to be confused with Fill or Kill. |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |                                     |
| -------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| Individual Prices                                                                                                                            | The price of a multileg order strategy is given using individual prices for the legs.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | \[MultilegPrice Method]             |
| Initial                                                                                                                                      | A reserve order instruction to refresh using the initially displayed quantity.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | \[DisplayMethod]                    |
| Initial Inventory Due Date                                                                                                                   | Specifies the first date on which purchase dates for a contract must be provided to the service provider.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | \[EventType]                        |
| Initiator                                                                                                                                    | An “initiator” may be one of the following:                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Quoting and other messages Volume 7 |
|                                                                                                                                              | an institutional client                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |                                     |
|                                                                                                                                              | a financial planner                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |                                     |
|                                                                                                                                              | a retail broker representing a retail customer                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |                                     |
|                                                                                                                                              | a broker/dealer                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |                                     |
|                                                                                                                                              | an inter-dealer broker (or broker’s broker)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |                                     |
|                                                                                                                                              | an issuer                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |                                     |
| Institutions Only                                                                                                                            | Broker is restricted to dealing with other buy side firms.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | \[ExecInst]                         |
| Instrument Denominator                                                                                                                       | Specifies the price denominator of a fractionally quoted instrument such as treasure note futures or corn futures. Values are whole numbers. Examples are Quarters (4), Eights (8), Sixteenths (16), Thirty-seconds (32), etc.                                                                                                                                                                                                                                                                                                                                                                                                                 | \[InstrAttribType]                  |
| Instrument Numerator                                                                                                                         | Specifies the denominator for the fractional portion of a numerator for a fractionally quoted price. For example, a price 5 3¼ / 32 has an Instrument Numerator of 4. Values are whole numbers.                                                                                                                                                                                                                                                                                                                                                                                                                                                | \[InstrAttribType]                  |
| Instrument Price Precision                                                                                                                   | Specifies the number of decimal places that are provided in order to correctly format a price. Generally, used for the formatting of fractional prices. Values are whole numbers.                                                                                                                                                                                                                                                                                                                                                                                                                                                              | \[InstrAttribType]                  |
| Instrument Strike Price Precision                                                                                                            | Specifies the number of decimal places that are provided in order to correctly format a strike price. Generally, used for the formatting of fractional prices. Values are whole numbers.                                                                                                                                                                                                                                                                                                                                                                                                                                                       | \[InstrAttribType]                  |
| Interest Accrual Date                                                                                                                        | The start date used for calculating accrued interest on debt instruments which are being sold between interest payment dates. Often but not always the same as the "Issue Date" and the "Dated Date".                                                                                                                                                                                                                                                                                                                                                                                                                                          |                                     |
| Intermarket sweep                                                                                                                            | An order that is an intermarket sweep as defined by the SEC in Regulation NMS. This value is used on Immediate or Cancel limit orders (or other order type and time in force). It indicates that the party sending the order has taken responsibility for price protection, and the recipient of the order should execute it, if possible, without regard to protection of other markets’ prices. While the term “Intermarket sweep” is specific to the United States, the ExecInst value that represents it may be used in other markets, where appropriate, to indicate an order that should be executed without regard to price protection. | \[ExecInst]                         |
| Internalized Trade                                                                                                                           | A trade done internally at a marketplace member. Often restricted to the inside of the current spread. In cases where the trade must be offered to the market for price improvement it is often called Cross Trade.                                                                                                                                                                                                                                                                                                                                                                                                                            | \[TradeCondition]                   |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 140 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Term                       | Definition                                                                                                                                                                                                                                                                                                                                                                                                      | Field where used     |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| Introducing Firm           | The broker or other intermediary with the closest association with the investor.                                                                                                                                                                                                                                                                                                                                | \[PartyRole]         |
| Inverse Floater Bond Yield | Inverse floater semi-annual bond equivalent rate.                                                                                                                                                                                                                                                                                                                                                               | \[YieldType]         |
| Investor ID                | For Equities: Identifies beneficiary or broker acting on behalf of beneficiary. This field is mandatory for various exchanges either pre or post trade. Numerical entry containing no dashes. For CIV: An Investor identifier such as a taxpayer reference (NINO, NPN, DSS, SSN number etc) for an individual investor or a registration number (EIN, etc.) for a company. May contain alphanumeric and dashes. | \[PartyRole]         |
| Issue Date                 | The date on which a bond or stock offering is issued. It may or may not be the same as the effective date ("Dated Date") or the date on which interest begins to accrue ("Interest Accrual Date").                                                                                                                                                                                                              |                      |
| Issue Price Stabilization  | This indication must, according to certain regulation, be used when a broker is contracted by the issuer to stabilize the price before an issue of new stock of a security that is already traded (or a buy-back/buy-out).                                                                                                                                                                                      | \[OrderRestrictions] |
| Issuer Holding             | Certain regulation requires this indicator to be attached when an issuer is buying back its own stock (or units of other instruments). It can also be used in cases when a new issue is auctioned out over the exchange.                                                                                                                                                                                        | \[OrderRestrictions] |
| Issuing/Buy-back Auction   | A call auction with the purpose of issuing new or buying back stock, bonds or other security.                                                                                                                                                                                                                                                                                                                   | \[MatchType]         |
| Last Delivery Date         | Specifies the last delivery date of the delivery period for a physically delivered instrument.                                                                                                                                                                                                                                                                                                                  | \[EventType]         |
| Last Intent Date           | Specifies the last date of the delivery period on which intents may be submitted for a physically delivered instrument.                                                                                                                                                                                                                                                                                         | \[EventType]         |
| Last Peg                   | A pegged order specifying that the order should be priced relative to the last sale price.                                                                                                                                                                                                                                                                                                                      | \[PegPriceType]      |
| Limit                      | An order to buy a security at or below a stated price, or to sell a security at or above a stated price.                                                                                                                                                                                                                                                                                                        | \[OrdType]           |
| Limit or Better            | Indicates an order to buy a security at the indicated limit price or lower, or to sell a security at the indicated limit price or higher.                                                                                                                                                                                                                                                                       | \[OrdType]           |
| Limit With or Without      | An order to be executed at a limit price, with or without round-lot sales; valid only for odd lot orders.                                                                                                                                                                                                                                                                                                       | \[OrdType]           |
| Liquidity Provider         | Identifies the individual that provided liquidity, e.g. was the market maker (specialist) involved in a trade. Used to identify the liquidity.                                                                                                                                                                                                                                                                  | \[PartyRole]         |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 141 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Term

# Definition

# Field where used

| provider                        | involved in a block of EFP trade for listed futures markets.                                                                                                                                                                           |                   |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------- |
| Locate/Lending Firm             | Identity of the firm which is loaning the security in a short sale.                                                                                                                                                                    | \[PartyRole]      |
| Locked                          | Generally indicates that the current bid and offer prices are the same, a situation where a trade normally should have occurred. The rules for locked book situations are bilaterally agreed between counterparties. Also see Crossed. | \[QuoteCondition] |
| Look-back Option                | The underlying price is set to the optimal value of the daily settlement prices over a specified period.                                                                                                                               | \[from EP92]      |
| Marked To Market Yield          | An adjustment in the valuation of a securities portfolio to reflect the current market values of the respective securities in the portfolio.                                                                                           | \[YieldType]      |
| Market                          | Indicates an order to buy or sell a stated amount of a security at the most advantageous price obtainable after the order is represented in the Trading Crowd.                                                                         | \[OrdType]        |
| Market If Touched               | Indicates an order to buy or sell a stated amount of a security or commodity as soon as a preset market price is reached, at which point it becomes a Market order.                                                                    | \[OrdType]        |
| Market On Close                 | Indicated price is held to the closing price ("firm" instruction).                                                                                                                                                                     | \[IOIQualifier]   |
| Market operations Entered Trade | Identifies an OTC (privately negotiated) trade is entered by a marketplace official on behalf of the reporting party.                                                                                                                  | \[TradeCondition] |
| Market Or Better                | Indicates an order to buy or sell a stated amount of a security at the quoted market or better.                                                                                                                                        | \[OrdType]        |
| Market Peg                      | A pegged order specifying that the order should be priced relative to the offer price if buying or bid price if selling.                                                                                                               | \[PegPriceType]   |
| Market Segment                  | A subdivision of a market or marketplace that is setup and operates within that market. A market segment may be setup for a variety of reasons including (examples):                                                                   |                   |
|                                 | - regulatory - e.g. separate derivatives trading from stock trading                                                                                                                                                                    |                   |
|                                 | - membership - e.g. requiring separate membership or trading authorization                                                                                                                                                             |                   |
|                                 | - geographical - e.g. separate segments for various countries                                                                                                                                                                          |                   |
|                                 | - separate venues - e.g. floor/pit trading vs. electronic                                                                                                                                                                              |                   |
|                                 | - trading rules - e.g. different rules and behaviours that a market needs to support for the entities it offers to trade                                                                                                               |                   |
| Market with Leftover as Limit   | Indicates an order to buy or sell a stated amount of a security at the prevailing market price with any unexecuted (leftover) quantity becoming a Limit order at the last executed price.                                              | \[OrdType]        |
| Median Price                    | One type of average, found by arranging the values in order and then selecting the one in the middle. If the total number of values in the sample is even, then the median is the mean of the two                                      | \[QuoteCondition] |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 142 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

| Term                                     | Definition                                                                                                                                                                                                                                                                                                                                                                                                      | Field where used       |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| Mid Price                                | The average of a set of prices (not taking quantity into account). Also see “Trading Session VWAP Price”                                                                                                                                                                                                                                                                                                        | \[QuoteCondition]      |
| Mid-Price Peg (Midprice of inside quote) | A pegged order specifying that the order should be priced relative to the mid-price of the inside quotes.                                                                                                                                                                                                                                                                                                       | \[PegPriceType]        |
| Most Recent Closing Yield                | The last available yield stored in history, computed using price.                                                                                                                                                                                                                                                                                                                                               | \[YieldType]           |
| Multi asset class Multileg Trade         | Identifies that the trade occurred due to an equity / derivatives (or other multi asset class) combination, either in the legs or in a multileg order book.                                                                                                                                                                                                                                                     | \[TradeCondition]      |
| Multileg-to-Multileg Trade               | Identifies that a multileg order has executed against another multileg order in a multileg order book (i.e. the legs were not involved).                                                                                                                                                                                                                                                                        | \[TradeCondition]      |
| Net Price                                | Relevant for multileg orders. The price is given as the sum of the Price \* Ratio for all legs. If buying the strategy, the price of a bought leg (which is a buy-leg in the multileg definition) is added, and the price of a sold leg is subtracted. If selling the strategy, the price of a bought leg (which is a sell-leg in the multileg definition) is subtracted, and the price of a sold leg is added. | \[MultilegPriceMethod] |
| New                                      | A reserve order instruction to refresh using a quantity defined separately from the initial display quantity.                                                                                                                                                                                                                                                                                                   | \[DisplayMethod]       |
| Next Fund Valuation Point                | For CIV orders - indicates that the Investor wishes the order to be dealt at the unit price determined at the next Valuation Point, a.k.a. a Forward price.                                                                                                                                                                                                                                                     | \[OrdType]             |
| Next Roll Date                           | Next roll date for a swap                                                                                                                                                                                                                                                                                                                                                                                       | \[EventType]           |
| No Cross                                 | The broker executing this trade is forbidden from taking the other side of the trade. Opposite of OK to Cross.                                                                                                                                                                                                                                                                                                  | \[ExecInst]            |
| Non-standard settlement                  | A Trade that deviates from the standard settlement and delivery period.                                                                                                                                                                                                                                                                                                                                         | \[TrdType]             |
| Not Held                                 | The firm executing the order is not held to best execution requirements, and may be able to make some discretionary decisions. Opposite of Held.                                                                                                                                                                                                                                                                | \[ExecInst]            |
| Odd Lot                                  | An amount of a security that is less than the normal unit of trading for that particular security. For stocks, any transaction less than 100 shares is usually considered to be an odd lot. (Source: Investopedia)                                                                                                                                                                                              | \[LotType]             |
| Off Hours Trade                          | A trade that occurs outside normal trading hours. Used to qualify privately negotiated trades (subject to regulations)                                                                                                                                                                                                                                                                                          | \[TrdSubType]          |
| OK to Cross                              | The broker executing this trade is allowed to take the other side of the trade.                                                                                                                                                                                                                                                                                                                                 | \[ExecInst]            |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 143 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Term                           | Definition                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Field where used       |
| ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| Omnibus Account                | An account where the positions for multiple entities (usually customers) are maintained. The omnibus accounting is usually done on a gross basis where long and short positions are not netted together.                                                                                                                                                                                                                                                                                                                                                                                                                               | \[PartyRole]           |
| On Basis                       | An order to buy or sell at the basis price. The basis price is established by joint agreement of odd lot dealers in 100 share unit stocks when no round lot sale has occurred during the trading session, the spread between the closing bid and offer is two points or more, and an odd lot dealer has been given a basis price order.                                                                                                                                                                                                                                                                                                | \[OrdType]             |
| On Hours Trade                 | A trade that occurs during normal trading hours. Used to qualify privately negotiated trades (subject to regulations)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | \[TrdSubType]          |
| One Cancels the Other (OCO)    | An OCO order is an order whose execution results in the immediate cancellation of another order linked to it. Cancellation of the Contingent Order happens on a best efforts basis. In an OCO order, both orders are live in the marketplace at the same time. The execution of either order triggers an attempt to cancel the unexecuted order. Partial executions will also trigger an attempt to cancel the other order.                                                                                                                                                                                                            | \[ListExecInst]        |
| One-Party Report for Matching  | A trade report for one side of a privately negotiated trade. When both sides are reported the receiver matches and confirms the trade.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | \[TradeHandling Instr] |
| One-Party Report for Pass Thru | A trade report for one side of a privately negotiated trade. The receiver forwards a request to the counterparty who declines or accepts the trade.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | \[TradeHandling Instr] |
| One Triggers the Other (OTO)   | An OTO orders involves two orders—a primary order and a secondary order. The primary order is a live marketplace order. The secondary order, held in a separate order file, is not. If the primary order executes in full, the secondary order is released to the marketplace and becomes live. An OTO order can be made up of stock orders, option orders, or a combination of both.                                                                                                                                                                                                                                                  | \[ListExecInst]        |
| One Touch Option               | One Touch further defines the terms, usually specific dates and times, for when the option will pay out in the context of a trigger price.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | \[from EP92]           |
| One Updates the Other (OUO)    | An OUO order is an order whose execution results in the immediate reduction of quantity in another order linked to it. The quantity reduction happens on a best effort basis. In an OUO order both orders are live in the marketplace at the same time. The execution of either order triggers an attempt to reduce the remaining quantity of the other order, partial executions included. There are two sub-types of OUO orders: 1. In the Proportional Quantity Reduction model, the quantity of the unfilled order is reduced proportionally to size of the order that got filled. Example: Order A is for 100; Order B is for 50. |                        |

© Copyright, 2008-2009 2011, FIX Protocol, Limited Page 144 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Term

# Definition

# Field where used

| When order B is partially filled for 25 (50 %), order A is restated to a quantity of 50 (50 %).                                                   |                                                                                                                                                                                                                                                                                                                                                                                      |                          |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------ |
| In the Absolute Quantity Reduction model, the quantity of the unfilled order is reduced with the fill size of the order that got filled. Example: |                                                                                                                                                                                                                                                                                                                                                                                      |                          |
| Order A is for 100; Order B is for 50.                                                                                                            |                                                                                                                                                                                                                                                                                                                                                                                      |                          |
| When order B is partially filled for 25, order A is restated to a quantity of 75.                                                                 |                                                                                                                                                                                                                                                                                                                                                                                      |                          |
| Open                                                                                                                                              | An instruction to position keeping that the trade should open a new position.                                                                                                                                                                                                                                                                                                        | \[PositionEffect]        |
| Opening Peg                                                                                                                                       | A pegged order specifying that the order should be priced relative to the opening price.                                                                                                                                                                                                                                                                                             | \[PegPriceType]          |
| Opposite                                                                                                                                          | Sides of the legs are the opposite of their definition in the multileg instrument.                                                                                                                                                                                                                                                                                                   | \[Side]                  |
| Order imbalance, auction is extended                                                                                                              | Used when a call auction (e.g. opening or closing a market) is extended due to an imbalance at the auction clearing price.                                                                                                                                                                                                                                                           | \[SecurityTradingEvent]  |
| Order Origination Firm                                                                                                                            | Buyside firm associated with Order Origination Firm which originates/submits the order.                                                                                                                                                                                                                                                                                              | \[PartyRole]             |
| Order Origination Trader                                                                                                                          | Buyside trader id associated with Order Origination Firm which originates/submits the order.                                                                                                                                                                                                                                                                                         | \[PartyRole]             |
| OTC                                                                                                                                               | Used to report into an exchange trades or prices that were agreed directly, outside of the exchange rules, between the counterparties ("over the counter").                                                                                                                                                                                                                          | \[TrdType]               |
| OTC Privately Negotiated                                                                                                                          | A privately negotiated trade in a non-regulated product.                                                                                                                                                                                                                                                                                                                             | \[TradeCondition]        |
| OTC Quote                                                                                                                                         | A OTC trade that occurs as the result of a Quote Negotiation. Used to qualify privately negotiated trades (subject to regulations).                                                                                                                                                                                                                                                  | \[TrdSubType]            |
| Outside Spread                                                                                                                                    | A trade done outside the current spread of the book. Often allowed due to special considerations only.                                                                                                                                                                                                                                                                               | \[TradeCondition]        |
| Par                                                                                                                                               | Equal to the face value (nominal) of a security, e.g. A bond selling at par is worth an equivalent to its original issue value, typically $1000/bond.                                                                                                                                                                                                                                | \[QuantityType]          |
| Participate Don’t Initiate                                                                                                                        | An order that may participate in a transaction initiated by another party, but may not initiate a transaction. For example, on US ECNs / Exchanges, this may represent an order that will be quoted to the marketplace and will trade if another party initiates a trade (i.e. hits the posted quote), but cannot be routed to initiate a trade with another market or market maker. | \[ExecInst]              |
| Peg to VWAP                                                                                                                                       | A pegged order specifying that the order should be priced relative to the VWAP price.                                                                                                                                                                                                                                                                                                | \[PegPriceType]          |
| Peg Refresh                                                                                                                                       | An execution restatement reason indicating that the order is restated only because the pegged price was refreshed.                                                                                                                                                                                                                                                                   | \[ExecRestatementReason] |

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited Page 145 of 158


---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Term                          | Definition                                                                                                                                                                                                                                                                                                                                | Field where used          |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- |
| Pegged Order                  | An order that acts like a limit order except that the limit price is set relative to another price, such as the last sale price, midpoint price, opening price, bid, offer or VWAP (Volume Weighted Average Price).                                                                                                                       |                           |
| Per Unit                      | The currency price per unit, e.g. per equity share or per contract.                                                                                                                                                                                                                                                                       | \[PriceType]              |
| Percent of Par                | The ratio between the current price of a bond and its par value adjusted for amortization or indexing and expressed as a percent. For example if a EUR1,000 bond is trading at EUR1032.50 its price is expressed as 103.25 or 103¼. In the US this is usually referred to as the "dollar price" even in scholarly material and handbooks. | \[PriceType]              |
| Percent of Volume             | The sender does not want to be all of the volume on the floor.                                                                                                                                                                                                                                                                            | \[ExecInst]               |
| Portfolio Trade               | The reporting of the portfolio trade should be executed when the final leg of the transaction is completed. Used for trading an index vs. a cash basket. Certain market regulations require that participants report a portfolio trade when the final leg of the transaction is completed.                                                | \[TrdType]                |
| Position Account              | Account for positions resulting from derivatives trades. Each position account has a long and short quantity. Position quantities stored in the long and short quantity fields can be kept net or gross. Accounts that are kept gross are usually omnibus accounts.                                                                       | \[PartyRole]              |
| Position Removal Date         | Specifies the date the firm positions will be removed for a financially (cash settled) delivered instrument.                                                                                                                                                                                                                              | \[EventType]              |
| Post-Close                    | Trading status which denotes trading outside of normal hours for the security.                                                                                                                                                                                                                                                            | \[SecurityTrading Status] |
| Pre-listed only               | Instruments must be pre-listed.                                                                                                                                                                                                                                                                                                           | \[ListMethod]             |
| Predefined Multileg Security  | An order for a multileg security that is already defined at the marketplace or with the receiver of the order.                                                                                                                                                                                                                            | \[MultilegModel]          |
|                               | \[Note: see Vol. 4's Multileg Order section on predefined multileg security models.]                                                                                                                                                                                                                                                      |                           |
| Premium                       | When a bond sells above its par value, it is said to be selling at a premium. A price with a PriceType of "premium" is the difference between the bond's percent-of-par price and 100.                                                                                                                                                    | \[PriceType]              |
| Previous Fund Valuation Point | For CIV orders - indicates that the Investor prefers that the order be dealt at the unit price determined at the immediately preceding Valuation Point, a.k.a. a Historic price. (This can be overridden by the constitution of the fund or, in certain circumstances, by the Fund Manager.)                                              | \[OrdType]                |
| Price Band                    | Range (from-to or high-low) within which a valid price needs to be specified.                                                                                                                                                                                                                                                             |                           |
| Price Volatility Interruption | To ensure price continuity, continuous trading is interrupted by a volatility interruption whenever the potential execution price of an order lies outside the dynamic and/or static price range around a.                                                                                                                                | \[SecurityTradingEvent]   |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 146 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Term                        | Definition                                                                                                                                                                                                                                                                                                                                    | Field where used |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| reference price             | A volatility interruption triggers a change of trading form: e.g. continuous trading is interrupted and an auction is initiated.                                                                                                                                                                                                              |                  |
| Primary Peg                 | A pegged order specifying that the order should be priced relative to the bid price if buying or offer price if selling.                                                                                                                                                                                                                      | \[PegPriceType]  |
| Open Average Yield          | The average yield of the respective securities in the portfolio.                                                                                                                                                                                                                                                                              | \[YieldType]     |
| Order Originator            | ID of the party entering the trade into the system (data entry, userid, buy side trader, etc.).                                                                                                                                                                                                                                               | \[PartyRole]     |
| Put Date                    | The date on which the buyer of a security has the right but not the obligation to sell the security back to the issuer at a predetermined price.                                                                                                                                                                                              | \[EventType]     |
| Previous Close Yield        | The yield of a bond based on the closing price 1 day ago.                                                                                                                                                                                                                                                                                     | \[YieldType]     |
| Previously indicated        | An order sent in response to an Indication of Interest message.                                                                                                                                                                                                                                                                               | \[OrdType]       |
| Previously quoted           | An order sent in response to a Quote message.                                                                                                                                                                                                                                                                                                 | \[OrdType]       |
| Proceeds Yield              | The CD equivalent yield when the remaining time to maturity is less than two years.                                                                                                                                                                                                                                                           | \[YieldType]     |
| Random                      | A reserve order instruction to refresh using a randomized display quantity.                                                                                                                                                                                                                                                                   | \[DisplayMethod] |
| Recovery Rate               | The recovery rate for the underlying bond on a CDS in the case of a credit event.                                                                                                                                                                                                                                                             | \[MDEntryType]   |
| Recovery Rate Long          | In the case of a credit event the recovery rate on a long position for the bond underlying a CDS.                                                                                                                                                                                                                                             | \[MDEntryType]   |
| Recovery Rate Short         | In the case of a credit event the recovery rate on a short position for the underlying bond on a CDS.                                                                                                                                                                                                                                         | \[MDEntryType]   |
| Redeem                      | For CIV: A “sell” order for CIV units which must be forwarded to the fund manager (or their transfer agent) rather than being matched / crossed with a “buy” order, e.g. by an intermediary, funds supermarket, broker/dealer etc. This would be used in markets where the originator requires specific tax treatment and/or dealing charges. | \[Side]          |
| Reinstate on System Failure | If a system failure interrupts trading or order routing, attempt to reinstate this order, subject to time in force limitations. Note that depending on the type and severity of the failure, this might not be possible.                                                                                                                      | \[ExecInst]      |
| Reinstate on Trading Halt   | If trading in this instrument is halted, reinstate this order when/if trading resumes, subject to time in force limitations.                                                                                                                                                                                                                  | \[ExecInst]      |
| Repurchase Agreement        | Used to report both the initial deal and the repurchase. Allows                                                                                                                                                                                                                                                                               | \[TrdType]       |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 147 of 158
---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 1

# August 18, 2011



| Term                           | Definition                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Field where used           |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------- |
| Request to Intermediary        | Used in a model where an intermediary, e.g. clearing house, is involved in communicating allocation details and actions between two parties                                                                                                                                                                                                                                                                                                                                               | \[AllocType]               |
| Respondent                     | A “respondent” may be one of the following: - a broker/dealer - an inter-dealer broker (or broker’s broker) - an electronic service - bid or offer prices provided by one or more market makers - bid or offer prices provided by an inter-dealer broker matching system with limit orders entered by customers (dealers or institutions) - an issuer                                                                                                                                     | Quoting and other messages |
| Reset Barrier Option           | After hitting the barrier the next specified barrier goes into effect. The payoff depends on the maximum or minimum of the underlying price over the look-back period. The option becomes active (knock-in) or inactive (knock-out) based on a predetermined price level.                                                                                                                                                                                                                 | \[from EP92]               |
| Reversed Net Price             | Relevant for multileg orders. This pricing convention is often used at commodities markets. The price is given as the sum of the Price \* Ratio for all legs. If buying the strategy, the price of a bought leg (which is a buy-leg in the multileg definition) is subtracted, and the price of a sold leg is added. If selling the strategy, the price of a bought leg (which is a sell-leg in the multileg definition) is added, and the price of a sold leg is subtracted.             | \[MultilegPrice Method]    |
| Riskless Principal             | "Riskless" principal transactions are generally described as trades in which, after receiving an order to buy (or sell) from a customer, the broker-dealer purchases (or sells) the security from (or to) another person in a contemporaneous offsetting transaction. Above from the SEC web-site HTUhttp\://www\.sec.gov/rules/final/34-44291.htmUUUTH See Exchange Act Rule 10b-10(a)(2)(ii)(A) \[17 CFR 240. 10b-10(a)(2)(ii)(A)]; Exchange Act Rel. No. 33743 (Mar. 9, 1994) at n.11. | \[OrderCapacity]           |
| Rolling/Ratchet Barrier Option | The option is issued with a sequence of barriers either all below (roll-down calls) or all above (roll-up puts) the current underlying price. Upon reaching each barrier the options strike is lowered for calls or raised for puts. The option is knocked-out at the last barrier.                                                                                                                                                                                                       | \[from EP92]               |

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited

Page 148 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Term

# Definition

# Field where used

| Round Lot           | Also known as as "Board Lot" or "Trade Lot". A standardized number of shares defined by a stock exchange as a trading unit. In most cases, this means 100 shares. Source: www\.investopedia.com                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | \[LotType]   |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------ |
| SEA                 | Succession Event Adjustment quantity (SEA) is used to represent the position transferred from the source CDS position to the target CDS position due to a succession event on processing date. \[from EP83]                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | \[PosType]   |
| Sell Plus           | A round-lot market order to sell “plus” is an order to sell a stated amount of a stock provided that its price is: - not lower than the last sale if the last sale was a “plus” or “zero plus” tick and - not lower than the last sale minus the minimum fractional change in the stock if the last sale was a “minus” or “zero minus” tick. A limit-price order to sell “plus” also states the lowest price at which it can be executed.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | \[OrdType]   |
| Sell Short          | An order to sell a security that the seller does not own; a sale effected by delivering a security borrowed by, or for the account of, the seller. Can only be executed on a “plus” or “zero plus” tick.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | \[OrdType]   |
| Sell Short Exempt   | Short sale exempt from short-sale rules.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | \[OrdType]   |
| Semi-annual Yield   | The yield of a bond whose coupon payments are reinvested semi-annually.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | \[YieldType] |
| Settlement currency | Commonly referred to as "counter currency" in FX nomenclature. For non-NDF deals (FX swaps, spot and forward) the term "settlement currency" can only mean one thing: the currency that is on the opposite from the dealt currency (expressed in FIX using Currency field (tag 15)). For example: Symbol is EUR/USD, and the dealt is EUR then SettlCurrency is USD. For NDF deals the term "settlement currency" could be either the dealt currency or the "counter currency" or a third currency. For example: In a USD/KRW NDF deal where the dealt currency is KRW, the settlement currency is USD, if the dealt currency is USD then the settlement currency can also be USD. In a GBP/KRW NDF deal where the deal typically settles in a third currency, USD in this case, then the settlement currency is USD. (NDFs will be discussed in detail in Phase 2 gap analysis). For FX OTC Spot Options, the settlement currency can refer to either the counter currency or the currency of the option premium (or premia). However, for the purposes of FIX usage, it will refer to the currency of the option premium. |              |
| Settlement Location | Identifies Settlement Depository or, if local settlement, the ISO Country Code.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | \[PartyRole] |
| Simple Yield        | The yield of a bond assuming no reinvestment of coupon payments. (Act/360 day count)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | \[YieldType] |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 149 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Term                                | Definition                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Field where used          |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- |
| Sponsoring Firm                     | A member of the exchange that is sponsoring an Entering Entity to send orders to the exchange. The Sponsoring Member Firm permits sponsorees (e.g. Entering Entities) to trade thereby allowing them to enter orders directly to the exchange via automated means. (e.g. NYSE allowing direct access via Anonymous DOT service).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | \[PartyRole]              |
| Spread                              | A "spread" price is one of four things all denominated in basis points:1) For an outright security trade, the "spread" price is the difference in yield between the object security and its benchmark - implied or explicit.
2) For a swap (or switch) of two issued securities the "spread" price is the difference in yield between the security being sold and the one being bought.
3) For a roll of a futures contract with a contract in the same commodity but having a different contract settlement month the "spread" price is the price difference between the contract being sold and the one being bought.
4) For a floating-rate Financing transaction the “spread” is the difference in yield extended above or below the yield of the stated benchmark.All four types are expressed in basis points (the price or yield difference times 100) and may be negative. | \[PriceType]              | | |
| Stop                                | A stop order to buy which becomes a market order when the security trades at - or above - the stop price after the order is represented in the Trading Crowd. A stop order to sell which becomes a market order when the security trades at - or below - the stop price after the order is represented in the Trading Crowd.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | \[OrdType]                |
| Stop Limit                          | A stop order to buy which becomes a limit order at the limit price when the security trades at - or above - the stop price after the order is represented in the Trading Crowd. A stop order to sell which becomes a limit order at the limit price when the security trades at - or below - the stop price after the order is represented in the Trading Crowd.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | \[OrdType]                |
| Stopped                             | A trade is guaranteed for the order, usually at a stated price or better, but has not yet occurred. For example, a specialist on an exchange may "stop" an order while searching for a better price.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | \[OrdStatus]              |
| Streetside Trade Capture Reporting  | Reporting of completed trades for clearance and settlement or compliance purposes. Reports may be originated by Exchanges or by clearing firms and sent to clearing firms directly or via a clearing corporation or central counterparty such as DTCC in the US.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | A “Section” in “Volume 5” |
| Strict Limit (No Price Improvement) | A limit order that must be traded at the exact limit price specified without any price improvement. Requires OrdType=Limit.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | \[ExecInst]               |
| Subscribe                           | For CIV: A “buy” order for CIV units which must be forwarded to the fund                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | \[Side]                   |

© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited

Page 150 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Definitions

| Term                                 | Definition                                                                                                                                                                                                                                                                                                    | Field where used                        |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------- |
| manager (or their transfer agent)    | rather than being matched / crossed with a “sell” order, e.g. by an intermediary funds supermarket, broker/dealer etc. This would be used in markets where the originator requires specific tax treatment and/or dealing charges.                                                                             |                                         |
| Submission to Clearing               | The timestamp when the trade was officially acknowledged by the Clearing House                                                                                                                                                                                                                                | \[SideTrdRegTimestampType] \[from EP77] |
| Substitution of futures for forwards | An OTC deal where the participants have agreed that the OTC deal will immediately be extinguished and replaced with a transaction in a regulated future.                                                                                                                                                      | \[TradeCondition]                       |
| Succession Event                     | An event in which one entity succeeds to the obligations of another entity due to a corporate action (as a result of operation or law or pursuant to any agreement), which may include events such as a merger, consolidation, an amalgamation, a transfer of assets or liabilities, a de-merger, a spin-off. | \[CorporateAction]                      |
| Suspended                            | The order is not eligible for trading. This usually happens as a result of a verbal or otherwise out of band request to suspend the order, or because the order was submitted, or modified via a Cancel/Replace Request, with ExecInst=Suspended.                                                             | \[OrdStatus]                            |
| Swap Start Date                      | Starting date of an interest rate swap. Corresponds to the roll date date.                                                                                                                                                                                                                                    | \[EventType]                            |
| Swap End Date                        | End date of an interest rate swap. Corresponds to the termination date.                                                                                                                                                                                                                                       | \[EventType]                            |
| Swap Next Start Date                 | Next starting date of an interest rate swap subsequent to the roll date.                                                                                                                                                                                                                                      | \[EventType]                            |
| Swap Roll date                       | Date on which a swap contract is rolled to the next period or start date. Corresponds to the Fixing Date on which the floating rate of the swap is set.                                                                                                                                                       | \[EventType]                            |
| Swap Value Factor                    | The daily change in Net Present Value of a swap trade or position. Used in calculating the mark-to-market amount.                                                                                                                                                                                             | \[MDEntryType]                          |
| Tax Equivalent Yield                 | The after tax yield grossed up by the maximum federal tax rate of 39.6%. For comparison to taxable yields.                                                                                                                                                                                                    | \[YieldType]                            |
| TED Price                            | The price spread between the active 3 month treasury bill futures contract and the 3 month Eurodollar futures contract. Used as an indicator of investor confidence in the U.S. markets.                                                                                                                      | \[PriceType]                            |
| TED Yield                            | The difference in basis points between the yield-to-maturity of the bond / note and the yield-to-maturity of a Hypothetical Euromarket bond with identical coupon and maturity.                                                                                                                               | \[PriceType]                            |
| Time In                              | According to US futures markets (CFTC): Timestamp of when order was received on the trading floor (booth).                                                                                                                                                                                                    | \[TrdRegTimestampType]                  |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 151 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Errata

| Term                                                | Definition                                                                                                                                                                                                                                       | Field where used       |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------- |
| Time Out                                            | According to US futures markets (CFTC): Timestamp when the trade was received from the pit.                                                                                                                                                      | \[TrdRegTimestampType] |
| Tradable Indicator                                  | Specifies whether an instrument can be traded or not.                                                                                                                                                                                            | \[InstrAttribType]     |
| Trade Along                                         | Clients who specify "Trade Along" give brokers permission to handle and place their order in the market even if the broker already has its own proprietary orders for the same security placed in the market.                                    | \[ExecInst]            |
| Trade Confirmation                                  | A trade report confirming that a trade has been made.                                                                                                                                                                                            | \[TradeHandlingInstr]  |
| Trading Session VWAP Price                          | The volume weighted average price for a trading session.                                                                                                                                                                                         | \[QuoteCondition]      |
| Trailing Stop Peg                                   | A pegged order representing a stop order with a stop price pegged to trail a specified distance behind the last sale price. The price of a trailing stop to buy can never increase, and the price of a trailing stop to sell can never decrease. | \[ExecInst]            |
| Triggered or Activated by System                    | An execution instruction that indicates that the sending system activated or updated the order. The triggering or activation is normally based on instructions provided in the original order – e.g. Stop order instructions.                    | \[ExecType]            |
| True Gross Yield                                    | Yield calculated using the price including accrued interest, where coupon dates are moved from holidays and weekends to the next trading day.                                                                                                    | \[YieldType]           |
| True Yield                                          | The yield calculated with coupon dates moved from a weekend or holiday to the next valid settlement date.                                                                                                                                        | \[YieldType]           |
| Try to Stop                                         | Used in specialist-driven markets to direct the specialist to try and stop the order.                                                                                                                                                            | \[ExecInst]            |
| Two-Party Report                                    | A trade report requesting the receiver to register both sides of a privately negotiated trade.                                                                                                                                                   | \[TradeHandlingInstr]  |
| Two-Party Trade Report (privately negotiated trade) | Specifies that the source for a trade report is a privately negotiated Two-Party Trade Report.                                                                                                                                                   | \[MatchType]           |
| Unacceptable Counterparty                           | A counterparty not eligible for trading with the order or quote Initiator.                                                                                                                                                                       | \[PartyRole]           |
| Underlying Contra Firm                              | The broker or other firm which is the contra side of the trade for the underlying security.                                                                                                                                                      | \[PartyRole]           |
| Uneven swap                                         | An FX Swap where the given amount to be bought and sold is different on the near and far legs.                                                                                                                                                   |                        |
| Up and In Option                                    | The option becomes active if the underlying price exceeds the specified barrier.                                                                                                                                                                 | \[from EP92]           |
| Up and Out Option                                   | the option becomes inactive if the underlying price exceeds the                                                                                                                                                                                  |                        |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 152 of 158
---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

| Term                                    | Definition                                                                                                                                                                                                                                                                                                                                | Field where used        |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| URI (Uniform Resource Identifier)       | W3C standard defined as "the generic set of all names/addresses that are short strings that refer to resources". Note that "URL" (Uniform Resource Locator), commonly referred to by web browsers, is a subset of the URI standard. The W3C standards body considers URL an "informal term (no longer used in technical specifications)". | See Appendix 6-B        |
| User-defined Multileg Security          | An order for a multileg security where the user defines the security together with the order and that multileg security is made available to other actors. The lifetime of the multileg is bilaterally agreed between parties. \[Note: see Vol. 4's Multileg Order section on predefined multileg security models]                        | \[MultilegModel]        |
| User-defined, Non-Securitized, Multileg | A multileg order that does not involve securitization. The multileg is not made available to other parties and the order is represented only as implied-out prices. The multileg expires with the order. \[Note: see Vol. 4's Multileg Order section on predefined multileg security models]                                              | \[MultilegModel]        |
| User requested                          | Instruments can be user requested as well as pre-listed.                                                                                                                                                                                                                                                                                  | \[ListMethod]           |
| Value date                              | The delivery or settlement date of a foreign exchange trade transaction.                                                                                                                                                                                                                                                                  |                         |
| Vanilla Payout Option                   | The payout amount is determined by the difference between the strike and the underlying.                                                                                                                                                                                                                                                  | \[from EP92]            |
| Variable Tick Rule                      | A cabinet order can trade in even increments that can be significantly different than a conventional tick increment.                                                                                                                                                                                                                      | \[TickRuleType]         |
| Volume Weighted Average Price           | Volume Weighted Average Price is used for trades in which the price is set at a guaranteed VWAP.                                                                                                                                                                                                                                          | \[TrdType]              |
| With or Without                         | An odd lot order filled on an effective round lot transaction, or on an effective bid or offer, whichever occurs first after the specialist receives the order. (e.g. NYSE order type)                                                                                                                                                    | \[OrdType]              |
| Yield At Issue                          | Municipals. The yield of the bond offered on the issue date.                                                                                                                                                                                                                                                                              | \[YieldType]            |
| Yield Change Since Close                | The change in the yield since the previous day's closing yield.                                                                                                                                                                                                                                                                           | \[YieldType]            |
| Yield Difference                        | The price of a strategy (multileg) order is given as a spread between the two legs.                                                                                                                                                                                                                                                       | \[MultilegPrice Method] |
| Yield To Average Maturity               | The yield achieved by substituting a bond's average maturity for the issue's final maturity date.                                                                                                                                                                                                                                         | \[YieldType]            |
| Yield To Next Call                      | The yield of a bond to the next possible call date.                                                                                                                                                                                                                                                                                       | \[YieldType]            |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 153 of 158
---
Version 5.0 Service Pack 2 - Errata   VOLUME 1                                                            August 18, 2011

# Definitions

| Term                            | Definition                                                                                                                                                                             | Field where used |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| Yield To Longest Average Life   | The yield assuming only mandatory sinks are taken. This results in a lower paydown of debt; the yield is then calculated to the final payment date.                                    | \[YieldType]     |
| Yield To Maturity               | The yield of a bond to its maturity date.                                                                                                                                              | \[YieldType]     |
| Yield To Next Put               | The yield to the date at which the bond holder can next put the bond to the issuer.                                                                                                    | \[YieldType]     |
| Yield To Next Refund            | Sinking Fund Bonds. Yield assuming all bonds are redeemed at the next refund date at the redemption price.                                                                             | \[YieldType]     |
| Yield To Shortest Average Life  | The yield assuming that all sinks (mandatory and voluntary) are taken at par. This results in a faster paydown of debt; the yield is then calculated to the final payment date.        | \[YieldType]     |
| Yield To Tender Date            | The yield on a Municipal bond to its mandatory tender date.                                                                                                                            | \[YieldType]     |
| Yield To Worst                  | The lowest yield to all possible redemption date scenarios.                                                                                                                            | \[YieldType]     |
| Yield Value of 1/32             | The amount that the yield will change for a 1/32PⁿᵈP change in price.                                                                                                                  | \[YieldType]     |
| Yield with Inflation Assumption | Based on price, the return an investor would require on a normal bond that would make the real return equal to that of the inflation-indexed bond, assuming a constant inflation rate. | \[YieldType]     |

# Other sources for definitions of financial terms include:

- http://www.investinginbonds.com
- http://www.investopedia.com
- http://www.investorwords.com
- http://www.riskglossary.com
- http://www.888options.com

© Copyright, 2008-   ~~2009~~2011, FIX Protocol, Limited                                                 Page 154 of 158
---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                                        August 18, 2011


# Appendix 1-A: Abbreviations used within FIXML

| Acct    | Account               |
| ------- | --------------------- |
| Acrl    | Accrual               |
| Acrd    | Accrued               |
| Ack     | Acknowledgement       |
| Actn    | Action                |
| Adj     | Adjust                |
| Adjmt   | Adjustment            |
| Adv     | Advertisement         |
| Afctd   | Affected              |
| Algo    | Algorithm             |
| Alloc   | Allocation            |
| AOS     | AllowableOneSidedness |
| Amt     | Amount                |
| Appl    | Application           |
| Asgn    | Assignment            |
| Attch   | Attachment            |
| Attrb   | Attribute             |
| Base    | Base                  |
| Bhf     | Behalf                |
| Bnchmk  | Benchmark             |
| Bkng    | Booking               |
| Brkr    | Broker                |
| Brkrs   | Brokers               |
| Biz     | Business              |
| Calc    | Calculation           |
| Cxl     | Cancel                |
| Cpcty   | Capacity              |
| Capt    | Capture               |
| Csh     | Cash                  |
| Catgy   | Category              |
| Cl      | Client                |
| Cls     | Close                 |
| Cd      | Code                  |
| Coll    | Collateral            |
| Comm    | Commission            |
| Cmn     | Common                |
| Comp    | Company               |
| Cmplx   | Complex               |
| Cond    | Condition             |
| Cnfm    | Confirmation          |
| Confirm | Confirmation          |
| Cntxt   | Context               |
| Cntra   | Contra                |
| Ctrl    | Control               |
| Corp    | Corporate             |
| Ctry    | Country               |
| Cpn     | Coupon                |
| Crss    | Cross                 |
| Cum     | Cumulative            |
| Ccy     | Currency              |
| Crv     | Curve                 |
| Data    | Data                  |
| Db      | Database              |
| Dt      | Date                  |
| Def     | Definition            |
| Del     | Delete                |
| Dlvr    | Deliver               |
| Deriv   | Derivative            |
| Desc    | Description           |
| Dest    | Destination           |
| Detl    | Detail                |
| Dtrmn   | Determination         |


© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                               Page 155 of 158

---
Version 5.0 Service Pack 2 - Errata VOLUME 1 August 18, 2011

# Glossary of Terms

| Dev     | Device                 |
| ------- | ---------------------- |
| Disc    | Discount               |
| Dsctn   | Discretion             |
| Dsctnry | Discretionary          |
| DK      | Don't Know             |
| Dup     | Duplicate              |
| Efctv   | Effective              |
| Enc     | Encoded                |
| Err     | Error                  |
| Evnt    | Event                  |
| Exch    | Exchange               |
| EFP     | ExchangeForPhysical    |
| Exct    | Execute                |
| Exctn   | Execution              |
| Exr     | Exercise               |
| Fctr    | Factor                 |
| Feed    | Feed                   |
| Force   | Force                  |
| FX      | Foreign Currency       |
| Fwd     | Forward                |
| Fut     | Future                 |
| GTD     | Good Till Date         |
| Grp     | Group                  |
| Hndl    | Handling               |
| ID      | Identifier             |
| Implct  | Implicit               |
| Incr    | Increment              |
| Ndx     | Index                  |
| IOI     | Indication of Interest |
| Ind     | Indicator              |
| Info    | Information            |
| Inpt    | Input                  |
| Inq     | Inquiry                |
| Instn   | Institution            |
| Inst    | Instruction            |
| Instrmt | Instrument             |
| Int     | Interest               |
| Iss     | Issue                  |
| Issr    | Issuer                 |
| Lang    | Language               |
| Lvl     | Level                  |
| Lmt     | Limit                  |
| Lqdty   | Liquidity              |
| List    | List                   |
| Loc     | Locate                 |
| Lctn    | Location               |
| Lot     | Lot                    |
| Mnt     | Maintenance            |
| Mgn     | Margin                 |
| Mkt     | Market                 |
| Mass    | Mass                   |
| Mtch    | Match                  |
| Mat     | Maturity               |
| Max     | Maximum                |
| Msg     | Message                |
| Meth    | Method                 |
| Min     | Minimum                |
| Misc    | Miscellaneous          |
| Model   | Model                  |
| Mod     | Modification           |
| Mny     | Money                  |
| Mo      | Month                  |
| Mleg    | Multileg               |
| Mult    | Multiplier             |
| Nme     | Name                   |
| Nst     | Nested                 |
| Ntwk    | Network                |

© Copyright, 2008-2009, 2011, FIX Protocol, Limited Page 156 of 158
---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                                        August 18, 2011


# Abbreviations

| News    | Notification   |
| ------- | -------------- |
| Notl    | Notional       |
| Num     | Number         |
| No      | Number         |
| Oblig   | Obligation     |
| Ofr     | Offer          |
| Oper    | Operator       |
| Opt     | Option         |
| Ord     | Order          |
| Orig    | Original       |
| Oth     | Other          |
| Out     | Outstanding    |
| Prm     | Parameter      |
| Pty     | Party          |
| Pmt     | Payment        |
| Pct     | Percent        |
| Pctg    | Percentage     |
| Pltfm   | Platform       |
| Pnt     | Point          |
| Pos     | Position       |
| Psbl    | Possible       |
| Prcsn   | Precision      |
| Prlm    | Preliminary    |
| Prev    | Previous       |
| Px      | Price          |
| Pri     | Priority       |
| Prtztn  | Prioritization |
| Prod    | Product        |
| Pub     | Publish        |
| Qual    | Qualifier      |
| Qlty    | Quality        |
| Qty     | Quantity       |
| Quot    | Quote          |
| Rng     | Range          |
| Rt      | Rate           |
| Rtng    | Rating         |
| Rsn     | Reason         |
| Red     | Redemption     |
| Ref     | Reference      |
| Rgst    | Registration   |
| Rgstry  | Registry       |
| Rej     | Reject         |
| Reltd   | Related        |
| Rltnshp | Relationship   |
| Rpt     | Report         |
| Rpts    | Reports        |
| Repo    | Repurchase     |
| Req     | Request        |
| Reset   | Reset          |
| Rsp     | Response       |
| Rstmt   | Restatement    |
| Rstct   | Restrict       |
| Rstctn  | Restriction    |
| Rstctns | Restrictions   |
| Rstrct  | Restructuring  |
| Rslt    | Result         |
| Risk    | Risk           |
| R       | Roles          |
| Rnd     | Round          |
| Rules   | Rules          |
| Scope   | Scope          |
| 2       | Secondary      |
| Sec     | Security       |
| Seg     | Segment        |
| Snd     | Sender         |


© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                               Page 157 of 158

---

Version 5.0 Service Pack 2 - Errata   VOLUME 1                                          August 18, 2011


# Abbreviations

| Sndg    | Sending        |
| ------- | -------------- |
| Snrty   | Seniority      |
| Seq     | Sequence       |
| Svc     | Service        |
| Ses     | Session        |
| Settl   | Settlement     |
| Shrt    | Short          |
| Sz      | Size           |
| Src     | Source         |
| Stand   | Standing       |
| Start   | Start          |
| St      | State          |
| Stat    | Status         |
| Stip    | Stipulation    |
| Strt    | Strategy       |
| Strm    | Stream         |
| Strk    | Strike         |
| Sub     | Subscription   |
| Subsid  | Subsidiary     |
| Sfx     | Suffix         |
| Sym     | Symbol         |
| Sys     | System         |
| Tgt     | Target         |
| Trm     | Term           |
| Tick    | Tick           |
| Tkt     | Ticket         |
| Tm      | Time           |
| TS      | Timestamp      |
| Tot     | Total          |
| Trkng   | Tracking       |
| Trd     | Trade          |
| Trdg    | Trading        |
| TrdgSes | TradingSession |
| Txn     | Transaction    |
| Typ     | Type           |
| Und     | Underlying     |
| Upd     | Update         |
| Val     | Valuation      |
| Valu    | Value          |
| Venu    | Venue          |
| Vol     | Volume         |
| Warn    | Warning        |
| Yr      | Year           |
| Yld     | Yield          |


© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                                 Page 158 of 158
