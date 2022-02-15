#[path = ""]
pub mod sciobjsdb {
    #[path = ""]
    pub mod api {
        #[path = ""]
        pub mod storage {
            #[path = ""]
            pub mod models {
                #[path = "sciobjsdb.api.storage.models.v1.rs"]
                pub mod v1;
            }
            #[path = ""]
            pub mod services {
                #[path = "sciobjsdb.api.storage.services.v1.rs"]
                pub mod v1;
            }
        }
        #[path = ""]
        pub mod notification {
            #[path = ""]
            pub mod services {
                #[path = "sciobjsdb.api.notification.services.v1.rs"]
                pub mod v1;
            }
        }
    }
}
