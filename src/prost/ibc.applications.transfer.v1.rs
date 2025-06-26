// @generated
/// DenomTrace contains the base denomination for ICS20 fungible tokens and the
/// source tracing information path.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomTrace {
    /// path defines the chain of port/channel identifiers used for tracing the
    /// source of the fungible token.
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    /// base denomination of the relayed fungible token.
    #[prost(string, tag="2")]
    pub base_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for DenomTrace {
const NAME: &'static str = "DenomTrace";
const PACKAGE: &'static str = "ibc.applications.transfer.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.applications.transfer.v1.{}", Self::NAME)
            }}
/// Params defines the set of IBC transfer parameters.
/// NOTE: To prevent a single token from being transferred, set the
/// TransfersEnabled parameter to true and then set the bank module's SendEnabled
/// parameter for the denomination to false.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// send_enabled enables or disables all cross-chain token transfers from this
    /// chain.
    #[prost(bool, tag="1")]
    pub send_enabled: bool,
    /// receive_enabled enables or disables all cross-chain token transfers to this
    /// chain.
    #[prost(bool, tag="2")]
    pub receive_enabled: bool,
}
impl ::prost::Name for Params {
const NAME: &'static str = "Params";
const PACKAGE: &'static str = "ibc.applications.transfer.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.applications.transfer.v1.{}", Self::NAME)
            }}
/// MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between
/// ICS20 enabled chains. See ICS Spec here:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransfer {
    /// the port on which the packet will be sent
    #[prost(string, tag="1")]
    pub source_port: ::prost::alloc::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag="2")]
    pub source_channel: ::prost::alloc::string::String,
    /// the tokens to be transferred
    #[prost(message, optional, tag="3")]
    pub token: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// the sender address
    #[prost(string, tag="4")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag="5")]
    pub receiver: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(message, optional, tag="6")]
    pub timeout_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Timeout timestamp in absolute nanoseconds since unix epoch.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag="7")]
    pub timeout_timestamp: u64,
    /// optional memo
    #[prost(string, tag="8")]
    pub memo: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgTransfer {
const NAME: &'static str = "MsgTransfer";
const PACKAGE: &'static str = "ibc.applications.transfer.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.applications.transfer.v1.{}", Self::NAME)
            }}
/// MsgTransferResponse defines the Msg/Transfer response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferResponse {
    /// sequence number of the transfer packet sent
    #[prost(uint64, tag="1")]
    pub sequence: u64,
}
impl ::prost::Name for MsgTransferResponse {
const NAME: &'static str = "MsgTransferResponse";
const PACKAGE: &'static str = "ibc.applications.transfer.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.applications.transfer.v1.{}", Self::NAME)
            }}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// signer address
    #[prost(string, tag="1")]
    pub signer: ::prost::alloc::string::String,
    /// params defines the transfer parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
const NAME: &'static str = "MsgUpdateParams";
const PACKAGE: &'static str = "ibc.applications.transfer.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.applications.transfer.v1.{}", Self::NAME)
            }}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
impl ::prost::Name for MsgUpdateParamsResponse {
const NAME: &'static str = "MsgUpdateParamsResponse";
const PACKAGE: &'static str = "ibc.applications.transfer.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.applications.transfer.v1.{}", Self::NAME)
            }}
include!("ibc.applications.transfer.v1.tonic.rs");
// @@protoc_insertion_point(module)