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
