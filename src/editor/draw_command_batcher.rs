use std::sync::mpsc::{channel, Receiver, SendError, Sender};

use crate::{editor::DrawCommand, event_aggregator::EVENT_AGGREGATOR, LoggingTx};

pub struct DrawCommandBatcher {
    window_draw_command_sender: Sender<DrawCommand>,
    window_draw_command_receiver: Receiver<DrawCommand>,
    draw_command_tx: LoggingTx<Vec<DrawCommand>>,
}

impl DrawCommandBatcher {
    pub fn new() -> DrawCommandBatcher {
        let (sender, receiver) = channel();

        DrawCommandBatcher {
            window_draw_command_sender: sender,
            window_draw_command_receiver: receiver,
            draw_command_tx: EVENT_AGGREGATOR.get_sender(),
        }
    }

    pub fn queue(&self, draw_command: DrawCommand) -> Result<(), Box<SendError<DrawCommand>>> {
        self.window_draw_command_sender
            .send(draw_command)
            .map_err(Box::new)
    }

    pub fn send_batch(&self) {
        let batch: Vec<DrawCommand> = self.window_draw_command_receiver.try_iter().collect();
        self.draw_command_tx.send(batch).unwrap();
    }
}
