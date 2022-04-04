use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use regex::Regex;
use uuid::Uuid;

use crate::{
    configuration::app_data_pool::AppDataPool,
    errors::{bad_request::BadRequest, internal_server_error::InternalServerError},
    persistence::user::model::user::User,
    routes::{
        convert_user_to_dto,
        user::dto::{
            create_user::CreateUser, update_password::UpdatePassword, update_user::UpdateUser,
        },
        EMAIL_REGEX_PATTERN,
    },
};

#[post("/")]
pub async fn create_user(
    create_user: web::Json<CreateUser>,
    pool: web::Data<AppDataPool>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_CREATE_USER").await;
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

    if create_user.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Username cannot be empty!"));
    }

    if create_user.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Password cannot be empty!"));
    }

    if create_user.email_address.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Email address cannot be empty!"));
    }

    let email_regex = Regex::new(EMAIL_REGEX_PATTERN).unwrap();
    if !email_regex.is_match(&create_user.email_address) {
        return HttpResponse::BadRequest().json(BadRequest::new("Invalid email address!"));
    }

    let optional = match pool
        .services
        .user_service
        .find_by_username(&pool.database, &create_user.username)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    if let Some(_d) = optional {
        return HttpResponse::BadRequest().json(BadRequest::new("Username is already taken!"));
    }

    let optional = match pool
        .services
        .user_service
        .find_by_email_address(&pool.database, &create_user.email_address)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    if let Some(_d) = optional {
        return HttpResponse::BadRequest().json(BadRequest::new("Email address is already taken!"));
    }

    // Check if the roles that are given actually exist
    for role in &create_user.roles {
        match pool
            .services
            .role_service
            .find_by_uuid(&pool.database, role)
            .await
        {
            Ok(d) => {
                if d.is_none() {
                    return HttpResponse::BadRequest()
                        .json(BadRequest::new(&format!("Invalid role {}", role)));
                }
            }
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        }
    }

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: String::from(&create_user.username),
        email_address: String::from(&create_user.email_address),
        password: hash(&create_user.password, DEFAULT_COST).unwrap(),
        first_name: String::from(&create_user.first_name),
        last_name: String::from(&create_user.last_name),
        enabled: true,
        roles: create_user.roles.clone(),
        created_at: Utc::now().to_string(),
        last_active: String::from(""),
    };

    let res = match pool
        .services
        .user_service
        .create(new_user, &pool.database)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    let user = match res {
        Some(d) => d,
        None => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new("Unable to create user!"))
        }
    };

    let user = match convert_user_to_dto(
        user,
        &pool.database,
        &pool.services.role_service,
        &pool.services.permission_service,
    )
    .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(user)
}

#[get("/")]
pub async fn find_all_users(pool: web::Data<AppDataPool>, req: HttpRequest) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_READ_USER").await;

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

    let users = match pool.services.user_service.find_all(&pool.database).await {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    if users.is_empty() {
        HttpResponse::NotFound().body("");
    }

    let mut user_dto = vec![];
    for user in users {
        let user = match convert_user_to_dto(
            user,
            &pool.database,
            &pool.services.role_service,
            &pool.services.permission_service,
        )
        .await
        {
            Ok(d) => d,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        };
        user_dto.push(user);
    }

    HttpResponse::Ok().json(user_dto)
}

#[get("/{uuid}")]
pub async fn find_by_uuid(
    pool: web::Data<AppDataPool>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_READ_USER").await;

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

    let user = match pool
        .services
        .user_service
        .find_by_uuid(&pool.database, &path)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    let user = match user {
        Some(d) => d,
        None => {
            return HttpResponse::NotFound().body("");
        }
    };

    let user = match convert_user_to_dto(
        user,
        &pool.database,
        &pool.services.role_service,
        &pool.services.permission_service,
    )
    .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()));
        }
    };

    HttpResponse::Ok().json(user)
}

