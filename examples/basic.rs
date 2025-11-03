use hyros_connector::{
    client::{HyrosApiClient, API_URL_PROD},
    model::{CreateOrderItem, CreateOrderRequest},
};

#[tokio::main]
async fn main() {
    // let api_key = std::env::var("HYROS_API_KEY").unwrap();
    let client = HyrosApiClient::new("API_e5d31e85ad1c9b722d777ca9e2174718bbc67c56ff168c5fb2afc974eaf04b0b", API_URL_PROD);

    client
        .create_order(&CreateOrderRequest {
            order_id: "test-order-id",
            currency: "USD",
            email: "hyros-test-123435@mailinator.com",
            first_name: "test-first-name",
            last_name: "test-last-name",
            items: vec![CreateOrderItem {
                name: "test",
                external_id: "test-id",
                price: 0.0,
                item_discount: Some(1.0),
            }],
            stage: None,
        })
        .await
        .unwrap();

    println!("Run basic example: FINISHED");
}
