use bincode::{deserialize,serialize};
use serde::{Deserialize, Serialize};
use sled::IVec;
use {
    byteorder::BigEndian,
    zerocopy::{AsBytes, FromBytes, Unaligned, U64 },
};



#[warn(dead_code)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Shortened {
    pub id: u64, /* key */
    pub keyword: String,
    pub url: String,
    pub title: Option<String>,
}

impl Shortened {
    pub fn db_id(id: u64) -> U64<BigEndian> {
        id.into()
    }

    pub fn to_ivec(self: &Self) -> IVec {
        //let entity: Result<Shortened,_> = deserialize(&data[..]);
        //return entity.expect("should have deserialized");
        return serialize(self).expect("should have serialized").into();
    }
}
impl std::convert::From<IVec> for Shortened {
    fn from(data: IVec) -> Shortened {
        let entity: Result<Shortened,_> = deserialize(&data[..]);
        return entity.expect("should have deserialized");
    }
}
//let encoded: Vec<u8> = bincode::serialize(&entity).unwrap();
//let decoded: Entity = bincode::deserialize(&encoded[..]).unwrap();
#[derive(FromBytes, AsBytes, Unaligned)]
#[repr(C)]
pub struct UrlId {
    t: U64<BigEndian>,
    id: U64<BigEndian>,
}
