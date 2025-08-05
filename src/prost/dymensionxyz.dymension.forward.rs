// @generated
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
    /// optional, can be empty
    #[prost(bytes="vec", tag="3")]
    pub hook_forward_to_hl: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for HlMetadata {
const NAME: &'static str = "HLMetadata";
const PACKAGE: &'static str = "dymensionxyz.dymension.forward";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dymensionxyz.dymension.forward.{}", Self::NAME)
            }}
// @@protoc_insertion_point(module)
