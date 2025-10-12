use crate::controllers::order_controller::__path_create_order;
use crate::controllers::product_controller::{
    __path_create_product, __path_get_all_products, __path_get_product,
};
use crate::models::order::Order;
use crate::models::product::{CreateProductDto, Product};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_product, get_all_products, get_product, create_order),
    components(schemas(Product, CreateProductDto, Order)),
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
