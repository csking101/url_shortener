#!/bin/bash
curl -X "POST"\
    -H "Content-Type: application/json"\
    -d '{"long_url":"https://www.google.com/search?channel=fs&client=ubuntu-sn&q=woof+woof","rate_limit":900000000000,"permission_rule":null}'\
    http://127.0.0.1:7878/create_short_url