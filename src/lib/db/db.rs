use crate::model::{Shortened};
use crate::Error;
use zerocopy::AsBytes;
use sled;
        use sled::Transactional;

const TREE_URLS: &str = "urls";
const TREE_KEYWORDS: &str = "keywords";

pub struct Db {
    db: sled::Db,
}

// for use with figment
pub struct Config {
    pub path: String,
    pub temporary: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: "turlsdata".to_owned(),
            temporary: false,
        }
    }
}

impl Db {
    pub fn init(config: &Config) -> Result<Db, Error> {
        let db = sled::Config::new()
            .path(config.path.clone())
            .temporary(config.temporary)
            .open()?;
        Ok(Db { db })
    }

    pub fn find_url_by_keyword(self: &Self, keyword: &str) -> Result<Option<String>, Error> {
        self.db.open_tree(TREE_KEYWORDS)?
            .get(keyword.as_bytes())
            .map(|o| o.map(|b| String::from_utf8_lossy(&b).to_string()))
            .map_err( Error::from)
    }

    pub fn find_url(self: &Self, id: u64) -> Result<Option<Shortened>, Error> {
        let dbid = Shortened::db_id(id);
        self.db.open_tree(TREE_URLS)?
            .get(dbid.as_bytes())
            .map(|o| o.map(Shortened::from))
            .map_err( Error::from)
    }

    pub fn delete_url(self: &Self, id: u64) -> Result<bool, Error> {

        let dbid = Shortened::db_id(id);
        let urls = self.db.open_tree(TREE_URLS)?;
        let keywords = self.db.open_tree(TREE_KEYWORDS)?;

        (&urls,&keywords).transaction(|(u,k)| {
            let record = u.get(dbid.as_bytes()).map(|o| o.map(Shortened::from))?;
            if record.is_none() {
                return Ok(false);
            }
            let record = record.unwrap();
            k.remove(record.keyword.as_bytes())?;
            u.remove(dbid.as_bytes())?;
            Ok(true)
        }).map_err(|e: sled::transaction::TransactionError| Error::DbError(format!("{:?}", e))).map(|_|true)
    }

    pub fn insert_url(
        self: &Self,
        keyword: &str,
        url: &str,
        title: Option<&str>,
    ) -> Result<Shortened, Error> {
        let new_shortened = Shortened {
            id: self.db.generate_id().unwrap(),
            keyword: keyword.to_owned(),
            url: url.to_owned(),
            title: title.map(ToOwned::to_owned),
        };
        let dbid = Shortened::db_id(new_shortened.id);

        let urls = self.db.open_tree(TREE_URLS)?;
        let keywords = self.db.open_tree(TREE_KEYWORDS)?;

        (&urls,&keywords).transaction(|(u,k)| {
            let result = u.insert( dbid.as_bytes(), new_shortened.to_ivec())?;
            k.remove( new_shortened.keyword.as_bytes())?;
            k.insert( new_shortened.keyword.as_bytes(), new_shortened.url.as_bytes())?;
            Ok(result)
        }).map_err(|e: sled::transaction::TransactionError| Error::DbError(format!("{:?}", e)))?;
        Ok(new_shortened)
    }
}
