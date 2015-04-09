#[macro_export]
macro_rules! param_pairs {
    ($($param: expr),+) => {
        {
        let mut params: Vec<(&str, String)> = Vec::new();

        $(
        if let Some(ref param) = $param {
            params.push((param.get_name(), param.get_value()));
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
macro_rules! impl_query_param {
    ($param_type: ty , $name:expr, { |$re:ident| $str_expr:expr }) => {
        impl QueryParam for $param_type {
            fn get_name(&self) -> &'static str { $name }
            fn get_value(&self) -> String { let $re = self; $str_expr }
        }
    }
}
