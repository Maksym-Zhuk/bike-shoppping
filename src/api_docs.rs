use crate::models::product::Product;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(schemas(Product)),
    info(title = "Bike Shop API", version = "0.1.0")
)]
pub struct ApiDoc;
