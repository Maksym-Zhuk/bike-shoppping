use crate::{
    models::product::{CreateProductDto, Product},
    services::product,
};
use actix_web::{HttpResponse, Responder, web};
use mongodb::Database;

#[utoipa::path(
    get,
    path = "/products",
    request_body = [Product],
    responses(
        (status = 200, body = [Product]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_all_products(db: web::Data<Database>) -> impl Responder {
    match product::get_all_products(&db).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/create_product",
    request_body = CreateProductDto,
    responses(
        (status = 201, description = "Product created successfully", body = Product),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_product(
    db: web::Data<Database>,
    new_product_data: web::Json<CreateProductDto>,
) -> impl Responder {
    match product::create_product(&db, new_product_data).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
