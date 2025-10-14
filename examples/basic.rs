use hyros_connector::{
    client::{HyrosApiClient, API_URL_PROD},
    model::CreateOrderRequest,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("HYROS_API_KEY").unwrap();
    let client = HyrosApiClient::new(&api_key, API_URL_PROD);

    client
        .create_order(&CreateOrderRequest {
            order_id: "test-order-id",
            currency: "USD",
            email: "hyros-test-123435@mailinator.com",
            first_name: "test-first-name",
            last_name: "test-last-name",
            items: vec![],
            stage: None,
        })
        .await
        .unwrap();

    println!("Run basic example: FINISHED");
}
