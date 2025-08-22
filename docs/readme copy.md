
# Ticket Platform API Documentation

This document provides a complete reference for the API endpoints of the ticketing platform backend. The API is designed to be RESTful and uses standard HTTP conventions.

## Getting Started

### 1. Environment Configuration
Before running the server, create a `.env` file in the root of the project. This file must contain all necessary configuration variables. Use the `.env.example` as a template.

**Key Variables:**
- `DATABASE_URL`: The connection string for your PostgreSQL database.
- `SERVER_ADDRESS`: The address and port for the server (e.g., `127.0.0.1:7878`).
- `JWT_SECRET`: A long, secure, randomly generated string for signing JWTs.
- `CSRF_SECRET`: A long, secure, randomly generated string for CSRF protection.
- `FRONTEND_ORIGIN`: The URL of your frontend application for CORS.
- `STRIPE_SECRET_KEY`: Your Stripe secret API key (`sk_test_...`).
- `STRIPE_WEBHOOK_SECRET`: The signing secret for your Stripe webhook endpoint (`whsec_...`).

### 2. Running the Server
1.  Install dependencies: `cargo build`
2.  Run database migrations: `sqlx migrate run`
3.  Start the server: `cargo run`

## General Concepts

### Authentication
- The API uses JWTs for authentication.
- Upon successful login/registration, an `auth-token` is set as an `HttpOnly`, `Secure` cookie. This cookie is automatically sent by the browser on subsequent requests.
- Endpoints marked as **User Required** or higher require this valid cookie.

### Authorization
- **User Required**: Any logged-in user can access the endpoint.
- **Organizer Required**: The logged-in user must have an "organizer" role. Our current implementation assumes any logged-in user can be an organizer, but authorization checks ensure they can only modify their own resources.
- **Admin Required**: The logged-in user must have an "admin" role, obtained via a separate admin login flow.

### Error Handling
- Errors are returned with appropriate HTTP status codes.
- The response body for errors is a JSON object:
  ```json
  {
    "error": "A descriptive error message."
  }
  ```

---

## Part 1: Public & Authentication API

This section covers endpoints for user registration, login, and publicly accessible data.

### Authentication

#### `POST /api/auth/register`
- **Description**: Creates a new user account.
- **Authentication**: Public.
- **Request Body**:
  ```json
  {
    "username": "newuser",
    "email": "user@example.com",
    "password": "StrongPassword123!"
  }
  ```
- **Success Response**: `201 CREATED` with the new `User` object and sets the `auth-token` cookie.

#### `POST /api/auth/login`
- **Description**: Authenticates a user and starts a session.
- **Authentication**: Public.
- **Request Body**:
  ```json
  {
    "email": "user@example.com",
    "password": "StrongPassword123!"
  }
  ```
- **Success Response**: `200 OK` and sets the `auth-token` cookie.

#### `POST /api/auth/logout`
- **Description**: Clears the authentication cookie, logging the user out.
- **Authentication**: Public.
- **Success Response**: `200 OK`.

#### `GET /api/auth/me`
- **Description**: Retrieves the profile of the currently authenticated user.
- **Authentication**: **User Required**.
- **Success Response**: `200 OK` with the `User` object.

### Password Reset

#### `POST /api/auth/forgot-password`
- **Description**: Initiates the password reset process by generating a 6-digit OTP. The backend returns this OTP to the caller, assuming a separate service (like "mail js") will handle sending the email.
- **Authentication**: Public.
- **Request Body**:
  ```json
  {
    "email": "user@example.com"
  }
  ```
- **Success Response**: `200 OK`
  ```json
  {
    "message": "If an account with this email exists, a password reset code has been generated.",
    "reset_code": "123456" 
  }
  ```

#### `POST /api/users/reset-password-otp`
- **Description**: Completes the password reset process using the OTP.
- **Authentication**: Public.
- **Request Body**:
  ```json
  {
    "email": "user@example.com",
    "otp": "123456",
    "new_password": "NewStrongPassword123!"
  }
  ```
