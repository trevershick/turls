pub mod model;
pub mod db;

extern crate serde;

#[derive(Debug)]
pub enum Error {
   DbError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            DbError(m) => write!(f, "{}", m),
        }
    }
}

impl From<sled::Error> for Error {
    fn from(e : sled::Error) -> Self {
        Error::DbError(e.to_string())
    }
}


// #[cfg(test)]
// mod tests {
//     use super::model::Shortened;
//     #[test]
//     fn it_works() {
//         let _s = Shortened {
//             id: 1,
//             keyword: "k".to_owned(),
//             url: "u".to_owned(),
//             title: "t".to_owned(),
//         };
//         assert_eq!(2 + 2, 4);
//     }
// }
