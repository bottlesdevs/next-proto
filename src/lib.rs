pub mod winebridge {
    tonic::include_proto!("winebridge");
}

pub mod bottles {
    pub mod common {
        pub mod v1 {
            tonic::include_proto!("bottles.common.v1");
        }
    }
    pub mod profiles {
        pub mod v1 {
            tonic::include_proto!("bottles.profiles.v1");
        }
    }
    pub mod plugin {
        pub mod v1 {
            tonic::include_proto!("bottles.plugin.v1");
        }
    }
    pub mod steam {
        pub mod v1 {
            tonic::include_proto!("bottles.steam.v1");
        }
    }
    pub mod accounts {
        pub mod v1 {
            tonic::include_proto!("bottles.accounts.v1");
        }
    }
    pub mod registry {
        pub mod v1 {
            tonic::include_proto!("bottles.registry.v1");
        }
    }
    pub mod library {
        pub mod v1 {
            tonic::include_proto!("bottles.library.v1");
        }
    }
    pub mod bottle {
        pub mod v1 {
            tonic::include_proto!("bottles.bottle.v1");
        }
    }
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("bottles_descriptor");
