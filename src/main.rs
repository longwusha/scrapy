extern crate reqwest;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("POST \"hello world\" to http://httpbin.org/post");

    let client = reqwest::Client::new();
    let mut res = client
        .post("http://httpbin.org/post")
        .body("hello world")
        .send()?;

    println!("Status :{}", res.status());
    println!("Headers:{:?}\n", res.headers());

    // copy the response body directly to stdout
    std::io::copy(&mut res, &mut std::io::stdout())?;

    println!("\n\nDone.");
    Ok(())
}
