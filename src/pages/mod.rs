pub mod about;
pub mod blog;
pub mod contact;
pub mod counter;
pub mod faq;
pub mod gallery;
pub mod game;
pub mod home;
pub mod photos;
pub mod register;
pub mod users;
pub mod login;

// Re-export page components
pub use about::About;
pub use blog::Blog;
pub use contact::Contact;
pub use faq::Faq;
pub use gallery::Gallery;
pub use game::Game;
pub use home::Home;
pub use photos::Photos;
pub use register::Register;
pub use users::Users;
pub use login::Login;