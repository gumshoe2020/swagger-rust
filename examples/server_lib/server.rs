//! Server implementation of swagger_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;
use futures::Stream;
use std::collections::HashMap;
use std::io::Error;
use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use swagger_client::{Api, ApiError,
                      AddPetResponse,
                      DeletePetResponse,
                      FindPetsByStatusResponse,
                      FindPetsByTagsResponse,
                      GetPetByIdResponse,
                      UpdatePetResponse,
                      UpdatePetWithFormResponse,
                      UploadFileResponse,
                      DeleteOrderResponse,
                      GetInventoryResponse,
                      GetOrderByIdResponse,
                      PlaceOrderResponse,
                      CreateUserResponse,
                      CreateUsersWithArrayInputResponse,
                      CreateUsersWithListInputResponse,
                      DeleteUserResponse,
                      GetUserByNameResponse,
                      LoginUserResponse,
                      LogoutUserResponse,
                      UpdateUserResponse
};
use swagger_client::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{

    /// Add a new pet to the store
    fn add_pet(&self, body: models::Pet, context: &C) -> Box<Future<Item=AddPetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("add_pet({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Deletes a pet
    fn delete_pet(&self, pet_id: i64, api_key: Option<String>, context: &C) -> Box<Future<Item=DeletePetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_pet({}, {:?}) - X-Span-ID: {:?}", pet_id, api_key, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Finds Pets by status
    fn find_pets_by_status(&self, status: &Vec<String>, context: &C) -> Box<Future<Item=FindPetsByStatusResponse, Error=ApiError>> {
        let context = context.clone();
        println!("find_pets_by_status({:?}) - X-Span-ID: {:?}", status, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Finds Pets by tags
    fn find_pets_by_tags(&self, tags: &Vec<String>, context: &C) -> Box<Future<Item=FindPetsByTagsResponse, Error=ApiError>> {
        let context = context.clone();
        println!("find_pets_by_tags({:?}) - X-Span-ID: {:?}", tags, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Find pet by ID
    fn get_pet_by_id(&self, pet_id: i64, context: &C) -> Box<Future<Item=GetPetByIdResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_pet_by_id({}) - X-Span-ID: {:?}", pet_id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Update an existing pet
    fn update_pet(&self, body: models::Pet, context: &C) -> Box<Future<Item=UpdatePetResponse, Error=ApiError>> {
        let context = context.clone();
        println!("update_pet({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Updates a pet in the store with form data
    fn update_pet_with_form(&self, pet_id: i64, name: Option<String>, status: Option<String>, context: &C) -> Box<Future<Item=UpdatePetWithFormResponse, Error=ApiError>> {
        let context = context.clone();
        println!("update_pet_with_form({}, {:?}, {:?}) - X-Span-ID: {:?}", pet_id, name, status, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// uploads an image
    fn upload_file(&self, pet_id: i64, additional_metadata: Option<String>, file: Box<Future<Item=Option<Box<Stream<Item=Vec<u8>, Error=Error> + Send>>, Error=Error> + Send>, context: &C) -> Box<Future<Item=UploadFileResponse, Error=ApiError>> {
        let context = context.clone();
        println!("upload_file({}, {:?}, ) - X-Span-ID: {:?}", pet_id, additional_metadata, context.get().0.clone());
        let _ = file; //Suppresses unused param warning
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Delete purchase order by ID
    fn delete_order(&self, order_id: i64, context: &C) -> Box<Future<Item=DeleteOrderResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_order({}) - X-Span-ID: {:?}", order_id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Returns pet inventories by status
    fn get_inventory(&self, context: &C) -> Box<Future<Item=GetInventoryResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_inventory() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Find purchase order by ID
    fn get_order_by_id(&self, order_id: i64, context: &C) -> Box<Future<Item=GetOrderByIdResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_order_by_id({}) - X-Span-ID: {:?}", order_id, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Place an order for a pet
    fn place_order(&self, body: models::Order, context: &C) -> Box<Future<Item=PlaceOrderResponse, Error=ApiError>> {
        let context = context.clone();
        println!("place_order({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Create user
    fn create_user(&self, body: models::User, context: &C) -> Box<Future<Item=CreateUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("create_user({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Creates list of users with given input array
    fn create_users_with_array_input(&self, body: &Vec<models::User>, context: &C) -> Box<Future<Item=CreateUsersWithArrayInputResponse, Error=ApiError>> {
        let context = context.clone();
        println!("create_users_with_array_input({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Creates list of users with given input array
    fn create_users_with_list_input(&self, body: &Vec<models::User>, context: &C) -> Box<Future<Item=CreateUsersWithListInputResponse, Error=ApiError>> {
        let context = context.clone();
        println!("create_users_with_list_input({:?}) - X-Span-ID: {:?}", body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Delete user
    fn delete_user(&self, username: String, context: &C) -> Box<Future<Item=DeleteUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("delete_user(\"{}\") - X-Span-ID: {:?}", username, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Get user by user name
    fn get_user_by_name(&self, username: String, context: &C) -> Box<Future<Item=GetUserByNameResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_user_by_name(\"{}\") - X-Span-ID: {:?}", username, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Logs user into the system
    fn login_user(&self, username: String, password: String, context: &C) -> Box<Future<Item=LoginUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("login_user(\"{}\", \"{}\") - X-Span-ID: {:?}", username, password, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Logs out current logged in user session
    fn logout_user(&self, context: &C) -> Box<Future<Item=LogoutUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("logout_user() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Updated user
    fn update_user(&self, username: String, body: models::User, context: &C) -> Box<Future<Item=UpdateUserResponse, Error=ApiError>> {
        let context = context.clone();
        println!("update_user(\"{}\", {:?}) - X-Span-ID: {:?}", username, body, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
