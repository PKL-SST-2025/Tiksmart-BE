// src/api/mod.rs

use crate::{middleware::{auth_guard::auth_guard, csrf_guard::csrf_guard}, AppState}; // <-- IMPORTANT: Make sure AppState is imported
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
// We don't need to import PgPool here anymore, as it's part of AppState
// use sqlx::PgPool; 

// Declare your handler modules
pub mod auth_handler;
pub mod csrf_handler;
pub mod member_handler;
pub mod project_handler;
pub mod role_handler;
pub mod subtask_handler;
pub mod task_handler;
pub mod user_handler; 

/// Assembles a router for all API-specific endpoints.
// Change the return type from Router<PgPool> to Router<AppState>
pub fn api_router() -> Router<AppState> {
    // --- Public Routes ---
    // These routes are accessible without any authentication.
    let public_router = Router::new()
        // Auth routes for logging in and password management.
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/forgot-password", post(auth_handler::forgot_password))
        .route("/users/reset-password", post(user_handler::reset_password_handler))
        // User creation routes.
        .route("/users", post(user_handler::create_user))
        .layer(middleware::from_fn(csrf_guard));

    let public_router_no_csrf = Router::new()
        // CSRF token provider.
        .route("/csrf/token", get(csrf_handler::get_csrf_token));

    // --- Protected Routes ---
    // All routes defined here will require a valid JWT, as enforced by the `auth_guard`.
    let protected_router = Router::new()
        // --- Authenticated User & Auth Routes ---
        .route("/auth/me", get(auth_handler::get_me))
        .route("/users/:id", get(user_handler::get_user_by_id))

        // --- Project Routes ---
        .route("/projects", get(project_handler::get_my_projects).post(project_handler::create_project))
        .route("/projects/:project_id", get(project_handler::get_project_by_id))
        
        // --- Member Routes (Nested under Projects) ---
        .route("/projects/:project_id/members", get(member_handler::get_project_members).post(member_handler::add_member_to_project))

        // --- Role Routes (Nested under Projects) ---
        .route("/projects/:project_id/roles", get(role_handler::get_roles_for_project).post(role_handler::create_role_for_project))

        // --- Task Routes ---
        // Listing/Creation is nested under a project for context.
        .route("/projects/:project_id/tasks", get(task_handler::get_tasks_for_project).post(task_handler::create_task))
        // Actions on a specific task use the task's ID directly.
        .route("/tasks/:task_id/contributors", post(task_handler::assign_contributor_handler))
        .route("/tasks/:task_id/required-roles", post(task_handler::add_required_role_handler))
        
        // --- Subtask Routes (Nested under Tasks) ---
        .route("/tasks/:task_id/subtasks", get(subtask_handler::get_subtasks_for_task).post(subtask_handler::create_subtask))

        // --- Middleware Layer ---
        // This is the correct and modern way to apply middleware to a group of routes.
        // The `auth_guard` will run for every single request handled by `protected_router`.
        .layer(middleware::from_fn(auth_guard))
        .layer(middleware::from_fn(csrf_guard));

    // --- Final Router ---
    // Merge the public and protected routers into one.
    // The final router will first try to match a route in `public_router`.
    // If it fails, it will then try to match a route in `protected_router`.
    Router::new()
        .merge(public_router_no_csrf) 
        .merge(public_router)
        .merge(protected_router)
}