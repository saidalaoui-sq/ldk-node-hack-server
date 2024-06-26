// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeStatusRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeStatusResponse {
	#[prost(string, tag = "1")]
	pub public_key: ::prost::alloc::string::String,
	#[prost(message, optional, tag = "2")]
	pub current_best_block: ::core::option::Option<BestBlock>,
	#[prost(uint64, optional, tag = "3")]
	pub latest_wallet_sync_timestamp: ::core::option::Option<u64>,
	#[prost(uint64, optional, tag = "4")]
	pub latest_onchain_wallet_sync_timestamp: ::core::option::Option<u64>,
	#[prost(uint64, optional, tag = "5")]
	pub latest_fee_rate_cache_update_timestamp: ::core::option::Option<u64>,
	#[prost(uint64, optional, tag = "6")]
	pub latest_rgs_snapshot_timestamp: ::core::option::Option<u64>,
	#[prost(uint64, optional, tag = "7")]
	pub latest_node_announcement_broadcast_timestamp: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BestBlock {
	#[prost(string, tag = "1")]
	pub block_hash: ::prost::alloc::string::String,
	#[prost(uint32, tag = "2")]
	pub height: u32,
}
/// Retrieve a new on-chain/funding address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnchainReceiveRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnchainReceiveResponse {
	#[prost(string, tag = "1")]
	pub address: ::prost::alloc::string::String,
}
/// Send an on-chain payment to the given address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnchainSendRequest {
	#[prost(string, tag = "1")]
	pub address: ::prost::alloc::string::String,
	#[prost(uint64, optional, tag = "2")]
	pub amount_sats: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnchainSendResponse {
	#[prost(string, tag = "1")]
	pub txid: ::prost::alloc::string::String,
}
/// Return a BOLT11 invoice for the given amount, if specified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bolt11ReceiveRequest {
	#[prost(string, tag = "1")]
	pub description: ::prost::alloc::string::String,
	#[prost(uint64, tag = "2")]
	pub expiry_secs: u64,
	#[prost(uint64, optional, tag = "3")]
	pub amount_msat: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bolt11ReceiveResponse {
	#[prost(string, tag = "1")]
	pub invoice: ::prost::alloc::string::String,
}
/// Send a payment for a BOLT11 invoice.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bolt11SendRequest {
	#[prost(string, tag = "1")]
	pub invoice: ::prost::alloc::string::String,
	#[prost(uint64, optional, tag = "2")]
	pub amount_msat: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bolt11SendResponse {
	#[prost(message, optional, tag = "1")]
	pub payment_id: ::core::option::Option<PaymentId>,
}
/// Return a BOLT12 offer for the given amount, if specified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bolt12ReceiveRequest {
	#[prost(string, tag = "1")]
	pub description: ::prost::alloc::string::String,
	#[prost(uint64, optional, tag = "2")]
	pub amount_msat: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bolt12ReceiveResponse {
	#[prost(string, tag = "1")]
	pub offer: ::prost::alloc::string::String,
}
/// Send a payment for a BOLT11 invoice.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bolt12SendRequest {
	#[prost(string, tag = "1")]
	pub offer: ::prost::alloc::string::String,
	#[prost(uint64, optional, tag = "2")]
	pub amount_msat: ::core::option::Option<u64>,
	#[prost(string, optional, tag = "3")]
	pub payer_note: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bolt12SendResponse {
	#[prost(message, optional, tag = "1")]
	pub payment_id: ::core::option::Option<PaymentId>,
}
/// An identifier for making a payment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentId {
	#[prost(bytes = "vec", tag = "1")]
	pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsResponse {
	#[prost(message, repeated, tag = "1")]
	pub channels: ::prost::alloc::vec::Vec<Channel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
	/// The channel ID (prior to funding transaction generation, this is a random 32-byte
	/// identifier, afterwards this is the transaction ID of the funding transaction XOR the
	/// funding transaction output).
	///
	/// Note that this means this value is *not* persistent - it can change once during the
	/// lifetime of the channel.
	#[prost(string, tag = "1")]
	pub channel_id: ::prost::alloc::string::String,
	/// The node ID of our the channel's remote counterparty.
	#[prost(string, tag = "2")]
	pub counterparty_node_id: ::prost::alloc::string::String,
	/// The channel's funding transaction output, if we've negotiated the funding transaction with
	/// our counterparty already.
	#[prost(message, optional, tag = "3")]
	pub funding_txo: ::core::option::Option<Outpoint>,
	/// The value, in satoshis, of this channel as it appears in the funding output.
	#[prost(uint64, tag = "4")]
	pub channel_value_sats: u64,
	/// The currently negotiated fee rate denominated in satoshi per 1000 weight units,
	/// which is applied to commitment and HTLC transactions.
	#[prost(uint32, tag = "5")]
	pub feerate_sat_per_1000_weight: u32,
	/// The available outbound capacity for sending HTLCs to the remote peer.
	///
	/// The amount does not include any pending HTLCs which are not yet resolved (and, thus, whose
	/// balance is not available for inclusion in new outbound HTLCs). This further does not include
	/// any pending outgoing HTLCs which are awaiting some other resolution to be sent.
	#[prost(uint64, tag = "6")]
	pub outbound_capacity_msat: u64,
	/// The available outbound capacity for sending HTLCs to the remote peer.
	///
	/// The amount does not include any pending HTLCs which are not yet resolved
	/// (and, thus, whose balance is not available for inclusion in new inbound HTLCs). This further
	/// does not include any pending outgoing HTLCs which are awaiting some other resolution to be
	/// sent.
	#[prost(uint64, tag = "7")]
	pub inbound_capacity_msat: u64,
	/// The number of required confirmations on the funding transactions before the funding is
	/// considered "locked". The amount is selected by the channel fundee.
	///
	/// The value will be `None` for outbound channels until the counterparty accepts the channel.
	#[prost(uint32, optional, tag = "8")]
	pub confirmations_required: ::core::option::Option<u32>,
	/// The current number of confirmations on the funding transaction.
	#[prost(uint32, optional, tag = "9")]
	pub confirmations: ::core::option::Option<u32>,
	/// Is `true` if the channel was initiated (and therefore funded) by us.
	#[prost(bool, tag = "10")]
	pub is_outbound: bool,
	/// Is `true` if both parties have exchanged `channel_ready` messages, and the channel is
	/// not currently being shut down. Both parties exchange `channel_ready` messages upon
	/// independently verifying that the required confirmations count provided by
	/// `confirmations_required` has been reached.
	#[prost(bool, tag = "11")]
	pub is_channel_ready: bool,
	/// Is `true` if the channel (a) `channel_ready` messages have been exchanged, (b) the
	/// peer is connected, and (c) the channel is not currently negotiating shutdown.
	///
	/// This is a strict superset of `is_channel_ready`.
	#[prost(bool, tag = "12")]
	pub is_usable: bool,
	/// Is `true` if this channel is (or will be) publicly-announced
	#[prost(bool, tag = "13")]
	pub is_public: bool,
	/// The difference in the CLTV value between incoming HTLCs and an outbound HTLC forwarded over
	/// the channel.
	#[prost(uint32, optional, tag = "14")]
	pub cltv_expiry_delta: ::core::option::Option<u32>,
	/// The smallest value HTLC (in msat) the remote peer will accept, for this channel.
	///
	/// This field is only `None` before we have received either the `OpenChannel` or
	/// `AcceptChannel` message from the remote peer.
	#[prost(uint64, optional, tag = "15")]
	pub counterparty_outbound_htlc_minimum_msat: ::core::option::Option<u64>,
	/// The largest value HTLC (in msat) the remote peer currently will accept, for this channel.
	#[prost(uint64, optional, tag = "16")]
	pub counterparty_outbound_htlc_maximum_msat: ::core::option::Option<u64>,
	/// The available outbound capacity for sending a single HTLC to the remote peer. This is
	/// similar to `outbound_capacity_msat` but it may be further restricted by
	/// the current state and per-HTLC limit(s). This is intended for use when routing, allowing us
	/// to use a limit as close as possible to the HTLC limit we can currently send.
	#[prost(uint64, tag = "17")]
	pub next_outbound_htlc_limit_msat: u64,
	/// The minimum value for sending a single HTLC to the remote peer. This is the equivalent of
	/// `next_outbound_htlc_limit_msat` but represents a lower-bound, rather than
	/// an upper-bound. This is intended for use when routing, allowing us to ensure we pick a
	/// route which is valid.
	#[prost(uint64, tag = "18")]
	pub next_outbound_htlc_minimum_msat: u64,
	/// The number of blocks (after our commitment transaction confirms) that we will need to wait
	/// until we can claim our funds after we force-close the channel. During this time our
	/// counterparty is allowed to punish us if we broadcasted a stale state. If our counterparty
	/// force-closes the channel and broadcasts a commitment transaction we do not have to wait any
	/// time to claim our non-HTLC-encumbered funds.
	///
	/// This value will be `None` for outbound channels until the counterparty accepts the channel.
	#[prost(uint32, optional, tag = "19")]
	pub force_close_spend_delay: ::core::option::Option<u32>,
	/// Amount (in millionths of a satoshi) charged per satoshi for payments forwarded outbound
	/// over the channel.
	#[prost(uint32, tag = "20")]
	pub forwarding_fee_proportional_millionths: u32,
	/// Amount (in milli-satoshi) charged for payments forwarded outbound over the channel, in
	/// excess of \[`forwarding_fee_proportional_millionths`\].
	#[prost(uint32, tag = "21")]
	pub forwarding_fee_base_msat: u32,
}
/// Represents to transaction output.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Outpoint {
	/// The referenced transaction's txid.
	#[prost(string, tag = "1")]
	pub txid: ::prost::alloc::string::String,
	/// The index of the referenced output in its transaction's vout.
	#[prost(uint32, tag = "2")]
	pub vout: u32,
}
