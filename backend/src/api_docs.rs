use crate::controllers::order_controller::{
    __path_create_order, __path_delete_order, __path_get_all_orders, __path_get_order,
    __path_update_order,
};
use crate::controllers::product_controller::{
    __path_create_product, __path_delete_product, __path_get_all_products,
    __path_get_most_advantageous, __path_get_product, __path_update_product,
};
use crate::dto::order::{CreateOrderDto, UpdateOrderDto};
use crate::dto::product::{CreateProductDto, UpdateProductDto};
use crate::models::order::Order;
use crate::models::product::Product;
use crate::models::res::{ErrorResponse, MessageResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_product, get_all_products, get_most_advantageous, get_product, update_product, delete_product, create_order, get_all_orders, get_order, update_order, delete_order),
    components(schemas(Product, CreateProductDto, UpdateProductDto, Order, CreateOrderDto, UpdateOrderDto, MessageResponse, ErrorResponse)),
    info(title = "Bike Shop API", version = "0.1.0"),
    servers(
        (url = "/api", description = "API prefix")
    ),
    tags(
        (name = "Products", description = "Product management endpoints"),
        (name = "Orders", description = "Order management endpoints")
    )
)]
pub struct ApiDoc;
