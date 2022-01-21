use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType
};
use aws_sdk_dynamodb::{Client};
use std::process;
use tracing::info;

#[derive(Deserialize, Debug)]
struct Request {
    table_name: String,
    key_name: String,
}

#[derive(Serialize, Debug)]
struct Response {
    req_id: String,
    msg: String,
}

async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    info!("[handler-fn] : event is {:?}", event);
    let table_name = event.payload.table_name;
    let key_name = event.payload.key_name;

    create_table(&client, &table_name, &key_name).await?;

    let resp = Response {
        req_id: "1".to_string(), //event.table_name,
        msg: format!("table  {} created.", table_name),
    };

    Ok(resp)
}

async fn create_table(client: &Client, table: &str, key: &str) -> Result<(), Error> {
    let a_name: String = key.into();
    let table_name: String = table.into();

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    match client
        .create_table()
        .table_name(table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await
    {
        Ok(_) => println!("Added table with key {key}"),
        Err(e) => {
            println!("Got an error creating table");
            println!("{e}");
            process::exit(1);
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .without_time()
        .init();

    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
