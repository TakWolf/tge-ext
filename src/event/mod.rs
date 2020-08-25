use tge::prelude::*;

pub trait EventHandler {
    fn handle_event(&mut self, engine: &mut Engine, event: Event) -> GameResult<bool> {
        match event {
            Event::AppSuspend => self.on_app_suspend(engine),
            Event::AppResume => self.on_app_resume(engine),
            Event::WindowClose => return self.on_window_close_request(engine),
            Event::WindowResize(size) => self.on_window_resize(engine, size),
            Event::WindowMove(position) => self.on_window_move(engine, position),
            Event::WindowFocusChange(focused) => self.on_window_focus_change(engine, focused),
            Event::ReceiveChar(character) => self.on_receive_char(engine, character),
            Event::KeyboardInput { key, action, repeated } => self.on_keyboard_input(engine, key, action, repeated),
            Event::ModifiersChange(state) => self.on_modifiers_change(engine, state),
            Event::MouseMove(position) => self.on_mouse_move(engine, position),
            Event::MouseEnterWindow => self.on_mouse_enter_window(engine),
            Event::MouseLeaveWindow => self.on_mouse_leave_window(engine),
            Event::MouseWheelScroll(delta) => self.on_mouse_wheel_scroll(engine, delta),
            Event::MouseInput { button, action } => self.on_mouse_input(engine, button, action),
            Event::Touch { id, phase, position } => self.on_touch(engine, id, phase, position),
            Event::TouchpadScroll { delta, phase } => self.on_touchpad_scroll(engine, delta, phase),
            Event::TouchpadPress { pressure, click_stage } => self.on_touchpad_press(engine, pressure, click_stage),
            Event::GamepadConnect(gamepad_id) => self.on_gamepad_connect(engine, gamepad_id),
            Event::GamepadDisconnect(gamepad_id) => self.on_gamepad_disconnect(engine, gamepad_id),
            Event::GamepadButtonInput { id, button, action } => self.on_gamepad_button_input(engine, id, button, action),
            Event::GamepadButtonChange { id, button, value } => self.on_gamepad_button_change(engine, id, button, value),
            Event::GamepadAxisChange { id, axis, value } => self.on_gamepad_axis_change(engine, id, axis, value),
        }.map(|_| false)
    }

    fn on_app_suspend(&mut self, _engine: &mut Engine) -> GameResult {
        Ok(())
    }

    fn on_app_resume(&mut self, _engine: &mut Engine) -> GameResult {
        Ok(())
    }

    fn on_window_close_request(&mut self, _engine: &mut Engine) -> GameResult<bool> {
        Ok(false)
    }

    fn on_window_resize(&mut self, _engine: &mut Engine, _size: LogicalSize) -> GameResult {
        Ok(())
    }

    fn on_window_move(&mut self, _engine: &mut Engine, _position: LogicalPosition) -> GameResult {
        Ok(())
    }

    fn on_window_focus_change(&mut self, _engine: &mut Engine, _focused: bool) -> GameResult {
        Ok(())
    }

    fn on_receive_char(&mut self, _engine: &mut Engine, _c: char) -> GameResult {
        Ok(())
    }

    fn on_keyboard_input(&mut self, _engine: &mut Engine, _key: KeyCode, _action: KeyAction, _repeated: bool) -> GameResult {
        Ok(())
    }

    fn on_modifiers_change(&mut self, _engine: &mut Engine, _state: ModifiersState) -> GameResult {
        Ok(())
    }
    
    fn on_mouse_move(&mut self, _engine: &mut Engine, _position: LogicalPosition) -> GameResult {
        Ok(())
    }
    
    fn on_mouse_enter_window(&mut self, _engine: &mut Engine) -> GameResult {
        Ok(())
    }
    
    fn on_mouse_leave_window(&mut self, _engine: &mut Engine) -> GameResult {
        Ok(())
    }
    
    fn on_mouse_wheel_scroll(&mut self, _engine: &mut Engine, _delta: Vector) -> GameResult {
        Ok(())
    }
    
    fn on_mouse_input(&mut self, _engine: &mut Engine, _button: MouseButton, _action: KeyAction) -> GameResult {
        Ok(())
    }
    
    fn on_touch(&mut self, _engine: &mut Engine, _id: u64, _phase: TouchPhase, _position: LogicalPosition) -> GameResult {
        Ok(())
    }
    
    fn on_touchpad_scroll(&mut self, _engine: &mut Engine, _delta: Vector, _phase: TouchPhase) -> GameResult {
        Ok(())
    }
    
    fn on_touchpad_press(&mut self, _engine: &mut Engine, _pressure: f32, _click_stage: i64) -> GameResult {
        Ok(())
    }
    
    fn on_gamepad_connect(&mut self, _engine: &mut Engine, _id: GamepadId) -> GameResult {
        Ok(())
    }
    
    fn on_gamepad_disconnect(&mut self, _engine: &mut Engine, _id: GamepadId) -> GameResult {
        Ok(())
    }
    
    fn on_gamepad_button_input(&mut self, _engine: &mut Engine, _id: GamepadId, _button: GamepadButton, _action: KeyAction) -> GameResult {
        Ok(())
    }

    fn on_gamepad_button_change(&mut self, _engine: &mut Engine, _id: GamepadId, _button: GamepadButton, _value: f32) -> GameResult {
        Ok(())
    }

    fn on_gamepad_axis_change(&mut self, _engine: &mut Engine, _id: GamepadId, _axis: GamepadAxis, _value: f32) -> GameResult {
        Ok(())
    }
}
