mod singleton;

use singleton::GET_INSTANCE;

fn main() {
    {
        let one = GET_INSTANCE.read().unwrap();
        println!("# one.inputは、\"{}\" でした", one.get_value());
        println!("");
    }
    {
        let mut two = GET_INSTANCE.write().unwrap();
        println!("# two.inputは、\"{}\" でした", two.get_value());
        println!("# two.inputに、\"2\" を設定します");
        two.set_value(2);
        println!("");
    }
    {
        let mut three = GET_INSTANCE.write().unwrap();
        println!("# three.inputは、\"{}\" でした", three.get_value());
        println!("");

        println!("# three.inputに、\"3\" を設定します");
        three.set_value(3);
        println!("# three.inputは、\"{}\" でした", three.get_value());
        println!("");

        println!("# three.inputに、\"0\" を設定します");
        three.input = 0;
        println!("# three.inputは、\"{}\" でした", three.get_value());
    }
}
