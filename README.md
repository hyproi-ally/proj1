After reading this article “Seamless Deductive Inference via Macros”, I gained some insights. Firstly, Ascent uses lattice structures to control how inference results are merged through lattice operations such as join and meet, which correspond to least upper bounds and greatest lower bounds. This allows inference results to move through multiple layers of abstract states, from more precise values toward more general elements such as Top.

By comparing Datafrog and Ascent with five Benchmarks, in the analysis, Datafrog is faster than Ascent except for clap-rs. However, Datafrog requires relation tuples to be in total order. Ascent does not impose this restriction. It provides higher flexibility. However, this flexibility may sacrifice some performance.

I successfully invoked Ascent in a Rust project, wrote a Datalog-style logic program using Ascent with lattice structures, and ran it without issues, verifying that the program correctly executed logical inference rules and produced the expected output. I would like to find another Benchmark to test the performance between Datafrog and Ascent.

Suggestion 1
In comparison experiment of Datafrog and Ascent, I think it still needs a new benchmark that could use larger and more complex facts to verify the difference between Dataforg and Ascent. I will try to implement it 

Suggestion 2
Another possible improvement is to use benchmarks with more varied LOC sizes. In comparison experiment of Datafrog and Ascent, it covers programs from 170 to 2100 LOC. However, it does not include very small programs below 100 LOC or very large programs above 5000 LOC. Small programs could show fixed overhead, and large programs could test scalability. This could show whether the performance gap between Datafrog and Ascent changes with program size.

Suggestion 3: Test graphs with different densities
In comparison experiment of Flix time and Ascent, another possible improvement is to test graphs with different densities. This experiment includes sparse graphs and complete graphs, but it does not systematically test medium-density graphs. Adding sparse, medium, and dense graphs could show how graph density affects the performance difference between Flix and Ascent. Try larger graphs with more edges.

Suggestion 4: Report memory usage
Another useful improvement is to report memory usage. This experience only reports running time. However, Flix fails on the HE PhysTH benchmark with an out-of-memory error. Reporting memory usage could better explain why Flix fails and why Ascent can finish the benchmark.

Next, I will continue reading the papers that Yihao gave me and attempt to reproduce the experiment through coding.
