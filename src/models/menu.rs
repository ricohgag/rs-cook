use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Menu {

    //主键    
    pub id: i64,

    //菜单名称    
    pub name: String,

    
    pub create_time: DateTime<Utc>,

    
    pub update_time: DateTime<Utc>,


}


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct MenuParam {

    //主键
    pub id: i64,

    //菜单名称
    pub name: String,


    pub create_time: DateTime<Utc>,


    pub update_time: DateTime<Utc>,


}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct MenuQueryParam {

    //主键
    pub id: Option<i64>,
    //菜单名称
    pub name: Option<String>,

    pub create_time: Option<DateTime<Utc>>,

    pub update_time: Option<DateTime<Utc>>,

}
