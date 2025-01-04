Implemented ```ping``` in Rust using ```pnet```.
### Build
#### Linux/MacOS
Nothing special for Linux/MacOS, just run ```cargo build```.  

#### Windows
Before running ```cargo build``` download Packet.lib from [WinPcap Developer Pack](https://www.winpcap.org/devel.htm) and place it in your repository like this:
```
pingrc
 ├── Cargo.lock
 ├── Cargo.toml
 ├── Packet.lib
 ├── README.md
 ├── src
 └── target
 ``` 
Your Rust should be configured to use MSVC toolchain which is explained [here](https://rust-lang.github.io/rustup/installation/windows.html).
More information on this requirement can be found on the github repository for [libpnet](https://github.com/libpnet/libpnet/tree/main).


### Usage
```
./pingrc target_ip [num_pings]
```
Sample output
```
PS D:\Rust\pingrc\target\debug> ./pingrc.exe 103.243.32.90 4
Pinging 103.243.32.90 with 32 bytes of data
Reply from: 103.243.32.90 in 162ms
Reply from: 103.243.32.90 in 161ms
Reply from: 103.243.32.90 in 161ms
Reply from: 103.243.32.90 in 162ms
4 out of 4 packets received successfully.
Approximate round trip times in milli-seconds:
Max: 162ms, Min: 161ms, Average: 161.5ms
```
