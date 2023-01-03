
use observable::examples::internetstuff::InternetStuff;


fn main() {
    let mut it = InternetStuff::new();
    
    it.connect("Jeremy");
    it.connect("John");

    it.disconnect("Jeremy");

}