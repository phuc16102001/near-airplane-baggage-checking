use near_sdk::Balance;
use near_sdk::collections::UnorderedMap;
use near_sdk::{serde::{Serialize, Deserialize}};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};  


use crate::fee::*;
use crate::types::*;
use crate::baggage::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, PartialEq)]
pub enum FlightClass {
    First,
    Business,
    Economy
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, PartialEq)]
pub enum FlightState {
    Idle,
    Checked,
    Delivered,
    Claimed
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct FlightDetail {
    flight_id: FlightId,
    flight_class: FlightClass, 
    distance: Distance,
    baggages: UnorderedMap<BaggageId, Baggage>,
    state: FlightState
}

impl FlightDetail {
    pub fn new(
        flight_id: FlightId,
        flight_class: FlightClass,
        distance: Distance,
    ) -> Self {
        Self {
            flight_id,
            flight_class,
            distance,
            baggages: UnorderedMap::new(format!("baggages_{}",flight_id).as_bytes()),
            state: FlightState::Idle
        }
    }

    pub fn add_baggage(&mut self, baggage: Baggage) {
        self.baggages.insert(baggage.get_id(), &baggage);
    }

    pub fn remove_baggage(&mut self, baggage_id: BaggageId) {
        self.baggages.remove(&baggage_id);
    }

    pub fn clear_baggages(&mut self) {
        self.baggages.clear();
    }

    pub fn set_state(&mut self, new_state: FlightState) {
        self.state = new_state;
    }

    pub fn get_state(&self) -> &FlightState {
        &self.state
    }

    pub fn get_flight_id(&mut self) -> &FlightId{
        &self.flight_id
    }

    pub fn get_flight_class(&self) -> &FlightClass{
        &self.flight_class
    }
    
    pub fn get_distance(&self) -> &Distance{
        &self.distance
    }
    
    pub fn get_baggages(&self) -> &UnorderedMap<BaggageId, Baggage>{
        &self.baggages
    }

    pub fn get_fee(&self) -> Balance {
        self.get_fee_strategy().calculate_fee(
            self.distance,
            &self.baggages
        )
    }

    fn get_fee_strategy(&self) -> Box<dyn FeeStrategy> {
        match &self.flight_class {
            FlightClass::First => Box::new(FirstFee{}),
            FlightClass::Business => Box::new(BusinessFee{}),
            FlightClass::Economy => Box::new(EconomyFee{})
        }
    }
}