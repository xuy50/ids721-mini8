use serde::{Deserialize, Serialize};
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3, GetObjectRequest};
use csv::ReaderBuilder;
use std::error::Error;
use tokio::io::AsyncReadExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Product")]
    product: String,
    #[serde(rename = "Price")]
    price: f64,
    #[serde(rename = "Quantity")]
    quantity: i32,
}

pub async fn price_filter_cli(low: f64, high: f64) -> Result<Vec<Record>, Box<dyn Error>> {
    let s3_client = S3Client::new(Region::UsEast2);
    let get_req = GetObjectRequest {
        bucket: "ids-721-data".to_string(),
        key: "dataset_sample.csv".to_string(),
        ..Default::default()
    };

    let result = s3_client.get_object(get_req).await.map_err(|e| e.to_string())?;
    let body = result.body.ok_or("Body is empty".to_string())?;
    let mut body_reader = body.into_async_read();
    let mut csv_content = Vec::new();
    body_reader.read_to_end(&mut csv_content).await.map_err(|e| e.to_string())?;

    let mut rdr = ReaderBuilder::new().from_reader(csv_content.as_slice());
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result.map_err(|e| e.to_string())?;
        if record.price >= low && record.price <= high {
            records.push(record);
        }
    }

    let json = serde_json::to_string(&records)?;
    println!("{}", json);
    Ok(records)
}

/// Filters records by their price, inclusive of the bounds.
pub fn filter_records(records: Vec<Record>, low: f64, high: f64) -> Vec<Record> {
    records.into_iter()
        .filter(|record| record.price >= low && record.price <= high)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_price_filter() {
        let records = vec![
            Record { date: "2023-09-01".into(), product: "Apple".into(), price: 1.2, quantity: 50 },
            Record { date: "2023-09-02".into(), product: "Apple".into(), price: 1.3, quantity: 45 },
            Record { date: "2023-09-03".into(), product: "Apple".into(), price: 1.1, quantity: 55 },
            Record { date: "2023-09-01".into(), product: "Banana".into(), price: 0.5, quantity: 40 },
            Record { date: "2023-09-01".into(), product: "Cherry".into(), price: 2.5, quantity: 20 },
        ];

        let filtered_records = filter_records(records, 1.0, 2.0);
        assert_eq!(filtered_records.len(), 3);
        assert!(filtered_records.iter().all(|r| r.product == "Apple"));
    }
}
