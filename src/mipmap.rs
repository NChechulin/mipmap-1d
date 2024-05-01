use num_traits::{FromPrimitive, Num, ToPrimitive};

/// Creates several downsampled versions of given vector.
/// This data structure takes 2x space of original data.
/// Example:
/// ```rust
/// use mipmap_1d::MipMap1D;
/// 
/// let data = vec![2, 4, 6, 8, 9];
/// let mipmap = MipMap1D::new(data);
/// assert_eq!(mipmap.num_levels(), 4);
/// assert_eq!(*mipmap.get_level(0).unwrap(), [2, 4, 6, 8, 9]);
/// assert_eq!(*mipmap.get_level(1).unwrap(), [3, 7, 9]);
/// assert_eq!(*mipmap.get_level(2).unwrap(), [5, 9]);
/// assert_eq!(*mipmap.get_level(3).unwrap(), [7]);
/// assert_eq!(mipmap.get_level(4), None);
/// ```
pub struct MipMap1D<T: Num + ToPrimitive + FromPrimitive> {
    data: Vec<Vec<T>>,
}

impl<T: Num + ToPrimitive + FromPrimitive + Copy> MipMap1D<T> {
    pub fn new(source: Vec<T>) -> Self {
        let mut data = vec![source.clone()];
        let mut current = source;

        while current.len() > 1 {
            let mipmap = Self::downsample(&current);
            current.clone_from(&mipmap);
            data.push(mipmap);
        }

        Self { data }
    }

    /// Returns the total number of downsampled levels.
    /// Equal to `ceil(log2(source.len())`
    pub fn num_levels(&self) -> usize {
        self.data.len()
    }

    /// Returns the data on given level.
    /// Level `0` returns the source data; the higher the level, the higher the compression (i.e. smaller vectors are returned).
    /// If the level is out of bounds, returns None
    pub fn get_level(&self, level: usize) -> Option<&Vec<T>> {
        if level >= self.num_levels() {
            return None;
        }

        Some(&self.data[level])
    }

    /// Downsamples a vector to `ceil(len / 2)`` elements.
    /// Currently, downsampling is done by averaging the pair of elements
    fn downsample(source: &[T]) -> Vec<T> {
        source
            .chunks(2)
            .map(|pair| match pair.len() {
                1 => pair[0],
                2 => T::from_f64((pair[0] + pair[1]).to_f64().unwrap() / 2.0).unwrap(),
                _ => panic!("Unsound condition"),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_downsample_ints() {
        let data = vec![2, 4, 6, 8];
        assert_eq!(MipMap1D::downsample(&data), vec![3, 7]);
    }

    #[test]
    fn test_uneven_downsample() {
        let data = vec![2, 4, 6, 8, 9];
        assert_eq!(MipMap1D::downsample(&data), vec![3, 7, 9]);
    }

    #[test]
    fn test_uneven_mipmap() {
        let data = vec![2, 4, 6, 8, 9];
        let target = vec![vec![2, 4, 6, 8, 9], vec![3, 7, 9], vec![5, 9], vec![7]];
        let mipmap = MipMap1D::new(data);
        assert_eq!(mipmap.data, target);
    }

    #[test]
    fn test_mipmap_levels() {
        let data = vec![2, 4, 6, 8, 9];
        let target = [vec![2, 4, 6, 8, 9], vec![3, 7, 9], vec![5, 9], vec![7]];
        let mipmap = MipMap1D::new(data);

        assert_eq!(mipmap.num_levels(), target.len());
        for (level, target_item) in target.iter().enumerate() {
            let res = mipmap.get_level(level);
            assert!(res.is_some());
            let res = res.unwrap();

            assert_eq!(*res, *target_item)
        }
    }

    #[test]
    fn test_fails_on_nonexistent_level() {
        let data = vec![2, 4, 6, 8, 9];
        let mipmap = MipMap1D::new(data);

        assert_eq!(mipmap.get_level(mipmap.num_levels()), None);
    }
}
