
# FINANCIAL INFORMATION

# EXCHANGE PROTOCOL

# (FIX)

# Version 5.0 Service Pack 2 - Errata

# VOLUME 2 – TRANSPORT PROTOCOLS

# August 18, 2011



© Copyright, 2008-2009 2011, FIX Protocol, Limited



---

# Version 5.0 Service Pack 2 - Errata

# VOLUME 2

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


Page 2 of 9


---
Version 5.0 Service Pack 2 - Errata   VOLUME 2                                 August 18, 2011

# Contents – Volume 2

# INTRODUCTION

........................................................................................................................................................ 4

# TRANSPORT INDEPENDENCE (TI) FRAMEWORK

.........................................................................................................................................................4

# APPLICATION VERSIONING

......................................................................................................................................................... 5

# SERVICE PACK MANAGEMENT

......................................................................................................................................................... 6

# EXTENSION PACK MANAGEMENT

.........................................................................................................................................................6

# USE CASE 1 – FORMAL FIX 5.0 RELEASE

.........................................................................................................................................................6

# USE CASE 2 – FIX 5.0 RELEASE W/ SUPPORT FOR LEGACY

......................................................................................................................................................... 8

# USE CASE 3 – TRANSPORT INDEPENDENCE

.........................................................................................................................................................8

# TRANSPORT PROTOCOLS

.........................................................................................................................................................8

# FIX SESSION PROTOCOL

.........................................................................................................................................................8

# FIX USING A MULTICAST TRANSPORT

.........................................................................................................................................................8

# FIX USING THE FAST SESSION CONTROL PROTOCOL (SCP)

.........................................................................................................................................................8

# FIX USING MQSERIES

......................................................................................................................................................... 9

# FIX USING WEB SERVICES

.........................................................................................................................................................9

© Copyright, 2008-~~2009~~2011, FIX Protocol, Limited                             Page 3 of 9
---
Version 5.0 Service Pack 2 - Errata   VOLUME 2                                               August 18, 2011

# Introduction

With the release of FIX 5.0 in December 2006, the FPL Global Technical Committee (GTC) introduced a new framework, the transport independence (TI) framework, which separated the FIX Session Protocol from the FIX Application Protocol. Under the TI framework the application protocol messages can be sent over any suitable session transport technology (e.g. WS-RX, MQ, publish/subscribe message bus), where the FIX Session Protocol is one of the available options as a session transport for FIX application messages. From this release forward the FIX Application layer and the FIX Session layer will have their own versioning moniker. The FIX Application layer will retain the traditional version moniker of "FIX x.y" while the FIX Session layer will utilize a new version moniker of "FIXT x.y" (note that the version numbers will be independent of each other). The diagram below illustrates how previously the FIX Session layer was tightly coupled to the Application layer. With the advent of Application Versioning and Transport Independence, the FIX Session and Application layers have been decoupled and are now independent.

This Volume of the FIX Protocol Specification will not discuss any particular session protocol, but will provide pointers to documents that describe the various transport protocols that the GTC has investigated and provided recommendation on how they should be used within the TI framework.

# FIX 5.0 Unlocks the Application Layer From the Session Layer

| FIX 4.0,4.1,4.2,4.3,4.4 | FIX 5.0                |
| ----------------------- | ---------------------- |
| FIX Application Layer   | Application Versioning |
| FIX Session Layer       | Transport Independence |
| FIX Session Layer       |                        |

# Transport Independence (TI) Framework

The transport independence (TI) framework separates the previously coupled FIX Session layer from the FIX Application layer. Under this framework the FIX Application Protocol can use any transport technology in addition to the FIX Session Protocol. The diagram below illustrates how various transport mechanisms, including the FIX Session layer, can be used to carry the full suite of FIX Application versions.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                         Page 4 of 9
---
Version 5.0 Service Pack 2 - Errata   VOLUME 2                                                August 18, 2011

# Transport Independence Framework

| FIX Session          | Web Services         | Transport            | Multicast            | Other Transport      |                      |
| -------------------- | -------------------- | -------------------- | -------------------- | -------------------- | -------------------- |
| Layer                | HTTP                 | MQSeries             | UDP                  |                      | Transport            |
| FKXLL                | Transport            |                      |                      |                      |                      |
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

