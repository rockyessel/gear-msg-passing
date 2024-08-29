#![no_std]

use gmeta::{InOut, Metadata};
use goods_io::Goods;
use gstd::{prelude::*, ActorId, MessageId};

pub static mut STORE_GOODS: Vec<StoreGoods> = Vec::new();

pub type UserId = ActorId;

pub struct StoreGoodsMetadata;

impl Metadata for StoreGoodsMetadata {
    type Init = ();
    type Handle = InOut<StoreGoodsActions, StoreGoodsEvents>;
    type State = InOut<StoreGoodsStateActions, StoreGoodsStateEvents>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct StoreGoods {
    pub name: String,
    pub goods_id: MessageId,
    pub req_created_by: UserId,
    pub is_transfer_active: bool,
    pub transfer_req_by: Option<UserId>,
    pub is_sold: bool,
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StoreGoodsActions {
    ReceiveGoods(Goods),
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StoreGoodsEvents {
    ReceivedGoods(MessageId),
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StoreGoodsStateActions {
    GetAllStoreGoods,
}

#[derive(TypeInfo, Encode, Decode, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StoreGoodsStateEvents {
    GetAllStoreGoods(Vec<StoreGoods>),
}
