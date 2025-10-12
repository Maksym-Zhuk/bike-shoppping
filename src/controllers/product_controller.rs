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
    ),
    tag = "Products"
)]
pub async fn get_all_products(db: web::Data<Database>) -> impl Responder {
    match product::get_all_products(&db).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => {
            eprintln!("{}", err);

            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[utoipa::path(
    post,
    path = "/create_product",
    request_body = CreateProductDto,
    responses(
        (status = 201, description = "Product created successfully", body = Product),
        (status = 500, description = "Internal server error")
    ),
    tag = "Products"
)]
pub async fn create_product(
    db: web::Data<Database>,
    new_product_data: web::Json<CreateProductDto>,
) -> impl Responder {
    match product::create_product(&db, new_product_data).await {
        Ok(products) => HttpResponse::Created().json(products),
        Err(err) => {
            eprintln!("{}", err);

            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

#[utoipa::path(
    get,
    path = "/product/{id}",
    params(
        ("id" = String, Path, description = "Product ID")
    ),
    responses(
        (status = 200, body = Product),
        (status = 404, description = "Product not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Products"
)]
pub async fn get_product(db: web::Data<Database>, product_id: web::Path<String>) -> impl Responder {
    match product::get_product(&db, &product_id).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().body("Product not found"),
        Err(err) => {
            eprintln!("{}", err);

            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