- **Success Response**: `200 OK`.

### Public Data (Events, Venues, Categories)

#### `GET /api/events`
- **Description**: Lists all published events that are scheduled for the future.
- **Authentication**: Public.
- **Success Response**: `200 OK` with an array of `Event` objects.

#### `GET /api/events/:id`
- **Description**: Retrieves a single event by its ID.
- **Authentication**: Public.
- **Success Response**: `200 OK` with an `Event` object.

#### `GET /api/events/:event_id/offers`
- **Description**: Lists all publicly available sales offers for a specific event.
- **Authentication**: Public.
- **Success Response**: `200 OK` with an array of `Offer` objects.

#### `GET /api/events/:event_id/seat-map`
- **Description**: Fetches all the data required to render a visual, interactive seat map for an event.
- **Authentication**: Public.
- **Success Response**: `200 OK` with an array of `SeatMapInfo` objects.

*Similar `GET` endpoints exist for `/venues`, `/venues/:id`, `/segments`, `/segments/:id/genres`, and `/genres/:id/sub-genres`.*

---

## Part 2: Protected API (Customers & Organizers)

These endpoints require a valid `auth-token` cookie.

### Checkout & Tickets (Customer)

#### `POST /api/orders`
- **Description**: Initiates the checkout process. Locks inventory/seats for 15 minutes and creates a Stripe Payment Intent.
- **Authentication**: **User Required**.
- **Request Body**:
  ```json
  {
    "items": [
      {
        "offer_id": 1,
        "seat_id": 101, // Null for General Admission
        "quantity": 1   // Must be 1 for reserved seats
      },
      {
        "offer_id": 2,
        "seat_id": null, // General Admission
        "quantity": 2
      }
    ]
  }
  ```
- **Success Response**: `201 CREATED`
  ```json
  {
    "order": {
      // Full Order object
    },
    "stripe_client_secret": "pi_..._secret_..."
  }
  ```

#### `GET /api/me/tickets`
- **Description**: Retrieves a list of all tickets owned by the authenticated user.
- **Authentication**: **User Required**.
- **Success Response**: `200 OK` with an array of `TicketDetails` objects.

### Event & Pricing Management (Organizer)

#### `POST /api/events`
- **Description**: Creates a new event. The organizer is automatically assigned based on the authenticated user.
- **Authentication**: **Organizer Required**.
- **Request Body**: `CreateEventPayload` object.
- **Success Response**: `201 CREATED` with the new `Event` object.

#### `PATCH /api/events/:id`
- **Description**: Updates an event. The user must be the organizer of the event.
- **Authentication**: **Organizer Required**.
- **Request Body**: `UpdateEventPayload` object (all fields optional).
- **Success Response**: `200 OK` with the updated `Event` object.

#### `DELETE /api/events/:id`
- **Description**: Deletes an event. The user must be the organizer of the event.
- **Authentication**: **Organizer Required**.
- **Success Response**: `204 No Content`.

#### `POST /api/events/:event_id/tiers`
- **Description**: Creates a new ticket tier (e.g., "General Admission", "VIP") for an event.
- **Authentication**: **Organizer Required**.
- **Request Body**: `CreateTicketTierPayload` object.
- **Success Response**: `201 CREATED` with the new `TicketTier` object.

*Similar `POST`, `PATCH`, `DELETE` endpoints exist for managing nested resources like `/events/:event_id/attractions` and `/tiers/:tier_id/offers`.*

### Organizer Onboarding

