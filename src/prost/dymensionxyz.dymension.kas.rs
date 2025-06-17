// @generated
/// Kaspa transaction outpoint
/// <https://github.com/kaspanet/rusty-kaspa/blob/1adeae8e5e2bdf7b65265420d294a356edc6d9e6/consensus/client/src/outpoint.rs#L91>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionOutpoint {
    /// 32 byte hash
    #[prost(bytes="vec", tag="1")]
    pub transaction_id: ::prost::alloc::vec::Vec<u8>,
    /// pointer to the output in the transaction
    #[prost(uint32, tag="2")]
    pub index: u32,
}
impl ::prost::Name for TransactionOutpoint {
const NAME: &'static str = "TransactionOutpoint";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
/// an index into a set of a dispatched Hyperlane withdrawal messages
/// see
/// <https://github.com/dymensionxyz/hyperlane-cosmos/blob/5b73e596185ce009f7d9d412e26294c52e3108a8/x/core/keeper/query_server.go#L39>
/// and
/// <https://github.com/dymensionxyz/hyperlane-cosmos/blob/5b73e596185ce009f7d9d412e26294c52e3108a8/proto/hyperlane/core/v1/query.proto#L88-L92>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawalId {
    /// in stringified hex address format
    #[prost(string, tag="1")]
    pub message_id: ::prost::alloc::string::String,
}
impl ::prost::Name for WithdrawalId {
const NAME: &'static str = "WithdrawalID";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
/// signed by validators to attest to successfully relayed withdrawals
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgressIndication {
    /// current/'old' outpoint the validator sees on the hub
    #[prost(message, optional, tag="1")]
    pub old_outpoint: ::core::option::Option<TransactionOutpoint>,
    /// new outpoint after processing withdrawals
    #[prost(message, optional, tag="2")]
    pub new_outpoint: ::core::option::Option<TransactionOutpoint>,
    /// the processed withdrawals
    #[prost(message, repeated, tag="3")]
    pub processed_withdrawals: ::prost::alloc::vec::Vec<WithdrawalId>,
}
impl ::prost::Name for ProgressIndication {
const NAME: &'static str = "ProgressIndication";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WithdrawalStatus {
    Unspecified = 0,
    Unprocessed = 1,
    Processed = 2,
}
impl WithdrawalStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WithdrawalStatus::Unspecified => "WITHDRAWAL_STATUS_UNSPECIFIED",
            WithdrawalStatus::Unprocessed => "WITHDRAWAL_STATUS_UNPROCESSED",
            WithdrawalStatus::Processed => "WITHDRAWAL_STATUS_PROCESSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WITHDRAWAL_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "WITHDRAWAL_STATUS_UNPROCESSED" => Some(Self::Unprocessed),
            "WITHDRAWAL_STATUS_PROCESSED" => Some(Self::Processed),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBootstrap {
}
impl ::prost::Name for EventBootstrap {
const NAME: &'static str = "EventBootstrap";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdate {
    #[prost(message, optional, tag="1")]
    pub update: ::core::option::Option<ProgressIndication>,
}
impl ::prost::Name for EventUpdate {
const NAME: &'static str = "EventUpdate";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(bool, tag="1")]
    pub bootstrapped: bool,
    #[prost(string, tag="2")]
    pub mailbox: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub ism: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub outpoint: ::core::option::Option<TransactionOutpoint>,
    #[prost(message, repeated, tag="5")]
    pub processed_withdrawals: ::prost::alloc::vec::Vec<WithdrawalId>,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawalStatusRequest {
    #[prost(message, repeated, tag="1")]
    pub withdrawal_id: ::prost::alloc::vec::Vec<WithdrawalId>,
}
impl ::prost::Name for QueryWithdrawalStatusRequest {
const NAME: &'static str = "QueryWithdrawalStatusRequest";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawalStatusResponse {
    #[prost(enumeration="WithdrawalStatus", repeated, packed="false", tag="1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag="2")]
    pub outpoint: ::core::option::Option<TransactionOutpoint>,
}
impl ::prost::Name for QueryWithdrawalStatusResponse {
const NAME: &'static str = "QueryWithdrawalStatusResponse";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutpointRequest {
}
impl ::prost::Name for QueryOutpointRequest {
const NAME: &'static str = "QueryOutpointRequest";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutpointResponse {
    #[prost(message, optional, tag="1")]
    pub outpoint: ::core::option::Option<TransactionOutpoint>,
}
impl ::prost::Name for QueryOutpointResponse {
const NAME: &'static str = "QueryOutpointResponse";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBootstrap {
    /// Authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// the kaspa escrow mailbox
    #[prost(string, tag="2")]
    pub mailbox: ::prost::alloc::string::String,
    /// the kaspa escrow ism
    #[prost(string, tag="3")]
    pub ism: ::prost::alloc::string::String,
    /// the seed kaspa escrow outpoint
    #[prost(message, optional, tag="4")]
    pub outpoint: ::core::option::Option<TransactionOutpoint>,
}
impl ::prost::Name for MsgBootstrap {
const NAME: &'static str = "MsgBootstrap";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBootstrapResponse {
}
impl ::prost::Name for MsgBootstrapResponse {
const NAME: &'static str = "MsgBootstrapResponse";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIndicateProgress {
    #[prost(string, tag="1")]
    pub signer: ::prost::alloc::string::String,
    /// sig verification info
    /// <https://github.com/dymensionxyz/hyperlane-cosmos/blob/89bed40d16e362c92c12166aa0f86f3db42b3db7/x/core/01_interchain_security/types/message_id_multisig_raw.go#L48>
    #[prost(bytes="vec", tag="2")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
    /// what is signed by validators
    #[prost(message, optional, tag="3")]
    pub payload: ::core::option::Option<ProgressIndication>,
}
impl ::prost::Name for MsgIndicateProgress {
const NAME: &'static str = "MsgIndicateProgress";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIndicateProgressResponse {
}
impl ::prost::Name for MsgIndicateProgressResponse {
const NAME: &'static str = "MsgIndicateProgressResponse";
const PACKAGE: &'static str = "dymensionxyz.dymension.kas";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.kas.{}", Self::NAME)
            }}
include!("dymensionxyz.dymension.kas.tonic.rs");
// @@protoc_insertion_point(module)