#![allow(unknown_lints)]
#[path = ""]
pub mod aruna {
    #[path = ""]
    pub mod api {
        #[path = ""]
        pub mod storage {
            #[path = ""]
            pub mod models {
                #[path = "aruna.api.storage.models.v2.rs"]
                pub mod v2;
            }
            #[path = ""]
            pub mod services {
                #[path = "aruna.api.storage.services.v2.rs"]
                pub mod v2;
            }
        }
        #[path = ""]
        pub mod notification {
            #[path = ""]
            pub mod services {
                #[path = "aruna.api.notification.services.v2.rs"]
                pub mod v2;
            }
        }
        #[path = ""]
        pub mod dataproxy {
            #[path = ""]
            pub mod services {
                #[path = "aruna.api.dataproxy.services.v2.rs"]
                pub mod v2;
            }
        }
        #[path = ""]
        pub mod hooks {
            #[path = ""]
            pub mod services {
                #[path = "aruna.api.hooks.services.v2.rs"]
                pub mod v2;
            }
        }
        #[path = ""]
        pub mod health {
            #[path = "aruna.api.health.v2.rs"]
            pub mod v2;
        }
    }
}
