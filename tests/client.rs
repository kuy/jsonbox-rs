use jsonbox::{Client, Error};
use matches::*;
use mockito::mock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    count: i32,
}

#[test]
fn test_create() {
    let _m = mock("POST", "/00000000000000000000")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"_id":"11111111111111111111","name":"rust","count":42,"_createdOn":"2019-09-22T12:24:37.513Z"}"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let data = Data {
        name: "rust".into(),
        count: 42,
    };
    let res = client.create(&data);
    assert!(res.is_ok());

    let (data, meta) = res.unwrap();
    assert_eq!(data.name, "rust");
    assert_eq!(data.count, 42);
    assert_eq!(meta.id, "11111111111111111111");
    assert_eq!(meta.created_on, "2019-09-22T12:24:37.513Z");
    assert_eq!(meta.updated_on, "2019-09-22T12:24:37.513Z");
}

#[test]
fn test_create_bulk() {
    let _m = mock("POST", "/00000000000000000000")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"[{"_id":"11111111111111111111","name":"rust","count":42,"_createdOn":"2019-09-22T12:24:37.513Z"},{"_id":"22222222222222222222","name":"cargo","count":7,"_createdOn":"2019-09-22T12:24:37.513Z"}]"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let data = vec![
        Data {
            name: "rust".into(),
            count: 42,
        },
        Data {
            name: "cargo".into(),
            count: 7,
        },
    ];
    let res = client.create_bulk(&data);
    assert!(res.is_ok());

    let bulk = res.unwrap();
    assert_eq!(bulk.len(), 2);

    let (data, meta) = bulk.first().unwrap();
    assert_eq!(data.name, "rust");
    assert_eq!(meta.id, "11111111111111111111");

    let (data, meta) = bulk.last().unwrap();
    assert_eq!(data.name, "cargo");
    assert_eq!(meta.id, "22222222222222222222");
}

#[test]
fn test_read_all() {
    let _m = mock("GET", "/00000000000000000000")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"[{"_id":"11111111111111111111","name":"kuy","count":42,"_createdOn":"2019-09-23T12:24:37.513Z"},{"_id":"22222222222222222222","name":"github","count":7,"_createdOn":"2019-09-22T12:24:37.513Z"}]"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let res = client.read().all::<Data>();
    assert!(res.is_ok());

    let all = res.unwrap();
    assert_eq!(all.len(), 2);

    let (data, meta) = all.first().unwrap();
    assert_eq!(data.name, "kuy");
    assert_eq!(meta.id, "11111111111111111111");

    let (data, meta) = all.last().unwrap();
    assert_eq!(data.name, "github");
    assert_eq!(meta.id, "22222222222222222222");
}

#[test]
fn test_read_all_empty() {
    let _m = mock("GET", "/99999999999999999999")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body("[]")
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("99999999999999999999").with_base_url(&server_url);
    let res = client.read().all::<Data>();
    assert!(res.is_ok());

    let all = res.unwrap();
    assert_eq!(all.len(), 0);
}

#[test]
fn test_read_limit() {
    let _m = mock("GET", "/00000000000000000000?sort=-_createdOn&skip=0&limit=1")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"[{"_id":"11111111111111111111","name":"kuy","count":42,"_createdOn":"2019-09-23T12:24:37.513Z"}]"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let res = client.read().limit(1).run::<Data>();
    assert!(res.is_ok());

    let all = res.unwrap();
    assert_eq!(all.len(), 1);

    let (data, meta) = all.first().unwrap();
    assert_eq!(data.name, "kuy");
    assert_eq!(meta.id, "11111111111111111111");
}

#[test]
fn test_read_skip() {
    let _m = mock("GET", "/00000000000000000000?sort=-_createdOn&skip=1&limit=20")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"[{"_id":"22222222222222222222","name":"github","count":7,"_createdOn":"2019-09-22T12:24:37.513Z"}]"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let res = client.read().skip(1).run::<Data>();
    assert!(res.is_ok());

    let all = res.unwrap();
    assert_eq!(all.len(), 1);

    let (data, meta) = all.first().unwrap();
    assert_eq!(data.name, "github");
    assert_eq!(meta.id, "22222222222222222222");
}

#[test]
fn test_read_sort() {
    let _m = mock("GET", "/00000000000000000000?sort=count&skip=0&limit=20")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"[{"_id":"22222222222222222222","name":"github","count":7,"_createdOn":"2019-09-22T12:24:37.513Z"},{"_id":"11111111111111111111","name":"kuy","count":42,"_createdOn":"2019-09-23T12:24:37.513Z"}]"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let res = client.read().order_by("count").run::<Data>();
    assert!(res.is_ok());

    let all = res.unwrap();
    assert_eq!(all.len(), 2);

    let (data, meta) = all.first().unwrap();
    assert_eq!(data.name, "github");
    assert_eq!(data.count, 7);
    assert_eq!(meta.id, "22222222222222222222");

    let (data, meta) = all.last().unwrap();
    assert_eq!(data.name, "kuy");
    assert_eq!(data.count, 42);
    assert_eq!(meta.id, "11111111111111111111");
}

#[test]
fn test_read() {
    let _m = mock("GET", "/00000000000000000000/11111111111111111111")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"_id":"11111111111111111111","name":"kuy","count":42,"_createdOn":"2019-09-22T12:24:37.513Z"}"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let res = client.read().id::<Data>("11111111111111111111");
    assert!(res.is_ok());

    let (data, meta) = res.unwrap();
    assert_eq!(data.name, "kuy");
    assert_eq!(data.count, 42);
    assert_eq!(meta.id, "11111111111111111111");
    assert_eq!(meta.created_on, "2019-09-22T12:24:37.513Z");
    assert_eq!(meta.updated_on, "2019-09-22T12:24:37.513Z");
}

#[test]
fn test_read_unknown_record_id() {
    let _m = mock("GET", "/00000000000000000000/11111111111111111111")
        .with_status(500)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"message":"Cannot read property '_id' of null"}"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let res = client.read().id::<Data>("11111111111111111111");
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
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let data = Data {
        name: "cargo".into(),
        count: 42,
    };
    let res = client.update("33333333333333333333", &data);
    assert!(res.is_ok());

    let _m = mock("GET", "/00000000000000000000/33333333333333333333")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"_id":"33333333333333333333","name":"cargo","count":42,"_createdOn":"2019-09-22T12:24:37.513Z","_updatedOn":"2019-09-22T12:25:52.114Z"}"#)
        .create();

    let res = client.read().id::<Data>("33333333333333333333");
    assert!(res.is_ok());

    let (data, meta) = res.unwrap();
    assert_eq!(data.name, "cargo");
    assert_eq!(data.count, 42);
    assert_eq!(meta.id, "33333333333333333333");
    assert_eq!(meta.created_on, "2019-09-22T12:24:37.513Z");
    assert_eq!(meta.updated_on, "2019-09-22T12:25:52.114Z");
}

#[test]
fn test_update_unknown_record_id() {
    let _m = mock("PUT", "/00000000000000000000/11111111111111111111")
        .with_status(400)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(r#"{"message":"Invalid record Id"}"#)
        .create();
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let data = Data {
        name: "crates".into(),
        count: 42,
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
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
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
    let server_url = mockito::server_url();
    let client = Client::new("00000000000000000000").with_base_url(&server_url);
    let res = client.delete("44444444444444444444");
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert_matches!(err, Error::General { code, message: _ } if code == 400);
}
