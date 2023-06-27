#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(
	Clone,
	Copy,
	Default,
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Debug,
	Encode,
	Decode,
	TypeInfo,
	MaxEncodedLen,
)]
pub struct ConversionRate {
	pub native: u32,
	pub foreign: u32,
}

/// TODO: add decimal conversion
/// A type describing our custom additional metadata stored in the orml-asset-registry.
#[derive(
	Clone,
	Copy,
	Default,
	PartialOrd,
	Ord,
	PartialEq,
	Eq,
	Debug,
	Encode,
	Decode,
	TypeInfo,
	MaxEncodedLen,
)]
pub struct CustomMetadata {
	/// The fee charged for every second that an XCM message takes to execute.
	pub fee_per_second: Option<u128>,
	/// The token conversion rate of Native to Foreign, ie. 1::10
	pub conversion_rate: Option<ConversionRate>,
}

/// Identifier of a token or asset
pub type TokenId = u32;

/// The signed version of `Balance`
pub type Amount = i128;
