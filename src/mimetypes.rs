/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for FindPetsByStatus
    lazy_static! {
        pub static ref FIND_PETS_BY_STATUS_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for FindPetsByTags
    lazy_static! {
        pub static ref FIND_PETS_BY_TAGS_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetPetById
    lazy_static! {
        pub static ref GET_PET_BY_ID_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for UploadFile
    lazy_static! {
        pub static ref UPLOAD_FILE_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetInventory
    lazy_static! {
        pub static ref GET_INVENTORY_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetOrderById
    lazy_static! {
        pub static ref GET_ORDER_BY_ID_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for PlaceOrder
    lazy_static! {
        pub static ref PLACE_ORDER_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetUserByName
    lazy_static! {
        pub static ref GET_USER_BY_NAME_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for LoginUser
    lazy_static! {
        pub static ref LOGIN_USER_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for AddPet
    lazy_static! {
        pub static ref ADD_PET: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for UpdatePet
    lazy_static! {
        pub static ref UPDATE_PET: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for UpdatePetWithForm
    lazy_static! {
        pub static ref UPDATE_PET_WITH_FORM: Mime = "application/x-www-form-urlencoded".parse().unwrap();
    }
   /// Create Mime objects for the request content types for PlaceOrder
    lazy_static! {
        pub static ref PLACE_ORDER: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for CreateUser
    lazy_static! {
        pub static ref CREATE_USER: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for CreateUsersWithArrayInput
    lazy_static! {
        pub static ref CREATE_USERS_WITH_ARRAY_INPUT: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for CreateUsersWithListInput
    lazy_static! {
        pub static ref CREATE_USERS_WITH_LIST_INPUT: Mime = "application/json".parse().unwrap();
    }
   /// Create Mime objects for the request content types for UpdateUser
    lazy_static! {
        pub static ref UPDATE_USER: Mime = "application/json".parse().unwrap();
    }

}
