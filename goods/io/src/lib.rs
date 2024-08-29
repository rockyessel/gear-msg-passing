#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId, MessageId};

pub static mut TRANSFER_REQUESTS: Vec<TransferRequest> = Vec::new();
pub static mut GOODS: Vec<Goods> = Vec::new();

pub type UserId = ActorId;

pub struct GoodsMetadata;

impl Metadata for GoodsMetadata {
    type Init = ();
    type Handle = InOut<GoodsActions, GoodsEvents>;
    type State = InOut<GoodsStateActions, GoodsStateEvents>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Goods {
    pub name: String,
    pub goods_id: MessageId,
    pub req_created_by: UserId,
    pub is_transfer_active: bool,
    pub transfer_req_by: Option<UserId>,
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct GoodsParams {
    pub name: String,
    pub req_created_by: UserId,
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GoodsActions {
    Create(GoodsParams),
    CreateTransferRequest(CreateReqParams),
    ApproveTransferRequest(TransferReqParams),
    CancelTransferRequest(TransferReqParams),
    DeclineTransferRequest(TransferReqParams),
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GoodsEvents {
    Created(MessageId),
    Transferred(String),
    TransferRequested(MessageId),
    TransferApproved(String),
    TransferCanceled(String),
    TransferDeclined(String),
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GoodsStateActions {
    GetAllGoods,
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum GoodsStateEvents {
    GetAllGoods(Vec<Goods>),
}

/// Represents a request to transfer ownership of a blog from one user to another.
#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TransferRequest {
    /// * `prog_id` - A `ActorId` representing the unique identifier of the program.
    pub prog_id: ActorId,
    /// * `goods_id` - A `String` representing the unique identifier of the goods.
    pub goods_id: MessageId,
    /// * `from` - A `UserId` representing the current owner of the goods.
    pub from: UserId,

    /// * `to` - A `UserId` representing the intended new owner of the blog.
    pub to: UserId,
    /// * `approved_from` - A `bool` indicating whether the current owner (`from`) has approved the transfer.
    ///  - `true` if approved, `false` otherwise.
    pub approved_from: bool,

    /// * `approved_to` - A `bool` indicating whether the intended new owner (`to`) has approved the transfer.
    ///   - `true` if approved, `false` otherwise.
    pub approved_to: bool,

    /// * `declined` - A `bool` indicating whether the transfer request has been declined by either party.
    ///   - `true` if declined, `false` otherwise.
    pub declined: bool,
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CreateReqParams {
    pub goods_id: MessageId,
    pub prog_id: ActorId,
    pub transferred_to: UserId,
    pub owner_id: UserId,
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TransferReqParams {
    pub goods_id: MessageId,
    pub user_id: UserId,
}
