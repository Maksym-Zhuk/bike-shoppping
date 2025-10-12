use crate::controllers::product_controller::{
    __path_create_product, __path_get_all_products, __path_get_product,
};
use crate::models::product::{CreateProductDto, Product};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_product, get_all_products, get_product),
    components(schemas(Product, CreateProductDto)),
    info(title = "Bike Shop API", version = "0.1.0")
)]
pub struct ApiDoc;
