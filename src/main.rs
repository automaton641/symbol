use sentidos::*;

fn mouse_input_handler(
    _window: &mut Window<SymbolProgram>,
    button: MouseButton,
    state: ElementState,
    position: (usize, usize),
) {
    match button {
        MouseButton::Left => match state {
            ElementState::Pressed => {
                println!("left mouse button pressed");
                println!("position {}, {}", position.0, position.1);
            }
            _ => (),
        },
        _ => (),
    }
}

struct SymbolProgram {}

impl SymbolProgram {
    fn new() -> SymbolProgram {
        SymbolProgram {}
    }
}

fn main() {
    let symbol_program = SymbolProgram::new();
    let mut window = Window::new(symbol_program, "symbol", 1024, 512);
    window.add_mouse_input_handler(mouse_input_handler);
    window.show();
}
