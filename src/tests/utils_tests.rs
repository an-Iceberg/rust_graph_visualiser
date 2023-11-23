use super::*;

// TODO: extract each assertion into its own test
#[test]
pub fn circle_tests()
{
  assert!(is_point_in_circle(240_f32, 78_f32, 233_f32, 73_f32, 13_f32));
  assert!(!is_point_in_circle(752_f32, 251_f32, 725_f32, 270_f32, 13_f32));
  assert!(!is_point_in_circle(81_f32, 24_f32, 40_f32, 40_f32, 20_f32));
  assert!(is_point_in_circle(25_f32, 29_f32, 40_f32, 40_f32, 20_f32));
  assert!(!is_point_in_circle(189_f32, 176_f32, 143_f32, 136_f32, 57_f32));
  assert!(is_point_in_circle(192_f32, 87_f32, 143_f32, 136_f32, 75_f32));
  assert!(!is_point_in_circle(177_f32, 243_f32, 153_f32, 250_f32, 4_f32));
  assert!(is_point_in_circle(201_f32, 251_f32, 200_f32, 250_f32, 4_f32));
  assert!(!is_point_in_circle(1020_f32, 315_f32, 873_f32, 310_f32, 48_f32));
  assert!(is_point_in_circle(606_f32, 411_f32, 615_f32, 404_f32, 22_f32));
}

#[test]
pub fn rectangle_tests()
{
  assert!(!is_point_in_rectangle(747_f32, 313_f32, 600_f32, 357_f32, 100_f32, 100_f32));
  assert!(!is_point_in_rectangle(433_f32, 226_f32, 448_f32, 170_f32, 38_f32, 100_f32));
  assert!(is_point_in_rectangle(382_f32, 274_f32, 213_f32, 165_f32, 324_f32, 238_f32));
  assert!(!is_point_in_rectangle(664_f32, 180_f32, 563_f32, 211_f32, 62_f32, 67_f32));
  assert!(!is_point_in_rectangle(601_f32, 182_f32, 587_f32, 389_f32, 31_f32, 92_f32));
  assert!(!is_point_in_rectangle(821_f32, 85_f32, 509_f32, 53_f32, 276_f32, 67_f32));
  assert!(!is_point_in_rectangle(229_f32, 328_f32, 77_f32, 139_f32, 143_f32, 181_f32));
  assert!(!is_point_in_rectangle(450_f32, 618_f32, 383_f32, 384_f32, 191_f32, 134_f32));
  assert!(!is_point_in_rectangle(729_f32, 685_f32, 813_f32, 418_f32, 96_f32, 248_f32));
  assert!(!is_point_in_rectangle(305_f32, 617_f32, 355_f32, 588_f32, 96_f32, 86_f32));
  assert!(is_point_in_rectangle(637_f32, 297_f32, 419_f32, 68_f32, 248_f32, 248_f32));
  assert!(is_point_in_rectangle(803_f32, 429_f32, 781_f32, 404_f32, 210_f32, 134_f32));
  assert!(is_point_in_rectangle(135_f32, 88_f32, 104_f32, 55_f32, 162_f32, 305_f32));
}
