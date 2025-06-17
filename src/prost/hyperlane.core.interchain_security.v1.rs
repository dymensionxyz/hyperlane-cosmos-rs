// @generated
/// EventCreateNoopIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateNoopIsm {
    /// ism_id ...
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateNoopIsm {
const NAME: &'static str = "EventCreateNoopIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// EventCreateMerkleRootMultisigIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateMerkleRootMultisigIsm {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="4")]
    pub threshold: u32,
}
impl ::prost::Name for EventCreateMerkleRootMultisigIsm {
const NAME: &'static str = "EventCreateMerkleRootMultisigIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// EventCreateMessageIdMultisigIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateMessageIdMultisigIsm {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="4")]
    pub threshold: u32,
}
impl ::prost::Name for EventCreateMessageIdMultisigIsm {
const NAME: &'static str = "EventCreateMessageIdMultisigIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// DYMENSION: dupe of non raw
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateMessageIdMultisigIsmRaw {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="4")]
    pub threshold: u32,
}
impl ::prost::Name for EventCreateMessageIdMultisigIsmRaw {
const NAME: &'static str = "EventCreateMessageIdMultisigIsmRaw";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// EventCreateMessageIdMultisigIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAnnounceStorageLocation {
    #[prost(string, tag="1")]
    pub mailbox_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub validator: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub storage_location: ::prost::alloc::string::String,
}
impl ::prost::Name for EventAnnounceStorageLocation {
const NAME: &'static str = "EventAnnounceStorageLocation";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// EventSetRoutingIsmDomain ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetRoutingIsmDomain {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub route_ism_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub route_domain: u32,
}
impl ::prost::Name for EventSetRoutingIsmDomain {
const NAME: &'static str = "EventSetRoutingIsmDomain";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// EventRemoveRoutingIsmDomain ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRemoveRoutingIsmDomain {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub route_domain: u32,
}
impl ::prost::Name for EventRemoveRoutingIsmDomain {
const NAME: &'static str = "EventRemoveRoutingIsmDomain";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// EventRemoveRoutingIsmDomain ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetRoutingIsm {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_owner: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub renounce_ownership: bool,
}
impl ::prost::Name for EventSetRoutingIsm {
const NAME: &'static str = "EventSetRoutingIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// EventCreateMessageIdMultisigIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateRoutingIsm {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateRoutingIsm {
const NAME: &'static str = "EventCreateRoutingIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// GenesisState defines the 01_interchain_security submodule's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// accounts are the accounts present at genesis.
    #[prost(message, repeated, tag="1")]
    pub isms: ::prost::alloc::vec::Vec<::tendermint_proto::google::protobuf::Any>,
    #[prost(message, repeated, tag="2")]
    pub validator_storage_locations: ::prost::alloc::vec::Vec<GenesisValidatorStorageLocationWrapper>,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// GenesisValidatorStorageLocationWrapper stores the information for
/// validator, mailbox and storage-location which validators have announced
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisValidatorStorageLocationWrapper {
    #[prost(uint64, tag="1")]
    pub mailbox_id: u64,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub index: u64,
    #[prost(string, tag="4")]
    pub storage_location: ::prost::alloc::string::String,
}
impl ::prost::Name for GenesisValidatorStorageLocationWrapper {
const NAME: &'static str = "GenesisValidatorStorageLocationWrapper";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// QueryIsmsRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsmsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryIsmsRequest {
const NAME: &'static str = "QueryIsmsRequest";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// QueryIsmsResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsmsResponse {
    #[prost(message, repeated, tag="1")]
    pub isms: ::prost::alloc::vec::Vec<::tendermint_proto::google::protobuf::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryIsmsResponse {
const NAME: &'static str = "QueryIsmsResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// QueryIsmRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsmRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryIsmRequest {
const NAME: &'static str = "QueryIsmRequest";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// QueryIsmResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsmResponse {
    #[prost(message, optional, tag="1")]
    pub ism: ::core::option::Option<::tendermint_proto::google::protobuf::Any>,
}
impl ::prost::Name for QueryIsmResponse {
const NAME: &'static str = "QueryIsmResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// QueryAnnouncedStorageLocationsRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnouncedStorageLocationsRequest {
    #[prost(string, tag="1")]
    pub mailbox_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAnnouncedStorageLocationsRequest {
const NAME: &'static str = "QueryAnnouncedStorageLocationsRequest";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// QueryAnnouncedStorageLocationsResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnouncedStorageLocationsResponse {
    #[prost(string, repeated, tag="1")]
    pub storage_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryAnnouncedStorageLocationsResponse {
const NAME: &'static str = "QueryAnnouncedStorageLocationsResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// QueryLatestAnnouncedStorageLocationRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestAnnouncedStorageLocationRequest {
    #[prost(string, tag="1")]
    pub mailbox_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLatestAnnouncedStorageLocationRequest {
const NAME: &'static str = "QueryLatestAnnouncedStorageLocationRequest";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// QueryLatestAnnouncedStorageLocationResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLatestAnnouncedStorageLocationResponse {
    #[prost(string, tag="1")]
    pub storage_location: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLatestAnnouncedStorageLocationResponse {
const NAME: &'static str = "QueryLatestAnnouncedStorageLocationResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// Route
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// ism ...
    #[prost(string, tag="1")]
    pub ism: ::prost::alloc::string::String,
    /// domain ...
    #[prost(uint32, tag="2")]
    pub domain: u32,
}
impl ::prost::Name for Route {
const NAME: &'static str = "Route";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// Routing ISM ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingIsm {
    /// id ...
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// Routes associated with the Routing ISM.
    /// These are stored directly within the ISM to simplify the design,
    /// as the number of routes is expected to remain small.
    /// This approach avoids the added complexity of managing a separate
    /// collection.
    #[prost(message, repeated, tag="3")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
}
impl ::prost::Name for RoutingIsm {
const NAME: &'static str = "RoutingISM";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MessageIdMultisigISM ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageIdMultisigIsm {
    /// id ...
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// validators
    /// these are 20 byte long ethereum style addresses
    #[prost(string, repeated, tag="3")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// threshold ...
    #[prost(uint32, tag="4")]
    pub threshold: u32,
}
impl ::prost::Name for MessageIdMultisigIsm {
const NAME: &'static str = "MessageIdMultisigISM";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// DYMENSION
/// MessageIdMultisigISM has unnecessary code because of it's implicit merkle
/// tree
/// (<https://github.com/dymensionxyz/hyperlane-monorepo/blob/00b8642100af822767ceb605bc2627de7ddde610/rust/main/hyperlane-core/src/types/checkpoint.rs#L32-L51>)
/// Otherwise it's a dupe of the non raw version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageIdMultisigIsmRaw {
    /// id ...
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// validators
    /// these are 20 byte long ethereum style addresses
    #[prost(string, repeated, tag="3")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// threshold ...
    #[prost(uint32, tag="4")]
    pub threshold: u32,
}
impl ::prost::Name for MessageIdMultisigIsmRaw {
const NAME: &'static str = "MessageIdMultisigISMRaw";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MerkleRootMultisigISM ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleRootMultisigIsm {
    /// XXX ...
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// validators
    /// these are 20 byte long ethereum style addresses
    #[prost(string, repeated, tag="3")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// threshold ...
    #[prost(uint32, tag="4")]
    pub threshold: u32,
}
impl ::prost::Name for MerkleRootMultisigIsm {
const NAME: &'static str = "MerkleRootMultisigISM";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// NoopISM ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoopIsm {
    /// id ...
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for NoopIsm {
const NAME: &'static str = "NoopISM";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgCreateMessageIdMultisigIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMessageIdMultisigIsm {
    /// creator is the message sender.
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    /// validators
    /// these are 20 byte long ethereum style addresses
    #[prost(string, repeated, tag="2")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// threshold ...
    #[prost(uint32, tag="3")]
    pub threshold: u32,
}
impl ::prost::Name for MsgCreateMessageIdMultisigIsm {
const NAME: &'static str = "MsgCreateMessageIdMultisigIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgCreateMessageIdMultisigIsmResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMessageIdMultisigIsmResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMessageIdMultisigIsmResponse {
const NAME: &'static str = "MsgCreateMessageIdMultisigIsmResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// DYMENSION: dupe of non raw
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMessageIdMultisigIsmRaw {
    /// creator is the message sender.
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    /// validators
    /// these are 20 byte long ethereum style addresses
    #[prost(string, repeated, tag="2")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// threshold ...
    #[prost(uint32, tag="3")]
    pub threshold: u32,
}
impl ::prost::Name for MsgCreateMessageIdMultisigIsmRaw {
const NAME: &'static str = "MsgCreateMessageIdMultisigIsmRaw";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// DYMENSION: dupe of non raw
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMessageIdMultisigIsmRawResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMessageIdMultisigIsmRawResponse {
const NAME: &'static str = "MsgCreateMessageIdMultisigIsmRawResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgCreateMultisigIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMerkleRootMultisigIsm {
    /// creator is the message sender.
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    /// validators
    /// these are 20 byte long ethereum style addresses
    #[prost(string, repeated, tag="2")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// threshold ...
    #[prost(uint32, tag="3")]
    pub threshold: u32,
}
impl ::prost::Name for MsgCreateMerkleRootMultisigIsm {
const NAME: &'static str = "MsgCreateMerkleRootMultisigIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgCreateMultisigIsmResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMerkleRootMultisigIsmResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMerkleRootMultisigIsmResponse {
const NAME: &'static str = "MsgCreateMerkleRootMultisigIsmResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgCreateNoopIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateNoopIsm {
    /// creator is the message sender.
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateNoopIsm {
const NAME: &'static str = "MsgCreateNoopIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgCreateNoopIsmResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateNoopIsmResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateNoopIsmResponse {
const NAME: &'static str = "MsgCreateNoopIsmResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgAnnounceValidator ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAnnounceValidator {
    /// validator ...
    #[prost(string, tag="1")]
    pub validator: ::prost::alloc::string::String,
    /// storage_location ...
    #[prost(string, tag="2")]
    pub storage_location: ::prost::alloc::string::String,
    /// signature ...
    #[prost(string, tag="3")]
    pub signature: ::prost::alloc::string::String,
    /// mailbox_id ...
    #[prost(string, tag="4")]
    pub mailbox_id: ::prost::alloc::string::String,
    /// creator ...
    #[prost(string, tag="5")]
    pub creator: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgAnnounceValidator {
const NAME: &'static str = "MsgAnnounceValidator";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgAnnounceValidatorResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAnnounceValidatorResponse {
}
impl ::prost::Name for MsgAnnounceValidatorResponse {
const NAME: &'static str = "MsgAnnounceValidatorResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgCreateRoutingIsm ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateRoutingIsm {
    /// creator ...
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    /// routes ...
    #[prost(message, repeated, tag="2")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
}
impl ::prost::Name for MsgCreateRoutingIsm {
const NAME: &'static str = "MsgCreateRoutingIsm";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgCreateRoutingIsmResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateRoutingIsmResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateRoutingIsmResponse {
const NAME: &'static str = "MsgCreateRoutingIsmResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgSetRoutingIsmDomain ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetRoutingIsmDomain {
    /// ism_id ...
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    /// route ...
    #[prost(message, optional, tag="2")]
    pub route: ::core::option::Option<Route>,
    /// owner ...
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSetRoutingIsmDomain {
const NAME: &'static str = "MsgSetRoutingIsmDomain";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgSetRoutingIsmDomainResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetRoutingIsmDomainResponse {
}
impl ::prost::Name for MsgSetRoutingIsmDomainResponse {
const NAME: &'static str = "MsgSetRoutingIsmDomainResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgRemoveRoutingIsmDomain ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveRoutingIsmDomain {
    /// ism_id ...
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    /// domain ...
    #[prost(uint32, tag="2")]
    pub domain: u32,
    /// owner ...
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRemoveRoutingIsmDomain {
const NAME: &'static str = "MsgRemoveRoutingIsmDomain";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgRemoveRoutingIsmDomainResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveRoutingIsmDomainResponse {
}
impl ::prost::Name for MsgRemoveRoutingIsmDomainResponse {
const NAME: &'static str = "MsgRemoveRoutingIsmDomainResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgUpdateRoutingIsmOwner ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateRoutingIsmOwner {
    /// ism_id ...
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// new owner
    #[prost(string, tag="3")]
    pub new_owner: ::prost::alloc::string::String,
    /// renounce_ownership
    #[prost(bool, tag="4")]
    pub renounce_ownership: bool,
}
impl ::prost::Name for MsgUpdateRoutingIsmOwner {
const NAME: &'static str = "MsgUpdateRoutingIsmOwner";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
/// MsgUpdateRoutingIsmOwnerResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateRoutingIsmOwnerResponse {
}
impl ::prost::Name for MsgUpdateRoutingIsmOwnerResponse {
const NAME: &'static str = "MsgUpdateRoutingIsmOwnerResponse";
const PACKAGE: &'static str = "hyperlane.core.interchain_security.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.interchain_security.v1.{}", Self::NAME)
            }}
include!("hyperlane.core.interchain_security.v1.tonic.rs");
// @@protoc_insertion_point(module)