pub mod vegetables;
use crate::garden::vegetables::Vegetables;

pub struct Garden {
    pub season: String,
    pub crop: Vegetables,
}
