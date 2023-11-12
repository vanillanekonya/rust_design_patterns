use core::panic;
use std::thread::panicking;


trait Dialog {
    fn createButton(&self) -> Box<dyn Button>;
    fn render(&self) {
        let okButton = self.createButton();
        okButton.onClick();
        okButton.render();
    }
}

struct WindowsDialog {}
struct WebDialog {}

impl Dialog for WindowsDialog {
    fn createButton(&self) -> Box<dyn Button> {
        return Box::new(WindowsButton{});
    }
}

impl Dialog for WebDialog {
    fn createButton(&self) -> Box<dyn Button> {
        return Box::new(HTMLButton{});
    }
}

trait Button {
    fn render(&self);
    fn onClick(&self);
}

struct WindowsButton {}
struct HTMLButton {}

impl Button for WindowsButton {
    fn render(&self) {
        println!("windows button render");
    }

    fn onClick(&self) {
        println!("windows button onclick");
    }
}

impl Button for HTMLButton {
    fn render(&self) {
        println!("html button render");
    }

    fn onClick(&self) {
        println!("html button onclick");
    }
}


fn main() {
    loop {
        let mut input_text = String::new();
        // 通过 stdin() 获取标准输入的句柄
        std::io::stdin().read_line(&mut input_text)
            .expect("Failed to read line");

        println!("input text equal");

        let dialog: &dyn Dialog;
        if input_text == "Windows" {
            dialog = &WindowsDialog{}
        }
        else if input_text == "Web" {
            dialog = &WebDialog{}
        }
        else {
            panic!("type not right");
        }

        dialog.render();
    }
    
}
