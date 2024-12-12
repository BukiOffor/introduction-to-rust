/// Best to use your “indoor voice” sometimes, writing entirely in lowercase.
/// There are methods that can be called on data types. We used the `checked_add()`
/// in class to safely add a number in the runtime. In this program we will explore methods that
/// can be found on string types

const _PLEA: &str = "PLEASE, study at HOME";

/// This function takes a string literal as an argument and returns an Owned string
/// in all lower case
pub fn indoor_voice(sentence: &str) -> String {
    let str:String = "this is a string".to_owned();
    todo!()
}
/// This function takes a string literal as an argument and returns an Owned string
/// with leading or trailing whitespace remove.
pub fn remove_whitespce(sentence: &str) -> String {
    todo!()
}
/// This function takes a string literal and adds ... where ever it finds a white space
pub fn slur_my_speech(sentence: &str) -> String {
    todo!()
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

// Please do not touch the tests ❌ ❌ ❌
#[test]
fn methods_test_indoor_voice() {
    assert!(indoor_voice("SENTENCE").ends_with("sentence"))
}

#[test]
fn methods_test_remove_with_space() {
    assert!(remove_whitespce("  buki   ").ends_with("buki"))
}

#[test]
fn methods_test_slur_my_speech() {
    assert!(slur_my_speech("please i need you to study")
        .ends_with("need...you...to...study"))
}
