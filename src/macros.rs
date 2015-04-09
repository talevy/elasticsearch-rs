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

#[macro_export]
macro_rules! field_setters {
    ($c: ty , $(($field: ident, $t: ty)),+) => {
        $(
        pub fn $field(&'a mut self, $field: $t) -> &'a mut $c {
            self.$field = Some($field);
            self
        }
        )+
    }
}

#[macro_export]
macro_rules! param_push {
    ($params:ident, $key:ident => $field_val:expr , { |$re:ident| $expr:expr }) => {
        if let Some(ref $re) = $field_val {
            $params.push(($key.into(), $expr));
        }
    }
}
