# Buffer

The implementatie of a buffer for multiple readers/writers.

To realise a buffer capable to handle multiple readers/writers which is tread save we use an asynchronous FIFO-buffer (First-In, First-Out) based on  Bounded Channel.

In rust the best way for dealing with data between multiple threads is: "Do not communicate by sharing memory; instead, share memory by communicating."  We use the crossbeam-channel library. It is designed for Multi-Producer Multi-Consumer (MPMC) scenario's.