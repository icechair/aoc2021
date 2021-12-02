#!/bin/bash
day=${1:-1}
# browser inspect the input "get input link"
#
curl "https://adventofcode.com/2021/day/${day}/input" \
  -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:94.0) Gecko/20100101 Firefox/94.0' \
  -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8' \
  -H 'Accept-Language: en-US,en;q=0.5' \
  --compressed \
  -H "Referer: https://adventofcode.com/2021/${day}/1" \
  -H 'Connection: keep-alive' \
  -H 'Cookie: <your cookie data here>' \ # careful here with the checkin
  -H 'Upgrade-Insecure-Requests: 1' \
  -H 'Sec-Fetch-Dest: document' \
  -H 'Sec-Fetch-Mode: navigate' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'Cache-Control: max-age=0' \
  -H 'TE: trailers'
