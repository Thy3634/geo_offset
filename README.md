# geo_offset

GCJ-02/BD09/BD09MC 坐标偏移转换, implemented in Rust

坐标类型参见 [georust](https://georust.org/) 

> **注意**：
>
>（禁止）未经批准，在测绘活动中擅自采用国际坐标系统 —— 中华人民共和国测绘法，40 (1)
>
> 导航电子地图在公开出版、销售、传播、展示和使用前，必须进行空间位置技术处理。—— GB 20263―2006《导航电子地图安全处理技术基本要求》，4.1

## Install

```bash
cargo add geo_offset
```

## Usage

```rust
use geo_offset::{gcj02_to_wgs84, wgs84_to_gcj02, bd09_to_wgs84, wgs84_to_bd09, bd09mc_to_wgs84, wgs84_to_bd09mc};

fn main() {
    let coord = (116.404, 39.915).into();
    let gcj02 = wgs84_to_gcj02(coord);
    println!("gcj02: {:?}", gcj02);
    println!("wgs84: {:?}", gcj02_to_wgs84(gcj02));
    let bd09 = wgs84_to_bd09(coord);
    println!("bd09: {:?}", bd09);
    println!("wgs84: {:?}", bd09_to_wgs84(bd09));
    let bd09mc = wgs84_to_bd09mc(coord);
    println!("bd09mc: {:?}", bd09mc);
    println!("wgs84: {:?}", bd09mc_to_wgs84(bd09mc));
}
```