use aper::StateMachineContainerProgram;
use aper_actix::ServerBuilder;
use state::Game;

fn main() -> std::io::Result<()> {
    ServerBuilder::new(StateMachineContainerProgram(Game::default())).serve()
}