© Copyright, 2008-    ~~2009~~2011, FIX Protocol, Limited                                         Page 5 of 9
---

Version 5.0 Service Pack 2 - Errata   VOLUME 2                                                   August 18, 2011


# Service Pack Management

ApplVerID is an enumerated field. These enumerations are used to express prior versions of FIX inclusive of FIX 4.0, 4.1, 4.2, 4.3 and 4.4 as well as the most recent version, FIX 5.0. Going forward, service packs will be applied to the base version, in this case FIX 5.0, and will be identified as FIX Version + Service Pack. This means that FIX 5.0 will be represented as an enumeration (7) rather than as an actual value in the ApplVerID field. Service Pack identifiers will consist of the base FIX version, the service pack number for that version, and the date the service pack was released. For example, the assigned value for service pack 1 may be “FIX 5.0 SP1 June 30, 2007”.

# Extension Pack Management

Extension Packs are the building blocks of a Service Pack and represent specific functional proposals that have been presented to the GTC. Prior to the release of a Service Pack, Extension Packs are applied to the most recent version of the repository so that they can be used at the point they become available. Extension Packs are applied to the repository in a cumulative manner and will at some point culminate in a Service Pack release. Extension Packs management will be conducted as follows:

1. Extension Packs will be assigned a unique, sequential number at the point they are approved by the GTC
2. Extension Packs are applied to the most recent version of the repository and may be inclusive of prior Extension Packs
3. At the point an Extension Pack has been applied, the updated repository, schema, and message tables will be available
4. When implementing a specific Extension Pack, the field CustomApplVerID (1129) will be used to specify the Extension Pack Identifier
5. User’s of an Extension Pack need not implement other Extension Packs present in the repository. Rules of engagement need to be bilaterally agreed on.

# Use Case 1 – Formal FIX 5.0 Release

This is the ‘GTC approved’ approach which separates the FIX session layer from the application layer, provides support for application versioning, and creates a platform for transport independence. This approach will treat the FIX session like ‘any other’ transport and allow the unambiguous use of any application version via the ApplVerID field. A value of FIXT.1.1 in the BeginString of the FIX Session will indicate that application versioning is in effect and the version should be determined either through the Logon's NoMsgType repeating group or the AppVerID field. Future extensions to the session layer or application layer will be supported independent of each other as point releases to BeginString and ApplVerID, respectively. Major Tags describing the session and application versions are: BeginString=FIXT.1.1 (or later versions) and ApplVerID=7 (FIX.5.0₁) or later versions. A BeginString=FIX.5.0 (or later versions) will not be valid.

The diagram below illustrates how the new FIXT.1.1 Session layer makes use of the ApplVerID in the Application layer in order to support a broad set of application versions.

1. The value FIX.5.0 is represented by enumeration value 7
2. FIX.4.0, FIX.4.1, FIX.4.2, FIX.4.3, FIX.4.4, FIX.5.0, FIX.5.0.SP1, FIX.5.0.SP2 are represented using enumerations


© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                                Page 6 of 9

---
Version 5.0 Service Pack 2 - Errata   VOLUME 2                                              August 18, 2011

# Buy Side

# FIX Application Layer

| ApplVerID = FIX.4.0 | New Order           | FIX.4.1 |
| ------------------- | ------------------- | ------- |
|                     | ApplVerID = FIX.4.1 | Quote   |

# FIX Session Layer

| BeginString = FIXT.1.1 | ApplVerID = FIX.4.2 | FIX.4.2 |
| ---------------------- | ------------------- | ------- |
| Market Data            |                     |         |

# FIX Session Layer

| BeginString = FIXT.1.1 | ApplVerID = FIX.4.4 | FIX.4.4 |
| ---------------------- | ------------------- | ------- |
| Allocation Instruction |                     |         |

# FIX Application Layer

| ApplVerID = FIX.5.0 | TradeCapture Report |
| ------------------- | ------------------- |

© Copyright, 2008-     ~~2009~~2011, FIX Protocol, Limited                                    Page 7 of 9
---
Version 5.0 Service Pack 2 - Errata   VOLUME 2                                                  August 18, 2011

