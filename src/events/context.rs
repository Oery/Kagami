use crate::context::BufferedStream;

pub struct Context<'a, 'b, T> {
    pub payload: T,
    pub writer: &'b mut BufferedStream<'a>,
    pub should_filter: bool,
}

impl<'a, 'b, T> Context<'a, 'b, T> {
    pub fn new(payload: T, ctx: &'b mut BufferedStream<'a>) -> Context<'a, 'b, T> {
        Context { payload, writer: ctx, should_filter: false }
    }
}
