use super::*;

pub struct Instance {
    rx: Receiver<DaemonCommand>,
}

impl Instance {
    pub fn new(rx: Receiver<DaemonCommand>) -> Self {
        Self { rx }
    }

    pub async fn run(&mut self) {
        while let Some(command) = self.rx.recv().await {
            match command {
                DaemonCommand::Start(_options) => {
                    todo!()
                }
                DaemonCommand::End => {
                    todo!()
                }
            }
        }
    }
}
