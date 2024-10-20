# Quick Image Resizer
Resize your images quickly with this Rust program.  
I also use this to make different logo sizes for my web applications.

### Usage
```bash
cargo build --release 
sudo cp ./target/release/quick-image-resizer /usr/bin/quick-image-resizer

quick-image-resizer resize ~/quick-image-resizer/examples/image.jpg ~/quick-image-resizer/examples/output_image.jpg 100x100
```
