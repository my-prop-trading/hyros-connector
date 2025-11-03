use serde::Serialize;

#[derive(Serialize)]
pub struct CreateOrderRequest<'a> {
    #[serde(rename = "orderId")]
    pub order_id: &'a str,
    pub currency: &'a str,
    pub email: &'a str,
    #[serde(rename = "firstName")]
    pub first_name: &'a str,
    #[serde(rename = "lastName")]
    pub last_name: &'a str,
    pub items: Vec<CreateOrderItem<'a>>,
    pub stage: Option<&'a str>,
}

#[derive(Serialize)]
pub struct CreateOrderItem<'a> {
    pub name: &'a str,
    #[serde(rename = "externalId")]
    pub external_id: &'a str,
    pub price: f64,
    #[serde(rename = "itemDiscount")]
    pub item_discount: Option<f64>,
}
