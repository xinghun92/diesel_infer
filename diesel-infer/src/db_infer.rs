pub trait InferDBFields {
    fn get_fields() -> Vec<String>;
    fn get_table_name() -> &'static str;
    fn get_values(&self) -> Vec<String>;
}

pub fn get_i32_sql(input: i32) -> String {
    input.to_string()
}

pub fn get_i64_sql(input: i64) -> String {
    input.to_string()
}

pub fn get_bool_sql(input: bool) -> String {
    (input as i32).to_string()
}

pub fn get_vec_sql(input: &[u8]) -> String {
    let values: Vec<String> = input.iter()
            .map(|v| format!("{:02x}", v))
            .collect();
    let sql = values.join("");
    format!(r#"x'{}'"#, sql)
}

pub fn get_str_sql(input: &str) -> String {
    format!(r#"'{}'"#, input)
}
