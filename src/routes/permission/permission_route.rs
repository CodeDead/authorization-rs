use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::{
    configuration::app_data_pool::AppDataPool,
    errors::{bad_request::BadRequest, internal_server_error::InternalServerError},
    persistence::permission::model::permission::Permission,
    routes::{
        convert_permission_to_dto,
        permission::dto::{
            create_permission::CreatePermission, update_permission::UpdatePermission,
        },
    },
};

#[post("/")]
pub async fn create_permission(
    create: web::Json<CreatePermission>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    if !crate::routes::check_user_permissions(&req, &pool, "CAN_CREATE_PERMISSION").await {
        return HttpResponse::Unauthorized().body("");
    }

    if create.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Name cannot be empty!"));
    }

    match pool
        .services
        .permission_service
        .find_by_name(&pool.database, &create.name)
        .await
    {
        Ok(d) => {
            if let Some(x) = d {
                return HttpResponse::BadRequest().json(BadRequest::new(&format!(
                    "Permission with name {} already exists!",
                    x.name
                )));
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let new_permission = Permission {
        id: Uuid::new_v4().to_string(),
        name: create.name.clone(),
        description: create.description.clone(),
    };

    let res = match pool
        .services
        .permission_service
        .create(new_permission, &pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let res = match res {
        Some(d) => d,
        None => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Unable to create permission!"))
        }
    };

    HttpResponse::Ok().json(convert_permission_to_dto(res))
}

#[get("/")]
pub async fn get_all_permissions(pool: web::Data<AppDataPool>, req: HttpRequest) -> HttpResponse {
    if !crate::routes::check_user_permissions(&req, &pool, "CAN_READ_PERMISSION").await {
        return HttpResponse::Unauthorized().body("");
    }

    let res = match pool
        .services
        .permission_service
        .find_all(&pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let mut permissions = vec![];
    for permisison in res {
        permissions.push(convert_permission_to_dto(permisison));
    }

    HttpResponse::Ok().json(permissions)
}

#[get("/{uuid}")]
pub async fn find_by_uuid(
    path: web::Path<String>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    if !crate::routes::check_user_permissions(&req, &pool, "CAN_READ_PERMISSION").await {
        return HttpResponse::Unauthorized().body("");
    }

    let res = match pool
        .services
        .permission_service
        .find_by_uuid(&pool.database, &path)
        .await
    {
        Ok(d) => match d {
            Some(d) => d,
            None => return HttpResponse::NotFound().body(""),
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(convert_permission_to_dto(res))
}

#[put("/{uuid}")]
pub async fn update_permission(
    pool: web::Data<AppDataPool>,
    update: web::Json<UpdatePermission>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    if !crate::routes::check_user_permissions(&req, &pool, "CAN_UPDATE_PERMISSION").await {
        return HttpResponse::Unauthorized().body("");
    }

    if update.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Name cannot be empty!"));
    }

    let mut old_permission = match pool
        .services
        .permission_service
        .find_by_uuid(&pool.database, &path)
        .await
    {
        Ok(d) => match d {
            Some(d) => d,
            None => return HttpResponse::NotFound().body(""),
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    match pool
        .services
        .permission_service
        .find_by_name(&pool.database, &update.name)
        .await
    {
        Ok(d) => {
            if let Some(x) = d {
                if x.id != old_permission.id {
                    return HttpResponse::BadRequest().json(BadRequest::new(&format!(
                        "Permission with name {} already exists!",
                        update.name
                    )));
                }
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    old_permission.name = update.name.clone();
    old_permission.description = update.description.clone();

    let res = match pool
        .services
        .permission_service
        .update(&pool.database, &path, old_permission)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let res = match res {
        Some(d) => d,
        None => return HttpResponse::NoContent().body(""),
    };

    HttpResponse::Ok().json(convert_permission_to_dto(res))
}

#[delete("/{uuid}")]
pub async fn delete_permission(
    path: web::Path<String>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    if !crate::routes::check_user_permissions(&req, &pool, "CAN_DELETE_PERMISSION").await {
        return HttpResponse::Unauthorized().body("");
    }

    let mut roles_with_permission = match pool
        .services
        .role_service
        .find_by_permission_id(&pool.database, &path)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    for role in &mut roles_with_permission {
        role.permissions.retain(|x| *x != path.to_string());

        let response = pool
            .services
            .role_service
            .update(&pool.database, &role.id, role.clone())
            .await;
        if let Err(e) = response {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    }

    if let Err(e) = pool
        .services
        .permission_service
        .delete(&pool.database, &path)
        .await
    {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    };

    HttpResponse::Ok().body("")
}