#[put("/{uuid}")]
pub async fn update_by_uuid(
    update: web::Json<UpdateUser>,
    pool: web::Data<AppDataPool>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_UPDATE_USER").await;

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

    if path.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Invalid UUID"));
    }

    if update.username.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Username cannot be empty!"));
    }

    if update.email_address.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Email address cannot be empty!"));
    }

    let email_regex = Regex::new(EMAIL_REGEX_PATTERN).unwrap();
    if !email_regex.is_match(&update.email_address) {
        return HttpResponse::BadRequest().json(BadRequest::new("Invalid email address!"));
    }

    let mut old_user = match pool
        .services
        .user_service
        .find_by_uuid(&pool.database, &path)
        .await
    {
        Ok(d) => match d {
            Some(d) => d,
            None => return HttpResponse::NotFound().body(""),
        },
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    // Check if the roles that are given actually exist
    for role in &update.roles {
        match pool
            .services
            .role_service
            .find_by_uuid(&pool.database, role)
            .await
        {
            Ok(d) => {
                if d.is_none() {
                    return HttpResponse::BadRequest()
                        .json(BadRequest::new(&format!("Invalid role {}", role)));
                }
            }
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(InternalServerError::new(&e.to_string()));
            }
        }
    }

    let user_by_username = match pool
        .services
        .user_service
        .find_by_username(&pool.database, &update.username)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    if let Some(x) = user_by_username {
        if x.id != path.to_string() {
            return HttpResponse::BadRequest().json(BadRequest::new("Username is already taken!"));
        }
    }

    let user_by_email = match pool
        .services
        .user_service
        .find_by_email_address(&pool.database, &update.email_address)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    if let Some(x) = user_by_email {
        if x.id != path.to_string() {
            return HttpResponse::BadRequest()
                .json(BadRequest::new("Email address is already taken!"));
        }
    }

    old_user.username = update.username.clone();
    old_user.email_address = update.email_address.clone();
    old_user.first_name = update.first_name.clone();
    old_user.last_name = update.last_name.clone();
    old_user.enabled = update.enabled;
    old_user.roles = update.roles.clone();

    let user = match pool
        .services
        .user_service
        .update(&pool.database, &path, old_user)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    let user = match user {
        Some(d) => d,
        None => {
            return HttpResponse::NoContent().body("");
        }
    };

    match convert_user_to_dto(
        user,
        &pool.database,
        &pool.services.role_service,
        &pool.services.permission_service,
    )
    .await
    {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[put("/{uuid}/password")]
pub async fn update_password(
    update: web::Json<UpdatePassword>,
    pool: web::Data<AppDataPool>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_UPDATE_USER").await;
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

    if path.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Invalid UUID"));
    }

    if update.password.is_empty() {
        return HttpResponse::BadRequest().json(BadRequest::new("Password cannot be empty!"));
    }

    match pool
        .services
        .user_service
        .find_by_uuid(&pool.database, &path)
        .await
    {
        Ok(d) => {
            if d.is_none() {
                return HttpResponse::NotFound().body("");
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    let new_password = hash(&update.password, DEFAULT_COST).unwrap();

    let user = match pool
        .services
        .user_service
        .update_password(&pool.database, &path, &new_password)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(InternalServerError::new(&e.to_string()))
        }
    };

    let user = match user {
        Some(d) => d,
        None => {
            return HttpResponse::NoContent().body("");
        }
    };

    match convert_user_to_dto(
        user,
        &pool.database,
        &pool.services.role_service,
        &pool.services.permission_service,
    )
    .await
    {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()))
        }
    }
}

#[delete("/{uuid}")]
pub async fn delete_by_uuid(
    pool: web::Data<AppDataPool>,
    path: web::Path<String>,
    req: HttpRequest,
) -> HttpResponse {
    let token_res = crate::routes::check_user_permissions(&req, &pool, "CAN_DELETE_USER").await;
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

    if let Err(e) = pool
        .services
        .user_service
        .delete(&pool.database, &path)
        .await
    {
        return HttpResponse::InternalServerError().json(InternalServerError::new(&e.to_string()));
    };

    HttpResponse::Ok().body("")
}
