pub mod garden;
use garden::vegetables::Vegetables;

use crate::garden::Garden;

fn main() {
    let garden = Garden {
        season: String::from("Summer"),
        crop: Vegetables::Carrots,
    };
}