# Use Case 2 – FIX 5.0 Release w/ Support for Legacy

This approach builds on Use Case 1 above but makes a provision for legacy installations by allowing the application version ID of the legacy version to be specified as a default in the Logon message in the DefaultAppVerID (tag 1137) field. This would apply to those all messages not carrying the ApplVerID field or overridden through the Logon's NoMsgType repeating group. Major Tags describing the session and application versions are: BeginString=FIXT.1.1 (or later versions) and DefaultAppVerID set to other earlier versions of FIX. It should be noted that DefaultApplVerID can also be used in Use Case 1 in a similar manner to default the session's application version to FIX 5.0 or later versions - this eliminates the need to have to specify ApplVerID in every message if the session will primarily carry FIX 5.0 application messages.

# Use Case 3 – Transport Independence

This is a standalone release of FIX 5.0. A transport other than the FIX session protocol is used and all application messages must carry the ApplVerID field to identify the version of the FIX application message. BeginString is not used.

Under this use case, session level messages as defined in FIXT 1.1 are not available and are thus unsupported message types. Note that this includes the session level Reject message. In situations where rejection of messages may be needed, the alternate transport might offer a similar reject message that can be used to reject messages that violate session-level rules. If this is not the case, the Business Message Reject message should be used instead.

# Transport Protocols

# FIX Session Protocol

The FIX Session Protocol Version 1.1 specification can be found at http://www.fixprotocol.org/specifications. This is the traditional FIX Session Protocol with additions to support TI.

# FIX Using a Multicast Transport

A best practices guide to using a multicast transport can be found at http://fixprotocol.org/documents/2519/FIX_Multicast_Transport_v1.0.pdf. Although this document specifically discusses the dissemination of FIX Market Data messages over a Multicast Transport, the concepts discussed can be used on any FIX messages. However, users are cautioned that using a Multicast Transport for trade transactions is not recommended.

# FIX Using the FAST Session Control Protocol (SCP)

SCP 1.0 provides a set of predefined messages that are used to initiate and control the exchange of FAST encoded messages. SCP 1.0 is an integral component of the FAST Protocol Version 1.0. The use of SCP 1.0 for the transmission of FAST 1.0 Encoded Messages is optional. However, if an application requires session control, then SCP 1.0 is the appropriate and only FAST 1.0 compliant session control protocol that should be used.

Creation of the Session Control Protocol arose to address a need for session control to support FIX session encapsulation over FAST. This is commonly referred to as FAST tunneling. A later usage was identified to use the SCP FastReset message to explicitly identify Frame boundaries to communicate to the recipient of FAST Messages when the encoding state should be reset. The complete SCP 1.0 specification can be found at http://fixprotocol.org/documents/2375/FAST_SessionControlProtocol_v1.00.pdf.

© Copyright, 2008-  ~~2009~~2011, FIX Protocol, Limited                                            Page 8 of 9
---

Version 5.0 Service Pack 2 - Errata   VOLUME 2                                               August 18, 2011


# The FIA standards working group has drafted a set of proposed standards for the use of IBM WebSphere MQ

(formerly MQSeries) to facilitate intercommunication between its members. The recommended standards will be based on best practices germane to the post-trade environment and the post-trade messages exchanged by participating counterparties.

# Application Guidelines

include recommendations for the use of the MQI standard WMQ application programming interface for sending and receiving messages. Administrative Guidelines include recommendations for defining the Queue Managers and the objects through which intercommunication between participants is achieved. The document can be found at http://fixprotocol.org/documents/1869/MQStandardizationrequirements5.doc

# FIX Using Web Services

Currently the Web Services Working Group is continuing its investigation of web services while the group waits for OASIS to approve the WS-RX specification. The Web Service Working Group is looking at WS-RX as the specification to base FIX's WS policies on. For current information on the Web Services Working Group please see the group's web page at: http://www.fixprotocol.org/working_groups/wswg. To participate in this working group please contact the FPL Program Office at fpl@fixprotocol.org


© Copyright, 2008- ~~2009~~2011, FIX Protocol, Limited                                          Page 9 of 9
