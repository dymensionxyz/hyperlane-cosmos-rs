// @generated
/// EventCreateMerkleTreeHook ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateMerkleTreeHook {
    /// id ...
    #[prost(string, tag="1")]
    pub merkle_tree_hook_id: ::prost::alloc::string::String,
    /// mailbox_id ...
    #[prost(string, tag="2")]
    pub mailbox_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateMerkleTreeHook {
const NAME: &'static str = "EventCreateMerkleTreeHook";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// EventInsertedIntoTree ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventInsertedIntoTree {
    /// message_id ...
    #[prost(string, tag="1")]
    pub message_id: ::prost::alloc::string::String,
    /// index ...
    #[prost(uint32, tag="2")]
    pub index: u32,
    /// merkle_tree_hook_id ...
    #[prost(string, tag="3")]
    pub merkle_tree_hook_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventInsertedIntoTree {
const NAME: &'static str = "EventInsertedIntoTree";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// EventGasPayment ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventGasPayment {
    /// message_id ...
    #[prost(string, tag="1")]
    pub message_id: ::prost::alloc::string::String,
    /// destination ...
    #[prost(uint32, tag="2")]
    pub destination: u32,
    /// gas_amount ...
    #[prost(string, tag="3")]
    pub gas_amount: ::prost::alloc::string::String,
    /// payment ...
    #[prost(string, tag="4")]
    pub payment: ::prost::alloc::string::String,
    /// igp_id ...
    #[prost(string, tag="5")]
    pub igp_id: ::prost::alloc::string::String,
}
impl ::prost::Name for EventGasPayment {
const NAME: &'static str = "EventGasPayment";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// EventCreateNoopHook ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateNoopHook {
    /// id ...
    #[prost(string, tag="1")]
    pub noop_hook_id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateNoopHook {
const NAME: &'static str = "EventCreateNoopHook";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// EventCreateIgp ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateIgp {
    #[prost(string, tag="1")]
    pub igp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateIgp {
const NAME: &'static str = "EventCreateIgp";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// EventSetIgp ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetIgp {
    #[prost(string, tag="1")]
    pub igp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_owner: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub renounce_ownership: bool,
}
impl ::prost::Name for EventSetIgp {
const NAME: &'static str = "EventSetIgp";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// EventSetDestinationGasConfig ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetDestinationGasConfig {
    #[prost(string, tag="1")]
    pub igp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub remote_domain: u32,
    #[prost(string, tag="5")]
    pub gas_overhead: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub token_exchange_rate: ::prost::alloc::string::String,
}
impl ::prost::Name for EventSetDestinationGasConfig {
const NAME: &'static str = "EventSetDestinationGasConfig";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// EventClaimIgp ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventClaimIgp {
    #[prost(string, tag="1")]
    pub igp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventClaimIgp {
const NAME: &'static str = "EventClaimIgp";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// InterchainGasPaymaster ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterchainGasPaymaster {
    /// id ...
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// denom ...
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
    /// claimable_fees ...
    #[prost(message, repeated, tag="4")]
    pub claimable_fees: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for InterchainGasPaymaster {
const NAME: &'static str = "InterchainGasPaymaster";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// DestinationGasConfig ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationGasConfig {
    /// remote_domain ...
    #[prost(uint32, tag="1")]
    pub remote_domain: u32,
    /// gas_oracle ...
    #[prost(message, optional, tag="2")]
    pub gas_oracle: ::core::option::Option<GasOracle>,
    /// gas_overhead ...
    #[prost(string, tag="3")]
    pub gas_overhead: ::prost::alloc::string::String,
}
impl ::prost::Name for DestinationGasConfig {
const NAME: &'static str = "DestinationGasConfig";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// GasOracle ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasOracle {
    /// token_exchange_rate ...
    #[prost(string, tag="1")]
    pub token_exchange_rate: ::prost::alloc::string::String,
    /// gas_price ...
    #[prost(string, tag="2")]
    pub gas_price: ::prost::alloc::string::String,
}
impl ::prost::Name for GasOracle {
const NAME: &'static str = "GasOracle";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MerkleTreeHook ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleTreeHook {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mailbox_id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    /// tree ...
    #[prost(message, optional, tag="4")]
    pub tree: ::core::option::Option<Tree>,
}
impl ::prost::Name for MerkleTreeHook {
const NAME: &'static str = "MerkleTreeHook";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// Tree represents an incremental merkle tree.
/// Contains current branch and the number of inserted leaves in the tree.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tree {
    /// branch ...
    #[prost(bytes="vec", repeated, tag="1")]
    pub branch: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// count ...
    #[prost(uint32, tag="2")]
    pub count: u32,
}
impl ::prost::Name for Tree {
const NAME: &'static str = "Tree";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// NoopHook ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoopHook {
    /// id ...
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for NoopHook {
const NAME: &'static str = "NoopHook";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// GenesisState defines the post dispatch submodule's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub igps: ::prost::alloc::vec::Vec<InterchainGasPaymaster>,
    #[prost(message, repeated, tag="2")]
    pub igp_gas_configs: ::prost::alloc::vec::Vec<GenesisDestinationGasConfigWrapper>,
    #[prost(message, repeated, tag="3")]
    pub merkle_tree_hooks: ::prost::alloc::vec::Vec<MerkleTreeHook>,
    #[prost(message, repeated, tag="4")]
    pub noop_hooks: ::prost::alloc::vec::Vec<NoopHook>,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// GenesisDestinationGasConfigWrapper ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisDestinationGasConfigWrapper {
    /// remote_domain ...
    #[prost(uint32, tag="1")]
    pub remote_domain: u32,
    /// gas_oracle ...
    #[prost(message, optional, tag="2")]
    pub gas_oracle: ::core::option::Option<GasOracle>,
    /// gas_overhead ...
    #[prost(string, tag="3")]
    pub gas_overhead: ::prost::alloc::string::String,
    /// igp_id is required for the Genesis handling.
    #[prost(uint64, tag="4")]
    pub igp_id: u64,
}
impl ::prost::Name for GenesisDestinationGasConfigWrapper {
const NAME: &'static str = "GenesisDestinationGasConfigWrapper";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryIgpsRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIgpsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryIgpsRequest {
const NAME: &'static str = "QueryIgpsRequest";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryIgpsResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIgpsResponse {
    #[prost(message, repeated, tag="1")]
    pub igps: ::prost::alloc::vec::Vec<InterchainGasPaymaster>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryIgpsResponse {
const NAME: &'static str = "QueryIgpsResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryIgpRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIgpRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryIgpRequest {
const NAME: &'static str = "QueryIgpRequest";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryIgpResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIgpResponse {
    #[prost(message, optional, tag="1")]
    pub igp: ::core::option::Option<InterchainGasPaymaster>,
}
impl ::prost::Name for QueryIgpResponse {
const NAME: &'static str = "QueryIgpResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryDestinationGasConfigsRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDestinationGasConfigsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryDestinationGasConfigsRequest {
const NAME: &'static str = "QueryDestinationGasConfigsRequest";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryDestinationGasConfigsResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDestinationGasConfigsResponse {
    #[prost(message, repeated, tag="1")]
    pub destination_gas_configs: ::prost::alloc::vec::Vec<DestinationGasConfig>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryDestinationGasConfigsResponse {
const NAME: &'static str = "QueryDestinationGasConfigsResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryQuoteGasPaymentRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryQuoteGasPaymentRequest {
    #[prost(string, tag="1")]
    pub igp_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub destination_domain: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub gas_limit: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryQuoteGasPaymentRequest {
const NAME: &'static str = "QueryQuoteGasPaymentRequest";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryQuoteGasPaymentResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryQuoteGasPaymentResponse {
    #[prost(message, repeated, tag="1")]
    pub gas_payment: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryQuoteGasPaymentResponse {
const NAME: &'static str = "QueryQuoteGasPaymentResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryMerkleTreeHooksRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMerkleTreeHooksRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryMerkleTreeHooksRequest {
const NAME: &'static str = "QueryMerkleTreeHooksRequest";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryMerkleTreeHooksResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMerkleTreeHooksResponse {
    #[prost(message, repeated, tag="1")]
    pub merkle_tree_hooks: ::prost::alloc::vec::Vec<WrappedMerkleTreeHookResponse>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryMerkleTreeHooksResponse {
const NAME: &'static str = "QueryMerkleTreeHooksResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryMerkleTreeHookRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMerkleTreeHookRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryMerkleTreeHookRequest {
const NAME: &'static str = "QueryMerkleTreeHookRequest";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryMerkleTreeHookResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMerkleTreeHookResponse {
    #[prost(message, optional, tag="1")]
    pub merkle_tree_hook: ::core::option::Option<WrappedMerkleTreeHookResponse>,
}
impl ::prost::Name for QueryMerkleTreeHookResponse {
const NAME: &'static str = "QueryMerkleTreeHookResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// WrappedMerkleTreeHookResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WrappedMerkleTreeHookResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mailbox_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub merkle_tree: ::core::option::Option<TreeResponse>,
}
impl ::prost::Name for WrappedMerkleTreeHookResponse {
const NAME: &'static str = "WrappedMerkleTreeHookResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// TreeResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TreeResponse {
    /// leafs ...
    #[prost(bytes="vec", repeated, tag="1")]
    pub leafs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// count ...
    #[prost(uint32, tag="2")]
    pub count: u32,
    /// root ...
    #[prost(bytes="vec", tag="3")]
    pub root: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for TreeResponse {
const NAME: &'static str = "TreeResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryNoopHookRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNoopHookRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryNoopHookRequest {
const NAME: &'static str = "QueryNoopHookRequest";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryNoopHookResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNoopHookResponse {
    #[prost(message, optional, tag="1")]
    pub noop_hook: ::core::option::Option<NoopHook>,
}
impl ::prost::Name for QueryNoopHookResponse {
const NAME: &'static str = "QueryNoopHookResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryNoopHooksRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNoopHooksRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryNoopHooksRequest {
const NAME: &'static str = "QueryNoopHooksRequest";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// QueryNoopHooksResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNoopHooksResponse {
    #[prost(message, repeated, tag="1")]
    pub noop_hooks: ::prost::alloc::vec::Vec<NoopHook>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryNoopHooksResponse {
const NAME: &'static str = "QueryNoopHooksResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgCreateIgp ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateIgp {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// denom
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateIgp {
const NAME: &'static str = "MsgCreateIgp";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgCreateIgpResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateIgpResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateIgpResponse {
const NAME: &'static str = "MsgCreateIgpResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgSetIgpOwner ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetIgpOwner {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// igp_id
    #[prost(string, tag="2")]
    pub igp_id: ::prost::alloc::string::String,
    /// new_owner
    #[prost(string, tag="3")]
    pub new_owner: ::prost::alloc::string::String,
    /// renounce_ownership
    #[prost(bool, tag="4")]
    pub renounce_ownership: bool,
}
impl ::prost::Name for MsgSetIgpOwner {
const NAME: &'static str = "MsgSetIgpOwner";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgCreateIgpResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetIgpOwnerResponse {
}
impl ::prost::Name for MsgSetIgpOwnerResponse {
const NAME: &'static str = "MsgSetIgpOwnerResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgSetDestinationGasConfig ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDestinationGasConfig {
    /// owner ...
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// igp_id ...
    #[prost(string, tag="2")]
    pub igp_id: ::prost::alloc::string::String,
    /// destination_gas_config ...
    #[prost(message, optional, tag="3")]
    pub destination_gas_config: ::core::option::Option<DestinationGasConfig>,
}
impl ::prost::Name for MsgSetDestinationGasConfig {
const NAME: &'static str = "MsgSetDestinationGasConfig";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgSetDestinationGasConfigResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDestinationGasConfigResponse {
}
impl ::prost::Name for MsgSetDestinationGasConfigResponse {
const NAME: &'static str = "MsgSetDestinationGasConfigResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgPayForGas ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPayForGas {
    /// sender ...
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// igp_id ...
    #[prost(string, tag="2")]
    pub igp_id: ::prost::alloc::string::String,
    /// message_id ...
    #[prost(string, tag="3")]
    pub message_id: ::prost::alloc::string::String,
    /// destination_domain ...
    #[prost(uint32, tag="4")]
    pub destination_domain: u32,
    /// gas_limit ...
    #[prost(string, tag="5")]
    pub gas_limit: ::prost::alloc::string::String,
    /// amount ...
    #[prost(message, optional, tag="6")]
    pub amount: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgPayForGas {
const NAME: &'static str = "MsgPayForGas";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgPayForGasResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPayForGasResponse {
}
impl ::prost::Name for MsgPayForGasResponse {
const NAME: &'static str = "MsgPayForGasResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgClaim ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaim {
    /// sender ...
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// igp_id ...
    #[prost(string, tag="2")]
    pub igp_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgClaim {
const NAME: &'static str = "MsgClaim";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgClaimResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimResponse {
}
impl ::prost::Name for MsgClaimResponse {
const NAME: &'static str = "MsgClaimResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgMerkleTreeHook ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMerkleTreeHook {
    /// sender ...
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mailbox_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMerkleTreeHook {
const NAME: &'static str = "MsgCreateMerkleTreeHook";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgCreateMerkleTreeHookResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMerkleTreeHookResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMerkleTreeHookResponse {
const NAME: &'static str = "MsgCreateMerkleTreeHookResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgMerkleTreeHook ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateNoopHook {
    /// sender ...
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateNoopHook {
const NAME: &'static str = "MsgCreateNoopHook";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
/// MsgCreateMerkleTreeHookResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateNoopHookResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateNoopHookResponse {
const NAME: &'static str = "MsgCreateNoopHookResponse";
const PACKAGE: &'static str = "hyperlane.core.post_dispatch.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.post_dispatch.v1.{}", Self::NAME)
            }}
include!("hyperlane.core.post_dispatch.v1.tonic.rs");
// @@protoc_insertion_point(module)