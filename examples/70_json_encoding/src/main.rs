//! Starts an HTTP server on any open port and listens for JSON FIX messages.

mod state;

use fefix::definitions::fix42;
use fefix::{json, json::FieldOrGroup, tagvalue::Fv};
use state::{Encoder, State};

#[tokio::main]
async fn main() -> tide::Result<()> {
    server().listen("127.0.0.1:8080").await?;
    Ok(())
}

fn server() -> tide::Server<State> {
    let state = State::new();
    let mut app = tide::with_state(state);
    app.at("/").post(serve_json_relay);
    app
}

fn fix_json_to_tagvalue<'a, 'b>(
    state: &'b State,
    encoder: &'a mut Encoder,
    json: json::Message<'b>,
) -> Result<&'a [u8], &'static str> {
    let msg_type = json.fv_opt(fix42::MSG_TYPE).unwrap().unwrap();
    let mut response = encoder.start_message(b"FIX.4.2", msg_type);
    for (field_name, field_value) in json.iter_fields() {
        let tag = state.lookup_field_name(field_name).unwrap();
        match field_value {
            FieldOrGroup::Field(s) => {
                response.set_any(tag, s.as_ref());
            }
            FieldOrGroup::Group(_g) => {}
        }
    }
    Ok(response.wrap())
}

async fn serve_json_relay(mut request: tide::Request<State>) -> tide::Result {
    let request_body = request.body_bytes().await?;
    let state = request.state();
    let codec = &mut state.codec().await;
    let (ref mut decoder, ref mut encoder) = **codec;
    let inbound_message = decoder.decode(&request_body[..]).unwrap();
    let outbound_message = fix_json_to_tagvalue(state, encoder, inbound_message).unwrap();
    let bytes = outbound_message.to_vec();
    Ok(String::from_utf8(bytes).unwrap().into())
}

#[cfg(test)]
mod test {
    use super::*;
    use fefix::{tagvalue, Dictionary};
    use tide::http::{Method, Request, Response};

    const HOME_URL: &str = "http://localhost:8080/";
    const SAMPLE_MESSAGE: &str = r#"
        {
            "Header": {
                "BeginString": "FIX.4.2",
                "MsgType": "W",
                "MsgSeqNum": "4567",
                "SenderCompID": "SENDER",
                "TargetCompID": "TARGET",
                "SendingTime": "20160802-21:14:38.717"
            },
            "Body": {
                "SecurityIDSource": "8",
                "SecurityID": "ESU6",
                "MDReqID": "789",
                "NoMDEntries": [
                    { "MDEntryType": "0", "MDEntryPx": "1.50", "MDEntrySize": "75", "MDEntryTime": "21:14:38.688" },
                    { "MDEntryType": "1", "MDEntryPx": "1.75", "MDEntrySize": "25", "MDEntryTime": "21:14:38.688" }
                ]
            },
            "Trailer": {
            }
        }
    "#;

    async fn response_to(json: &str) -> tide::Result<Response> {
        let server = server();
        let mut request = Request::new(Method::Post, HOME_URL);
        request.set_body(json);
        server.respond(request).await
    }

    #[tokio::test]
    async fn logon() {
        let mut response = response_to(SAMPLE_MESSAGE).await.unwrap();
        assert_eq!(response.status(), 200);
        let bytes = response.body_bytes().await.unwrap();
        let fix_decoder = &mut tagvalue::Decoder::<tagvalue::Config>::new(Dictionary::fix42());
        let fix_message = fix_decoder.decode(&bytes[..]).unwrap();
        assert_eq!(fix_message.fv(fix42::BEGIN_STRING), Ok("FIX.4.2"));
        assert_eq!(fix_message.fv(fix42::MSG_TYPE), Ok("W"));
        assert_eq!(fix_message.fv(fix42::SENDER_COMP_ID), Ok("SENDER"));
        assert_eq!(fix_message.fv(fix42::TARGET_COMP_ID), Ok("TARGET"));
        assert_eq!(fix_message.fv(fix42::MSG_SEQ_NUM), Ok("4567"));
    }
}
