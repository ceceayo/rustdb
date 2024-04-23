use ceayo_db::loader;

#[test]
fn test_load_string() {
    //assert_eq!(load_string("dGVzdA=="), "test");
    assert_eq!(loader::deserialize_string("dGVzdA==".to_string()), "test");
}