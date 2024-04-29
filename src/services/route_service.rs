use actix_web::web;
use crate::models::route::Route;
use mongodb::{
    bson::{oid::ObjectId, doc},
    Collection
};

pub struct RouteService {
    pub collection: web::Data<Collection<Route>>
}

impl RouteService {
    pub async fn create(&self, route: Route) -> Result<(), mongodb::error::Error> {
        self.collection.insert_one(route, None).await?;
        Ok(())
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Route>, mongodb::error::Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();

        let filter = doc! { "_id": obj_id };
        let route = self.collection.find_one(filter, None).await?;
        Ok(route)
    }
}