use bil_canonical::{encode_canonical, BilValue};

#[test]
fn test_encode_null() {
    let value = BilValue::Null;
    let bytes = encode_canonical(&value).unwrap();
    assert_eq!(bytes, vec![0xf6]);
}

#[test]
fn test_encode_bool() {
    let value_true = BilValue::Bool(true);
    let bytes_true = encode_canonical(&value_true).unwrap();
    assert_eq!(bytes_true, vec![0xf5]);

    let value_false = BilValue::Bool(false);
    let bytes_false = encode_canonical(&value_false).unwrap();
    assert_eq!(bytes_false, vec![0xf4]);
}

#[test]
fn test_encode_integer() {
    let value_0 = BilValue::Integer(0);
    assert_eq!(encode_canonical(&value_0).unwrap(), vec![0x00]);

    let value_1 = BilValue::Integer(1);
    assert_eq!(encode_canonical(&value_1).unwrap(), vec![0x01]);

    let value_10 = BilValue::Integer(10);
    assert_eq!(encode_canonical(&value_10).unwrap(), vec![0x0a]);

    let value_23 = BilValue::Integer(23);
    assert_eq!(encode_canonical(&value_23).unwrap(), vec![0x17]);

    let value_24 = BilValue::Integer(24);
    assert_eq!(encode_canonical(&value_24).unwrap(), vec![0x18, 0x18]);

    let value_neg_1 = BilValue::Integer(-1);
    assert_eq!(encode_canonical(&value_neg_1).unwrap(), vec![0x20]);

    let value_neg_10 = BilValue::Integer(-10);
    assert_eq!(encode_canonical(&value_neg_10).unwrap(), vec![0x29]);
}

#[test]
fn test_encode_text() {
    let value = BilValue::Text("".to_string());
    assert_eq!(encode_canonical(&value).unwrap(), vec![0x60]);

    let value = BilValue::Text("a".to_string());
    assert_eq!(encode_canonical(&value).unwrap(), vec![0x61, 0x61]);

    let value = BilValue::Text("IETF".to_string());
    assert_eq!(
        encode_canonical(&value).unwrap(),
        vec![0x64, 0x49, 0x45, 0x54, 0x46]
    );
}

#[test]
fn test_encode_array() {
    let value = BilValue::Array(vec![]);
    assert_eq!(encode_canonical(&value).unwrap(), vec![0x80]);

    let value = BilValue::Array(vec![
        BilValue::Integer(1),
        BilValue::Integer(2),
        BilValue::Integer(3),
    ]);
    assert_eq!(
        encode_canonical(&value).unwrap(),
        vec![0x83, 0x01, 0x02, 0x03]
    );
}

#[test]
fn test_hash256_base64() {
    let hash = bil_canonical::Hash256([1; 32]);
    let b64 = hash.to_base64();
    assert_eq!(b64, "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE=");

    let decoded = bil_canonical::Hash256::from_base64(&b64).unwrap();
    assert_eq!(hash, decoded);
}

#[test]
fn test_hash256_hex() {
    let hash = bil_canonical::Hash256([1; 32]);
    let hex = hash.to_hex();
    assert_eq!(
        hex,
        "0101010101010101010101010101010101010101010101010101010101010101"
    );

    let decoded = bil_canonical::Hash256::from_hex(&hex).unwrap();
    assert_eq!(hash, decoded);
}

#[test]
fn test_commitment_hash() {
    use bil_canonical::BilCanonical;

    struct TestStruct;
    impl BilCanonical for TestStruct {
        fn to_canonical_value(&self) -> Result<BilValue, bil_canonical::CanonicalError> {
            Ok(BilValue::Text("test".to_string()))
        }
    }

    let ts = TestStruct;
    let hash = ts.commitment_hash().unwrap();
    // "test" -> 0x64 0x74 0x65 0x73 0x74
    // sha256(0x6474657374) = 6fe3180f700090697285ac1e0e8dc400259373d7bb94f0b1a9b086e7ba22dc3d
    assert_eq!(
        hash.to_hex(),
        "6fe3180f700090697285ac1e0e8dc400259373d7bb94f0b1a9b086e7ba22dc3d"
    );
}

#[test]
fn test_normalize_timestamp() {
    let ms = 1672531200000; // 2023-01-01T00:00:00Z
    let val = BilValue::normalize_timestamp(ms);
    assert_eq!(val, BilValue::Integer(1672531200000));
}

#[test]
fn test_json_to_bil_value() {
    use serde_json::json;

    let j = json!({
        "name": "Alice",
        "age": 30,
        "is_active": true,
        "tags": ["admin", "user"],
        "metadata": null
    });

    let bil_val = BilValue::try_from(&j).unwrap();

    let expected = BilValue::Map(vec![
        (
            BilValue::Text("name".to_string()),
            BilValue::Text("Alice".to_string()),
        ),
        (BilValue::Text("age".to_string()), BilValue::Integer(30)),
        (
            BilValue::Text("is_active".to_string()),
            BilValue::Bool(true),
        ),
        (
            BilValue::Text("tags".to_string()),
            BilValue::Array(vec![
                BilValue::Text("admin".to_string()),
                BilValue::Text("user".to_string()),
            ]),
        ),
        (BilValue::Text("metadata".to_string()), BilValue::Null),
    ]);

    // Order of map entries from JSON object is not guaranteed, so we can't just assert_eq!
    // But we can encode both and compare the canonical bytes
    let bytes1 = encode_canonical(&bil_val).unwrap();
    let bytes2 = encode_canonical(&expected).unwrap();
    assert_eq!(bytes1, bytes2);
}

#[test]
fn test_json_float_rejection() {
    use serde_json::json;
    let j = json!({"value": 3.14});
    let res = BilValue::try_from(&j);
    assert!(res.is_err());
}

#[test]
fn test_encode_map_sorting() {
    // Map keys should be sorted by their CBOR byte representation
    let value = BilValue::Map(vec![
        (BilValue::Text("b".to_string()), BilValue::Integer(2)),
        (BilValue::Text("a".to_string()), BilValue::Integer(1)),
    ]);

    // "a" is 0x61 0x61, "b" is 0x61 0x62
    // So "a" should come first
    let bytes = encode_canonical(&value).unwrap();
    assert_eq!(
        bytes,
        vec![
            0xa2, // map(2)
            0x61, 0x61, // "a"
            0x01, // 1
            0x61, 0x62, // "b"
            0x02, // 2
        ]
    );
}
