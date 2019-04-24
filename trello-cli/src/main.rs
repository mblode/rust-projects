use std::io::Read;

fn main() {
    // let body = reqwest::get("https://www.rust-lang.org");

    // println!("body = {:?}", body);

    // let client = reqwest::Client::new();
    // let res = client.post("http://httpbin.org/post")
    //     .body("the exact body that is sent")
    //     .send();

    // println!("response = {:?}", res);

    let mut response =
        reqwest::get("https://httpbin.org/status/418").expect("Failed to send request");
    println!("{}", response.status());

    for header in response.headers().iter() {
        println!("{}", header);
    }

    let mut buf = String::new();

    response
        .read_to_string(&mut buf)
        .expect("Failed to read response");

    println!("{}", buf);
}
