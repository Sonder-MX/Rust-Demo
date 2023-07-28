#![allow(dead_code)]
#[derive(Debug)]
pub struct Settings<'a> {
    pub ip: &'a str,
    pub port: &'a str,
    // other ...
}

impl<'a> Settings<'a> {
    pub fn new() -> Self {
        Settings {
            ip: "127.0.0.1",
            port: "9090",
        }
    }

    pub fn read_args(&mut self, args: &'a Vec<String>) {
        let mut args = args.iter();
        args.next(); // 跳过第一个参数
        while let Some(arg) = args.next() {
            let mut arg = arg.split("=");
            let key = arg.next().unwrap();
            let value = arg.next().unwrap();
            match key {
                "ip" => self.ip = value,
                "port" => self.port = value,
                _ => panic!("无效的参数: {}", key),
            }
        }
    }

    pub fn ip_port(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
