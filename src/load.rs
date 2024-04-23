use crate::load;


#[test]
fn test_load_string() {
    //assert_eq!(load_string("dGVzdA=="), "test");
    assert_eq!(load::load_string("dGVzdA==".to_string()), "test");
}