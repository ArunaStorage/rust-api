#![allow(unknown_lints)]
#[path = ""]
pub mod aruna {
    #[path = ""]
    pub mod api {
        #[path = ""]
        pub mod storage {
            #[path = ""]
            pub mod models {
                #[path = "aruna.api.storage.models.v1.rs"]
                pub mod v1;
            }
            #[path = ""]
            pub mod services {
                #[path = "aruna.api.storage.services.v1.rs"]
                pub mod v1;
            }
        }
        #[path = ""]
        pub mod internal {
            #[path = "aruna.api.internal.v1.rs"]
            pub mod v1;
        }
        #[path = ""]
        pub mod notification {
            #[path = ""]
            pub mod services {
                #[path = "aruna.api.notification.services.v1.rs"]
                pub mod v1;
            }
        }
    }
    #[path = ""]
    pub mod hooks {
        #[path = ""]
        pub mod services {
            #[path = "aruna.api.hooks.services.v1.rs"]
            pub mod v1;
        }
    }
}
pub mod implements;
