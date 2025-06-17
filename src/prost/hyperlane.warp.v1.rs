// @generated
/// EventCreateSyntheticToken ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateSyntheticToken {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub origin_mailbox: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub origin_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateSyntheticToken {
const NAME: &'static str = "EventCreateSyntheticToken";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// EventCreateCollateralToken ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateCollateralToken {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub origin_mailbox: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub origin_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for EventCreateCollateralToken {
const NAME: &'static str = "EventCreateCollateralToken";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// EventSetToken ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetToken {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_owner: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub renounce_ownership: bool,
}
impl ::prost::Name for EventSetToken {
const NAME: &'static str = "EventSetToken";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// EventEnrollRemoteRouter ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventEnrollRemoteRouter {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub receiver_domain: u32,
    #[prost(string, tag="4")]
    pub receiver_contract: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub gas: ::prost::alloc::string::String,
}
impl ::prost::Name for EventEnrollRemoteRouter {
const NAME: &'static str = "EventEnrollRemoteRouter";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// EventUnrollRemoteRouter ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUnrollRemoteRouter {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub receiver_domain: u32,
}
impl ::prost::Name for EventUnrollRemoteRouter {
const NAME: &'static str = "EventUnrollRemoteRouter";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// EventSendRemoteTransfer ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSendRemoteTransfer {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub destination_domain: u32,
    #[prost(string, tag="4")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventSendRemoteTransfer {
const NAME: &'static str = "EventSendRemoteTransfer";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// EventReceiveRemoteTransfer ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventReceiveRemoteTransfer {
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub origin_domain: u32,
    #[prost(string, tag="4")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for EventReceiveRemoteTransfer {
const NAME: &'static str = "EventReceiveRemoteTransfer";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// Params
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
}
impl ::prost::Name for Params {
const NAME: &'static str = "Params";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// HypToken ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HypToken {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(enumeration="HypTokenType", tag="3")]
    pub token_type: i32,
    #[prost(string, tag="4")]
    pub origin_mailbox: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub origin_denom: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub collateral_balance: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub ism_id: ::prost::alloc::string::String,
}
impl ::prost::Name for HypToken {
const NAME: &'static str = "HypToken";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// RemoteRouter ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteRouter {
    #[prost(uint32, tag="1")]
    pub receiver_domain: u32,
    #[prost(string, tag="2")]
    pub receiver_contract: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub gas: ::prost::alloc::string::String,
}
impl ::prost::Name for RemoteRouter {
const NAME: &'static str = "RemoteRouter";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// HypTokenType ...
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HypTokenType {
    /// HYP_TOKEN_TYPE_UNSPECIFIED ...
    Unspecified = 0,
    /// HYP_TOKEN_TYPE_COLLATERAL ...
    Collateral = 1,
    /// HYP_TOKEN_TYPE_SYNTHETIC ...
    Synthetic = 2,
}
impl HypTokenType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HypTokenType::Unspecified => "HYP_TOKEN_TYPE_UNSPECIFIED",
            HypTokenType::Collateral => "HYP_TOKEN_TYPE_COLLATERAL",
            HypTokenType::Synthetic => "HYP_TOKEN_TYPE_SYNTHETIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HYP_TOKEN_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "HYP_TOKEN_TYPE_COLLATERAL" => Some(Self::Collateral),
            "HYP_TOKEN_TYPE_SYNTHETIC" => Some(Self::Synthetic),
            _ => None,
        }
    }
}
/// GenesisState is the state that must be provided at genesis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub tokens: ::prost::alloc::vec::Vec<HypToken>,
    #[prost(message, repeated, tag="3")]
    pub remote_routers: ::prost::alloc::vec::Vec<GenesisRemoteRouterWrapper>,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// GenesisRemoteRouterWrapper ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisRemoteRouterWrapper {
    #[prost(uint64, tag="1")]
    pub token_id: u64,
    #[prost(message, optional, tag="2")]
    pub remote_router: ::core::option::Option<RemoteRouter>,
}
impl ::prost::Name for GenesisRemoteRouterWrapper {
const NAME: &'static str = "GenesisRemoteRouterWrapper";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryTokensRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokensRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryTokensRequest {
const NAME: &'static str = "QueryTokensRequest";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryTokensResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokensResponse {
    /// params defines the parameters of the module.
    #[prost(message, repeated, tag="1")]
    pub tokens: ::prost::alloc::vec::Vec<WrappedHypToken>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryTokensResponse {
const NAME: &'static str = "QueryTokensResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryTokenRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryTokenRequest {
const NAME: &'static str = "QueryTokenRequest";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryTokenResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenResponse {
    #[prost(message, optional, tag="1")]
    pub token: ::core::option::Option<WrappedHypToken>,
}
impl ::prost::Name for QueryTokenResponse {
const NAME: &'static str = "QueryTokenResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// WrappedHypToken
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WrappedHypToken {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(enumeration="HypTokenType", tag="3")]
    pub token_type: i32,
    #[prost(string, tag="4")]
    pub origin_mailbox: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub origin_denom: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub ism_id: ::prost::alloc::string::String,
}
impl ::prost::Name for WrappedHypToken {
const NAME: &'static str = "WrappedHypToken";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryBridgedSupplyRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBridgedSupplyRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBridgedSupplyRequest {
const NAME: &'static str = "QueryBridgedSupplyRequest";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryBridgedSupplyResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBridgedSupplyResponse {
    #[prost(message, optional, tag="1")]
    pub bridged_supply: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryBridgedSupplyResponse {
const NAME: &'static str = "QueryBridgedSupplyResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryRemoteRoutersRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRemoteRoutersRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryRemoteRoutersRequest {
const NAME: &'static str = "QueryRemoteRoutersRequest";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryRemoteRoutersResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRemoteRoutersResponse {
    /// Remote Routers ...
    #[prost(message, repeated, tag="1")]
    pub remote_routers: ::prost::alloc::vec::Vec<RemoteRouter>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryRemoteRoutersResponse {
const NAME: &'static str = "QueryRemoteRoutersResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryQuoteRemoteTransferRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryQuoteRemoteTransferRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub destination_domain: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryQuoteRemoteTransferRequest {
const NAME: &'static str = "QueryQuoteRemoteTransferRequest";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// QueryQuoteRemoteTransferResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryQuoteRemoteTransferResponse {
    #[prost(message, repeated, tag="1")]
    pub gas_payment: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryQuoteRemoteTransferResponse {
const NAME: &'static str = "QueryQuoteRemoteTransferResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgCreateCollateralToken ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateCollateralToken {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub origin_mailbox: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub origin_denom: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateCollateralToken {
const NAME: &'static str = "MsgCreateCollateralToken";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgCreateCollateralTokenResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateCollateralTokenResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateCollateralTokenResponse {
const NAME: &'static str = "MsgCreateCollateralTokenResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgCreateSyntheticToken ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSyntheticToken {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub origin_mailbox: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateSyntheticToken {
const NAME: &'static str = "MsgCreateSyntheticToken";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgCreateSyntheticTokenResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateSyntheticTokenResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateSyntheticTokenResponse {
const NAME: &'static str = "MsgCreateSyntheticTokenResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgSetToken ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetToken {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(bool, tag="7")]
    pub renounce_ownership: bool,
}
impl ::prost::Name for MsgSetToken {
const NAME: &'static str = "MsgSetToken";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgSetTokenResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetTokenResponse {
}
impl ::prost::Name for MsgSetTokenResponse {
const NAME: &'static str = "MsgSetTokenResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgEnrollRemoteRouter ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEnrollRemoteRouter {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub remote_router: ::core::option::Option<RemoteRouter>,
}
impl ::prost::Name for MsgEnrollRemoteRouter {
const NAME: &'static str = "MsgEnrollRemoteRouter";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgEnrollRemoteRouterResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEnrollRemoteRouterResponse {
}
impl ::prost::Name for MsgEnrollRemoteRouterResponse {
const NAME: &'static str = "MsgEnrollRemoteRouterResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgUnrollRemoteRouter ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnrollRemoteRouter {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub receiver_domain: u32,
}
impl ::prost::Name for MsgUnrollRemoteRouter {
const NAME: &'static str = "MsgUnrollRemoteRouter";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgUnrollRemoteRouterResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnrollRemoteRouterResponse {
}
impl ::prost::Name for MsgUnrollRemoteRouterResponse {
const NAME: &'static str = "MsgUnrollRemoteRouterResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgRemoteTransfer ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoteTransfer {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub destination_domain: u32,
    #[prost(string, tag="4")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
    /// Post Dispatch
    #[prost(string, tag="6")]
    pub custom_hook_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub gas_limit: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub max_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="9")]
    pub custom_hook_metadata: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRemoteTransfer {
const NAME: &'static str = "MsgRemoteTransfer";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
/// MsgRemoteTransferResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoteTransferResponse {
    #[prost(string, tag="1")]
    pub message_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRemoteTransferResponse {
const NAME: &'static str = "MsgRemoteTransferResponse";
const PACKAGE: &'static str = "hyperlane.warp.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.v1.{}", Self::NAME)
            }}
include!("hyperlane.warp.v1.tonic.rs");
// @@protoc_insertion_point(module)