#### `POST /api/organizer/stripe/onboarding-link`
- **Description**: Creates a Stripe Connect account for the organizer (if one doesn't exist) and returns a unique, short-lived URL for them to complete Stripe's onboarding process.
- **Authentication**: **Organizer Required**.
- **Request Body**: None.
- **Success Response**: `200 OK`
  ```json
  {
    "url": "https://connect.stripe.com/setup/s/..."
  }
  ```

---

## Part 3: Admin & Webhook API

These endpoints are for platform management and automated integrations.

### Platform Administration

- **Authentication**: **Admin Required** for all endpoints in this section.

#### `POST /api/venues`
- **Description**: Creates a new venue for the platform.
- **Request Body**: `CreateVenuePayload` object.
- **Success Response**: `201 CREATED` with the new `Venue` object.

#### `POST /api/segments`
- **Description**: Creates a new top-level event category (e.g., "Music", "Sports").
- **Request Body**: `CreateCategoryPayload` object.
- **Success Response**: `201 CREATED` with the new `Segment` object.

*Similar `POST` endpoints exist for creating genres (`/segments/:id/genres`) and sub-genres (`/genres/:id/sub-genres`).*

### Webhooks

#### `POST /api/webhooks/stripe`
- **Description**: An endpoint for receiving webhook events from Stripe. **This is not for direct human use.**
- **Authentication**: Public, but requests are verified using the `Stripe-Signature` header.
- **Success Response**: `200 OK` (to acknowledge receipt to Stripe).
- **Primary Use**: Listens for `payment_intent.succeeded` events to finalize orders and issue tickets.


## Part 4: Data Models (JSON Structures)

This section defines the structure of the primary JSON objects returned by the API.

### `Event`
Represents a single event.
```json
{
  "id": 1,
  "organizer_id": 10,
  "venue_id": 5,
  "segment_id": 2,
  "genre_id": 8,
  "sub_genre_id": 22,
  "title": "The Grand Rock Concert",
  "description": "An unforgettable night of rock music.",
  "status": "published", // "draft" | "published" | "cancelled" | "completed" | "on_sale" | "sold_out"
  "start_time": "2024-10-26T19:00:00Z",
  "end_time": "2024-10-26T23:00:00Z",
  "price_min": "75.50",
  "price_max": "250.00",
  "created_at": "2024-05-10T12:00:00Z",
  "last_updated": "2024-05-11T09:30:00Z"
}
```

### `Venue`
Represents a physical or online location for an event.
```json
{
  "id": 5,
  "name": "The Grand Arena",
  "address_line_1": "123 Music Lane",
  "address_line_2": null,
  "city": "Metropolis",
  "state": "CA",
  "postal_code": "90210",
  "country": "USA",
  "capacity": 15000,
  "latitude": "34.0522",
  "longitude": "-118.2437",
  "phone_number": "555-123-4567",
  "website_url": "https://grandarena.com",
  "is_active": true,
  "created_at": "2024-01-15T10:00:00Z",
  "last_updated": "2024-03-20T14:00:00Z"
}
```

### `User`
Represents a user account. Sensitive fields like `password_hash` are never returned.
```json
{
  "id": 10,
  "email": "organizer@example.com",
  "username": "rockorganizer",
  "created_at": "2024-02-01T11:00:00Z"
}
```

### `Offer`
Represents a specific sales offer for a ticket tier.
```json
{
  "id": 15,
  "ticket_tier_id": 7,
  "name": "General Admission - Early Bird",
  "status": "on_sale", // "scheduled" | "on_sale" | "paused" | "sold_out" | "ended"
  "price": "75.50",
  "quantity_for_sale": 500,
  "quantity_sold": 150,
  "sale_start_time": "2024-06-01T10:00:00Z",
  "sale_end_time": "2024-07-01T10:00:00Z",
  "min_per_order": 1,
  "max_per_order": 8
}
```

### `TicketDetails` (DTO)
A combined object representing a user's ticket, joining data from multiple tables for convenience.
```json
{
  "ticket_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "qr_code_data": "unique-qr-string-for-scanning",
  "ticket_status": "valid", // "valid" | "checked_in" | "voided"
  "event_title": "The Grand Rock Concert",
  "event_start_time": "2024-10-26T19:00:00Z",
  "venue_name": "The Grand Arena",
  "venue_city": "Metropolis",
  "ticket_tier_name": "Section 101, Row A",
  "section_name": "Section 101",
  "row_name": "A",
  "seat_number": "12"
}
```

---

## Part 5: Complete Route Summary

This table provides a quick reference for all available API endpoints.

| Method | Endpoint                                        | Authentication        | Description                                       |
| :----- | :---------------------------------------------- | :-------------------- | :------------------------------------------------ |
| **Authentication & Users** |                               |                       |                                                   |
| `POST` | `/api/auth/register`                            | Public                | Register a new user account.                      |
| `POST` | `/api/auth/login`                               | Public                | Log in a user and set the auth cookie.            |
| `POST` | `/api/auth/logout`                              | Public                | Log out a user by clearing the auth cookie.       |
| `GET`  | `/api/auth/me`                                  | **User Required**     | Get the profile of the logged-in user.            |
| `POST` | `/api/auth/forgot-password`                     | Public                | Request a password reset OTP.                     |
| `POST` | `/api/users/reset-password-otp`                 | Public                | Reset password using a valid OTP.                 |
| **Public Browsing** |                                       |                       |                                                   |
| `GET`  | `/api/events`                                   | Public                | List all published, upcoming events.              |
| `GET`  | `/api/events/:id`                               | Public                | Get details for a single event.                   |
| `GET`  | `/api/events/:event_id/offers`                  | Public                | List public sales offers for an event.            |
| `GET`  | `/api/events/:event_id/seat-map`                | Public                | Get the full data to render an event's seat map.  |
| `GET`  | `/api/venues`                                   | Public                | List all active venues.                           |
| `GET`  | `/api/venues/:id`                               | Public                | Get details for a single venue.                   |
| `GET`  | `/api/segments`                                 | Public                | List all top-level event categories.              |
| `GET`  | `/api/segments/:id/genres`                      | Public                | List genres within a segment.                     |
| `GET`  | `/api/genres/:id/sub-genres`                    | Public                | List sub-genres within a genre.                   |
| **Checkout & Tickets** |                                  |                       |                                                   |
| `POST` | `/api/orders`                                   | **User Required**     | Create a pending order and get a Stripe secret.   |
| `GET`  | `/api/me/tickets`                               | **User Required**     | Get all tickets owned by the logged-in user.      |
| **Organizer Management** |                                 |                       |                                                   |
| `POST` | `/api/events`                                   | **Organizer Required**| Create a new event.                               |
| `PATCH`| `/api/events/:id`                               | **Organizer (Owner)** | Update an event owned by the user.                |
| `DELETE`| `/api/events/:id`                              | **Organizer (Owner)** | Delete an event owned by the user.                |
| `POST` | `/api/events/:event_id/tiers`                   | **Organizer (Owner)** | Create a new ticket tier for an event.            |
| `POST` | `/api/events/:event_id/attractions`             | **Organizer (Owner)** | Add an attraction to an event's lineup.           |
| `DELETE`| `/api/events/:event_id/attractions/:attr_id`    | **Organizer (Owner)** | Remove an attraction from an event.               |
| `POST` | `/api/tiers/:tier_id/offers`                    | **Organizer (Owner)** | Create a new sales offer for a tier.              |
| `POST` | `/api/organizer/stripe/onboarding-link`         | **Organizer Required**| Get a link to onboard with Stripe Connect.        |
| **Platform Administration** |                               |                       |                                                   |
| `POST` | `/api/venues`                                   | **Admin Required**    | Create a new venue on the platform.               |
| `POST` | `/api/segments`                                 | **Admin Required**    | Create a new top-level category.                  |
| `POST` | `/api/segments/:id/genres`                      | **Admin Required**    | Create a new genre within a segment.              |
| `POST` | `/api/genres/:id/sub-genres`                    | **Admin Required**    | Create a new sub-genre within a genre.            |
| **Integrations & System** |                                 |                       |                                                   |
| `GET`  | `/api/csrf/token`                               | Public                | Get a CSRF token for state-changing requests.     |
| `POST` | `/api/webhooks/stripe`                          | Webhook (Verified)    | Endpoint for receiving Stripe webhook events.     |


## Part 6: Project Architecture

This project follows a layered architecture to ensure separation of concerns, making the codebase easier to maintain, test, and scale. A request flows through these layers in a specific order.

### Request Lifecycle

A typical request to a protected endpoint follows this path:

```
Incoming HTTP Request
       |
       v
[ Middleware (CORS, Tracing) ]
       |
       v
[ CSRF Guard Middleware ]
       |
       v
[ Auth Guard Middleware ]  (Authenticates user, attaches ID/role to request)
       |
       v
[ API Handler (in `api/`) ]  (Extracts data, calls service)
       |
       v
[ Service Layer (in `service/`) ]  (Enforces business logic, manages transactions)
       |
       v
[ Database Query Layer (in `db/`) ]  (Executes raw SQL, maps to models)
       |
       v
[ Database (PostgreSQL) ]
       |
       v
Outgoing HTTP Response
```

### Directory Structure

-   **/src/api**: Contains the **Handlers**. Each handler is responsible for parsing HTTP requests, calling the appropriate service function, and formatting the HTTP response. It knows nothing about the database.
-   **/src/service**: Contains the **Business Logic**. Services orchestrate operations, enforce authorization rules (e.g., "is this user the owner of this event?"), manage database transactions, and call one or more database queries.
-   **/src/db**: Contains the **Database Queries**. This is the only layer that directly interacts with the database. It contains functions that execute specific, parameterized SQL queries.
-   **/src/models**: Contains the **Data Structures**. This includes structs that map to database tables (`User`, `Event`), structs for API request/response bodies (`CreateUserPayload`, `TicketDetails`), and enums.
-   **/src/errors**: Defines the custom `AppError` type and its conversion into a user-friendly HTTP response.
-   **/src/middleware**: Contains custom Axum middleware for tasks like authentication (`auth_guard`), authorization (`admin_guard`), and security (`csrf_guard`).
-   **/src/config**: Handles loading and providing application configuration from environment variables.

---

## Part 7: Key Workflows Explained

Understanding how individual API endpoints are chained together is crucial for building a frontend. Here are the two most important workflows.

### Workflow 1: Organizer Onboarding & Event Creation

This workflow describes how a new user becomes an event organizer and lists their first event for sale.

1.  **Register Account**: The user creates a standard account via `POST /api/auth/register`.
2.  **Initiate Stripe Onboarding**: From their dashboard, the user clicks "Connect with Stripe". The frontend calls `POST /api/organizer/stripe/onboarding-link`.
3.  **Redirect to Stripe**: The frontend receives the unique URL from the API response and redirects the user to Stripe's secure onboarding portal.
4.  **Complete Onboarding**: The user fills out their details on Stripe. Upon completion, Stripe redirects them back to the `return_url` specified in the service logic (e.g., `https://yourapp.com/stripe/return`). The backend is notified via a webhook (`account.updated`) when the account is fully capable of processing payments.
5.  **Create Event**: The now-onboarded organizer creates a draft of their event by calling `POST /api/events`.
6.  **Define Pricing**:
    -   The organizer adds pricing tiers (e.g., "General Admission", "VIP") by calling `POST /api/events/:event_id/tiers`.
    -   For each tier, they create one or more sales offers (e.g., "Early Bird", "Standard Price") by calling `POST /api/tiers/:tier_id/offers`.
7.  **Publish Event**: The organizer updates the event's status to `published` via `PATCH /api/events/:id`. The event is now live and visible to the public.

### Workflow 2: Customer Ticket Purchase (Reserved Seating)

This workflow describes how a customer finds an event, selects a specific seat, and completes the purchase.

1.  **Browse Events**: The customer views a list of events from `GET /api/events`.
2.  **View Seat Map**: The customer selects an event and the frontend calls `GET /api/events/:event_id/seat-map` to fetch all data needed to render the interactive map.
3.  **Select Seats**: The customer clicks on available seats on the map.
4.  **Initiate Checkout**: The customer clicks "Buy Tickets". The frontend sends the selected `offer_id` and `seat_id` for each ticket in a call to `POST /api/orders`.
5.  **Backend Locks Seats & Creates Payment Intent**: The backend validates the seats are available, locks them in the database for 15 minutes, creates a `pending` order, and requests a `PaymentIntent` from Stripe. It returns the order details and the `client_secret` from the Payment Intent.
6.  **Frontend Confirms Payment**: The frontend uses the `client_secret` with Stripe.js to securely collect the customer's payment information and confirm the payment.
7.  **Stripe Confirms Payment (Webhook)**: Stripe processes the payment and sends a `payment_intent.succeeded` event to the backend's `POST /api/webhooks/stripe` endpoint.
8.  **Backend Finalizes Order**: The webhook handler verifies the event, marks the `order` and `payment` as `completed`, and creates the final `ticket` records in the database.
9.  **View Tickets**: The customer can now see their purchased tickets by calling `GET /api/me/tickets`.

---

## Part 8: Testing & Development

### Integration Testing
The most effective way to test this application is through integration tests that interact with a real (but temporary) database.
-   Use the `sqlx::test` macro to create a separate, isolated test database for each test function. This ensures tests do not interfere with each other.
-   Use a crate like `reqwest` to make HTTP requests to your Axum application within the tests.
-   Structure tests to mimic real user workflows: register a user, log them in, create an event, and then try to purchase a ticket for it.

**Example Test Snippet (Conceptual):**
```rust
#[sqlx::test]
async fn test_create_event_succeeds_for_authenticated_organizer(pool: PgPool) {
    // 1. Setup: Create a test user and an Axum router instance.
    let app_state = AppState { db_pool: pool, ... };
    let app = api_router().with_state(app_state);
    
    // 2. Action: Simulate user login to get an auth cookie.
    let login_response = ... // Make a request to /api/auth/login
    let auth_cookie = login_response.headers().get("set-cookie")...;

    // 3. Action: Make a request to the protected endpoint using the cookie.
    let response = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/events")
            .header("cookie", auth_cookie)
            .json_body(&CreateEventPayload { ... })
            .build()
            .unwrap()
    ).await.unwrap();

    // 4. Assert: Check the status code and response body.
    assert_eq!(response.status(), StatusCode::CREATED);
    let event: Event = response_body_to_json(response).await;
    assert_eq!(event.title, "My Test Event");
}
```

---

## Part 9: Deployment Considerations

Before deploying to production, ensure the following steps are taken:

-   **[ ] Build in Release Mode**: Compile the application using `cargo build --release` for optimized performance.
-   **[ ] Production Database**: Configure the `DATABASE_URL` to point to a production-grade PostgreSQL instance.
-   **[ ] Generate Strong Secrets**: Create new, cryptographically secure random secrets for `JWT_SECRET` and `CSRF_SECRET` in your production environment. **Do not use development secrets.**
-   **[ ] Configure CORS**: Set `FRONTEND_ORIGIN` to the exact URL of your production frontend.
-   **[ ] Configure Stripe**: Switch from test keys (`sk_test_...`) to live keys (`sk_live_...`) and create a new live webhook endpoint with its own signing secret.
-   **[ ] HTTPS**: Do not run the Axum server directly on port 80 or 443. Place it behind a reverse proxy like **Nginx** or **Caddy** to handle TLS termination (HTTPS), load balancing, and static file serving.
-   **[ ] Logging**: Configure the `tracing` subscriber to log to a file or a logging service (like Datadog, Logtail) instead of just the console. Set the log level appropriately (e.g., `INFO`).
-   **[ ] Background Worker**: The logic for releasing expired seat locks (`seating_service::release_expired_locks`) needs to be run periodically. This can be done via a cron job, a separate worker process, or a background task system like `tokio::spawn`.

---

## Part 10: Security Best Practices

Security is not a feature but a continuous process. This API is built with a strong security foundation, but it's important to understand and maintain these principles.

### 1. Authentication & Session Management
-   **HttpOnly Cookies**: The use of `HttpOnly` cookies for JWTs is a primary defense against Cross-Site Scripting (XSS) attacks. Since frontend JavaScript cannot access these cookies, an XSS vulnerability on your site cannot be used to steal user session tokens.
-   **JWT Expiration**: JWTs have a built-in expiration (`exp` claim). The server validates this on every request. Keep the expiration time reasonable (e.g., 24-48 hours) to limit the window of opportunity for a compromised token to be used.
-   **Secure Flag**: In production (when served over HTTPS), cookies should be set with the `Secure` flag, ensuring they are only sent over encrypted connections.

### 2. Authorization
-   **Principle of Least Privilege**: The `auth_guard` and `admin_guard` middleware ensure that users can only access the endpoints they are permitted to. Business logic within services (e.g., checking `event.organizer_id`) further restricts access to specific resources.
-   **Never Trust the Client**: All authorization checks are performed on the backend. The frontend might hide a "delete" button, but the backend must always verify that the user making the `DELETE /api/events/:id` request is the actual owner of that event.

### 3. Cross-Site Request Forgery (CSRF)
-   **Double Submit Cookie Pattern**: This API uses the Double Submit Cookie pattern for CSRF protection on all state-changing endpoints (`POST`, `PATCH`, `DELETE`).
    1.  The frontend first calls the `GET /api/csrf/token` endpoint. The server sets a `csrf-token` cookie and returns the same token in a header.
    2.  For every subsequent state-changing request, the frontend must include the token value in an `X-CSRF-Token` header.
    3.  The `csrf_guard` middleware on the backend verifies that the token in the header matches the value signed in the cookie. If they don't match, the request is rejected.
-   This prevents a malicious website from making a user's browser submit an unwanted request to your API.

### 4. Input Validation
-   **Payload Validation**: The `validator` crate is used on all incoming DTOs (`CreateUserPayload`, etc.). This prevents malformed or malicious data from ever reaching the service layer.
-   **SQL Injection Prevention**: The `sqlx` crate uses **parameterized queries** exclusively. This is the single most effective defense against SQL injection attacks. Raw, unescaped user input is never concatenated into SQL strings.

### 5. Webhook Security
-   **Signature Verification**: The `POST /api/webhooks/stripe` endpoint is publicly accessible but highly secure. Every incoming request is verified using the `Stripe-Signature` header and your unique webhook signing secret. This cryptographically proves that the request originated from Stripe and was not tampered with. **Requests without a valid signature are always rejected.**

---

## Part 11: Advanced API Features

### Pagination
For endpoints that can return a large number of items (e.g., `/api/events`), pagination is essential to prevent overwhelming the client and the server. This API should be extended to support cursor-based or offset-based pagination.

**Example (Offset-based): `GET /api/events?page=2&limit=20`**

1.  **Handler**: The handler would extract `page` and `limit` from the query parameters, providing default values if they are absent.
2.  **Service**: The service would pass these values to the query layer.
3.  **Database Query**: The SQL query would be modified to use `LIMIT` and `OFFSET` clauses.
    ```sql
    SELECT * FROM events 
    WHERE status = 'published' AND start_time > NOW()
    ORDER BY start_time ASC
    LIMIT $1 OFFSET $2;
    ```
4.  **Response**: The API response should include pagination metadata.
    ```json
    {
      "data": [ /* array of Event objects */ ],
      "pagination": {
        "total_items": 150,
        "total_pages": 8,
        "current_page": 2,
        "page_size": 20
      }
    }
    ```

### Filtering & Sorting
Similarly, list endpoints can be enhanced with filtering and sorting capabilities.

**Example: `GET /api/events?sort_by=price_asc&venue_id=5`**
-   The handler would parse these query parameters.
-   The service layer would validate them and construct a dynamic query or call a specific query function. This requires careful implementation to prevent SQL injection if building query strings dynamically.

---

## Part 12: Troubleshooting Guide

A list of common issues and their solutions.

#### **Issue**: API requests fail with `403 Forbidden` and `{"error": "Invalid CSRF token"}`.
-   **Cause**: The `X-CSRF-Token` header is missing or incorrect for a state-changing request (`POST`, `PATCH`, `DELETE`).
-   **Solution**:
    1.  Ensure your frontend is first making a `GET` request to `/api/csrf/token`.
    2.  Verify that the value received in the `X-CSRF-Token` response header is being stored and sent back in the `X-CSRF-Token` header of the subsequent `POST` request.
    3.  Check that your browser is correctly sending the `csrf-token` cookie along with the request.

#### **Issue**: API requests fail with `401 Unauthorized` and `{"error": "AuthFailTokenNotFound"}`.
-   **Cause**: The `auth-token` cookie is missing from the request to a protected endpoint.
-   **Solution**:
    1.  Verify the user has successfully logged in.
    2.  Check the browser's developer tools (Application -> Cookies) to ensure the `auth-token` cookie was set correctly after login. It should be marked `HttpOnly`.
    3.  Ensure your frontend HTTP client is configured to send credentials/cookies with cross-origin requests (`withCredentials: true` in Axios/Fetch).

#### **Issue**: Stripe webhooks are not being processed and orders remain `pending`.
-   **Cause 1**: The webhook endpoint is not publicly accessible from the internet.
-   **Solution 1**: Use a tool like `ngrok` during local development to expose your local server to the internet so Stripe can reach it. In production, ensure your server is deployed and accessible.
-   **Cause 2**: The `STRIPE_WEBHOOK_SECRET` in your `.env` file does not match the signing secret for the specific webhook endpoint in your Stripe Dashboard.
-   **Solution 2**: Go to your Stripe Dashboard -> Developers -> Webhooks. Select your endpoint and reveal the signing secret. Copy it and paste it exactly into your `.env` file. Restart the server.
-   **Cause 3**: The webhook handler is returning an error (e.g., a `500` status code).
-   **Solution 3**: Check the server logs for errors occurring within the `stripe_webhook_handler`. Stripe will retry sending the webhook for a few days, but you should fix the underlying bug. Check the "Events" and "Logs" in your Stripe Dashboard for details on failed webhook deliveries.


---

# API Development To-Do List

This document tracks the development status of the ticketing platform API. It outlines completed milestones and identifies the next steps for feature implementation and production readiness.

## ✅ **Phase 1: Core Architecture & Foundation (COMPLETE)**

This phase established the entire project structure, from the database to the API router. All fundamental components are in place.

-   [x] **Project Setup**: Initialize Rust project with Axum, SQLx, and other core dependencies.
-   [x] **Configuration**: Implement a robust configuration system using a `.env` file and a global `CONFIG` struct.
-   [x] **Database Schema**:
    -   [x] Define and migrate all core tables (`users`, `admins`, `events`, `venues`, `organizer_profiles`).
    -   [x] Define and migrate all relationship/detail tables (`categories`, `attractions`, `pricing`, `seating`, `orders`, `tickets`, `payments`).
-   [x] **Layered Architecture**:
    -   [x] **Models (`/models`)**: Create all Rust structs and enums corresponding to the database schema and API payloads.
    -   [x] **Database Queries (`/db`)**: Implement the full data access layer with compile-time checked SQL queries.
    -   [x] **Services (`/service`)**: Implement the core business logic, including validation, authorization, and transaction management.
    -   [x] **API Handlers (`/api`)**: Implement the full set of API endpoints.
-   [x] **Error Handling**: Create a centralized `AppError` enum and `IntoResponse` implementation for consistent error handling.
-   [x] **Middleware**:
    -   [x] Implement `auth_guard` for JWT-based authentication.
    -   [x] Implement `admin_guard` for role-based authorization.
    -   [x] Implement `csrf_guard` for protection against CSRF attacks.
-   [x] **API Router**: Assemble the complete router, correctly layering public, protected, and admin-only routes with their respective middleware.
