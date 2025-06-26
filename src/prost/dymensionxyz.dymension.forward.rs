// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HookForwardToHl {
    #[prost(message, optional, tag="1")]
    pub hyperlane_transfer: ::core::option::Option<super::super::super::hyperlane::warp::v1::MsgRemoteTransfer>,
}
impl ::prost::Name for HookForwardToHl {
const NAME: &'static str = "HookForwardToHL";
const PACKAGE: &'static str = "dymensionxyz.dymension.forward";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.forward.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HookForwardToIbc {
    #[prost(message, optional, tag="1")]
    pub transfer: ::core::option::Option<super::super::super::ibc::applications::transfer::v1::MsgTransfer>,
}
impl ::prost::Name for HookForwardToIbc {
const NAME: &'static str = "HookForwardToIBC";
const PACKAGE: &'static str = "dymensionxyz.dymension.forward";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.forward.{}", Self::NAME)
            }}
/// Expected format of metadata received in HL warp route messages
/// There is only one metadata, so we need to share it amongst our applications,
/// so that they can compose and not conflict
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HlMetadata {
    /// optional, can be empty
    #[prost(bytes="vec", tag="1")]
    pub hook_forward_to_ibc: ::prost::alloc::vec::Vec<u8>,
    /// optional, can be empty
    /// see
    /// <https://www.notion.so/dymension/ADR-Kaspa-Bridge-Implementation-206a4a51f86a803980aec7099c826fb4?source=copy_link#208a4a51f86a8093a843cf4b5e903588>
    #[prost(bytes="vec", tag="2")]
    pub kaspa: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for HlMetadata {
const NAME: &'static str = "HLMetadata";
const PACKAGE: &'static str = "dymensionxyz.dymension.forward";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.forward.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventForward {
    /// success?
    #[prost(bool, tag="1")]
    pub ok: bool,
    /// empty if ok is true
    #[prost(string, tag="2")]
    pub err: ::prost::alloc::string::String,
    /// was it actually a forward operation? (maybe not if they dont include
    /// forward memo)
    #[prost(bool, tag="3")]
    pub was_forwarded: bool,
}
impl ::prost::Name for EventForward {
const NAME: &'static str = "EventForward";
const PACKAGE: &'static str = "dymensionxyz.dymension.forward";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.forward.{}", Self::NAME)
            }}
// @@protoc_insertion_point(module)
