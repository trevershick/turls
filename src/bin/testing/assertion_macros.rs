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
