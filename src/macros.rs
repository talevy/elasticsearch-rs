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

#[macro_export]
macro_rules! new_query_struct {
    ( $c:ident ( $($req_field:ident : $req_type:ty),* ) { fn_path => | $path_x:ident | $fn_path:block , query_params
        => [ $(($opt_field:ident : $opt_type:ty , $opt_default:expr)),* ] , body =>
        $body:ident , method => $method:ident } ) => {

            #[derive(Debug, Clone, PartialEq)]
            pub struct $c<'a> {
                connection: &'a Connection,
                $( $req_field: $req_type,)*
                $( $opt_field: Option<$opt_type>,)*
            }

            impl<'a> $c<'a> {
                pub fn new(connection: &'a Connection, $($req_field: $req_type),*) -> $c<'a> {
                    $c {
                        connection: connection,
                        $( $req_field: $req_field,)*
                        $( $opt_field: $opt_default,)*
                    }
                }

                field_setters!{ $c, $( ($opt_field, $opt_type)),* }

                pub fn get_path(&$path_x) -> Vec<String> $fn_path

                pub fn execute(&self) -> HttpResult<String> {
                    let params: Vec<(&str, String)> = param_pairs! {
                        $(self.$opt_field),*
                    };
                    let body: Option<&[u8]> = None;
                    let body_string: String = String::new();

                    if "$body" == "None" {
                        self.connection.request($method, self.get_path(), params, None)
                    } else {
                        let bod: String = json::encode(&self.$body).ok().expect(":(");
                        self.connection.request($method, self.get_path(), params, Some(bod.as_bytes()))
                    }
                }
            }
    }
}
