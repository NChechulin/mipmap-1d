# MipMap-1d

This Rust crate provides a small data structure that constructs and holds your 1-dimensional mipmaps.
MipMaps are just downsampled versions of your vector.
Each level is only half the size of a previous level.
Therefore, the entire data structure takes approximately 2x the size of initial data in memory.

The downsampling is currently performed by averaging the pairs of elements (see example below).
New ways might be introduced in later versions.

The crate currently works with all major numeric types (thanks to `num-traits` crate).

This can be used, for example, in plots.
If you have a large number of points (lets say, 10M), you clearly can not draw them all on the screen at once.
Also, drawing so many requires a lot of computational resources.

What you can do is to use 1d MipMap: you make several downsamples, and choose which one you want to display depending on a zoom level.
The more you zoom in, the 'higher resolution' data you display.

## Example

```rust
use mipmap_1d::MipMap1D;

let data = vec![2, 4, 6, 8, 9];
let mipmap = MipMap1D::new(data);
assert_eq!(mipmap.num_levels(), 4);
assert_eq!(*mipmap.get_level(0).unwrap(), [2, 4, 6, 8, 9]);
assert_eq!(*mipmap.get_level(1).unwrap(), [3, 7, 9]);
assert_eq!(*mipmap.get_level(2).unwrap(), [5, 9]);
assert_eq!(*mipmap.get_level(3).unwrap(), [7]);
assert_eq!(mipmap.get_level(4), None);
```

## Contributing

Currently, the only idea for improvement is to introduce new averaging methods that are used in signal processing.
Check [issue #1](https://github.com/NChechulin/mipmap-1d/issues/1) for details.
