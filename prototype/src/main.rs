mod framework;

use crate::framework::manager::{Manager, ManagerTrait};
use crate::framework::message_box_prototype::MessageBox;
use crate::framework::undeline_pen_prototype::UnderlinePen;

fn start_main(mut manager_object: Box<dyn ManagerTrait>) {
    let upen = UnderlinePen::new('-');
    let mbox = MessageBox::new('*');
    let sbox = MessageBox::new('/');
    manager_object.register("strong message".to_string(), Box::new(upen));
    manager_object.register("warning box".to_string(), Box::new(mbox));
    manager_object.register("slash box".to_string(), Box::new(sbox));

    let p1 = manager_object.create("strong message".to_string());
    let p2 = manager_object.create("warning box".to_string());
    let p3 = manager_object.create("slash box".to_string());
    p1.use_prototype("Hello World".to_string());
    p2.use_prototype("Hello World".to_string());
    p3.use_prototype("Hello World".to_string());
}

fn main() {
    start_main(Box::new(Manager::new()))
}
