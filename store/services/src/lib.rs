#![no_std]

use goods_io::Goods;
use store_io::*;

trait StoreGoodsTrait {
    fn receive_and_register(params: Goods) -> StoreGoodsEvents;
}

impl StoreGoodsTrait for StoreGoods {
    fn receive_and_register(params: Goods) -> StoreGoodsEvents {
        let store_goods = StoreGoods {
            goods_id: params.goods_id,
            is_sold: false,
            is_transfer_active: params.is_transfer_active,
            name: params.name,
            req_created_by: params.req_created_by,
            transfer_req_by: params.transfer_req_by,
        };

        // Push the new request to the global STORE_GOODS
        unsafe {
            STORE_GOODS.push(store_goods);
        }

        // received
        StoreGoodsEvents::ReceivedGoods(params.goods_id)
    }
}

pub fn store_action_handler(action: StoreGoodsActions) -> StoreGoodsEvents {
    match action {
        StoreGoodsActions::ReceiveGoods(params) => StoreGoods::receive_and_register(params),
    }
}

fn get_all_store_goods() -> StoreGoodsStateEvents {
    let store_goods = unsafe { STORE_GOODS.iter().cloned().collect() };
    StoreGoodsStateEvents::GetAllStoreGoods(store_goods)
}

pub fn store_state_handler(query: StoreGoodsStateActions) -> StoreGoodsStateEvents {
    match query {
        StoreGoodsStateActions::GetAllStoreGoods => get_all_store_goods(),
    }
}
