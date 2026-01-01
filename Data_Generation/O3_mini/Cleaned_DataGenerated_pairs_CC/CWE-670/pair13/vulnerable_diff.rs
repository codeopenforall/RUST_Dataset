                Mode::Idle if trigger == true => {
                    let new_state = Mode::Idle; 
                    self.mode = new_state;
