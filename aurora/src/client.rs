use std::{
    io::{self, Lines, StdinLock, StdoutLock, Write},
    marker::PhantomData,
};

use crate::{InitRequest, Message, Node, RequestBody};

pub struct Client<N> {
    marker: PhantomData<N>,
    stdin: Lines<StdinLock<'static>>,
    stdout: StdoutLock<'static>,
}

pub const INIT_ERR_MSG: &str = "init message not given";
pub const STDIN_ERR_MSG: &str = "failed to read line from stdin";
pub const STDOUT_ERR_MSG: &str = "failed to write line from stdin";
pub const PARSE_ERR_MSG: &str = "failed to parse given message";
pub const SER_ERR_MSG: &str = "failed to serialize given message";

impl<N: Node> Client<N> {
    pub fn new() -> (Self, N) {
        let mut stdin = io::stdin().lines();
        let raw_init: String = stdin.next().expect(INIT_ERR_MSG).expect(STDIN_ERR_MSG);
        let init: Message<InitRequest> =
            serde_json::from_str(&raw_init).expect("failed to parse init message");
        let mut node = N::init(&init);
        let stdout = io::stdout().lock();
        let mut digest = Self {
            marker: PhantomData,
            stdin,
            stdout,
        };
        digest.send_msg(init.into_response(node.next_id()));
        (digest, node)
    }

    pub fn next_msg(&mut self) -> Option<Message<N::Body>> {
        self.stdin
            .next()
            .map(|line| serde_json::from_str(&line.expect(STDIN_ERR_MSG)).expect(PARSE_ERR_MSG))
    }

    pub fn send_msg<B>(&mut self, msg: Message<B>)
    where
        B: RequestBody,
    {
        writeln!(
            self.stdout,
            "{}",
            serde_json::to_string(&msg).expect(SER_ERR_MSG)
        )
        .expect(STDOUT_ERR_MSG);
    }
}
