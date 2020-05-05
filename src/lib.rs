#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate serde_xml_rs;
extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "/v2";
pub const API_VERSION: &'static str = "1.0.5";


#[derive(Debug, PartialEq)]
pub enum AddPetResponse {
    /// Invalid input
    InvalidInput ,
}

#[derive(Debug, PartialEq)]
pub enum DeletePetResponse {
    /// Invalid ID supplied
    InvalidIDSupplied ,
    /// Pet not found
    PetNotFound ,
}

#[derive(Debug, PartialEq)]
pub enum FindPetsByStatusResponse {
    /// successful operation
    SuccessfulOperation ( Vec<models::Pet> ) ,
    /// Invalid status value
    InvalidStatusValue ,
}

#[derive(Debug, PartialEq)]
pub enum FindPetsByTagsResponse {
    /// successful operation
    SuccessfulOperation ( Vec<models::Pet> ) ,
    /// Invalid tag value
    InvalidTagValue ,
}

#[derive(Debug, PartialEq)]
pub enum GetPetByIdResponse {
    /// successful operation
    SuccessfulOperation ( models::Pet ) ,
    /// Invalid ID supplied
    InvalidIDSupplied ,
    /// Pet not found
    PetNotFound ,
}

#[derive(Debug, PartialEq)]
pub enum UpdatePetResponse {
    /// Invalid ID supplied
    InvalidIDSupplied ,
    /// Pet not found
    PetNotFound ,
    /// Validation exception
    ValidationException ,
}

#[derive(Debug, PartialEq)]
pub enum UpdatePetWithFormResponse {
    /// Invalid input
    InvalidInput ,
}

#[derive(Debug, PartialEq)]
pub enum UploadFileResponse {
    /// successful operation
    SuccessfulOperation ( models::ApiResponse ) ,
}

#[derive(Debug, PartialEq)]
pub enum DeleteOrderResponse {
    /// Invalid ID supplied
    InvalidIDSupplied ,
    /// Order not found
    OrderNotFound ,
}

