use aper::StateMachineContainerProgram;
use aper_actix::ServerBuilder;
use state::dice::Dice;

fn main() -> std::io::Result<()> {
    ServerBuilder::new(StateMachineContainerProgram(Dice::default())).serve()
}
