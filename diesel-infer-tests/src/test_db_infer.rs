use diesel_infer::InferDBFields;

#[test]
fn test_db_infer() {
    #[derive(InferDBFields)]
    #[table_name = "db_infer_tests"]
    pub struct Test<'a> {
        pub id: i64,
        pub name: &'a str,
        pub type_: i32,
        pub avatar_key: &'a str,
        pub update_time: i64,
        pub name_pinyin: &'a str,
        pub creator_id: i64,
        pub is_resigned: bool,
        pub is_registered: bool
    }

    let test = Test {
        id: 1,
        name: "name",
        type_: 2,
        avatar_key: "key",
        update_time: 100,
        name_pinyin: "name pinyin",
        creator_id: 200,
        is_resigned: true,
        is_registered: false,
    };

    assert_eq!(vec!["id".to_string(),
                    "name".to_string(),
                    "type_".to_string(),
                    "avatar_key".to_string(),
                    "update_time".to_string(),
                    "name_pinyin".to_string(),
                    "creator_id".to_string(),
                    "is_resigned".to_string(),
                    "is_registered".to_string()],
               Test::get_fields());
    assert_eq!("db_infer_tests", Test::get_table_name());
    assert_eq!(vec!["1".to_string(),
                    "\'name\'".to_string(),
                    "2".to_string(),
                    "\'key\'".to_string(),
                    "100".to_string(),
                    "\'name pinyin\'".to_string(),
                    "200".to_string(),
                    "1".to_string(),
                    "0".to_string()],
               test.get_values());
}
