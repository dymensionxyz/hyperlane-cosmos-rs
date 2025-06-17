pub use cosmrs::proto::cosmos;
pub use prost;
pub use tendermint_proto as tendermint;
pub use tendermint_proto::google::protobuf::{Any, Timestamp};

pub mod hyperlane {
    pub mod core {
        pub mod v1 {
            include!("prost/hyperlane.core.v1.rs");
        }
        pub mod interchain_security {
            pub mod v1 {
                include!("prost/hyperlane.core.interchain_security.v1.rs");
            }
        }
        pub mod post_dispatch {
            pub mod v1 {
                include!("prost/hyperlane.core.post_dispatch.v1.rs");
            }
        }
    }

    pub mod warp {
        pub mod v1 {
            include!("prost/hyperlane.warp.v1.rs");
        }
    }
}

pub mod dymensionxyz {
    pub mod dymension {
        pub mod kas {
            include!("prost/dymensionxyz.dymension.kas.rs");
        }
    }
}
