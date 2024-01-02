use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Food {

    //id    
    pub id: i64,

    
    pub name: String,

    
    pub type: Option<i32>,

    
    pub tag: Option<i32>,

    
    pub create_time: DateTime<Utc>,

    
    pub update_time: DateTime<Utc>,


}


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct FoodParam {

    //id
    pub id: i64,


    pub name: String,


    pub type: Option<i32>,


    pub tag: Option<i32>,


    pub create_time: DateTime<Utc>,


    pub update_time: DateTime<Utc>,


}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct FoodQueryParam {

    //id
    pub id: Option<i64>,

    pub name: Option<String>,

    pub type: Option<i32>,

    pub tag: Option<i32>,

    pub create_time: Option<DateTime<Utc>>,

    pub update_time: Option<DateTime<Utc>>,

}
