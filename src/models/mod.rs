// The main module file for all data models.

// Re-export modules for cleaner access paths.
pub mod auth;
pub mod user;
pub mod event;
pub mod venue;
pub mod attraction;
pub mod category;
pub mod pricing;
pub mod seating;
pub mod order;
pub mod ticket;
pub mod payment;

// Re-export specific structs for convenience.
pub use auth::{LoginPayload, LoginResponse, TokenClaims};
pub use user::{User, CreateUserPayload};
pub use event::{Event, EventStatus, CreateEventPayload, UpdateEventPayload};
pub use venue::{Venue, CreateVenuePayload};
pub use attraction::{Attraction, AttractionType, AssignAttractionPayload};
pub use category::{Segment, Genre, SubGenre, CreateCategoryPayload};
pub use pricing::{TicketTier, Offer, OfferStatus, CreateTicketTierPayload, CreateOfferPayload};
pub use seating::{SeatingChart, Section, Row, Seat, EventSeat, SeatStatus, SeatMapInfo};
pub use order::{Order, OrderStatus, CreateOrderPayload};
pub use ticket::{Ticket, TicketStatus, TicketDetails};
pub use payment::{Payment, PaymentStatus};