#[derive(Debug, PartialEq)]
pub enum GetInventoryResponse {
    /// successful operation
    SuccessfulOperation ( HashMap<String, i32> ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetOrderByIdResponse {
    /// successful operation
    SuccessfulOperation ( models::Order ) ,
    /// Invalid ID supplied
    InvalidIDSupplied ,
    /// Order not found
    OrderNotFound ,
}

#[derive(Debug, PartialEq)]
pub enum PlaceOrderResponse {
    /// successful operation
    SuccessfulOperation ( models::Order ) ,
    /// Invalid Order
    InvalidOrder ,
}

#[derive(Debug, PartialEq)]
pub enum CreateUserResponse {
    /// successful operation
    SuccessfulOperation ,
}

#[derive(Debug, PartialEq)]
pub enum CreateUsersWithArrayInputResponse {
    /// successful operation
    SuccessfulOperation ,
}

#[derive(Debug, PartialEq)]
pub enum CreateUsersWithListInputResponse {
    /// successful operation
    SuccessfulOperation ,
}

#[derive(Debug, PartialEq)]
pub enum DeleteUserResponse {
    /// Invalid username supplied
    InvalidUsernameSupplied ,
    /// User not found
    UserNotFound ,
}

#[derive(Debug, PartialEq)]
pub enum GetUserByNameResponse {
    /// successful operation
    SuccessfulOperation ( models::User ) ,
    /// Invalid username supplied
    InvalidUsernameSupplied ,
    /// User not found
    UserNotFound ,
}

#[derive(Debug, PartialEq)]
pub enum LoginUserResponse {
    /// successful operation
    SuccessfulOperation { body: String, x_expires_after: chrono::DateTime<chrono::Utc>, x_rate_limit: i32 } ,
    /// Invalid username/password supplied
    InvalidUsername ,
}

#[derive(Debug, PartialEq)]
pub enum LogoutUserResponse {
    /// successful operation
    SuccessfulOperation ,
}

#[derive(Debug, PartialEq)]
pub enum UpdateUserResponse {
    /// Invalid user supplied
    InvalidUserSupplied ,
    /// User not found
    UserNotFound ,
}


/// API
pub trait Api<C> {

    /// Add a new pet to the store
    fn add_pet(&self, body: models::Pet, context: &C) -> Box<Future<Item=AddPetResponse, Error=ApiError>>;

    /// Deletes a pet
    fn delete_pet(&self, pet_id: i64, api_key: Option<String>, context: &C) -> Box<Future<Item=DeletePetResponse, Error=ApiError>>;

    /// Finds Pets by status
    fn find_pets_by_status(&self, status: &Vec<String>, context: &C) -> Box<Future<Item=FindPetsByStatusResponse, Error=ApiError>>;

    /// Finds Pets by tags
    fn find_pets_by_tags(&self, tags: &Vec<String>, context: &C) -> Box<Future<Item=FindPetsByTagsResponse, Error=ApiError>>;

    /// Find pet by ID
    fn get_pet_by_id(&self, pet_id: i64, context: &C) -> Box<Future<Item=GetPetByIdResponse, Error=ApiError>>;

    /// Update an existing pet
    fn update_pet(&self, body: models::Pet, context: &C) -> Box<Future<Item=UpdatePetResponse, Error=ApiError>>;

    /// Updates a pet in the store with form data
    fn update_pet_with_form(&self, pet_id: i64, name: Option<String>, status: Option<String>, context: &C) -> Box<Future<Item=UpdatePetWithFormResponse, Error=ApiError>>;

    /// uploads an image
    fn upload_file(&self, pet_id: i64, additional_metadata: Option<String>, file: Box<Future<Item=Option<Box<Stream<Item=Vec<u8>, Error=Error> + Send>>, Error=Error> + Send>, context: &C) -> Box<Future<Item=UploadFileResponse, Error=ApiError>>;

    /// Delete purchase order by ID
    fn delete_order(&self, order_id: i64, context: &C) -> Box<Future<Item=DeleteOrderResponse, Error=ApiError>>;

    /// Returns pet inventories by status
    fn get_inventory(&self, context: &C) -> Box<Future<Item=GetInventoryResponse, Error=ApiError>>;

    /// Find purchase order by ID
    fn get_order_by_id(&self, order_id: i64, context: &C) -> Box<Future<Item=GetOrderByIdResponse, Error=ApiError>>;

    /// Place an order for a pet
    fn place_order(&self, body: models::Order, context: &C) -> Box<Future<Item=PlaceOrderResponse, Error=ApiError>>;

    /// Create user
    fn create_user(&self, body: models::User, context: &C) -> Box<Future<Item=CreateUserResponse, Error=ApiError>>;

    /// Creates list of users with given input array
    fn create_users_with_array_input(&self, body: &Vec<models::User>, context: &C) -> Box<Future<Item=CreateUsersWithArrayInputResponse, Error=ApiError>>;

    /// Creates list of users with given input array
    fn create_users_with_list_input(&self, body: &Vec<models::User>, context: &C) -> Box<Future<Item=CreateUsersWithListInputResponse, Error=ApiError>>;

    /// Delete user
    fn delete_user(&self, username: String, context: &C) -> Box<Future<Item=DeleteUserResponse, Error=ApiError>>;

    /// Get user by user name
    fn get_user_by_name(&self, username: String, context: &C) -> Box<Future<Item=GetUserByNameResponse, Error=ApiError>>;

    /// Logs user into the system
    fn login_user(&self, username: String, password: String, context: &C) -> Box<Future<Item=LoginUserResponse, Error=ApiError>>;

    /// Logs out current logged in user session
    fn logout_user(&self, context: &C) -> Box<Future<Item=LogoutUserResponse, Error=ApiError>>;

    /// Updated user
    fn update_user(&self, username: String, body: models::User, context: &C) -> Box<Future<Item=UpdateUserResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {

    /// Add a new pet to the store
    fn add_pet(&self, body: models::Pet) -> Box<Future<Item=AddPetResponse, Error=ApiError>>;

    /// Deletes a pet
    fn delete_pet(&self, pet_id: i64, api_key: Option<String>) -> Box<Future<Item=DeletePetResponse, Error=ApiError>>;

    /// Finds Pets by status
    fn find_pets_by_status(&self, status: &Vec<String>) -> Box<Future<Item=FindPetsByStatusResponse, Error=ApiError>>;

    /// Finds Pets by tags
    fn find_pets_by_tags(&self, tags: &Vec<String>) -> Box<Future<Item=FindPetsByTagsResponse, Error=ApiError>>;

    /// Find pet by ID
    fn get_pet_by_id(&self, pet_id: i64) -> Box<Future<Item=GetPetByIdResponse, Error=ApiError>>;

    /// Update an existing pet
    fn update_pet(&self, body: models::Pet) -> Box<Future<Item=UpdatePetResponse, Error=ApiError>>;

    /// Updates a pet in the store with form data
    fn update_pet_with_form(&self, pet_id: i64, name: Option<String>, status: Option<String>) -> Box<Future<Item=UpdatePetWithFormResponse, Error=ApiError>>;

    /// uploads an image
    fn upload_file(&self, pet_id: i64, additional_metadata: Option<String>, file: Box<Future<Item=Option<Box<Stream<Item=Vec<u8>, Error=Error> + Send>>, Error=Error> + Send>) -> Box<Future<Item=UploadFileResponse, Error=ApiError>>;

    /// Delete purchase order by ID
    fn delete_order(&self, order_id: i64) -> Box<Future<Item=DeleteOrderResponse, Error=ApiError>>;

    /// Returns pet inventories by status
    fn get_inventory(&self) -> Box<Future<Item=GetInventoryResponse, Error=ApiError>>;

    /// Find purchase order by ID
    fn get_order_by_id(&self, order_id: i64) -> Box<Future<Item=GetOrderByIdResponse, Error=ApiError>>;

    /// Place an order for a pet
    fn place_order(&self, body: models::Order) -> Box<Future<Item=PlaceOrderResponse, Error=ApiError>>;

    /// Create user
    fn create_user(&self, body: models::User) -> Box<Future<Item=CreateUserResponse, Error=ApiError>>;

    /// Creates list of users with given input array
    fn create_users_with_array_input(&self, body: &Vec<models::User>) -> Box<Future<Item=CreateUsersWithArrayInputResponse, Error=ApiError>>;

    /// Creates list of users with given input array
    fn create_users_with_list_input(&self, body: &Vec<models::User>) -> Box<Future<Item=CreateUsersWithListInputResponse, Error=ApiError>>;

    /// Delete user
    fn delete_user(&self, username: String) -> Box<Future<Item=DeleteUserResponse, Error=ApiError>>;

    /// Get user by user name
    fn get_user_by_name(&self, username: String) -> Box<Future<Item=GetUserByNameResponse, Error=ApiError>>;

    /// Logs user into the system
    fn login_user(&self, username: String, password: String) -> Box<Future<Item=LoginUserResponse, Error=ApiError>>;

    /// Logs out current logged in user session
    fn logout_user(&self) -> Box<Future<Item=LogoutUserResponse, Error=ApiError>>;

    /// Updated user
    fn update_user(&self, username: String, body: models::User) -> Box<Future<Item=UpdateUserResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {

    /// Add a new pet to the store
    fn add_pet(&self, body: models::Pet) -> Box<Future<Item=AddPetResponse, Error=ApiError>> {
        self.api().add_pet(body, &self.context())
    }

    /// Deletes a pet
    fn delete_pet(&self, pet_id: i64, api_key: Option<String>) -> Box<Future<Item=DeletePetResponse, Error=ApiError>> {
        self.api().delete_pet(pet_id, api_key, &self.context())
    }

    /// Finds Pets by status
    fn find_pets_by_status(&self, status: &Vec<String>) -> Box<Future<Item=FindPetsByStatusResponse, Error=ApiError>> {
        self.api().find_pets_by_status(status, &self.context())
    }

    /// Finds Pets by tags
    fn find_pets_by_tags(&self, tags: &Vec<String>) -> Box<Future<Item=FindPetsByTagsResponse, Error=ApiError>> {
        self.api().find_pets_by_tags(tags, &self.context())
    }

    /// Find pet by ID
    fn get_pet_by_id(&self, pet_id: i64) -> Box<Future<Item=GetPetByIdResponse, Error=ApiError>> {
        self.api().get_pet_by_id(pet_id, &self.context())
    }

    /// Update an existing pet
    fn update_pet(&self, body: models::Pet) -> Box<Future<Item=UpdatePetResponse, Error=ApiError>> {
        self.api().update_pet(body, &self.context())
    }

    /// Updates a pet in the store with form data
    fn update_pet_with_form(&self, pet_id: i64, name: Option<String>, status: Option<String>) -> Box<Future<Item=UpdatePetWithFormResponse, Error=ApiError>> {
        self.api().update_pet_with_form(pet_id, name, status, &self.context())
    }

    /// uploads an image
    fn upload_file(&self, pet_id: i64, additional_metadata: Option<String>, file: Box<Future<Item=Option<Box<Stream<Item=Vec<u8>, Error=Error> + Send>>, Error=Error> + Send>) -> Box<Future<Item=UploadFileResponse, Error=ApiError>> {
        self.api().upload_file(pet_id, additional_metadata, file, &self.context())
    }

    /// Delete purchase order by ID
    fn delete_order(&self, order_id: i64) -> Box<Future<Item=DeleteOrderResponse, Error=ApiError>> {
        self.api().delete_order(order_id, &self.context())
    }

    /// Returns pet inventories by status
    fn get_inventory(&self) -> Box<Future<Item=GetInventoryResponse, Error=ApiError>> {
        self.api().get_inventory(&self.context())
    }

    /// Find purchase order by ID
    fn get_order_by_id(&self, order_id: i64) -> Box<Future<Item=GetOrderByIdResponse, Error=ApiError>> {
        self.api().get_order_by_id(order_id, &self.context())
    }

    /// Place an order for a pet
    fn place_order(&self, body: models::Order) -> Box<Future<Item=PlaceOrderResponse, Error=ApiError>> {
        self.api().place_order(body, &self.context())
    }

    /// Create user
    fn create_user(&self, body: models::User) -> Box<Future<Item=CreateUserResponse, Error=ApiError>> {
        self.api().create_user(body, &self.context())
    }

    /// Creates list of users with given input array
    fn create_users_with_array_input(&self, body: &Vec<models::User>) -> Box<Future<Item=CreateUsersWithArrayInputResponse, Error=ApiError>> {
        self.api().create_users_with_array_input(body, &self.context())
    }

    /// Creates list of users with given input array
    fn create_users_with_list_input(&self, body: &Vec<models::User>) -> Box<Future<Item=CreateUsersWithListInputResponse, Error=ApiError>> {
        self.api().create_users_with_list_input(body, &self.context())
    }

    /// Delete user
    fn delete_user(&self, username: String) -> Box<Future<Item=DeleteUserResponse, Error=ApiError>> {
        self.api().delete_user(username, &self.context())
    }

    /// Get user by user name
    fn get_user_by_name(&self, username: String) -> Box<Future<Item=GetUserByNameResponse, Error=ApiError>> {
        self.api().get_user_by_name(username, &self.context())
    }

    /// Logs user into the system
    fn login_user(&self, username: String, password: String) -> Box<Future<Item=LoginUserResponse, Error=ApiError>> {
        self.api().login_user(username, password, &self.context())
    }

    /// Logs out current logged in user session
    fn logout_user(&self) -> Box<Future<Item=LogoutUserResponse, Error=ApiError>> {
        self.api().logout_user(&self.context())
    }

    /// Updated user
    fn update_user(&self, username: String, body: models::User) -> Box<Future<Item=UpdateUserResponse, Error=ApiError>> {
        self.api().update_user(username, body, &self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
