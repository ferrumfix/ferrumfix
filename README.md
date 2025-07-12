<div align="center">

# RustyFix

</div>

RustyFix is a free and open source FIX engine implementation forked from [FerrumFIX](https://github.com/ferrumfix/ferrumfix) in Rust. Please note that it's currently under heavy development and wildly unstable, so all interested parties should refrain from using it in production prior to its 1.0 release. Performance and full adherence to the FIX protocol are nevertheless core goals of the project which, if anything, might serve as a useful foundation for others' work.

- [RustyFix](#rustyfix)
	- [About](#about)
	- [Sponsors](#sponsors)
	- [Contributing](#contributing)
	- [Legal](#legal)

## About

RustyFix provides parsing, validation, error recovery, and (de)serialization for the FIX family of protocols.

![FIX Technical Standard stack](https://github.com/rusty-engine/rustyfix/raw/main/docs/FIX-Technical-Standard-Stack.png)

The above illustration succintly describes the full scope of FIX and it serves as a reference point for all modern FIX implementations. RustyFix aims with total compliance... *eventually*. Engineering efforts are initially focused on core features e.g. tag-value encoding and FIX 4.4.

RustyFix enforces strict separation of concerns according to the OSI model, as reasonably allowed by the FIX specification.

- Layer 4 (Transport Layer): `rustyfixs`.
- Layer 5 (Session Layer): `rustyfix::session`.
- Layer 6 (Presentation Layer): `rustyfix::tagvalue`, `rustyfix::json`, `rustyfast`.
- Layer 7 (Application Layer): `rustyfix::Dictionary`.

You don't have to understand the whole tech stack to use a single layer; in fact, RustyFix makes sure that you only ever need to worry about layers above your chosen abstraction level. For most users, that would be Layer 7 (i.e. semantics of FIX messages and business logic). On the other hand, you will need to delve deep into lower layers in case you plan on building a fully-fledged FIX engine.

**Core features:**

- [X] Code generation (Rust, possibly others).
- [X] FIX 4.2.
- [X] FIX 4.4.
- [X] FIX 5.0 Service Pack 2.

**Encodings:**

- [X] Tagvalue (classic FIX).
- [ ] FIXML.
- [ ] Simple Binary Encoding (SBE).
- [ ] Google Protocol Buffers (GPB).
- [X] JavaScript Object Notation (JSON).
- [ ] Abstract Syntax Notation (ASN.1).
- [ ] FIX Adapted for STreaming (FAST).

## Legal

RustyFix is available under the terms of the Apache License 2.0. See `LICENSE` in this repository for more information.

All FIX Protocol-related intellectual property, including but not limited to the original documentation that ships with RustyFix, is licensed by FIX Protocol Ltd. under *Creative Commons Attribution - No Derivatives 4.0 International* (CC BY-ND 4.0). By contributing to this project you agree to comply with all license requirements.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
