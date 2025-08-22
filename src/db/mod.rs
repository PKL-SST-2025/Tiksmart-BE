// The main module file for all database query modules.
// It declares all sub-modules, making them accessible via the `db::` path.

// The connection pool creation utility.
pub mod pool;

// The query module for users and authentication.
pub mod user_query;
pub mod admin_query;

// Query modules for the core entities of the application.
pub mod event_query;
pub mod venue_query;
pub mod attraction_query;

// Query modules for categorization and pricing logic.
pub mod category_query;
pub mod pricing_query;

// Query module for the complex seating chart system.
pub mod seating_query;

// Query modules for the core e-commerce transactional loop.
pub mod order_query;
pub mod ticket_query;
pub mod payment_query;

// Query modules for Stripes Multivendor organizers
pub mod organizer_query; 