# Eta-Graph

## What is it?
Eta-Graph is a low-overhead graph library with strong emphasis on performance and memory usage.

## Why use it?
* Low overhead
* Performance
* Ease of use
* Extensibility

## How to use it?
Documentation will be added soon. The library is still in development.

## Testing
There is extensive testing for the library. The tests are located in the `tests` folder.

## Benchmarks
Benchmarks are located in the `benches` folder.
Pet-graph was used a comparison library for the benchmarks. Thus why it is in the dev-dependencies.

I will later add a benchmark table. 
You can run them yourself to compare the performance on your machine.

On machine tested:

*Processor	AMD Ryzen 9 7900X 12-Core Processor  4.70 GHz
Installed RAM	32.0 GB (31.1 GB usable)*

Pathfinding was up to 2x faster than petgraph.

BFS was up to 40% faster than petgraph.

DFS was up to 2x faster than petgraph.


I am yet to make max-flow benchmarks.

I will document the exact numbers and do more detailed benchmarks later.

## Config
The library is configured with a build.rs script.
You can change the config by setting the environment variable `MSize` to the size you want.
The default is `u64`.

This is used to determine the size of the edges. Smaller = better cache performance.

Possible values are:
* `u16`
* `u32`
* `u64`


I recommend making .cargo/config.toml in the project root with the following content:\

```toml
[env]
MSize = "u64" # Change this to the size you want
```

This will make the library use u64 for edge sizes.

## License
Custom MIT-Based License

Copyright (c) 2024 Helecz Erik

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

1. The above copyright notice and this permission notice shall be included in all
   copies or substantial portions of the Software.

2. The Software may be used for any purpose, including commercial applications.

3. **Users are prohibited from forking or creating copies of the Software's repository
   or codebase for the purpose of creating a separate distribution.**

4. Any modifications, improvements, or additions to the Software must be submitted
   as pull requests to the original repository. The copyright holder reserves the
   right to approve, modify, or reject these pull requests.

5. This license explicitly disallows the redistribution of the Software or any
   modified versions under any other license terms.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.