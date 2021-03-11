//! Try the following links:
//!
//! - http://localhost:8080/mnemonics/FIX-4.4
//! - http://localhost:8080/mnemonics/FIX-4.4/97
//! - http://localhost:8080/mnemonics/FIX-4.2/10
//! - http://localhost:8080/structs/FIX-4.3

use fefix::{codegen, codegen_tag_mnemonics, AppVersion, Dictionary};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> tide::Result<()> {
    server().listen("127.0.0.1:8080").await?;
    Ok(())
}

fn server() -> tide::Server<State> {
    let state = State::new();
    let mut app = tide::with_state(state);
    app.at("/mnemonics/:version").get(serve_mnemonics);
    app.at("/mnemonics/:version/:tag").get(serve_mnemonics);
    app.at("/structs/:version").get(serve_structs);
    app
}

/// [`State`] contains any global state necessary to serve web requests. In this
/// case, JSON (en/de)coding devices.
#[derive(Clone)]
struct State {
    dictionaries: HashMap<AppVersion, Dictionary>,
}

impl State {
    fn new() -> Self {
        let mut state = State {
            dictionaries: HashMap::new(),
        };
        for version in AppVersion::all() {
            let dict = Dictionary::from_version(version);
            state.dictionaries.insert(version, dict);
        }
        state
    }
}

async fn serve_mnemonics(req: tide::Request<State>) -> tide::Result {
    let version_name = req.param("version").unwrap();
    let version = AppVersion::from_str(version_name).unwrap();
    let dict = req.state().dictionaries.get(&version).unwrap();
    let body_response = {
        if let Ok(tag) = req.param("tag") {
            let tag = tag.parse::<u32>().unwrap();
            dict.field_by_tag(tag).unwrap().name().to_string()
        } else {
            codegen_tag_mnemonics(&dict)
        }
    };
    Ok(body_response.into())
}

async fn serve_structs(req: tide::Request<State>) -> tide::Result {
    let version_name = req.param("version").unwrap();
    let version = AppVersion::from_str(version_name).unwrap();
    let dict = req.state().dictionaries.get(&version).unwrap();
    Ok(codegen(&dict).into())
}
