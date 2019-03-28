# Topsort
Get the top n results from a csv / log file a lot faster than doing sort -k 3 | tail


## How faster?
About 10x faster compared to sort without parallel or buffer size parameters

```

$ time cat /tmp/sample.log| sort -k 8 -t '|' | tail -100 > /dev/null

real	1m19.623s
user	1m13.641s
sys	0m8.369s

$ time cat /tmp/sample.log| ./target/release/topsort -k 8 -n 100 -t '|' > /dev/null

real	0m6.488s
user	0m6.037s
sys	0m1.531s

$ wc -c /tmp/sample.log
2264483634 /tmp/sample.log

$ wc -l /tmp/sample.log
17223119 /tmp/sample.log

```


## How?
We just care about the top n results. So we insert them into a btreemap until we have 2x the desired ammount and split the tree in half.  
As another optimization, we save what the nth top result is, so we don't insert anything smaller than that

## What's your use case?
Parsing through multi GB log files (like access logs) looking for anomalies can be anoying if normal operations take over a minute. 
This is an attempt to make one of those operations faster, eg: Give me the n requests with the highest response time.