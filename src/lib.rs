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
}
