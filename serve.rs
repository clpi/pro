use anyhow;
use smol::{
    stream::{self, StreamExt, CollectFuture},
    Async, 
};

fn main() -> anyhow::Result<()> {
    println!("Hi world");
    // smol::block_on(async {
    //     let lst = Async::<TcpListener>::bind(([127,0,0,1], 7070))?;
    //
    // });
    Ok(())
}
