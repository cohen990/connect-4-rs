use overengineered::game_loop;

#[cfg(test)]
mod acceptance_test;
mod game;
mod overengineered;

fn main() {
    game_loop::play()
}
