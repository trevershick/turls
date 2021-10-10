mod config;

use crate::model::{IdAndUrl, Shortened};
use crate::Error;
use sled::Transactional;
use zerocopy::AsBytes;

// reexport config
pub use config::Config;

const TREE_URLS: &str = "urls";
const TREE_KEYWORDS: &str = "keywords";

pub struct Db {
    db: sled::Db,
    config: config::Config,
}

impl Db {
    pub fn config(self: &Self) -> &config::Config {
        &self.config
    }
    pub fn init(config: &config::Config) -> Result<Db, Error> {
        let db = sled::Config::new()
            .path(config.path.clone())
            .temporary(config.temporary)
            .open()?;
        Ok(Db { db, config: config.clone() })
    }

    fn find_idandurl_by_keyword(self: &Self, keyword: &str) -> Result<IdAndUrl, Error> {
        let record = self.db.open_tree(TREE_KEYWORDS)?.get(keyword.as_bytes())?;
        match record {
            Some(iv) => Ok(IdAndUrl::from_ivec(iv)),
            None => Err(Error::KeywordDoesNotExist(keyword.to_owned())),
        }
    }

    pub fn find_url_by_keyword(self: &Self, keyword: &str) -> Result<String, Error> {
        Ok(self.find_idandurl_by_keyword(keyword)?.url)
    }

    pub fn find_url(self: &Self, id: u64) -> Result<Shortened, Error> {
        let dbid = Shortened::db_id(id);
        let record = self.db.open_tree(TREE_URLS)?.get(dbid.as_bytes())?;
        match record {
            Some(iv) => Ok(Shortened::from_ivec(iv)),
            None => Err(Error::UrlDoesNotExist(id)),
        }
    }

    pub fn delete_url(self: &Self, id: u64) -> Result<bool, Error> {
        let dbid = Shortened::db_id(id);
        let urls = self.db.open_tree(TREE_URLS)?;
        let keywords = self.db.open_tree(TREE_KEYWORDS)?;

        (&urls, &keywords)
            .transaction(|(u, k)| {
                let record = u
                    .get(dbid.as_bytes())
                    .map(|o| o.map(Shortened::from_ivec))?;
                if record.is_none() {
                    return Ok(false);
                }
                let record = record.unwrap();
                k.remove(record.keyword.as_bytes())?;
                u.remove(dbid.as_bytes())?;
                Ok(true)
            })
            .map_err(|e: sled::transaction::TransactionError| Error::DbError(format!("{:?}", e)))
            .map(|_| true)
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
        let new_idkv = IdAndUrl {
            id: new_shortened.id,
            url: url.to_owned(),
        };
        let keywords = self.db.open_tree(TREE_KEYWORDS)?;
        if keywords
            .compare_and_swap(
                keyword.as_bytes(),
                None as Option<sled::IVec>,
                Some(new_idkv.to_ivec()),
            )?
            .is_err()
        {
            return Err(Error::KeywordAlreadyExists(new_shortened.keyword));
        }

        let dbid = Shortened::db_id(new_shortened.id);
        let _old_value = self
            .db
            .open_tree(TREE_URLS)?
            .insert(dbid.as_bytes(), new_shortened.to_ivec())?;
        Ok(new_shortened)
    }
}
