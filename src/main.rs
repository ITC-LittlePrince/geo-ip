use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long = "no-proxy", default_value = "false")]
    no_proxy: bool,

    #[arg(long = "proxy-host", default_value = "127.0.0.1")]
    proxy_host: String,

    /// Number of times to greet
    #[arg(long = "proxy-port", default_value_t = 65056)]
    proxy_port: u16,
}

fn main() {
    let http_ip_0 = "https://api.myip.com";
    let http_geo_0 = "http://ip-api.com/json/";

    let args = Args::parse();
    // println!(
    //     "current proxy : socks5h://{}:{}",
    //     args.proxy_host, args.proxy_port
    // );

    let rt0 = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .unwrap();

    rt0.block_on(async move {
        let client: wreq::Client = match args.no_proxy {
            true => {
                println!("no current proxy");
                wreq::Client::new()
            }
            false => {
                println!(
                    "current proxy : socks5h://{}:{}",
                    args.proxy_host, args.proxy_port
                );
                wreq::Client::builder()
                    .proxy(
                        wreq::Proxy::all(format!(
                            "socks5h://{}:{}",
                            args.proxy_host, args.proxy_port
                        ))
                        .unwrap(),
                    )
                    .build()
                    .unwrap()
            }
        };

        let temp_req0 = match client
            .get(http_ip_0)
            .header_append(
                wreq::header::HeaderName::from_static("content-type"),
                wreq::header::HeaderValue::from_static("application/json"),
            )
            .send()
            .await
            .ok()
        {
            Some(x) => x,
            None => {
                return Err("Error code 0x0001".to_string());
            }
        };

        #[derive(Clone, serde::Serialize, serde::Deserialize)]
        struct ResponseIP {
            ip: String,
        }
        impl std::fmt::Debug for ResponseIP {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "IP\t\t\t\t{}", self.ip)?;
                Ok(())
            }
        }

        // let obj_req0: serde_json::Value = temp_req0.json().await.unwrap();
        let obj_req0: ResponseIP = temp_req0.json().await.unwrap();
        // println!("{:?}", obj_req0);

        let temp_req1 = match client
            .get(format!("{}/{}", http_geo_0, obj_req0.ip))
            .header_append(
                wreq::header::HeaderName::from_static("content-type"),
                wreq::header::HeaderValue::from_static("application/json"),
            )
            .send()
            .await
            .ok()
        {
            Some(x) => x,
            None => {
                return Err("Error code 0x0002".to_string());
            }
        };

        #[derive(Clone, serde::Serialize, serde::Deserialize)]
        struct ResponseGeo {
        	r#as: String,
            city: String,
            country: String,
            countryCode: String,
            isp: String,
            lat: f32,
            lon: f32,
            org: String,
            query: String,
            region: String,
            regionName: String,
            status: String,
            timezone: String,
            zip: String,
        }
        impl std::fmt::Debug for ResponseGeo {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            	writeln!(f, "Geo location:")?;
            	writeln!(f, "\tIP\t\t:\t{}", self.query)?;
             	writeln!(f, "\tAS\t\t:\t{}", self.r#as)?;
             	writeln!(f, "\tISP\t\t:\t{}", self.isp)?;
             	writeln!(f, "\tCountry code\t:\t{}", self.countryCode)?;
             	writeln!(f, "\tCountry\t\t:\t{}", self.country)?;
             	writeln!(f, "\tCity\t\t:\t{}", self.city)?;
                writeln!(f, "\tRegion name\t:\t{}", self.regionName)?;
                writeln!(f, "\tRegion\t\t:\t{}", self.region)?;
                writeln!(f, "\tTimezone\t:\t{}", self.timezone)?;
               	writeln!(f, "\tLatitude\t:\t{}", self.lat)?;
               	writeln!(f, "\tLongitude\t:\t{}", self.lon)?;
               	writeln!(f, "\tZip code\t:\t{}", self.zip)?;
                Ok(())
            }
        }

        // let obj_req1: serde_json::Value = temp_req1.json().await.unwrap();
        let obj_req1: ResponseGeo = temp_req1.json().await.unwrap();
        println!("{:?}", obj_req1);

        Ok(())
    });
}
