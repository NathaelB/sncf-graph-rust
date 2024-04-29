use actix_web::web;
use futures::TryStreamExt;
use mongodb::bson::{Bson, doc};
use mongodb::Collection;
use mongodb::error::Error;
use crate::models::stop_time::StopTime;

pub struct StopTimeService {
    pub collection: web::Data<Collection<StopTime>>
}

impl StopTimeService {
    pub async fn find_all(&self) -> Result<Vec<StopTime>, Error> {
        let cursor = self.collection.find(None, None).await?;

        let data: Vec<StopTime> = cursor.try_collect().await?;

        Ok(data)
    }

    pub async fn find_by_stop_ids(&self, stop_ids: &Vec<String>) -> Result<Vec<StopTime>, Error> {
        //let stop_ids_bson: Vec<Bson> = stop_ids.iter().map(Bson::String).collect();

        let stop_ids_bson: Vec<Bson> = stop_ids.iter().map(|id| Bson::String(id.to_string())).collect();

        let filter = doc! { "stop_id": { "$in": stop_ids_bson } };
        let cursor = self.collection.find(filter, None).await?;

        let data: Vec<StopTime> = cursor.try_collect().await?;
        Ok(data)
    }
}