use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()>{
    //http://example.org/index.html
    let input: Vec<String> = std::env::args().collect();
    let url = &input[1];

    //first arg is path to binary
    assert!(url.starts_with("http://"), "Only treats http");

    let s = url.split('/');
    let url_vec: Vec<&str> = s.collect();

    let host = url_vec[0].to_string();
    let path = "/".to_owned() + url_vec[2];
    // let port = url_vec[3];

    let mut res = reqwest::blocking::get(format!("{}{}", host, path))?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    //split headers into a map

    //assert that data is not being send to us in a weird way
    //look for:
    // assert "transfer-encoding" not in headers
    // assert "content-encoding" not in headers

    //print the web page but not the tags
    // in_angle = False
    // for c in body:
    //     if c == "<":
    //         in_angle = True
    //     elif c == ">":
    //         in_angle = False
    //     elif not in_angle:
    //         print(c, end="")

    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;

    Ok(())
}
