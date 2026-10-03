use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use hmac::{Hmac, KeyInit, Mac};
use line_bot_messaging_api::Error;
use line_bot_messaging_api::webhook::{self, Event, parse_request, verify_signature};
use sha2::Sha256;

const SECRET: &str = "channel-secret";

fn sign(secret: &str, body: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);
    STANDARD.encode(mac.finalize().into_bytes())
}

const BODY: &[u8] = br#"{"destination":"Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx","events":[{"type":"unfollow","mode":"active","timestamp":1462629479859,"source":{"type":"user","userId":"U4af4980629"},"webhookEventId":"01FZ74A0TDDPYRVKNK77XKC3ZR","deliveryContext":{"isRedelivery":false}}]}"#;

#[test]
fn verify_signature_accepts_valid_signature() {
    verify_signature(SECRET, BODY, &sign(SECRET, BODY)).unwrap();
}

#[test]
fn verify_signature_rejects_wrong_secret() {
    let err = verify_signature(SECRET, BODY, &sign("other-secret", BODY)).unwrap_err();
    assert!(matches!(err, Error::InvalidSignature));
}

#[test]
fn verify_signature_rejects_modified_body() {
    let signature = sign(SECRET, BODY);
    let err =
        verify_signature(SECRET, b"{\"destination\":\"U\",\"events\":[]}", &signature).unwrap_err();
    assert!(matches!(err, Error::InvalidSignature));
}

#[test]
fn verify_signature_rejects_non_base64_header() {
    let err = verify_signature(SECRET, BODY, "not base64!!").unwrap_err();
    assert!(matches!(err, Error::InvalidSignature));
}

#[test]
fn verify_signature_rejects_truncated_signature() {
    let full = sign(SECRET, BODY);
    let bytes = STANDARD.decode(&full).unwrap();
    let truncated = STANDARD.encode(&bytes[..16]);
    let err = verify_signature(SECRET, BODY, &truncated).unwrap_err();
    assert!(matches!(err, Error::InvalidSignature));
}

#[test]
fn parse_request_decodes_verified_body() {
    let request = parse_request(SECRET, BODY, &sign(SECRET, BODY)).unwrap();
    assert_eq!(request.destination, "Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    assert_eq!(request.events.len(), 1);
    assert!(matches!(request.events[0], Event::Unfollow(_)));
    assert_eq!(
        request.events[0].source().and_then(|s| s.user_id()),
        Some("U4af4980629")
    );
}

#[test]
fn parse_request_rejects_bad_signature() {
    let err = parse_request(SECRET, BODY, &sign("other-secret", BODY)).unwrap_err();
    assert!(matches!(err, Error::InvalidSignature));
}

#[test]
fn parse_request_reports_invalid_json() {
    let body = b"{\"destination\":\"U\"";
    let err = parse_request(SECRET, body, &sign(SECRET, body)).unwrap_err();
    match err {
        Error::Decode { body: raw, .. } => assert_eq!(raw, "{\"destination\":\"U\""),
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn parse_request_accepts_empty_events() {
    let body = br#"{"destination":"Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx","events":[]}"#;
    let request: webhook::CallbackRequest =
        parse_request(SECRET, body, &sign(SECRET, body)).unwrap();
    assert!(request.events.is_empty());
}
