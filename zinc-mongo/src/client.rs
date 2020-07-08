//!
//! The Zinc Mongo asynchronous client.
//!

use bson::Bson;
use bson::Document as BsonDocument;

use crate::error::Error;
use crate::storage::Storage;

///
/// The MongoDB asynchronous client adapter.
///
#[derive(Clone)]
pub struct Client {
    /// The wrapped inner client instance.
    inner: mongodb::Client,
}

impl Client {
    ///
    /// Initializes a client instance.
    ///
    pub async fn new(host: String, port: u16) -> Self {
        let address = format!("{}://{}:{}", zinc_const::mongodb::PROTOCOL, host, port,);

        let inner = mongodb::Client::with_uri_str(address.as_str())
            .await
            .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

        Self { inner }
    }

    ///
    /// Returns a contract storage data from the collection
    ///
    pub async fn get_storage(&self, name: &str) -> Result<Storage, Error> {
        let collection = self
            .inner
            .database(zinc_const::mongodb::DATABASE)
            .collection(name);

        let record = collection
            .find_one(bson::doc! {}, None)
            .await?
            .ok_or(Error::RecordNotFound)?;
        let storage: Storage =
            bson::from_bson(Bson::Document(record)).expect(zinc_const::panic::DATA_SERIALIZATION);

        Ok(storage)
    }

    ///
    /// Returns a contract storage data from the collection
    ///
    pub async fn update_storage(&self, name: &str, storage: Storage) -> Result<(), Error> {
        let collection = self
            .inner
            .database(zinc_const::mongodb::DATABASE)
            .collection(name);

        collection
            .update_one(
                bson::doc! {},
                mongodb::options::UpdateModifications::Document(bson::doc! {
                    "storage": storage.data,
                }),
                None,
            )
            .await?;

        Ok(())
    }

    ///
    /// Removes all the records from the `name` collection and writes the `record` thereto.
    ///
    pub async fn rewrite_collection(&self, name: &str, record: BsonDocument) -> Result<(), Error> {
        let collection = self
            .inner
            .database(zinc_const::mongodb::DATABASE)
            .collection(name);

        collection.delete_many(BsonDocument::new(), None).await?;
        collection.insert_one(record, None).await?;

        Ok(())
    }

    ///
    /// Drops the `name` collection.
    ///
    pub async fn drop_collection(&self, name: &str) -> Result<(), Error> {
        Ok(self
            .inner
            .database(zinc_const::mongodb::DATABASE)
            .collection(name)
            .drop(None)
            .await?)
    }
}
