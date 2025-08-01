// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositFxg {
    #[prost(enumeration="DepositVersion", tag="1")]
    pub version: i32,
    #[prost(bytes="vec", tag="2")]
    pub amount: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub utxo_index: u32,
    #[prost(string, tag="5")]
    pub accepting_block_hash: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub hl_message: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub containing_block_hash: ::prost::alloc::string::String,
}
impl ::prost::Name for DepositFxg {
const NAME: &'static str = "DepositFXG";
const PACKAGE: &'static str = "dymensionxyz.hyperlane.kaspa";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.hyperlane.kaspa.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HyperlaneMessages {
    #[prost(bytes="vec", repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for HyperlaneMessages {
const NAME: &'static str = "HyperlaneMessages";
const PACKAGE: &'static str = "dymensionxyz.hyperlane.kaspa";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.hyperlane.kaspa.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawFxg {
    #[prost(enumeration="WithdrawalVersion", tag="1")]
    pub version: i32,
    /// hex-encoded string due to how serialization is implemented in rusty-kaspa
    #[prost(string, tag="2")]
    pub pskt_bundle: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub messages: ::prost::alloc::vec::Vec<HyperlaneMessages>,
    #[prost(message, repeated, tag="4")]
    pub anchors: ::prost::alloc::vec::Vec<super::super::dymension::kas::TransactionOutpoint>,
}
impl ::prost::Name for WithdrawFxg {
const NAME: &'static str = "WithdrawFXG";
const PACKAGE: &'static str = "dymensionxyz.hyperlane.kaspa";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.hyperlane.kaspa.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmationFxg {
    #[prost(enumeration="ConfirmationVersion", tag="1")]
    pub version: i32,
    #[prost(message, repeated, tag="2")]
    pub outpoints: ::prost::alloc::vec::Vec<super::super::dymension::kas::TransactionOutpoint>,
    #[prost(message, optional, tag="3")]
    pub progress_indication: ::core::option::Option<super::super::dymension::kas::ProgressIndication>,
}
impl ::prost::Name for ConfirmationFxg {
const NAME: &'static str = "ConfirmationFXG";
const PACKAGE: &'static str = "dymensionxyz.hyperlane.kaspa";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.hyperlane.kaspa.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageIDs {
    /// array of raw HL message IDs
    #[prost(bytes="vec", repeated, tag="1")]
    pub message_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for MessageIDs {
const NAME: &'static str = "MessageIDs";
const PACKAGE: &'static str = "dymensionxyz.hyperlane.kaspa";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.hyperlane.kaspa.{}", Self::NAME)
            }}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DepositVersion {
    Unspecified = 0,
    DepositVersion1 = 1,
}
impl DepositVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DepositVersion::Unspecified => "DEPOSIT_VERSION_UNSPECIFIED",
            DepositVersion::DepositVersion1 => "DEPOSIT_VERSION_1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEPOSIT_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
            "DEPOSIT_VERSION_1" => Some(Self::DepositVersion1),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WithdrawalVersion {
    Unspecified = 0,
    WithdrawalVersion1 = 1,
}
impl WithdrawalVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WithdrawalVersion::Unspecified => "WITHDRAWAL_VERSION_UNSPECIFIED",
            WithdrawalVersion::WithdrawalVersion1 => "WITHDRAWAL_VERSION_1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WITHDRAWAL_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
            "WITHDRAWAL_VERSION_1" => Some(Self::WithdrawalVersion1),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConfirmationVersion {
    Unspecified = 0,
    ConfirmationVersion1 = 1,
}
impl ConfirmationVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConfirmationVersion::Unspecified => "CONFIRMATION_VERSION_UNSPECIFIED",
            ConfirmationVersion::ConfirmationVersion1 => "CONFIRMATION_VERSION_1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONFIRMATION_VERSION_UNSPECIFIED" => Some(Self::Unspecified),
            "CONFIRMATION_VERSION_1" => Some(Self::ConfirmationVersion1),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
