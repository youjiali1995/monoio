/// A example to show how to use UnixStream.
use local_sync::oneshot::channel;
use monoio::{
    io::{AsyncReadRent, AsyncWriteRentExt},
    net::{UnixListener, UnixStream},
};

const ADDRESS: &str = "/tmp/monoio-unix-test.sock";

#[monoio::main]
async fn main() {
    let (mut tx, rx) = channel::<()>();

    monoio::spawn(async move {
        tx.closed().await;
        let client = UnixStream::connect(ADDRESS).await.unwrap();
        let buf = "hello";
        let (ret, buf) = client.write_all(buf).await;
        ret.unwrap();
        println!("write {} bytes: {:?}", buf.len(), buf);
    });

    std::fs::remove_file(ADDRESS).ok();
    let listener = UnixListener::bind(ADDRESS).unwrap();
    println!("listening on {:?}", ADDRESS);
    drop(rx);
    let (conn, addr) = listener.accept().await.unwrap();
    println!("accepted a new connection from {:?}", addr);
    let buf = Vec::with_capacity(1024);
    let (ret, buf) = conn.read(buf).await;
    ret.unwrap();
    println!("read {} bytes: {:?}", buf.len(), buf);

    // clear the socket file
    drop(listener);
    std::fs::remove_file(ADDRESS).ok();
}
