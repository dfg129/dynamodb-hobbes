use lambda_runtime::{handler_fn, Context, Error};
use std::env;
use log::{debug, error, log_enabled, info, Level};
use serde_json::{json, Value};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType
};
use aws_sdk_dynamodb::{Client, Region, PKG_VERSION};
use std::process;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
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


async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    println!("-----   Region secured  -----");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    println!("-----   Client -----");

    let resp = client.list_tables().send().await?;

    println!("-----   Tables -----");

    let names= resp.table_names().unwrap_or_default();
    let len = names.len();

    for name in names {
        println!(" --  {} -- ", name);
    }

    let response = format!("**** found some tables ***");
    log::info!("{}", response);

    println!("and now to create a table");
    create_table(&client, "newtable2", "justanotherkey").await;

    println!("--- table up ---");
    Ok(json!({"response": response}))
}
