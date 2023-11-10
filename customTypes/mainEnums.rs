enum WebEvent {
    PageLoad,
    PageUnLoad,

    KeyPress(char),
    Paste(String),
    // c-like structures
    Click {x: i64, y:i64},
}

fn inspect (event:WebEvent){
    match event {
        WebEvent::PageLoad => println!("Page Loaded"),
        WebEvent::PageUnLoad => println!("Page Unloaded"),

        WebEvent::KeyPress(c) => println!("Pressed char is {}.", c),
        WebEvent::Paste(s) => println!("Paste \"{}\". ", s),

        WebEvent::Click{x, y} => {
            println!("clicked at x={}, y={},", x, y);
        },
    }
}


fn main(){
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click {x:20, y:80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
