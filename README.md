
To deploy on Android, use:
Trusted Web Activity


look into:
grpc-web-client

http2 envoy config
https://medium.com/@denis.zhbankov/grpc-web-via-http2-b05c8c8f9e6


setup DNSLink
_dnslink.manger.aadi.my TXT 60 dnslink=/ipfs/Qmc2o4ZNtbinEmRF9UGouBYTuiHbtCSShMFRbBY5ZiZDmU
http://manger.aadi.my.ipns.localhost:8080/

Certificate is saved at: /etc/letsencrypt/live/ipfs-pi.aadi.my/fullchain.pem
Key is saved at:         /etc/letsencrypt/live/ipfs-pi.aadi.my/privkey.pem


dig manger.aadi.my  +nostats +nocomments +nocmd
