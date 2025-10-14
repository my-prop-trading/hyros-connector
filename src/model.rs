use serde::Serialize;

#[derive(Serialize)]
pub struct CreateOrderRequest<'a> {
    pub order_id: &'a str,
    pub currency: &'a str,
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub items: Vec<CreateOrderItem<'a>>,
    pub stage: Option<&'a str>
}

#[derive(Serialize)]
pub struct CreateOrderItem<'a> {
    pub name: &'a str,
    pub external_id: &'a str,
    pub price: f64,
    pub item_discount: Option<f64>,
}
