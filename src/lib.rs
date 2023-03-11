pub mod bd09;
pub mod bd09mc;
pub mod gcj02;

#[cfg(test)]
mod tests {
    use crate::{
        bd09::{bd09_to_wgs84, wgs84_to_bd09},
        bd09mc::{bd09mc_to_wgs84, wgs84_to_bd09mc},
        gcj02::{gcj02_to_wgs84, wgs84_to_gcj02},
    };
    use approx::assert_relative_eq;

    #[test]
    fn gcj02() {
        let coordinate = (116.397428, 39.90923).into();
        let gcj_coord = wgs84_to_gcj02(coordinate);
        assert_relative_eq! {
            gcj02_to_wgs84(gcj_coord),
            coordinate,
            epsilon = 1e-6
        };
    }

    #[test]
    fn bd09() {
        let coordinate = (116.397428, 39.90923).into();
        let bd09_coord = wgs84_to_bd09(coordinate);
        assert_relative_eq! {
            bd09_to_wgs84(bd09_coord),
            coordinate,
            epsilon = 1e-6
        };
    }

    #[test]
    fn bd09mc() {
        let coordinate = (116.397428, 39.90923).into();
        let bd09mc_coord = wgs84_to_bd09mc(coordinate);
        assert_relative_eq! {
            bd09mc_to_wgs84(bd09mc_coord),
            coordinate,
            epsilon = 1e-6
        };
    }
}
