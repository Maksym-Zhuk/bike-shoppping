use crate::controllers::auth_controller::{__path_login, __path_refresh_token, __path_register};
use crate::controllers::order_controller::{
    __path_create_order, __path_delete_order, __path_get_all_orders, __path_get_order,
    __path_update_order,
};
use crate::controllers::product_controller::{
    __path_create_product, __path_delete_product, __path_get_all_products,
    __path_get_most_advantageous, __path_get_product, __path_update_product,
};
use crate::controllers::user_controller::__path_me;
use crate::dto::auth::{AuthResponse, LoginDto, RefreshTokenRequest, RegisterDto, UserInfo};
use crate::dto::order::{CreateOrderDto, UpdateOrderDto};
use crate::dto::product::{CreateProductDto, UpdateProductDto};
use crate::errors::ErrorResponse;
use crate::models::order::Order;
use crate::models::product::Product;
use crate::models::res::MessageResponse;
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(
        create_product, 
        get_all_products, 
        get_most_advantageous, 
        get_product, 
        update_product, 
        delete_product, 
        create_order, 
        get_all_orders, 
        get_order, 
        update_order, 
        delete_order, 
        register, 
        login, 
        refresh_token, 
        me
    ),
    components(
        schemas(
            Product, 
            CreateProductDto, 
            UpdateProductDto, 
            Order, 
            CreateOrderDto, 
            UpdateOrderDto, 
            MessageResponse, 
            ErrorResponse, 
            UserInfo, 
            LoginDto, 
            RegisterDto, 
            AuthResponse, 
            RefreshTokenRequest
        )
    ),
    info(title = "Bike Shop API", version = "0.1.0"),
    servers(
        (url = "/api", description = "API prefix")
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Products", description = "Product management endpoints"),
        (name = "Orders", description = "Order management endpoints"),
        (name = "Auth", description = "Auth management endpoints"),
        (name = "Users", description = "Users management endpoints")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
