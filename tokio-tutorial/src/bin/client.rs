#[tokio::main]
async fn main() {
    use bytes::Bytes;
    use tokio::sync::oneshot;

    type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

    #[derive(Debug)]
    enum Command {
        Get {
            key: String,
            resp: Responder<Option<Bytes>>,
        },
        Set {
            key: String,
            val: Bytes,
            resp: Responder<()>,
        },
    }

    // The tokio tutorial really does suck for newbs
    use tokio::sync::mpsc; // used for abstracted Get Set msgs
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel(); // oneshot used for responses
        let getcmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };
        tx.send(getcmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let setcmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };
        tx2.send(setcmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    //// Leftover code from b4 we switched to the 'Command' enum
    //tokio::spawn(async move {
    //    tx2.send("sending from second sender handle").await.unwrap();
    //});
    //
    //while let Some(msg) = rx.recv().await {
    //    println!("Got = {}", msg);
    //}

    use mini_redis::client;
    let mgr = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    mgr.await.unwrap();
}
