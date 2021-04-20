//! Starts an HTTP server on any open port and listens for JSON FIX messages.

use fefix::{json, tagvalue, AppVersion, Dictionary};

#[tokio::main]
async fn main() -> tide::Result<()> {
    server().listen("127.0.0.1:8080").await?;
    Ok(())
}

fn server() -> tide::Server<State> {
    let state = State::new();
    let mut app = tide::with_state(state);
    app.at("/").get(serve_hello_world);
    app.at("/fix-json").post(serve_json_relay);
    app
}

/// [`State`] contains any global state necessary to serve web requests. In this
/// case, JSON (en/de)coding devices.
#[derive(Clone)]
struct State {
    decoder: json::Decoder<json::Config>,
    encoder: tagvalue::Encoder,
}

impl State {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for State {
    fn default() -> Self {
        let dictionary = Dictionary::from_version(AppVersion::Fix42);
        let mut config = json::Config::default();
        config.set_pretty_print(true);
        Self {
            decoder: json::Decoder::with_config(dictionary, config),
            encoder: tagvalue::Encoder::<Vec<u8>, tagvalue::Config>::from_buffer(vec![]),
        }
    }
}

async fn serve_hello_world(_req: tide::Request<State>) -> tide::Result {
    Ok("Hello, world!".to_string().into())
}

async fn serve_json_relay(mut req: tide::Request<State>) -> tide::Result {
    let decoder = &mut req.state().decoder.clone();
    let encoder = &mut req.state().encoder.clone();
    let body: Vec<u8> = req.body_bytes().await?;
    let message = { decoder.decode(&body[..]).unwrap() };
    let body_response = {
        //let mut state = encoder.start_message(b"FIX-4.4", message.f_msg_type().unwrap().as_bytes());
        //for (tag, _value) in message.iter_fields() {
        //    state.set_any(tag, b"TODO");
        //}
        "TODO"
    };
    Ok(body_response.into())
}

#[cfg(test)]
mod test {
    use super::*;
    use tide::http::{Method, Request, Response};

    #[tokio::test]
    async fn hello_world() {
        let server = server();
        let req = Request::new(Method::Get, "http://localhost:8080/");
        let mut response: Response = server.respond(req).await.unwrap();
        assert_eq!(response.status(), 200);
        assert_eq!(response.body_string().await.unwrap(), "Hello, world!");
    }
}
