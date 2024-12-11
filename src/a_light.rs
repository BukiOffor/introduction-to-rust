/// This asssignment simulates a given state machine; in this case a light switch.
/// A state machine keeps a particular state and when a action is performed on that
/// state, the state changes to another. we simulate this with a light bulb switch.
///
/// At any given time, a light swich can either be on or off. if you turn off the switch
/// it becomes off and vice versa. For this program we assume that the state of the on switch is
/// the boolean `true` while turning off is the boolean `false`.

pub trait StateMachine {
    /// The states that can be occupied by this machine
    type State;

    /// The transitions that can be made between states
    type Transition;

    /// Change the state of the bulb, by turning on the switch
    fn turn_on(&self, t: &Self::Transition) -> Self::State;

    /// Change the state of the bulb, by turning off the switch

    fn turn_off(&self, t: &Self::Transition) -> Self::State;
    /// toggle wahtever state that was passed as an argument
    fn toggle(&self, t: &Self::State) -> Self::State;

    fn human_name() -> String {
        "light awitch".into()
    }
}

pub struct LightSwitch;

impl StateMachine for LightSwitch {
    type State = bool;
    type Transition = ();

    // turn the light on
    fn turn_on(&self, t: &Self::Transition) -> Self::State {
        todo!()
    }

    // turn the light off
    fn turn_off(&self, t: &Self::Transition) -> Self::State {
        todo!()
    }

    // change whatever state that was passed as an arguement
    fn toggle(&self, t: &Self::State) -> Self::State {
        todo!()
    }
}

/// Please do not touch the TESTS ⚠️ ⚠️ ⛔️

#[test]
fn light_turn_on_switch() {
    let switch = LightSwitch;
    assert_eq!(true, switch.turn_on(&()))
}

#[test]
fn light_turn_off_switch() {
    let switch = LightSwitch;
    assert_eq!(false, switch.turn_off(&()))
}

#[test]
fn light_toggle_from_on_to_off() {
    let switch = LightSwitch;
    assert_eq!(true, switch.toggle(&false))
}

#[test]
fn light_toggle_from_off_to_on() {
    let switch = LightSwitch;
    assert_eq!(false, switch.toggle(&true))
}
