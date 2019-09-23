use jsonbox::{Client, Error};
use matches::*;
use mockito::mock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    message: String,
}

#[test]
fn test_create() {
    let _m = mock("POST", "/00000000000000000000")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"_id":"11111111111111111111","name":"rust","message":"jsonbox","_createdOn":"2019-09-22T12:24:37.513Z"}"#)
        .create();
    let client = Client::with_base_url("00000000000000000000", &mockito::server_url());
    let data = Data {
        name: "rust".into(),
        message: "jsonbox".into(),
    };
    let res = client.create(&data);
    assert!(res.is_ok());

    let (data, meta) = res.unwrap();
    assert_eq!(data.name, "rust");
    assert_eq!(data.message, "jsonbox");
    assert_eq!(meta.id, "11111111111111111111");
    assert_eq!(meta.created_on, "2019-09-22T12:24:37.513Z");
}

#[test]
fn test_read_all() {
    let _m = mock("GET", "/00000000000000000000")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"[{"_id":"11111111111111111111","name":"kuy","message":"Hello, Jsonbox!","_createdOn":"2019-09-22T12:24:37.513Z"},{"_id":"22222222222222222222","name":"github","message":"Hello, Rust!","_createdOn":"2019-09-21T12:24:37.513Z"}]"#)
        .create();
    let client = Client::with_base_url("00000000000000000000", &mockito::server_url());
    let res = client.read_all::<Data>();
    assert!(res.is_ok());

    let all = res.unwrap();
    assert_eq!(all.len(), 2);

    let data = all.first().unwrap();
    assert_eq!(data.name, "kuy");

    // TODO: check record id

    let data = all.last().unwrap();
    assert_eq!(data.name, "github");

    // TODO: check record id
}

#[test]
fn test_read_all_empty() {
    let _m = mock("GET", "/99999999999999999999")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body("[]")
        .create();
    let client = Client::with_base_url("99999999999999999999", &mockito::server_url());
    let res = client.read_all::<Data>();
    assert!(res.is_ok());

    let all = res.unwrap();
    assert_eq!(all.len(), 0);
}

#[test]
fn test_read() {
    let _m = mock("GET", "/00000000000000000000/11111111111111111111")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"_id":"11111111111111111111","name":"kuy","message":"Hello, Jsonbox!","_createdOn":"2019-09-22T12:24:37.513Z"}"#)
        .create();
    let client = Client::with_base_url("00000000000000000000", &mockito::server_url());
    let res = client.read::<Data>("11111111111111111111");
    assert!(res.is_ok());

    let (data, meta) = res.unwrap();
    assert_eq!(data.name, "kuy");
    assert_eq!(data.message, "Hello, Jsonbox!");
    assert_eq!(meta.id, "11111111111111111111");
    assert_eq!(meta.created_on, "2019-09-22T12:24:37.513Z");
}

#[test]
fn test_read_unknown_record_id() {
    let _m = mock("GET", "/00000000000000000000/11111111111111111111")
        .with_status(500)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"message":"Cannot read property '_id' of null"}"#)
        .create();
    let client = Client::with_base_url("00000000000000000000", &mockito::server_url());
    let res = client.read::<Data>("11111111111111111111");
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert_matches!(err, Error::General { code, message: _ } if code == 500);
}

#[test]
fn test_update() {
    let _m = mock("PUT", "/00000000000000000000/33333333333333333333")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"message":"Record updated."}"#)
        .create();
    let client = Client::with_base_url("00000000000000000000", &mockito::server_url());
    let data = Data {
        name: "cargo".into(),
        message: "update".into(),
    };
    let res = client.update("33333333333333333333", &data);
    assert!(res.is_ok());
}

#[test]
fn test_update_unknown_record_id() {
    let _m = mock("PUT", "/00000000000000000000/11111111111111111111")
        .with_status(400)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"message":"Invalid record Id"}"#)
        .create();
    let client = Client::with_base_url("00000000000000000000", &mockito::server_url());
    let data = Data {
        name: "crates".into(),
        message: "io".into(),
    };
    let res = client.update("11111111111111111111", &data);
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert_matches!(err, Error::General { code, message: _ } if code == 400);
}

#[test]
fn test_delete() {
    let _m = mock("DELETE", "/00000000000000000000/22222222222222222222")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"message":"Record removed."}"#)
        .create();
    let client = Client::with_base_url("00000000000000000000", &mockito::server_url());
    let res = client.delete("22222222222222222222");
    assert!(res.is_ok());
}

#[test]
fn test_delete_unknown_record_id() {
    let _m = mock("DELETE", "/00000000000000000000/44444444444444444444")
        .with_status(400)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"message":"Invalid record Id"}"#)
        .create();
    let client = Client::with_base_url("00000000000000000000", &mockito::server_url());
    let res = client.delete("44444444444444444444");
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert_matches!(err, Error::General { code, message: _ } if code == 400);
}
