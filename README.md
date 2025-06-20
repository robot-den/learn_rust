## A project with Rust basics


### Development
How to build and run binaries? Call from specific folder:  
`rustc x.rs -o ../binaries/x && ../binaries/x`

#### Docker
1) Build a playground image:  
`docker build -t rust-playground .`  
2) Run a playground container (jump inside)  
`docker run -it --rm -v $(pwd):/usr/src/app rust-playground`