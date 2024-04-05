use actix_web::web;
use futures::TryStreamExt;
use mongodb::Collection;
use crate::models::stop::Stop;
use crate::pagination::{PaginationBuilder, PaginationResponse};

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
}