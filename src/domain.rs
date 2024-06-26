use unicode_segmentation::UnicodeSegmentation;
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
pub struct SubscriberName(String);
impl SubscriberName {
    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{} is not a valid subscriber name.", s)
        } else {
            Self(s)
        }
    }
    // Count also do
    /*
    pub fn inner(self) -> String {
        self.0 // Return the string
    }
    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.0 // Return the string
    }
    */
    pub fn inner_ref(&self) -> &String {
        &self.0 // Return the string
    }
}
