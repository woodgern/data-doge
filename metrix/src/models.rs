use super::schema::metrics;
use serde_json;
use chrono::naive::NaiveDateTime;
use diesel::sql_types::*;

#[derive(Queryable, QueryableByName)]
pub struct BucketResult {
    #[sql_type = "BigInt"]
    pub value: i64,
    #[sql_type = "Integer"]
    pub bucket_index: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Bucket {
    pub value: i64,
    pub bucket: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct Buckets {
    pub buckets: Vec<Bucket>,
}

#[derive(Serialize, Deserialize)]
pub struct BucketedData {
    pub data: Buckets,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName)]
#[table_name="metrics"]
pub struct Metric {
    pub id: i32,
    pub metric_name: String,
    pub data: serde_json::Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="metrics"]
pub struct NewMetric {
    pub metric_name: String,
    pub data: serde_json::Value,
}
