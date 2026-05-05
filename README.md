# mail-server
## This is the configuration needed more so on the environment
MAIL_SERVER: host:port ex. 192.168.12.123:2525 the port should be exactly as I have demonstrated because isp will block the smtp port i.e 25
Remember to add the allowed origins to avoid cors origin

## Start the mail server
cargo run

## Start the webserver 
npm run dev

on the pages part start the live server or just click on the page but note the port it is running to put it in env to prevent cors blocking