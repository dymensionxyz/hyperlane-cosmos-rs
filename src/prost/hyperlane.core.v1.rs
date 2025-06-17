// @generated
/// EventDispatch ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDispatch {
    /// origin_mailbox_id ...
    #[prost(string, tag="1")]
    pub origin_mailbox_id: ::prost::alloc::string::String,
    /// sender ...
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    /// destination ...
    #[prost(uint32, tag="3")]
    pub destination: u32,
    /// recipient ...
    #[prost(string, tag="4")]
    pub recipient: ::prost::alloc::string::String,
    /// message ...
    #[prost(string, tag="5")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for EventDispatch {
const NAME: &'static str = "EventDispatch";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// EventProcess ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventProcess {
    /// origin_mailbox_id ...
    #[prost(string, tag="1")]
    pub origin_mailbox_id: ::prost::alloc::string::String,
    /// origin ...
    #[prost(uint32, tag="2")]
    pub origin: u32,
    /// sender ...
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
    /// recipient ...
    #[prost(string, tag="4")]
    pub recipient: ::prost::alloc::string::String,
    /// message_id ...
    #[prost(string, tag="5")]
    pub message_id: ::prost::alloc::string::String,
    /// message ...
    #[prost(string, tag="6")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for EventProcess {
const NAME: &'static str = "EventProcess";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// EventCreateMailbox ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateMailbox {
    /// mailbox_id ...
    #[prost(string, tag="1")]
    pub mailbox_id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// default_ism ...
    #[prost(string, tag="3")]
    pub default_ism: ::prost::alloc::string::String,
    /// default_hook ...
    #[prost(string, tag="4")]
    pub default_hook: ::prost::alloc::string::String,
    /// required_hook ...
    #[prost(string, tag="5")]
    pub required_hook: ::prost::alloc::string::String,
    /// local_domain ...
    #[prost(uint32, tag="6")]
    pub local_domain: u32,
}
impl ::prost::Name for EventCreateMailbox {
const NAME: &'static str = "EventCreateMailbox";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// EventSetMailbox ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetMailbox {
    /// mailbox_id ...
    #[prost(string, tag="1")]
    pub mailbox_id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// default_ism ...
    #[prost(string, tag="3")]
    pub default_ism: ::prost::alloc::string::String,
    /// default_hook ...
    #[prost(string, tag="4")]
    pub default_hook: ::prost::alloc::string::String,
    /// new_owner ...
    #[prost(string, tag="5")]
    pub new_owner: ::prost::alloc::string::String,
    /// renounce_ownership ...
    #[prost(bool, tag="6")]
    pub renounce_ownership: bool,
}
impl ::prost::Name for EventSetMailbox {
const NAME: &'static str = "EventSetMailbox";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// Mailbox ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mailbox {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// message_sent ...
    #[prost(uint32, tag="3")]
    pub message_sent: u32,
    /// message_received ...
    #[prost(uint32, tag="4")]
    pub message_received: u32,
    /// default_ism ...
    #[prost(string, tag="5")]
    pub default_ism: ::prost::alloc::string::String,
    /// default_hook
    #[prost(string, tag="6")]
    pub default_hook: ::prost::alloc::string::String,
    /// required_hook
    #[prost(string, tag="7")]
    pub required_hook: ::prost::alloc::string::String,
    /// domain
    #[prost(uint32, tag="8")]
    pub local_domain: u32,
}
impl ::prost::Name for Mailbox {
const NAME: &'static str = "Mailbox";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// GenesisState is the state that must be provided at genesis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// ism_genesis
    #[prost(message, optional, tag="1")]
    pub ism_genesis: ::core::option::Option<super::interchain_security::v1::GenesisState>,
    /// post_dispatch_genesis
    #[prost(message, optional, tag="2")]
    pub post_dispatch_genesis: ::core::option::Option<super::post_dispatch::v1::GenesisState>,
    #[prost(message, repeated, tag="3")]
    pub mailboxes: ::prost::alloc::vec::Vec<Mailbox>,
    #[prost(message, repeated, tag="4")]
    pub messages: ::prost::alloc::vec::Vec<GenesisMailboxMessageWrapper>,
    #[prost(uint64, tag="5")]
    pub ism_sequence: u64,
    #[prost(uint64, tag="6")]
    pub post_dispatch_sequence: u64,
    #[prost(uint64, tag="7")]
    pub app_sequence: u64,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// GenesisMailboxMessageWrapper ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisMailboxMessageWrapper {
    #[prost(uint64, tag="1")]
    pub mailbox_id: u64,
    #[prost(string, tag="2")]
    pub message_id: ::prost::alloc::string::String,
}
impl ::prost::Name for GenesisMailboxMessageWrapper {
const NAME: &'static str = "GenesisMailboxMessageWrapper";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMailboxesRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMailboxesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryMailboxesRequest {
const NAME: &'static str = "QueryMailboxesRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMailboxesResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMailboxesResponse {
    #[prost(message, repeated, tag="1")]
    pub mailboxes: ::prost::alloc::vec::Vec<Mailbox>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryMailboxesResponse {
const NAME: &'static str = "QueryMailboxesResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMailboxRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMailboxRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryMailboxRequest {
const NAME: &'static str = "QueryMailboxRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMailboxResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMailboxResponse {
    #[prost(message, optional, tag="1")]
    pub mailbox: ::core::option::Option<Mailbox>,
}
impl ::prost::Name for QueryMailboxResponse {
const NAME: &'static str = "QueryMailboxResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryDeliveredRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDeliveredRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDeliveredRequest {
const NAME: &'static str = "QueryDeliveredRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryDeliveredResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDeliveredResponse {
    #[prost(bool, tag="1")]
    pub delivered: bool,
}
impl ::prost::Name for QueryDeliveredResponse {
const NAME: &'static str = "QueryDeliveredResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryRecipientIsmRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRecipientIsmRequest {
    #[prost(string, tag="1")]
    pub recipient: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryRecipientIsmRequest {
const NAME: &'static str = "QueryRecipientIsmRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryRecipientIsmResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRecipientIsmResponse {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryRecipientIsmResponse {
const NAME: &'static str = "QueryRecipientIsmResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryVerifyDryRunRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyDryRunRequest {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub gas_limit: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryVerifyDryRunRequest {
const NAME: &'static str = "QueryVerifyDryRunRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryVerifyDryRunResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyDryRunResponse {
    #[prost(bool, tag="1")]
    pub verified: bool,
}
impl ::prost::Name for QueryVerifyDryRunResponse {
const NAME: &'static str = "QueryVerifyDryRunResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryRegisteredISMs ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredIsMs {
}
impl ::prost::Name for QueryRegisteredIsMs {
const NAME: &'static str = "QueryRegisteredISMs";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryRegisteredISMsResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredIsMsResponse {
    #[prost(uint32, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<u32>,
}
impl ::prost::Name for QueryRegisteredIsMsResponse {
const NAME: &'static str = "QueryRegisteredISMsResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryRegisteredHooks ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredHooks {
}
impl ::prost::Name for QueryRegisteredHooks {
const NAME: &'static str = "QueryRegisteredHooks";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryRegisteredHooksResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredHooksResponse {
    #[prost(uint32, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<u32>,
}
impl ::prost::Name for QueryRegisteredHooksResponse {
const NAME: &'static str = "QueryRegisteredHooksResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryRegisteredApps ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredApps {
}
impl ::prost::Name for QueryRegisteredApps {
const NAME: &'static str = "QueryRegisteredApps";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryRegisteredAppsResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegisteredAppsResponse {
    #[prost(uint32, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<u32>,
}
impl ::prost::Name for QueryRegisteredAppsResponse {
const NAME: &'static str = "QueryRegisteredAppsResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMessageIDRequest ...
///
/// <messages,ids> should correspond by index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMessageIdRequest {
    /// in raw format
    #[prost(bytes="vec", repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// in stringified hex address format
    #[prost(string, repeated, tag="2")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryMessageIdRequest {
const NAME: &'static str = "QueryMessageIDRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// error returned instead if any message does not match the id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMessageIdResponse {
}
impl ::prost::Name for QueryMessageIdResponse {
const NAME: &'static str = "QueryMessageIDResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgCreateMailbox ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMailbox {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// local domain
    #[prost(uint32, tag="2")]
    pub local_domain: u32,
    #[prost(string, tag="3")]
    pub default_ism: ::prost::alloc::string::String,
    /// default_hook ...
    #[prost(string, tag="4")]
    pub default_hook: ::prost::alloc::string::String,
    /// required_hook ...
    #[prost(string, tag="5")]
    pub required_hook: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMailbox {
const NAME: &'static str = "MsgCreateMailbox";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgCreateMailboxResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMailboxResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMailboxResponse {
const NAME: &'static str = "MsgCreateMailboxResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgSetMailbox ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMailbox {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// mailbox_id
    #[prost(string, tag="2")]
    pub mailbox_id: ::prost::alloc::string::String,
    /// default_ism ...
    #[prost(string, tag="3")]
    pub default_ism: ::prost::alloc::string::String,
    /// default_hook ...
    #[prost(string, tag="4")]
    pub default_hook: ::prost::alloc::string::String,
    /// required_hook ...
    #[prost(string, tag="5")]
    pub required_hook: ::prost::alloc::string::String,
    /// new_owner ...
    #[prost(string, tag="6")]
    pub new_owner: ::prost::alloc::string::String,
    /// renounce_ownership
    #[prost(bool, tag="7")]
    pub renounce_ownership: bool,
}
impl ::prost::Name for MsgSetMailbox {
const NAME: &'static str = "MsgSetMailbox";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgSetMailboxResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMailboxResponse {
}
impl ::prost::Name for MsgSetMailboxResponse {
const NAME: &'static str = "MsgSetMailboxResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgProcessMessage ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgProcessMessage {
    /// mailbox_id ...
    #[prost(string, tag="1")]
    pub mailbox_id: ::prost::alloc::string::String,
    /// relayer ...
    #[prost(string, tag="2")]
    pub relayer: ::prost::alloc::string::String,
    /// metadata ...
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    /// message ...
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgProcessMessage {
const NAME: &'static str = "MsgProcessMessage";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgProcessMessageResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgProcessMessageResponse {
}
impl ::prost::Name for MsgProcessMessageResponse {
const NAME: &'static str = "MsgProcessMessageResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
include!("hyperlane.core.v1.tonic.rs");
// @@protoc_insertion_point(module)