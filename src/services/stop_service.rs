use actix_web::{web};
use futures::TryStreamExt;
use crate::models::stop::Stop;
use crate::pagination::{PaginationBuilder, PaginationResponse};

use mongodb::{
   bson::{extjson::de::Error, oid::ObjectId},
   Collection
};

pub struct StopService {
   pub collection: web::Data<Collection<Stop>>
}




impl StopService {
   pub async fn find_all(&self, page: i64, size: i64, base_url: &str) -> Result<PaginationResponse<Stop>, mongodb::error::Error> {
      let total = self.collection.count_documents(None, None).await?;

      let skip = (page - 1) * size;
      
      let cursor = self.collection.find(
         None,
         mongodb::options::FindOptions::builder()
             .skip(Some(skip as u64))
             .limit(Some(size))
             .build()
      ).await?;

      let data: Vec<Stop> = cursor.try_collect().await?;

      let builder = PaginationBuilder::new(data, total, page, size, base_url.to_string());
      Ok(builder.build_response())
   }

   pub async fn find_by_id(&self, id: &str) -> Result<Option<Stop>, Error>{
      let obj_id = ObjectId::parse_str(id)?;



      let filter = mongodb::bson::doc! {"_id": obj_id};
      let stop = self.collection.find_one(filter, None)
          .await.ok()
          .expect("Error getting stop's detail");

      Ok(stop)
   }
}