use actix_web::{web, HttpResponse, Result};
use validator::Validate;

use crate::{
    dto::product::{CreateProductDto, UpdateProductDto},
    errors::{AppErrors, ErrorResponse},
    models::{app::AppState, product::Product},
    services::product_service,
};

#[utoipa::path(
    get,
    path = "/product/products",
    responses(
        (status = 200, description = "List of all products", body = [Product]),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Products"
)]
pub async fn get_all_products(state: web::Data<AppState>) -> Result<HttpResponse, AppErrors> {
    let products = product_service::get_all_products(&state.mongo).await?;
    Ok(HttpResponse::Ok().json(products))
}

#[utoipa::path(
    post,
    path = "/product/admin/create",
    request_body = CreateProductDto,
    responses(
        (status = 201, description = "Product created successfully", body = ErrorResponse, example = json!({
            "message": "Product created successfully"
        })),
        (status = 400, description = "Validation failed", body = ErrorResponse, example = json!({
            "error": "validation_error",
            "message": "Validation failed"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 403, description = "Not enough rights", body = ErrorResponse,
            example = json!({
                "error": "insufficient_permissions",
                "message": "Necessary role: Admin"
            })
        ),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Products",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn create_product(
    state: web::Data<AppState>,
    new_product_data: web::Json<CreateProductDto>,
) -> Result<HttpResponse, AppErrors> {
    if let Err(e) = new_product_data.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "validation_error",
            "message": format!("Validation failed: {:?}", e)
        })));
    }

    let answer =
        product_service::create_product(&state.mongo, state.redis.clone(), new_product_data)
            .await?;
    Ok(HttpResponse::Created().json(serde_json::json!({
        "message": answer
    })))
}

#[utoipa::path(
    get,
    path = "/product/{id}",
    params(
        ("id" = String, Path, description = "Product ID (UUID format)")
    ),
    responses(
        (status = 200, description = "Product found", body = Product),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse, example = json!({
            "error": "invalid_uuid",
            "message": "Invalid UUID format",
            "details": "The UUID must be in the following format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
        })),
        (status = 404, description = "Product not found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "Product not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Products"
)]
pub async fn get_product(
    state: web::Data<AppState>,
    product_id: web::Path<String>,
) -> Result<HttpResponse, AppErrors> {
    let product = product_service::get_product(&state.mongo, &product_id).await?;
    Ok(HttpResponse::Ok().json(product))
}

#[utoipa::path(
    put,
    path = "/product/admin/update",
    request_body = UpdateProductDto,
    responses(
        (status = 200, description = "Product updated successfully", body = ErrorResponse, example = json!({
            "message": "Product updated successfully"
        })),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse, example = json!({
            "error": "invalid_uuid",
            "message": "Invalid UUID format"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 403, description = "Not enough rights", body = ErrorResponse,
            example = json!({
                "error": "insufficient_permissions",
                "message": "Necessary role: Admin"
            })
        ),
        (status = 404, description = "Product not found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "Product not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Products",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn update_product(
    state: web::Data<AppState>,
    new_product_data: web::Json<UpdateProductDto>,
) -> Result<HttpResponse, AppErrors> {
    let answer =
        product_service::update_product(&state.mongo, state.redis.clone(), new_product_data)
            .await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": answer
    })))
}

#[utoipa::path(
    delete,
    path = "/product/admin/delete/{id}",
    params(
        ("id" = String, Path, description = "Product ID (UUID format)")
    ),
    responses(
        (status = 200, description = "Product deleted successfully", body = ErrorResponse, example = json!({
            "message": "Product deleted successfully"
        })),
        (status = 400, description = "Invalid UUID format", body = ErrorResponse, example = json!({
            "error": "invalid_uuid",
            "message": "Invalid UUID format"
        })),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!({
            "error": "jwt_error",
            "message": "Authorization error"
        })),
        (status = 403, description = "Not enough rights", body = ErrorResponse,
            example = json!({
                "error": "insufficient_permissions",
                "message": "Necessary role: Admin"
            })
        ),
        (status = 404, description = "Product not found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "Product not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Products",
    security(
        ("bearer_auth" = [])  
    )
)]
pub async fn delete_product(
    state: web::Data<AppState>,
    product_id: web::Path<String>,
) -> Result<HttpResponse, AppErrors> {
    let answer =
        product_service::delete_product(&state.mongo, state.redis.clone(), &product_id).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": answer
    })))
}

#[utoipa::path(
    get,
    path = "/product/most_advantageous",
    responses(
        (status = 200, description = "Most advantageous product", body = Product),
        (status = 404, description = "No products found", body = ErrorResponse, example = json!({
            "error": "not_found",
            "message": "Product not found"
        })),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!({
            "error": "database_error",
            "message": "Database error"
        }))
    ),
    tag = "Products"
)]
pub async fn get_most_advantageous(state: web::Data<AppState>) -> Result<HttpResponse, AppErrors> {
    let product = product_service::get_most_advantageous(&state.mongo, state.redis.clone()).await?;
    Ok(HttpResponse::Ok().json(product))
}
