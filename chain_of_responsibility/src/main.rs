mod chain_of_responsibility;

use crate::chain_of_responsibility::support::{
    LimitSupport, NoSupport, OddSupport, SpecialSupport, SupportTrait,
};
use crate::chain_of_responsibility::trouble::Trouble;

fn start_main() {
    let mut alice = Box::new(NoSupport::new("Alice".to_string()));
    let mut bob = Box::new(LimitSupport::new("Bob".to_string(), 100));
    let mut charlie = Box::new(SpecialSupport::new("Charlie".to_string(), 429));
    let mut diana = Box::new(LimitSupport::new("Diana".to_string(), 200));
    let mut elmo = Box::new(OddSupport::new("Elmo".to_string()));
    let fred = Box::new(LimitSupport::new("Fred".to_string(), 300));

    elmo.set_next(fred);
    diana.set_next(elmo);
    charlie.set_next(diana);
    bob.set_next(charlie);
    alice.set_next(bob);

    let mut i = 0;
    while i < 500 {
        alice.handle(Trouble::new(i));
        i += 33;
    }
}

fn main() {
    start_main();
}
