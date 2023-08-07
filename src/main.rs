use core::time;
//use hex_lit::hex;
use std::{convert::TryFrom, env};
use teltonika::avl_data_packet::AVLDataPacket;
use tokio::net::UdpSocket;
use serde_json::to_string;
use tokio::spawn;
use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::task;

#[tokio::main]
async fn main() {
    // MQTT client setup
    let mqqt_user: Option<String> = env::var("MQQT_USER").ok();
    let mqqt_password: Option<String> = env::var("MQQT_PASSWORD").ok();
    let mqqt_host = env::var("MQQT_HOST").expect("$MQQT_HOST is not set");
    let mqqt_port: u16 = env::var("MQQT_PORT").expect("$MQQT_PORT is not set").parse().unwrap();
    let mut mqttoptions = MqttOptions::new("teltonika-parser", mqqt_host, mqqt_port);
    mqttoptions.set_keep_alive(time::Duration::from_secs(5));

    if let (Some(user), Some(password)) = (&mqqt_user, &mqqt_password) {
        mqttoptions.set_credentials(user, password);
    } else {
        println!("Either MQQT_USER or MQQT_PASSWORD or both are not set");
    }

    let (mqtt_client, mut mqtt_eventloop) = AsyncClient::new(mqttoptions, 10);

    // Spawn a task for mqtt eventloop
    task::spawn(async move {
        while let Ok(notification) = mqtt_eventloop.poll().await {
            println!("MQTT Eventloop notification: {:?}", notification);
        }
    });

    // UDP server setup
    let socket = UdpSocket::bind("127.0.0.1:12345").await.unwrap();

    loop {
        let mut buf = vec![0; 1024];
        let size = socket.recv_from(&mut buf).await.unwrap().0;

        let mqtt_client = mqtt_client.clone();
        let handle = spawn(async move {
            let buf = &buf[..size];
            if let Ok(deku_test) = AVLDataPacket::try_from(buf) {
                if let Ok(serialized) = to_string(&deku_test) {
                    // Publish to MQTT topic
                    if let Err(e) = mqtt_client.publish(format!("teltonika/{}/{}", deku_test.codec, deku_test.imei), QoS::AtMostOnce, false, serialized).await {
                        eprintln!("Failed to publish message: {}", e);
                    } else {
                        println!("Received and published packet id: {}", deku_test.avl_packet_id);
                    }
                } else {
                    eprintln!("Failed to serialize data to JSON");
                }
            } else {
                eprintln!("Failed to parse packet");
            }
        });

        if let Err(e) = handle.await {
            eprintln!("Failed to handle packet: {}", e);
        }
    }
}

// fn main() {
//     let test_data = hex!("003DCAFE0105000F33353230393330383634303336353508010000016B4F815B30010000000000000000000000000000000103021503010101425DBC000001").to_vec();

//     let deku_test = AVLDataPacket::try_from(test_data.as_ref()).unwrap();

//     let serialized = serde_json::to_string_pretty(&deku_test).unwrap();

//     println!("{}", serialized);

//     let test_data2 = hex!("01e4cafe0128000f333532303934303839333937343634080400000163c803eb02010a2524c01d4a377d00d3012f130032421b0a4503f00150051503ef01510052005900be00c1000ab50008b60006426fd8cd3d1ece605a5400005500007300005a0000c0000007c70000000df1000059d910002d33c65300000000570000000064000000f7bf000000000000000163c803e6e8010a2530781d4a316f00d40131130031421b0a4503f00150051503ef01510052005900be00c1000ab50008b60005426fcbcd3d1ece605a5400005500007300005a0000c0000007c70000000ef1000059d910002d33b95300000000570000000064000000f7bf000000000000000163c803df18010a2536961d4a2e4f00d50134130033421b0a4503f00150051503ef01510052005900be00c1000ab50008b6000542702bcd3d1ece605a5400005500007300005a0000c0000007c70000001ef1000059d910002d33aa5300000000570000000064000000f7bf000000000000000163c8039ce2010a25d8d41d49f42c00dc0123120058421b0a4503f00150051503ef01510052005900be00c1000ab50009b60005427031cd79d8ce605a5400005500007300005a0000c0000007c700000019f1000059d910002d32505300000000570000000064000000f7bf000000000004").to_vec();

//     let deku_test2 = AVLDataPacket::try_from(test_data2.as_ref()).unwrap();

//     let serialized2 = serde_json::to_string_pretty(&deku_test2).unwrap();
    
//     println!("{}", serialized2);
// }
