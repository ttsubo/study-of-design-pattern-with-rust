mod composite;

use crate::composite::directory::Directory;
use crate::composite::entry::Entry;
use crate::composite::file::File;

fn start_main() {
    println!("Making root entries...");
    let mut root_dir = Box::new(Directory::new("root".to_string()));
    let mut bin_dir = Box::new(Directory::new("bin".to_string()));
    let tmp_dir = Box::new(Directory::new("tmp".to_string()));
    let mut usr_dir = Box::new(Directory::new("usr".to_string()));

    bin_dir.add(Box::new(File::new("vi".to_string(), 10000)));
    bin_dir.add(Box::new(File::new("latex".to_string(), 20000)));

    root_dir.add(bin_dir);
    root_dir.add(tmp_dir);
    root_dir.print_list("".to_string());

    println!("");
    println!("Making user entries...");
    let mut yuki = Box::new(Directory::new("yuki".to_string()));
    let mut hanako = Box::new(Directory::new("hanako".to_string()));
    let mut tomura = Box::new(Directory::new("tomura".to_string()));
    yuki.add(Box::new(File::new("diary.html".to_string(), 100)));
    yuki.add(Box::new(File::new("Composite.java".to_string(), 200)));
    hanako.add(Box::new(File::new("memo.tex".to_string(), 300)));
    tomura.add(Box::new(File::new("game.doc".to_string(), 400)));
    tomura.add(Box::new(File::new("jumk.mail".to_string(), 500)));

    usr_dir.add(yuki);
    usr_dir.add(hanako);
    usr_dir.add(tomura);
    root_dir.add(usr_dir);
    root_dir.print_list("".to_string());
    println!("");
}

fn main() {
    start_main();
}
