use near_sdk::Balance;
use near_sdk::collections::{UnorderedMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};  
  
use crate::baggage::*;
use crate::types::*;

pub trait FeeStrategy {
    fn calculate_fee(
        &self,
        distance: Distance,
        baggages: &UnorderedMap<BaggageId, Baggage>
    ) -> Balance;
}

fn sum_weight(baggages: &UnorderedMap<BaggageId, Baggage>) -> Weight{
    let mut ret: Weight = 0.0;
    for baggage in baggages.values() {
        ret += baggage.get_weight();
    }
    ret
}

fn max_weight(baggages: &UnorderedMap<BaggageId, Baggage>) -> Weight{
    let mut ret: Weight = 0.0;
    for baggage in baggages.values() {
        if *(baggage.get_weight()) > ret {
            ret = *(baggage.get_weight());
        }
    }
    ret
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct FirstFee {}
impl FeeStrategy for FirstFee {
    fn calculate_fee(
        &self,
        _distance: Distance,
        baggages: &UnorderedMap<BaggageId, Baggage>
    ) -> Balance {
        ((sum_weight(baggages)-max_weight(baggages)) as Balance)*2
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BusinessFee {}
impl FeeStrategy for BusinessFee {
    fn calculate_fee(
        &self,
        _distance: Distance,
        baggages: &UnorderedMap<BaggageId, Baggage>
    ) -> Balance {
        (sum_weight(baggages) as Balance)*2
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct EconomyFee {}
impl FeeStrategy for EconomyFee {
    fn calculate_fee(
        &self,
        distance: Distance,
        baggages:&UnorderedMap<BaggageId, Baggage>
    ) -> Balance {
        ((sum_weight(baggages)+distance) as Balance)*2
    }
}