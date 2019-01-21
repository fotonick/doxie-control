Doxie-control
=============

This project provides the `doxiectl` command-line utility to interact with the [Doxie Go Wi-Fi](http://www.getdoxie.com/product/doxie-go-plus-and-wifi/) portable document scanner. It performs automatic discovery of the Doxie Go on your wifi network, and can list, download, and delete scans, but does not replace the OCR (optical character recognition) of the official GUI. It was developed referencing the [official API docs](http://help.getdoxie.com/content/doxiego/05-advanced/03-wifi/04-api/Doxie-API-Developer-Guide.pdf).

Quickstart
----------

1. You can install Cargo via [rustup](https://rustup.rs/), then install the nightly release with `rustup install nightly`.
2. Clone this repo and build with `cargo +nightly build --release`.
3. Put the Doxie Go into wifi mode. Wait for the blue LED to stop flashing and stay steady for several seconds.
4. Then:

```
$ target/release/doxiectl list
ScanEntry {
    name: "/DOXIE/JPEG/IMG_0001.JPG",
    size: 1371804
    modified: "2010-05-01 00:19:50"
}
ScanEntry {
    name: "/DOXIE/JPEG/IMG_0002.JPG",
    size: 794472
    modified: "2010-05-01 00:20:12"
}
$ target/release/doxiectl download_all
/DOXIE/JPEG/IMG_0001.JPG → IMG_0001.JPG
/DOXIE/JPEG/IMG_0002.JPG → IMG_0002.JPG
$ target/release/doxiectl delete_all
/DOXIE/JPEG/IMG_0001.JPG → 🗑️
/DOXIE/JPEG/IMG_0002.JPG → 🗑️
```

And that's just about it.

Elaboration
-----------

* Doxie-control requires nightly Rust because the `ssdp` crate requires it. The SSDP protocol is simple enough and my needs are narrow enough that it could be worth hand-rolling an alternative to avoid this dependency. Open an Issue if this is something you want.
* You can also download and delete individual scans with `doxiectl download /DOXIE/JPEG/IMG_0002.JPG` and `doxiectl delete /DOXIE/JPEG/IMG_0002.JPG`, respectively. These have the shorter aliases `dl` and `rm`, respectively.

License
-------

This project is licensed under the MIT license.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in doxie-control by you, shall be licensed as MIT, without any additional terms or conditions.
