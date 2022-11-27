use reqwest::blocking;
use regex::Regex;

#[derive(Clone)]
pub struct Proxy {
    pub ip: String,
    pub port: i32,
}

pub fn get_proxies() -> Vec<Proxy> {
    let mut proxies = vec![];

    let wwproxydownload = reqwest::blocking
        ::get("https://www.proxy-list.download/HTTPS")
        .unwrap()
        .text()
        .unwrap()
        .replace("\n", "")
        .replace(" ", "");

    let rgx = Regex::new(r"\d+\.\d+\.\d+\.\d+</td>\s*<td>\d+").unwrap();
    let rex = Regex::new(r"</td>\s*<td>").unwrap();
    for exp in rgx.find_iter(wwproxydownload.as_str()) {
        let exp = exp.as_str();
        let sp: Vec<&str> = rex.split(exp).collect();
        let ip = sp[0].to_string();
        let port = sp[1].parse::<i32>().unwrap();

        proxies.push(Proxy {
            ip: ip,
            port: port,
        });
    }

    proxies
}