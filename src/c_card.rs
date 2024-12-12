//! We discussed in class how a house can be broken down into 
//! different units and built programmatically. We will attempt to do so in this 
//! assigmnent with a struct called `Building`. 
//! 
//! Now in building a house, there are various steps that can work in parallel and
//! there are some steps that needs to build in sequence. for example the foundation of 
//! the house needs to be laid and a building erected before an electician can wire the house.
//! 
//! We will create a struct and initialize this, at the point of the initialization. the foundation
//! field will be set to true. This means that the foundation exists and other components of the house
//! can now come in.
//! 

use std::fmt::Display;
#[derive(Debug,Default)]
pub enum BuildingError {
    #[default]
    EngineerHasQuit,
    DeckOneHasNotBeenBuilt,
    DeckTwoHasNotBeenBuilt
}

impl Display for BuildingError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BuildingError{}

#[derive(Debug,Clone, Copy)]
pub struct Building {
    /// this must be set to true at the instanciation of the struct
    foundation: bool,
    deck_1: bool,
    deck_2: bool,
    roofing: bool,
    garage: bool,
    gate: bool
}




impl Building {
    /// create an instance of a building, 
    // Everything should be set to false asides from foundation.
    pub fn new() -> Self {
        todo!()
    }

    pub fn build_deck_1(&mut self) -> Self {
        todo!()   
    }

    pub fn build_deck_2(&mut self) -> Result<Self, BuildingError> {
        todo!()
    }

    pub fn build_roofing(&mut self) -> Result<Self, BuildingError> {
        todo!()
    }


    pub fn is_house_completed(&self) -> bool {
        todo!()
    }

    /// This method builds the garage in the building
    // it has already been implemented for you as an example
    pub fn build_garage(&mut self) -> Self {
        self.garage = true;
        self.clone()
    }

    /// This method builds the gate in the building
    // it has already been implemented for you as an example
    pub fn build_gate(&mut self) -> Self {
        self.gate = true;
        self.clone()
    }


}


/// This function is not graded. It is just for collecting feedback.
/// On a scale from 0 - 255, with zero being extremely easy and 255 being extremely hard,
/// how hard did you find this section of the exam.
pub fn how_hard_was_this_section() -> u8 {
	todo!()
	
}

/// This function is not graded. It is just for collecting feedback.
/// How much time (in hours) did you spend on this section of the exam?
pub fn how_many_hours_did_you_spend_on_this_section() -> u8 {
	todo!()
	
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn card_test_create_a_buiding(){
        let building = Building::new();
        assert_eq!(building.foundation, true, "You did not set the value of foundation to be true");
        assert_eq!(building.deck_1, false, "Deck 1 should be false on init");
        assert_eq!(building.deck_2, false, "Deck 2 should be false on init");
        assert_eq!(building.roofing, false, "The roofing should be false on init");
    }

    #[test]
    fn card_test_create_first_decking(){
        let mut building = Building::new();
        building.build_deck_1();
        assert!(building.foundation);
        assert_eq!(building.deck_1, true, "You did not set the value of deck_1 to be true");
        assert_eq!(building.deck_2, false, "Deck 2 should be false");
        assert_eq!(building.roofing, false, "The roofing should be false");
    }
    #[test]
    fn card_test_create_seconding_decking(){
        let mut building = Building::new();
        assert!(building.foundation);  
        building.build_deck_1();
        building.build_deck_2().unwrap();
        assert_eq!(building.deck_1, true, "Deck One should exists before building deck 2");
        assert_eq!(building.deck_2, true, "You did not set the value of deck_2 to be true");
        assert_eq!(building.roofing, false, "The roofing should be false");
    }

    #[test]
    fn card_test_create_seconding_decking_should_fail_if_deck_1_is_not_built(){
        let mut building = Building::new();
        assert!(building.foundation);  
        assert!(building.build_deck_2().is_err());
        
    }


    #[test]
    fn card_test_create_roof(){
        let mut building = Building::new();
        assert!(building.foundation);  
        building.build_deck_1();
        building.build_deck_2().unwrap();
        building.build_roofing().unwrap();
        assert_eq!(building.deck_1, true, "Deck One should exists before building deck 2");
        assert_eq!(building.deck_2, true, "You did not set the value of deck_2 to be true");
        assert_eq!(building.roofing, true, "The roofing should be true");
    }


    #[test]
    fn card_test_build_deck_2_fails(){
        let mut building = Building::new();
        assert!(building.foundation);  
        assert_eq!(building.build_deck_2().is_err(), true, "you should not build deck_2 without building deck_1");   
    }

    #[test]
    fn card_test_build_roofing_fails_without_building_deck_2(){
        let mut building = Building::new();
        assert!(building.foundation);  
        building.build_deck_1();
        assert_eq!(building.build_roofing().is_err(), true, "you should not build roofing without building deck_2");   
    }

    #[test]
    fn card_test_build_roofing_fails_without_building_any_deck(){
        let mut building = Building::new();
        assert!(building.foundation);  
        assert_eq!(building.build_roofing().is_err(), true, "you should not build roofing without building deck_1 and deck_2");   
    }

    #[test]
    fn card_test_building_is_completed(){
        let mut building = Building::new();
        assert!(building.foundation);  
        building.build_deck_1();
        building.build_deck_2().unwrap();
        building.build_roofing().unwrap();
        building.build_roofing().unwrap();
        building.build_gate();
        building.build_garage();
        assert!(building.is_house_completed()) 
    }

    #[test]
    fn card_test_building_is_not_completed(){
        let mut building = Building::new();
        assert!(building.foundation);  
        building.build_deck_1();
        building.build_deck_2().unwrap();
        building.build_roofing().unwrap();
        building.build_gate();
        assert!(!building.is_house_completed()) 
    }

}