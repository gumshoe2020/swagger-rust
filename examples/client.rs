#![allow(missing_docs, unused_variables, trivial_casts)]

extern crate swagger_client;
#[allow(unused_extern_crates)]
extern crate futures;
#[allow(unused_extern_crates)]
#[macro_use]
extern crate swagger;
#[allow(unused_extern_crates)]
extern crate uuid;
extern crate clap;
extern crate tokio_core;

use swagger::{ContextBuilder, EmptyContext, XSpanIdString, Has, Push, AuthData};

#[allow(unused_imports)]
use futures::{Future, future, Stream, stream};
use tokio_core::reactor;
#[allow(unused_imports)]
use swagger_client::{ApiNoContext, ContextWrapperExt,
                      ApiError,
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
use clap::{App, Arg};

fn main() {
    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
    "DeletePet",
    "FindPetsByStatus",
    "FindPetsByTags",
    "GetPetById",
    "UpdatePetWithForm",
    "UploadFile",
    "DeleteOrder",
    "GetInventory",
    "GetOrderById",
    "CreateUsersWithArrayInput",
    "CreateUsersWithListInput",
    "DeleteUser",
    "GetUserByName",
    "LoginUser",
    "LogoutUser",
])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("petstore.swagger.io")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let mut core = reactor::Core::new().unwrap();
    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());
    let client = if matches.is_present("https") {
        // Using Simple HTTPS
        swagger_client::Client::try_new_https(core.handle(), &base_url, "examples/ca.pem")
            .expect("Failed to create HTTPS client")
    } else {
        // Using HTTP
        swagger_client::Client::try_new_http(core.handle(), &base_url)
            .expect("Failed to create HTTP client")
    };

    let context: make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString) =
        make_context!(ContextBuilder, EmptyContext, None, XSpanIdString(self::uuid::Uuid::new_v4().to_string()));
    let client = client.with_context(context);

    match matches.value_of("operation") {

        // Disabled because there's no example.
        // Some("AddPet") => {
        //     let result = core.run(client.add_pet(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        Some("DeletePet") => {
            let result = core.run(client.delete_pet(789, Some("api_key_example".to_string())));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("FindPetsByStatus") => {
            let result = core.run(client.find_pets_by_status(&Vec::new()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("FindPetsByTags") => {
            let result = core.run(client.find_pets_by_tags(&Vec::new()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetPetById") => {
            let result = core.run(client.get_pet_by_id(789));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("UpdatePet") => {
        //     let result = core.run(client.update_pet(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        Some("UpdatePetWithForm") => {
            let result = core.run(client.update_pet_with_form(789, Some("name_example".to_string()), Some("status_example".to_string())));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("UploadFile") => {
            let result = core.run(client.upload_file(789, Some("additional_metadata_example".to_string()), Box::new(future::ok(Some(Box::new(stream::once(Ok(b"hello".to_vec()))) as Box<Stream<Item=_, Error=_> + Send>))) as Box<Future<Item=_, Error=_> + Send>));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("DeleteOrder") => {
            let result = core.run(client.delete_order(789));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetInventory") => {
            let result = core.run(client.get_inventory());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetOrderById") => {
            let result = core.run(client.get_order_by_id(789));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("PlaceOrder") => {
        //     let result = core.run(client.place_order(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        // Disabled because there's no example.
        // Some("CreateUser") => {
        //     let result = core.run(client.create_user(???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        Some("CreateUsersWithArrayInput") => {
            let result = core.run(client.create_users_with_array_input(&Vec::new()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("CreateUsersWithListInput") => {
            let result = core.run(client.create_users_with_list_input(&Vec::new()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("DeleteUser") => {
            let result = core.run(client.delete_user("username_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("GetUserByName") => {
            let result = core.run(client.get_user_by_name("username_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("LoginUser") => {
            let result = core.run(client.login_user("username_example".to_string(), "password_example".to_string()));
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        Some("LogoutUser") => {
            let result = core.run(client.logout_user());
            println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
         },

        // Disabled because there's no example.
        // Some("UpdateUser") => {
        //     let result = core.run(client.update_user("username_example".to_string(), ???));
        //     println!("{:?} (X-Span-ID: {:?})", result, (client.context() as &Has<XSpanIdString>).get().clone());
        //  },

        _ => {
            panic!("Invalid operation provided")
        }
    }
}

