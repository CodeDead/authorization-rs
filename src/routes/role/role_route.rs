use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    configuration::app_data_pool::AppDataPool,
    errors::{bad_request::BadRequest, internal_server_error::InternalServerError},
    persistence::role::model::role::Role,
    routes::{
        convert_role_to_dto,
        role::dto::{create_role::CreateRole, update_role::UpdateRole}, get_user_uuid_from_token,
    },
};

#[post("/")]
pub async fn create_new_role(
    create: web::Json<CreateRole>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_CREATE_ROLE").await;

    match token_res {
        Ok(d) => {
            if !d {
                return HttpResponse::Forbidden().body("");
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let id = match get_user_uuid_from_token(&req, &pool) {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let res = pool
        .services
        .user_service
        .update_last_active(&pool.database, &id, &Utc::now().to_string())
        .await;

    if let Err(e) = res {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    }

    if create.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Name cannot be empty!"));
    }

    match pool
        .services
        .role_service
        .find_by_name(&pool.database, &create.name)
        .await
    {
        Ok(d) => {
            if d.is_some() {
                return HttpResponse::BadRequest().json(BadRequest::new(&format!(
                    "Role with name {} already exists!",
                    &create.name
                )));
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    for permission in &create.permissions {
        match pool
            .services
            .permission_service
            .find_by_uuid(&pool.database, permission)
            .await
        {
            Ok(d) => {
                if d.is_none() {
                    return HttpResponse::BadRequest().json(BadRequest::new(&format!(
                        "Invalid permission {}",
                        permission
                    )));
                }
            }
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };
    }

    let new_role = Role {
        id: Uuid::new_v4().to_string(),
        name: create.name.clone(),
        description: create.description.clone(),
        permissions: create.permissions.clone(),
    };

    let res = match pool
        .services
        .role_service
        .create(new_role, &pool.database)
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
                .json(InternalServerError::new("Unable to create role!"))
        }
    };

    match convert_role_to_dto(res, &pool.database, &pool.services.permission_service).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[get("/")]
pub async fn get_all_roles(req: HttpRequest, pool: web::Data<AppDataPool>) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_READ_ROLE").await;

    match token_res {
        Ok(d) => {
            if !d {
                return HttpResponse::Forbidden().body("");
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let id = match get_user_uuid_from_token(&req, &pool) {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let res = pool
        .services
        .user_service
        .update_last_active(&pool.database, &id, &Utc::now().to_string())
        .await;

    if let Err(e) = res {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    }

    let roles = match pool.services.role_service.find_all(&pool.database).await {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let mut role_dto = vec![];
    for role in roles {
        match convert_role_to_dto(role, &pool.database, &pool.services.permission_service).await {
            Ok(d) => {
                role_dto.push(d);
            }
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        }
    }

    HttpResponse::Ok().json(role_dto)
}

#[get("/{uuid}")]
pub async fn get_role_by_id(
    path: web::Path<String>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_READ_ROLE").await;

    match token_res {
        Ok(d) => {
            if !d {
                return HttpResponse::Forbidden().body("");
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let id = match get_user_uuid_from_token(&req, &pool) {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let res = pool
        .services
        .user_service
        .update_last_active(&pool.database, &id, &Utc::now().to_string())
        .await;

    if let Err(e) = res {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    }

    let res = match pool
        .services
        .role_service
        .find_by_uuid(&pool.database, &path)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let role = match res {
        Some(d) => d,
        None => {
            return HttpResponse::NotFound().body("");
        }
    };

    match convert_role_to_dto(role, &pool.database, &pool.services.permission_service).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[put("/{uuid}")]
pub async fn update_role(
    path: web::Path<String>,
    update: web::Json<UpdateRole>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_UPDATE_ROLE").await;

    match token_res {
        Ok(d) => {
            if !d {
                return HttpResponse::Forbidden().body("");
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let id = match get_user_uuid_from_token(&req, &pool) {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let res = pool
        .services
        .user_service
        .update_last_active(&pool.database, &id, &Utc::now().to_string())
        .await;

    if let Err(e) = res {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    }

    if update.name.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Name cannot be empty!"));
    }

    let mut res = match pool
        .services
        .role_service
        .find_by_uuid(&pool.database, &path)
        .await
    {
        Ok(d) => match d {
            Some(x) => x,
            None => return HttpResponse::NotFound().body(""),
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    match pool
        .services
        .role_service
        .find_by_name(&pool.database, &update.name)
        .await
    {
        Ok(d) => {
            if let Some(x) = d {
                if x.id != res.id {
                    return HttpResponse::BadRequest().json(BadRequest::new(&format!(
                        "Role with name {} already exists!",
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

    for permission in &update.permissions {
        let p = pool
            .services
            .permission_service
            .find_by_uuid(&pool.database, permission)
            .await;

        if let Err(e) = p {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    }

    res.name = update.name.clone();
    res.description = update.description.clone();
    res.permissions = update.permissions.clone();

    let res = match pool
        .services
        .role_service
        .update(&pool.database, &path, res)
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

    match convert_role_to_dto(res, &pool.database, &pool.services.permission_service).await {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[delete("/{uuid}")]
pub async fn delete_role(
    path: web::Path<String>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_DELETE_ROLE").await;

    match token_res {
        Ok(d) => {
            if !d {
                return HttpResponse::Forbidden().body("");
            }
        }
        Err(_) => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let id = match get_user_uuid_from_token(&req, &pool) {
        Some(d) => d,
        None => {
            return HttpResponse::Unauthorized().body("");
        }
    };

    let res = pool
        .services
        .user_service
        .update_last_active(&pool.database, &id, &Utc::now().to_string())
        .await;

    if let Err(e) = res {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    }

    if let Err(e) = pool
        .services
        .user_service
        .delete_role(&pool.database, &path)
        .await
    {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    }

    if let Err(e) = pool
        .services
        .role_service
        .delete(&pool.database, &path)
        .await
    {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    };

    HttpResponse::Ok().body("")
}
