// lib.rs
// Substreams module that processes Starknet ERC721 and ERC1155 Transfer events
// Outputs JSON events for all collections, not just factory-created ones

mod abi; // ABI definitions for ERC721 and ERC1155 contracts
mod pb;  // Protobuf definitions for Substreams output

use pb::starknet::v1::*;
use crate::abi::erc1155_contract::ERC1155ComponentEvent;
use crate::abi::erc721_contract::ERC721ComponentEvent;
use crate::pb::sf::substreams::starknet::r#type::v1::Transactions;
use serde_json::json;
use starknet::core::types::{EmittedEvent, Felt};
use substreams::Hex;

// Converts a Felt to a hex string with 0x prefix
fn felt_to_hex(felt: &Felt) -> String {
    format!(
        "0x{}",
        felt.to_bytes_be()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    )
}

// Converts a Cairo U256 to a decimal string
fn u256_to_decimal(value: &cainome::cairo_serde::U256) -> String {
    let low = value.low;
    let high = value.high;
    if high == 0 {
        low.to_string()
    } else {
        let high_part = high as u128 * (1u128 << 64) * (1u128 << 64);
        let total = high_part + low as u128;
        total.to_string()
    }
}

// Converts a ContractAddress to a hex string
fn contract_address_to_hex(addr: &cainome::cairo_serde::ContractAddress) -> String {
    felt_to_hex(&addr.0)
}

// Processes Starknet transactions into JSON events
#[substreams::handlers::map]
fn map_nft_events(transactions: Transactions) -> Result<Events, substreams::errors::Error> {
    let mut proto_events = Events::default();

    for transaction in transactions.transactions_with_receipt {
        let receipt = match &transaction.receipt {
            Some(receipt) => receipt,
            None => continue,
        };

        for (index, event) in receipt.events.iter().enumerate() {
            let event_from_address = felt_to_hex(&Felt::from_bytes_be_slice(event.from_address.as_slice()));
            let id = format!("{}_{}", Hex(&receipt.transaction_hash), index);

            let mut data_felts = vec![];
            let mut keys_felts = vec![];
            for key in &event.keys {
                keys_felts.push(Felt::from_bytes_be_slice(key.as_slice()));
            }
            for bytes in &event.data {
                data_felts.push(Felt::from_bytes_be_slice(bytes.as_slice()));
            }

            let emitted_event = EmittedEvent {
                from_address: Felt::from_bytes_be_slice(event.from_address.as_slice()),
                keys: keys_felts,
                data: data_felts,
                block_hash: None,
                block_number: None,
                transaction_hash: Felt::default(),
            };

            // ERC721 Transfer
            match ERC721ComponentEvent::try_from(emitted_event.clone()) {
                Ok(ERC721ComponentEvent::Transfer(transfer)) => {
                    let event_json = json!({
                        "type": "Transfer",
                        "id": id,
                        "from": contract_address_to_hex(&transfer.from),
                        "to": contract_address_to_hex(&transfer.to),
                        "collection_address": event_from_address,
                        "token_id": u256_to_decimal(&transfer.token_id),
                        "transaction_hash": Hex(&receipt.transaction_hash).to_string()
                    });
                    proto_events.events.push(Event {
                        json_description: event_json.to_string(),
                    });
                }
                Ok(_) => {}
                Err(_) => {}
            }

            // ERC1155 TransferSingle
            match ERC1155ComponentEvent::try_from(emitted_event.clone()) {
                Ok(ERC1155ComponentEvent::TransferSingle(transfer)) => {
                    let event_json = json!({
                        "type": "TransferSingle",
                        "id": id,
                        "operator": contract_address_to_hex(&transfer.operator),
                        "from": contract_address_to_hex(&transfer.from),
                        "to": contract_address_to_hex(&transfer.to),
                        "collection_address": event_from_address,
                        "token_id": u256_to_decimal(&transfer.id),
                        "value": u256_to_decimal(&transfer.value),
                        "transaction_hash": Hex(&receipt.transaction_hash).to_string()
                    });
                    proto_events.events.push(Event {
                        json_description: event_json.to_string(),
                    });
                }
                Ok(_) => {}
                Err(_) => {}
            }

            // ERC1155 TransferBatch
            match ERC1155ComponentEvent::try_from(emitted_event) {
                Ok(ERC1155ComponentEvent::TransferBatch(transfer)) => {
                    let token_ids: Vec<String> = transfer.ids.iter().map(|id| u256_to_decimal(id)).collect();
                    let values: Vec<String> = transfer.values.iter().map(|value| u256_to_decimal(value)).collect();
                    let event_json = json!({
                        "type": "TransferBatch",
                        "id": id,
                        "operator": contract_address_to_hex(&transfer.operator),
                        "from": contract_address_to_hex(&transfer.from),
                        "to": contract_address_to_hex(&transfer.to),
                        "collection_address": event_from_address,
                        "token_ids": token_ids,
                        "values": values,
                        "transaction_hash": Hex(&receipt.transaction_hash).to_string()
                    });
                    proto_events.events.push(Event {
                        json_description: event_json.to_string(),
                    });
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    Ok(proto_events)
}