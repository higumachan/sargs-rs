use clap::Parser;
use itertools::Itertools;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncBufReadExt;
use tokio::process::Command;
use tokio::task::JoinSet;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(short = 'I')]
    input_placeholder: Option<String>,
    #[clap(long)]
    buffer_size: Option<usize>,
    args: Vec<String>,
}

struct Fifo {
    data_receivers: Vec<tokio::sync::oneshot::Receiver<String>>,
    last_wait_receiver: Option<tokio::sync::oneshot::Receiver<()>>,
    buffer_size: usize,
}

enum Argument {
    InputPlaceholder,
    Arg(String),
}

impl Fifo {
    fn new(buffer_size: usize) -> Self {
        Self {
            data_receivers: Vec::with_capacity(buffer_size),
            last_wait_receiver: None,
            buffer_size,
        }
    }

    fn sender(&mut self) -> Option<FifoSender> {
        if self.data_receivers.len() >= self.buffer_size {
            return None;
        }

        let (data_sender, data_receiver) = tokio::sync::oneshot::channel();
        let (wait_sender, wait_receiver) = tokio::sync::oneshot::channel();
        let last_wait_receiver = self.last_wait_receiver.replace(wait_receiver);
        self.data_receivers.push(data_receiver);

        Some(FifoSender {
            waiter: last_wait_receiver,
            data_sender,
            wait_sender,
        })
    }
}

struct FifoSender {
    waiter: Option<tokio::sync::oneshot::Receiver<()>>,
    data_sender: tokio::sync::oneshot::Sender<String>,
    wait_sender: tokio::sync::oneshot::Sender<()>,
}

impl FifoSender {
    async fn send(self, value: String) {
        if let Some(waiter) = self.waiter {
            waiter.await.unwrap();
        }
        self.data_sender.send(value).unwrap();
        self.wait_sender.send(()).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let args = Arc::new(
        cli.args
            .into_iter()
            .map(|s| {
                if matches!(&cli.input_placeholder, Some(input_placeholder) if &s == input_placeholder) {
                    Argument::InputPlaceholder
                } else {
                    Argument::Arg(s)
                }
            })
            .collect_vec(),
    );
    let has_placeholder = cli.input_placeholder.is_some();
    let mut fifo = Arc::new(Mutex::new(Fifo::new(cli.buffer_size.unwrap_or(1000))));
    let mut buf_reader = tokio::io::BufReader::new(tokio::io::stdin());
    let join_handle = tokio::spawn({
        let fifo = fifo.clone();
        async move {
            let mut line_buffer = String::new();
            let mut join_set = JoinSet::new();
            while matches!(buf_reader.read_line(&mut line_buffer).await, Ok(n) if n > 0) {
                if let Some(sender) = fifo.lock().unwrap().sender() {
                    join_set.spawn({
                        let mut args = args
                            .clone()
                            .iter()
                            .map(|arg| match arg {
                                Argument::InputPlaceholder => line_buffer.trim().to_string(),
                                Argument::Arg(s) => s.clone(),
                            })
                            .collect_vec();
                        if !has_placeholder {
                            args.push(line_buffer.trim().to_string());
                        }
                        async move {
                            let mut child = Command::new(&args[0])
                                .args(&args[1..])
                                .stdout(std::process::Stdio::piped())
                                .spawn()
                                .unwrap();

                            let output = child.wait_with_output().await.unwrap();

                            sender
                                .send(String::from_utf8_lossy(&output.stdout).to_string())
                                .await;
                        }
                    });
                    line_buffer.clear();
                } else {
                    eprintln!("Buffer full");
                    std::process::exit(-1);
                }
            }
            while let Some(res) = join_set.join_next().await {}
        }
    });
    while !join_handle.is_finished() {
        for data_receiver in fifo.lock().unwrap().data_receivers.drain(..) {
            print!("{}", data_receiver.await.unwrap());
        }
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
    for data_receiver in fifo.lock().unwrap().data_receivers.drain(..) {
        print!("{}", data_receiver.await.unwrap());
    }
}
