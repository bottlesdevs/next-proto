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
    pub mod store {
        pub mod v1 {
            tonic::include_proto!("bottles.store.v1");
        }
    }
    pub mod library {
        pub mod v1 {
            tonic::include_proto!("bottles.library.v1");
        }
    }
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("bottles_descriptor");
