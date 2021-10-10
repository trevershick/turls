use super::json;

type FieldName = String;

pub enum Error {
    ValueIsRequired(FieldName),
    ValueTooShort(FieldName, usize, String, usize),
    ValueTooLong(FieldName, usize, String, usize),
    InvalidShortenedCreate(json::ShortenedCreate, Vec<Error>),
}

pub trait Validated {
    fn validate(&self) -> Result<(), Error>;
}

impl Validated for json::ShortenedCreate {
    fn validate(self: &json::ShortenedCreate) -> Result<(), Error> {
        let mut errors = Vec::<Error>::new();
        Option::check_required(&self.keyword, "keyword", &mut errors);
        Option::check_required(&self.url, "url", &mut errors);
        Option::check_size(&self.url, "url", 2, 255, &mut errors);
        Option::check_size(&self.keyword, "keyword", 2, 25, &mut errors);
        match errors.len() {
            0 => Ok(()),
            _ => Err(Error::InvalidShortenedCreate(self.clone(), errors)),
        }
    }
}

trait RequiredCheck {
    fn check_required(value: &Self, field: &str, errors: &mut Vec<Error>);
}

impl<T> RequiredCheck for Option<T> {
    fn check_required(value: &Self, field: &str, errors: &mut Vec<Error>) {
        use Error::*;
        if value.is_none() {
            errors.push(ValueIsRequired(field.to_owned()))
        }
    }
}

trait BoundsCheck {
    fn check_size(value: &Self, field: &str, min: usize, max: usize, errors: &mut Vec<Error>);
}

impl<T> BoundsCheck for Option<T>
where
    T: ToString,
{
    fn check_size(value: &Self, field: &str, min: usize, max: usize, errors: &mut Vec<Error>) {
        use Error::*;
        if value.is_none() {
            return;
        }
        let s = value.as_ref().unwrap().to_string();
        let sz = s.len();
        match (sz < min, sz > max) {
            (true, false) => errors.push(ValueTooShort(field.to_owned(), min, s, sz)),
            (false, true) => errors.push(ValueTooLong(field.to_owned(), max, s, sz)),
            _ => (),
        };
    }
}
