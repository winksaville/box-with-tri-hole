# box-with-tri-hole

A 40x40x5mm box with a triangular hole circumscribed by a circle with a 7.5mm radius:

Implemented using [csgrs](https://git@github.com:timschmidt/csgrs), CAD kernel
written in Rust.

The src/main.rs is based on [csgrs-holed-cuboid](https://git@github.com:winksaville/csgrs-holed-cuboid)

## Usage

Here I'm running after having done a clean and it creates `box-with-tri-hole.stl`:
```
wink@3900x 25-12-05T19:34:19.372Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)
$ cargo clean
     Removed 3857 files, 1.9GiB total
wink@3900x 25-12-05T19:34:26.550Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)
$ cargo build
    Updating git repository `https://github.com/timschmidt/csgrs.git`
     Locking 1 package to latest Rust 1.91.1 compatible version
      Adding csgrs v0.20.1 (https://github.com/timschmidt/csgrs.git?branch=main#e48439d9)
   Compiling proc-macro2 v1.0.103
   Compiling unicode-ident v1.0.22
..

   Compiling rapier3d-f64 v0.24.0
   Compiling csgrs v0.20.1 (https://github.com/timschmidt/csgrs.git?branch=main#e48439d9)
   Compiling box-with-tri-hole v0.1.0 (/home/wink/data/prgs/3dprinting/box-with-tri-hole/csgrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 23.97s
wink@3900x 25-12-05T19:34:53.524Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)
```

Using meshlab to inspect the object using menus
[filters -> Quality Measures and Computations -> Compute Topological Measures]
and the result is perfect!
```
wink@3900x 25-12-05T19:35:28.381Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)
$ meshlab box-with-tri-hole.stl
Using OpenGL 4.6
LOG: 0 Opened mesh box-with-tri-hole.stl in 47 msec
LOG: 0 All files opened in 65 msec
"OpenSSL 3.6.0 1 Oct 2025"
"OpenSSL 3.6.0 1 Oct 2025"
Online version: 2025.07
LOG: 2 V:     20 E:     60 F:    40
LOG: 2 Unreferenced Vertices 0
LOG: 2 Boundary Edges 0
LOG: 2 Mesh is composed by 1 connected component(s)

LOG: 2 Mesh is two-manifold 
LOG: 2 Mesh has 0 holes
LOG: 2 Genus is 1
LOG: 0 Applied filter: Compute Topological Measures in 299 msec
^C
wink@3900x 25-12-05T19:38:48.855Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)

```

Here is a png:
![box-with-tri-hole.stl.png](./box-with-tri-hole.stl.png)

## Binary

The binary is 548,536 bytes:
```
wink@3900x 25-12-05T19:42:00.647Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)
$ ls -l target/release/box-with-tri-hole
-rwxr-xr-x 2 wink users 548536 Dec  5 11:42 target/release/box-with-tri-hole
wink@3900x 25-12-05T19:42:19.628Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)
```

Bloat info on the binary crates:
```
wink@3900x 25-12-05T19:43:19.037Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)
$ cargo  bloat
   Compiling proc-macro2 v1.0.103
   Compiling unicode-ident v1.0.22
   Compiling quote v1.0.42
   Compiling autocfg v1.5.0
..
   Compiling parry3d-f64 v0.19.0
   Compiling rapier3d-f64 v0.24.0
   Compiling csgrs v0.20.1 (https://github.com/timschmidt/csgrs.git?branch=main#e48439d9)
   Compiling box-with-tri-hole v0.1.0 (/home/wink/data/prgs/3dprinting/box-with-tri-hole/csgrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 24.51s
    Analyzing target/debug/box-with-tri-hole

File  .text     Size          Crate Name
0.1%   5.0%  61.9KiB          csgrs nalgebra::linalg::inverse::do_inverse4
0.0%   2.2%  27.9KiB         robust robust::incircleadapt
0.0%   1.4%  17.8KiB matrixmultiply matrixmultiply::dgemm_kernel::kernel_target_avx
0.0%   1.4%  17.0KiB matrixmultiply matrixmultiply::dgemm_kernel::kernel_target_fma
0.0%   1.3%  15.5KiB            std std::backtrace_rs::symbolize::gimli::Cache::with_global
0.0%   1.1%  13.9KiB matrixmultiply matrixmultiply::sgemm_kernel::kernel_target_avx
0.0%   1.1%  13.5KiB         robust robust::orient3dadapt
0.0%   1.1%  13.1KiB matrixmultiply matrixmultiply::sgemm_kernel::kernel_target_fma
0.0%   1.0%  12.6KiB            std std::backtrace_rs::symbolize::gimli::Context::new
0.0%   1.0%  11.9KiB          csgrs nalgebra::linalg::inverse::<impl nalgebra::base::matrix::Matrix<T,D,D,S>>::try_inverse_mut
0.0%   0.8%   9.8KiB    parry3d_f64 nalgebra::base::blas::<impl nalgebra::base::matrix::Matrix<T,R,C,S>>::dotc
0.0%   0.8%   9.8KiB    parry3d_f64 nalgebra::base::blas::<impl nalgebra::base::matrix::Matrix<T,R,C,S>>::dot
0.0%   0.8%   9.5KiB            std gimli::read::dwarf::Unit<R>::new
0.0%   0.7%   9.1KiB            std core::cell::once::OnceCell<T>::try_init
0.0%   0.6%   7.7KiB       nalgebra nalgebra::base::ops::<impl core::ops::arith::Mul<&nalgebra::base::matrix::Matrix<T,R2,C2,SB>> for &nalgebra::b...
0.0%   0.6%   7.7KiB       nalgebra nalgebra::base::ops::<impl core::ops::arith::Mul<&nalgebra::base::matrix::Matrix<T,R2,C2,SB>> for &nalgebra::b...
0.0%   0.5%   6.8KiB          csgrs csgrs::mesh::shapes::<impl csgrs::mesh::Mesh<S>>::cuboid
0.0%   0.5%   6.5KiB    miniz_oxide miniz_oxide::inflate::core::decompress
0.0%   0.4%   5.3KiB          csgrs csgrs::mesh::shapes::<impl csgrs::mesh::Mesh<S>>::frustum_ptp
0.0%   0.4%   5.1KiB            std gimli::read::unit::parse_attribute
0.8%  75.0% 931.8KiB                And 4074 smaller methods. Use -n N to show more.
1.1% 100.0%   1.2MiB                .text section size, the file size is 113.4MiB
wink@3900x 25-12-05T19:43:47.585Z:~/data/prgs/3dprinting/box-with-tri-hole/csgrs (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
