#[macro_export]
macro_rules! urlify {
    ($($component:expr),*) => {
        vec![$($component.to_string()),*]
    }
}


#[macro_export]
macro_rules! optional_query_pairs {
    ($($key: expr => $field_val: expr),+) => {
        {
        let mut params: Vec<(&str, String)> = Vec::new();

        $(
        if let Some(ref x) = $field_val {
            params.push(($key.into(), x.as_ref()));
        }
        )+

        params
        }
    }
}
