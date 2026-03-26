# geo-ip

## Example programm output
```
╭─[arch-kingston-0] as little-prince in ~
╰──➤ geo-ip --proxy-host "15.0.0.100"
current proxy : socks5h://15.0.0.100:65056
Geo location:
        IP              :       46.234.47.108
        AS              :       AS34288 Kantonsschule Zug
        ISP             :       Kantonsschule Zug
        Country code    :       CH
        Country         :       Switzerland
        City            :       Zug
        Region name     :       Zug
        Region          :       ZG
        Timezone        :       Europe/Zurich
        Latitude        :       47.1685
        Longitude       :       8.5035
        Zip code        :       6300
```

### Arguments
Address 15.0.0.100 - local/global address your SOCKS5 server for test connection

### MANUAL
```
╭─[arch-kingston-0] as little-prince in ~
╰──➤ geo-ip --help                   
Usage: geo-ip [OPTIONS]

Options:
  -n, --no-proxy                 
      --proxy-host <PROXY_HOST>  [default: 127.0.0.1]
      --proxy-port <PROXY_PORT>  Number of times to greet [default: 65056]
  -h, --help                     Print help
  -V, --version                  Print version
```
