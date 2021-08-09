mod factory;

use crate::factory::list_factory::ListFactory;
use crate::factory::table_factory::TableFactory;
use crate::factory::Factory;
use std::env;

fn start_main(factory: Box<dyn Factory>) {
    let asahi = factory.create_link("Asahi".to_string(), "http://www.asahi.com/".to_string());
    let yomiuri = factory.create_link(
        "Yomiuri".to_string(),
        "http://www.yomiuri.co.jp/".to_string(),
    );
    let us_yahoo = factory.create_link("Yahoo".to_string(), "http://www.yahoo.com/".to_string());
    let jp_yahoo = factory.create_link(
        "Yahoo!Japan".to_string(),
        "http://www.yahoo.co.jp/".to_string(),
    );
    let google = factory.create_link("Google".to_string(), "http://www.google.com/".to_string());
    let excite = factory.create_link("Excite".to_string(), "http://www.excite.com/".to_string());

    let mut tray_news = factory.create_tray("Newspaper".to_string());
    tray_news.add(asahi);
    tray_news.add(yomiuri);

    let mut tray_yahoo = factory.create_tray("Yahoo!".to_string());
    tray_yahoo.add(us_yahoo);
    tray_yahoo.add(jp_yahoo);

    let mut tray_search = factory.create_tray("Search Engine".to_string());
    tray_search.add(tray_yahoo);
    tray_search.add(excite);
    tray_search.add(google);

    let mut page = factory.create_page("LinkPage".to_string(), "Hiroshi Yuki".to_string());
    page.add(tray_news);
    page.add(tray_search);
    page.output();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1].as_str() == "ListFactory" {
        start_main(Box::new(ListFactory {}));
    } else if args[1].as_str() == "TableFactory" {
        start_main(Box::new(TableFactory {}));
    }
}
