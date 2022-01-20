use aws_config::meta::region::RegionProviderChain;
use lambda_runtime::{ handler_fn, Error };
use log::{debug, info};
use serde::{Serialize, Deserialize};
// use serde_json::{ json, Value };
use aws_sdk_dynamodb::model::{
     AttributeValue
};
use aws_sdk_dynamodb::{ Client };
//use std::process;

#[derive(Deserialize)]
struct Request {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl std::error::Error for FailureResponse{}

type Response = Result<SuccessResponse, FailureResponse>;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt::init();
    env_logger::init();
    debug!("logger is set up");

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn add_item(
    client: &Client,
    table: &str,
    username: &str,
    p_type: &str,
) -> Result<(), Error> {
    let user_av = AttributeValue::S(username.into());
    let type_av = AttributeValue::S(p_type.into());

    let request = client
        .put_item()
        .table_name(table)
        .item("justanotherkey", user_av);

    request.send().await?;

    println!(
        "Added value {username}"
    );

    Ok(())
}

async fn  handler(req: Request, _ctx: lambda_runtime::Context) -> Response {
    info!("handle the request");

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    add_item(&client, "newtable", "justanotherkey", "str");

    Ok(SuccessResponse {
        body: format!(
            "the lambda has landed"
        ),
    })
}
