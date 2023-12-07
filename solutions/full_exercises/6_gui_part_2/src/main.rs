//This is a simple GUI library 
//There are three widgets that implement the `Widget` trait: `Label`, `Button`, and `Window`
//For this exercise, you only have to print them to stdout, so the `Button` does not have any click functionality
//TODO complete the code to make it work

//info: trait objects do not have a known size at compile time, therefore, you cannot simply move its value into function arguments
//Notice how the main function uses Boxes to pass the trait objects
//This approach usually makes implementation easier because the trait objects now reside on the heap and the compiler does not need to know their size  
//Another approach is to use references to pass trait objects as function arguments

//SOLUTION: the `Window` structs now holds a vector of immutable REFERENCES to widgets, instead of the widgets themselves
//`Window` has thus no ownership anymore over the widgets, as was the case in the previous gui exercise
//The compiler checks if the program complies with the ownership rules at compile time, for which it needs to infer how long every object lives
//In some cases however, the compiler cannot infer these "lieftimes" of objects all by itself, which is, for example, the case when objects contain references to other objects
//Therfore, you (the programmer) has to add "lifetime parameters" that specify how long the involved objects live

pub trait Widget {
    //draw the widget into a buffer
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    //draw the widget on standard output
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", buffer);
    }
}

pub struct Label {
    label: String,
}

//SOLUTION
impl Label {
    fn new(label: &str) -> Self {
        Label {label: label.to_owned()}
    }
}

//SOLUTION
impl Widget for Label {
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        write!(buffer, "{}\n", self.label).unwrap();
    }
}

pub struct Button {
    label: Label,
}

//SOLUTION
impl Button {
    fn new(label: &str) -> Self {
        Button {label: Label::new(label)}
    }
}

//SOLUTION
impl Widget for Button {
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        write!(buffer, "| {} |\n", self.label.label).unwrap(); 
    }
}

//SOLUTION info: lifetime parameter `'a` says that the objects referenced in the vector live at least as long as the `Window` itself
pub struct Window<'a> {
    title: String,
    widgets: Vec<&'a dyn Widget>,
}

//SOLUTION info: the parameter tied to the `impl` keyword defines the used lifetime parameter in the block, and `Window` needs a parameter because its definition has one
impl<'a> Window<'a> {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    //SOLUTION: lifetime parameter `'a` says that the object referenced by the argument lives at least as long as the `Window` itself
    //note: the `self` argument does not need a parameter because the reference has no complex lifetime, it simply goes out of scope when the function returns and the compiler can infer this by itself
    fn add_widget(&mut self, widget: &'a dyn Widget) {
        self.widgets.push(widget);
    }
}

//SOLUTION info: the definition of `Window` has a lifetime parameter so you have to specify it
//However, the parameter itself is not used inside the block so it can be unnamed and the `impl` keyword doesn't need a parameter
impl Widget for Window<'_> {
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        write!(buffer, "========\n").unwrap();
        write!(buffer, "{}\n", self.title).unwrap();
        write!(buffer, "========\n").unwrap();

        for w in &self.widgets {
            w.draw_into(buffer);
        }
    }
}

fn main() {
    //the GUI this program prints should look like this:
    // ========
    // Rust GUI Demo
    // ========
    // This is a small text GUI demo.
    // | Click me! |
    
    let mut window = Window::new("Rust GUI Demo");
    let label = Label::new("This is a small text GUI demo.");
    let button = Button::new("Click me!");
    window.add_widget(&label);
    window.add_widget(&button);
    window.draw();
}
