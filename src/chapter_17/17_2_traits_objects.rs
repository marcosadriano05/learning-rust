// Trait objects are used to represent data that are realationed, implementing polimorphism.
// To store diferent objects (with diferent types), but implementing the same trait,
// the dyn keyword is used.
trait Drawn {
    fn drawn(&self);
}

struct Screen {
    conponents: Vec<Box<dyn Drawn>>,
}

impl Screen {
    fn run(&self) {
        for component in self.conponents.iter() {
            component.drawn();
        }
    }
}

struct Button {}

impl Drawn for Button {
    fn drawn(&self) {
        println!("Inside Button");
    }
}

struct TextArea {}

impl Drawn for TextArea {
    fn drawn(&self) {
        println!("inside TextArea");
    }
}

fn main() {
    let screen = Screen {
        conponents: vec![Box::new(Button {}), Box::new(TextArea {})],
    };

    screen.run();
}
