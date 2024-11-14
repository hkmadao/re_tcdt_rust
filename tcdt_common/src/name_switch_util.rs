/// pascal case to snake case
pub fn pascal_case_to_snake_case(pascal_case: &str) -> String {
    let mut result = String::with_capacity(pascal_case.len());
    let mut prev_char = '\0'; // 用于处理连续的大写字母
    for c in pascal_case.chars() {
        if c.is_ascii_alphabetic() && c.is_uppercase() && prev_char.is_lowercase() && prev_char != c
        {
            result.push('_');
        }
        result.push_str(&c.to_lowercase().to_string());
        prev_char = c;
    }
    result
}
/// pascal case to camel case
pub fn pascal_case_to_camel_case(pascal_case: &str) -> String {
    let mut result = String::new();
    let mut fg_init = false;
    for c in pascal_case.chars() {
        if fg_init {
            result.push(c);
        } else {
            fg_init = true;
            result.push_str(&c.to_lowercase().to_string());
        }
    }
    result
}
/// camel case to pascal case
pub fn camel_case_to_pascal_case(camel_case: &str) -> String {
    let mut result = String::new();
    let mut fg_init = false;
    for c in camel_case.chars() {
        if fg_init {
            result.push(c);
        } else {
            fg_init = true;
            result.push_str(&c.to_uppercase().to_string());
        }
    }
    result
}
/// snake case to macro case
pub fn snake_case_to_macro_case(snake_case: &str) -> String {
    snake_case.to_uppercase()
}
/// macro case to snake case
pub fn macro_case_to_snake_case(macro_case: &str) -> String {
    macro_case.to_lowercase()
}
/// snake case to camel case
pub fn snake_case_to_camel_case(snake_case: &str) -> String {
    let pascal_case_name = snake_case
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            if let Some(first_char) = chars.next() {
                first_char.to_uppercase().to_string()
            } else {
                "".to_owned()
            }
            .to_string()
                + &chars.as_str().to_lowercase()
        })
        .collect::<String>();
    pascal_case_to_camel_case(&pascal_case_name)
}


