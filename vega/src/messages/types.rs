// Copyright 2020 The Vega Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use crate::{proto::schema, runtime::AnyTx};

use chrono::{DateTime, Utc};
use vega_derive::{BinaryValue, ObjectHash};
use vega_merkledb::BinaryValue;
use vega_proto::ProtobufConvert;

use std::convert::TryFrom;

use crate::{
    crypto::{self, Hash, PublicKey, SecretKey, Signature},
    helpers::{Height, Round, ValidatorId},
    proto::schema::messages,
};

/// Protobuf-based container for an arbitrary signed message.
///
/// See module [documentation](index.html#examples) for examples.
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
#[derive(ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "messages::SignedMessage")]
pub struct SignedMessage {
    /// Payload of the message.
    pub payload: Vec<u8>,
    /// `PublicKey` of the author of the message.
    pub author: PublicKey,
    /// Digital signature over `payload` created with the secret key of the author of the message.
    pub signature: Signature,
}

impl SignedMessage {
    /// Creates a new signed message from the given binary value.
    pub fn new(payload: impl BinaryValue, author: PublicKey, secret_key: &SecretKey) -> Self {
        let payload = payload.into_bytes();
        let signature = crypto::sign(payload.as_ref(), secret_key);
        Self {
            payload,
            author,
            signature,
        }
    }
}

/// Pre-commit for a block, essentially meaning that a validator node endorses the block.
/// The consensus algorithm ensures that once a Byzantine majority of validators has
/// endorsed a block, no other block at the same height may be endorsed at any point in the future.
/// Thus, such a block can be considered committed.
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
#[derive(Serialize, Deserialize)]
#[derive(ProtobufConvert)]
#[protobuf_convert(source = "messages::Precommit")]
pub struct Precommit {
    /// ID of the validator endorsing the block.
    pub validator: ValidatorId,
    /// The height to which the message is related.
    pub height: Height,
    /// The round to which the message is related.
    pub round: Round,
    /// Hash of the block proposal. Note that the proposal format is not defined by the core.
    pub propose_hash: Hash,
    /// Hash of the new block.
    pub block_hash: Hash,
    /// Local time of the validator node when the `Precommit` was created.
    pub time: DateTime<Utc>,

    /// No-op field for forward compatibility.
    #[protobuf_convert(skip)]
    non_exhaustive: (),
}

impl Precommit {
    /// Create new `Precommit` message.
    pub fn new(
        validator: ValidatorId,
        height: Height,
        round: Round,
        propose_hash: Hash,
        block_hash: Hash,
        time: DateTime<Utc>,
    ) -> Self {
        Self {
            validator,
            height,
            round,
            propose_hash,
            block_hash,
            time,
            non_exhaustive: (),
        }
    }
    /// The validator id.
    pub fn validator(&self) -> ValidatorId {
        self.validator
    }
    /// The height to which the message is related.
    pub fn height(&self) -> Height {
        self.height
    }
    /// The round to which the message is related.
    pub fn round(&self) -> Round {
        self.round
    }
    /// Hash of the corresponding `Propose`.
    pub fn propose_hash(&self) -> &Hash {
        &self.propose_hash
    }
    /// Hash of the new block.
    pub fn block_hash(&self) -> &Hash {
        &self.block_hash
    }
    /// Time of the `Precommit`.
    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }
}

/// Subset of Vega messages defined in the Vega core.
///
/// This type is intentionally kept as minimal as possible to ensure compatibility
/// even if the consensus details change. Most of consensus messages are defined separately
/// in the `vega-node` crate; they are not public.
///
/// This type is not intended to be exhaustively matched. It can be extended in the future
/// without breaking the semver compatibility.
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
#[derive(BinaryValue, ObjectHash)]
pub enum CoreMessage {
    /// Transaction message.
    AnyTx(AnyTx),
    /// Precommit message.
    Precommit(Precommit),

    /// Never actually generated.
    #[doc(hidden)]
    __NonExhaustive,
}

impl ProtobufConvert for CoreMessage {
    type ProtoStruct = schema::messages::CoreMessage;

    fn to_pb(&self) -> Self::ProtoStruct {
        let mut pb = Self::ProtoStruct::new();
        match self {
            CoreMessage::AnyTx(any_tx) => {
                pb.set_any_tx(any_tx.to_pb());
            }
            CoreMessage::Precommit(precommit) => {
                pb.set_precommit(precommit.to_pb());
            }
            CoreMessage::__NonExhaustive => unreachable!("Never actually constructed"),
        }
        pb
    }

    fn from_pb(mut pb: Self::ProtoStruct) -> Result<Self, failure::Error> {
        let msg = if pb.has_any_tx() {
            let tx = AnyTx::from_pb(pb.take_any_tx())?;
            CoreMessage::AnyTx(tx)
        } else if pb.has_precommit() {
            let precommit = Precommit::from_pb(pb.take_precommit())?;
            CoreMessage::Precommit(precommit)
        } else {
            failure::bail!("Incorrect protobuf representation of CoreMessage")
        };

        Ok(msg)
    }
}

impl From<AnyTx> for CoreMessage {
    fn from(tx: AnyTx) -> Self {
        CoreMessage::AnyTx(tx)
    }
}

impl From<Precommit> for CoreMessage {
    fn from(precommit: Precommit) -> Self {
        CoreMessage::Precommit(precommit)
    }
}

impl TryFrom<CoreMessage> for AnyTx {
    type Error = failure::Error;

    fn try_from(msg: CoreMessage) -> Result<Self, failure::Error> {
        if let CoreMessage::AnyTx(tx) = msg {
            Ok(tx)
        } else {
            failure::bail!("Not an `AnyTx` variant")
        }
    }
}

impl TryFrom<CoreMessage> for Precommit {
    type Error = failure::Error;

    fn try_from(msg: CoreMessage) -> Result<Self, failure::Error> {
        if let CoreMessage::Precommit(precommit) = msg {
            Ok(precommit)
        } else {
            failure::bail!("Not a `Precommit` variant")
        }
    }
}

impl TryFrom<SignedMessage> for CoreMessage {
    type Error = failure::Error;

    fn try_from(value: SignedMessage) -> Result<Self, Self::Error> {
        <Self as BinaryValue>::from_bytes(value.payload.into())
    }
}

#[doc(hidden)] // Library users should not define new message types.
#[macro_export]
macro_rules! impl_vega_msg_try_from_signed {
    ( $base:ident => $( $name:ident ),* ) => {
        $(
            impl std::convert::TryFrom<$crate::messages::SignedMessage> for $name {
                type Error = failure::Error;

                fn try_from(value: $crate::messages::SignedMessage) -> Result<Self, Self::Error> {
                    <$base as $crate::merkledb::BinaryValue>::from_bytes(value.payload.into())
                        .and_then(Self::try_from)
                }
            }

            impl std::convert::TryFrom<&$crate::messages::SignedMessage> for $name {
                type Error = failure::Error;

                fn try_from(value: &$crate::messages::SignedMessage) -> Result<Self, Self::Error> {
                    let bytes = std::borrow::Cow::Borrowed(value.payload.as_slice());
                    <$base as $crate::merkledb::BinaryValue>::from_bytes(bytes)
                        .and_then(Self::try_from)
                }
            }

            impl $crate::messages::IntoMessage for $name {
                type Container = $base;
            }
        )*
    }
}

impl_vega_msg_try_from_signed!(CoreMessage => AnyTx, Precommit);
