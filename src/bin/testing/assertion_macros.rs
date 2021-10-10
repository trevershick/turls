#[cfg(test)]
#[macro_export]
macro_rules! assert_ok {
    ( $x:expr ) => {
        assert_eq!($x.status(), rocket::http::Status::Ok);
    };
}
#[cfg(test)]
#[macro_export]
macro_rules! assert_notfound {
    ( $x:expr ) => {
        assert_eq!($x.status(), rocket::http::Status::NotFound);
    };
}
#[cfg(test)]
#[macro_export]
macro_rules! assert_json {
    ( $x:expr ) => {
        assert_eq!($x.content_type(), Some(rocket::http::ContentType::JSON));
    };
}
#[cfg(test)]
#[macro_export]
macro_rules! assert_json_eq {
    ( $response:expr, $expected:expr ) => {
        let s = $response.into_string().unwrap();
        let actual: Value = serde_json::from_str(&s).unwrap();
        assert_eq!($expected, actual);
    };
}
