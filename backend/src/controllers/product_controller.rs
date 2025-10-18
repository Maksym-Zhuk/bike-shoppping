use actix_web::{HttpResponse, Responder, web};
use validator::Validate;

use crate::{
    AppState,
    models::{
        product::{CreateProductDto, Product, UpdateProductDto},
        res::{ErrorResponse, MessageResponse},
    },
    services::product_service,
};

#[utoipa::path(
    get,
    path = "/product/products",
    responses(
        (status = 200, description = "List of all products", body = [Product]),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn get_all_products(state: web::Data<AppState>) -> impl Responder {
    match product_service::get_all_products(&state.mongo).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[utoipa::path(
    post,
    path = "/product/create",
    request_body = CreateProductDto,
    responses(
        (status = 201, description = "Product created successfully", body = MessageResponse),
        (status = 400, description = "Validation failed", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn create_product(
    state: web::Data<AppState>,
    new_product_data: web::Json<CreateProductDto>,
) -> impl Responder {
    if let Err(e) = new_product_data.validate() {
        return HttpResponse::BadRequest().json(MessageResponse {
            message: format!("Validation failed: {:?}", e),
        });
    }

    match product_service::create_product(&state.mongo, new_product_data).await {
        Ok(answer) => HttpResponse::Created().json(MessageResponse { message: answer }),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
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
        (status = 200, description = "Product found", body = Product),
        (status = 404, description = "Product not found", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn get_product(
    state: web::Data<AppState>,
    product_id: web::Path<String>,
) -> impl Responder {
    match product_service::get_product(&state.mongo, &product_id).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Product not found".to_string(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[utoipa::path(
    put,
    path = "/product/update",
    request_body = UpdateProductDto,
    responses(
        (status = 200, description = "Product updated successfully", body = MessageResponse),
        (status = 404, description = "Product not found", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn update_product(
    state: web::Data<AppState>,
    new_product_data: web::Json<UpdateProductDto>,
) -> impl Responder {
    match product_service::update_product(&state.mongo, new_product_data).await {
        Ok(Some(answer)) => HttpResponse::Ok().json(MessageResponse { message: answer }),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Product not found".to_string(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[utoipa::path(
    delete,
    path = "/product/delete/{id}",
    params(
        ("id" = String, Path, description = "Product ID")
    ),
    responses(
        (status = 200, description = "Product deleted successfully", body = MessageResponse),
        (status = 404, description = "Product not found", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn delete_product(
    state: web::Data<AppState>,
    product_id: web::Path<String>,
) -> impl Responder {
    match product_service::delete_product(&state.mongo, &product_id).await {
        Ok(Some(answer)) => HttpResponse::Ok().json(MessageResponse { message: answer }),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Product not found".to_string(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}

#[utoipa::path(
    get,
    path = "/product/most_advantageous",
    responses(
        (status = 200, body = Product),
        (status = 404, description = "Product not found", body = MessageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn get_most_advantageous(state: web::Data<AppState>) -> impl Responder {
    match product_service::get_most_advantageous(&state.mongo, state.redis.clone()).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().json(MessageResponse {
            message: "Product not found".to_string(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            HttpResponse::InternalServerError().json(ErrorResponse {
                message: "Internal server error".to_string(),
            })
        }
    }
}
