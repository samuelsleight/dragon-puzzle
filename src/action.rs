use leafwing_input_manager::Actionlike;

#[derive(Clone, Copy, Hash, Debug)]
pub enum MovementAction {
    Forwards,
    TurnLeft,
    TurnRight,
}

#[derive(Actionlike, Clone, Copy, Hash, Debug)]
pub enum Action {
    MovementForwards,
    MovementTurnLeft,
    MovementTurnRight,
    SwitchLevel,
}

impl Action {
    pub fn movement(self) -> Option<MovementAction> {
        match self {
            Action::MovementForwards => Some(MovementAction::Forwards),
            Action::MovementTurnLeft => Some(MovementAction::TurnLeft),
            Action::MovementTurnRight => Some(MovementAction::TurnRight),
            _ => None,
        }
    }
}
