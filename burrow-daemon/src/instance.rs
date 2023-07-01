use super::*;

pub struct Instance {
    rx: Receiver<Command>,
}

impl Instance {
    pub fn new(rx: Receiver<Command>) -> Self {
        Self { rx }
    }

    pub async fn run(&mut self) {
        while let Some(command) = self.rx.recv().await {
            match command {
                Command::Start(_options) => {
                    todo!()
                }
                Command::End => {
                    todo!()
                }
            }
        }
    }
}
