#![no_std]

use goods_io::*;
use gstd::{msg, prelude::*, ActorId};
use store_io::*;

trait TransferRequestTrait {
    fn create_request(params: CreateReqParams) -> GoodsEvents;
    fn approve_request(params: TransferReqParams) -> GoodsEvents;
    fn cancel_request(params: TransferReqParams) -> GoodsEvents;
    fn decline_request(params: TransferReqParams) -> GoodsEvents;
}

trait GoodsTrait {
    fn create(params: GoodsParams) -> GoodsEvents;
}

impl TransferRequestTrait for TransferRequest {
    fn create_request(params: CreateReqParams) -> GoodsEvents {
        // Create a new transfer request
        let new_request = TransferRequest {
            prog_id: params.prog_id,
            goods_id: params.goods_id,
            from: params.owner_id,
            to: params.transferred_to,
            approved_from: false,
            approved_to: false,
            declined: false,
        };

        // Push the new request to the global TRANSFER_REQUESTS
        unsafe {
            TRANSFER_REQUESTS.push(new_request);
        }

        // Return the event indicating a transfer request has been created
        GoodsEvents::TransferRequested(params.goods_id)
    }

    fn approve_request(params: TransferReqParams) -> GoodsEvents {
        // Find the request based on goods_id
        unsafe {
            if let Some(request) = TRANSFER_REQUESTS
                .iter_mut()
                .find(|r| r.goods_id == params.goods_id && !r.declined)
            {
                if request.from == params.user_id {
                    request.approved_from = true;
                } else if request.to == params.user_id {
                    request.approved_to = true;
                }

                // If both approved, complete the transfer
                if request.approved_from && request.approved_to {
                    if let Some(goods) = GOODS
                        .iter_mut()
                        .find(|g| g.goods_id == request.goods_id.into())
                    {
                        goods.req_created_by = request.to;
                        goods.is_transfer_active = false;
                        request.declined = true;

                        // The transfer was successful
                        return GoodsEvents::Transferred(params.goods_id.to_string());
                    }
                }

                // Transfer request was approved but not yet completed
                return GoodsEvents::TransferApproved(params.goods_id.to_string());
            }
        }

        // Return an event indicating that the request was not found or some other condition failed
        GoodsEvents::TransferDeclined(params.goods_id.to_string())
    }

    fn cancel_request(params: TransferReqParams) -> GoodsEvents {
        // Find and remove the request
        unsafe {
            if let Some(request) = TRANSFER_REQUESTS
                .iter_mut()
                .find(|r| r.goods_id == params.goods_id && !r.declined)
            {
                if request.from == params.user_id || request.to == params.user_id {
                    request.declined = true;
                    if let Some(goods) = GOODS
                        .iter_mut()
                        .find(|g| g.goods_id == request.goods_id.into())
                    {
                        goods.is_transfer_active = false;

                        // transfer request was canceled
                        return GoodsEvents::TransferCanceled(params.goods_id.to_string());
                    }
                }
            }
        }

        // Return an empty event if the request was not found or some other condition failed
        GoodsEvents::TransferDeclined(params.goods_id.to_string())
    }

    fn decline_request(params: TransferReqParams) -> GoodsEvents {
        // Find and mark the request as declined
        unsafe {
            if let Some(request) = TRANSFER_REQUESTS
                .iter_mut()
                .find(|r| r.goods_id == params.goods_id && !r.declined)
            {
                if request.from == params.user_id || request.to == params.user_id {
                    request.declined = true;
                    if let Some(goods) = GOODS
                        .iter_mut()
                        .find(|g| g.goods_id == request.goods_id.into())
                    {
                        goods.is_transfer_active = false;

                        // Record goods to store, after it has been approved.
                        handle_transfer_articles(request.prog_id, goods);

                        // transfer request was declined
                        return GoodsEvents::TransferDeclined(params.goods_id.to_string());
                    }
                }
            }
        }
        // Return empty if the request was not found or some other condition failed
        GoodsEvents::TransferDeclined(params.goods_id.to_string())
    }
}

impl GoodsTrait for Goods {
    fn create(params: GoodsParams) -> GoodsEvents {
        // Create a new goods entry

        let msg_id = msg::id();

        let goods = Goods {
            name: params.name,
            goods_id: msg_id,
            req_created_by: params.req_created_by,
            is_transfer_active: false,
            transfer_req_by: None,
        };

        unsafe {
            GOODS.push(goods);
        }

        // Return the event indicating a transfer request has been created
        GoodsEvents::TransferRequested(msg_id)
    }
}

pub fn goods_action_handler(action: GoodsActions) -> GoodsEvents {
    match action {
        GoodsActions::Create(goods) => Goods::create(goods),
        GoodsActions::CreateTransferRequest(params) => TransferRequest::create_request(params),
        GoodsActions::ApproveTransferRequest(params) => TransferRequest::approve_request(params),
        GoodsActions::CancelTransferRequest(params) => TransferRequest::cancel_request(params),
        GoodsActions::DeclineTransferRequest(params) => TransferRequest::decline_request(params),
    }
}

fn get_all_goods() -> GoodsStateEvents {
    let goods = unsafe { GOODS.iter().cloned().collect() };
    GoodsStateEvents::GetAllGoods(goods)
}

pub fn goods_state_handler(action: GoodsStateActions) -> GoodsStateEvents {
    match action {
        GoodsStateActions::GetAllGoods => get_all_goods(),
    }
}


fn handle_transfer_articles(prog_id: ActorId, goods: &Goods) {
    let store_action_msg = StoreGoodsActions::ReceiveGoods(goods.clone());
    msg::send(prog_id, store_action_msg.encode(), 0).expect("Failed to send message");
    // let id = &goods.goods_id;
    // msg::reply(StoreGoodsEvents::ReceivedGoods(id.clone()), 0).expect("Error in sending reply.");
}
