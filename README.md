# **VISA**
Ergonomic Rust bindings to a VISA (**V**irtual **I**nstrument **S**oftware **A**rchitecture) library.

## **Requirements**
Being a binding crate it's required to have installed
an implementation to a VISA library, for example
[NI-VISA](https://www.ni.com/en/support/downloads/drivers/download.ni-visa.html#558610).  
A C compiler is also needed (clang, for example). On Windows it can be easily obtained via Visual Studio.

## **Usage**
```TOML
[dependencies]
visa = {version = "*"}
```

```Rust
let resource_manager = ResourceManager::new().unwrap();

let mut instrument = resource_manager
    .open_with_identification(
        "manufacturer",
        "model",
        "serial_number",
        AccessMode::NO_LOCK,
        Scope::Local,
        Duration::from_secs(0),
    )
    .unwrap();

let identification = instrument.query_identification().unwrap();

println!("{:?}", identification);
```

## **Cross Compilation**
Sadly cross compilation is not yet possible due to the need of the library to link to a valid VISA library path at compile time. This means that if you're compiing
on Linux for Windows, the library will attempt to find a VISA library at the
path it would expect it to be on Windows.

## **License**
```
The MIT License (MIT)

Copyright (c) 2025 Gianluca Gaiardi

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```