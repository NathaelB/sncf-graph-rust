use actix_web::web;
use futures::TryStreamExt;
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
}