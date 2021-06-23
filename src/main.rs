use aper::StateMachineContainerProgram;
use aper_actix::ServerBuilder;
use state::Game;

fn main() -> std::io::Result<()> {
    let builder = ServerBuilder::new(StateMachineContainerProgram(Game::default()));
    if std::env::args().len() > 1 {
        builder.serve_on("0.0.0.0", 666)
    } else {
        builder.serve()
    }
}
