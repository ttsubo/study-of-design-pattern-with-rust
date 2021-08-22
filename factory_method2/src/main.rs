mod framework;
use crate::framework::id_card_factory::IDCardFactory;

fn start_main(mut factory: IDCardFactory) {
    let card1 = factory.create("Hiroshi Yuki".to_string());
    let card2 = factory.create("Tomura".to_string());
    let card3 = factory.create("Hanako Sato".to_string());
    card1.use_product();
    card2.use_product();
    card3.use_product();
}

fn main() {
    start_main(IDCardFactory::new())
}
