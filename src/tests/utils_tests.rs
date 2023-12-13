use super::*;

#[test]
pub(crate) fn circle_tests()
{
  assert!(is_point_in_circle(240., 78., 233., 73., 13.));
  assert!(!is_point_in_circle(752., 251., 725., 270., 13.));
  assert!(!is_point_in_circle(81., 24., 40., 40., 20.));
  assert!(is_point_in_circle(25., 29., 40., 40., 20.));
  assert!(!is_point_in_circle(189., 176., 143., 136., 57.));
  assert!(is_point_in_circle(192., 87., 143., 136., 75.));
  assert!(!is_point_in_circle(177., 243., 153., 250., 4.));
  assert!(is_point_in_circle(201., 251., 200., 250., 4.));
  assert!(!is_point_in_circle(1020., 315., 873., 310., 48.));
  assert!(is_point_in_circle(606., 411., 615., 404., 22.));
}

#[test]
pub(crate) fn rectangle_tests()
{
  assert!(!is_point_in_rectangle(747., 313., 600., 357., 100., 100.));
  assert!(!is_point_in_rectangle(433., 226., 448., 170., 38., 100.));
  assert!(is_point_in_rectangle(382., 274., 213., 165., 324., 238.));
  assert!(!is_point_in_rectangle(664., 180., 563., 211., 62., 67.));
  assert!(!is_point_in_rectangle(601., 182., 587., 389., 31., 92.));
  assert!(!is_point_in_rectangle(821., 85., 509., 53., 276., 67.));
  assert!(!is_point_in_rectangle(229., 328., 77., 139., 143., 181.));
  assert!(!is_point_in_rectangle(450., 618., 383., 384., 191., 134.));
  assert!(!is_point_in_rectangle(729., 685., 813., 418., 96., 248.));
  assert!(!is_point_in_rectangle(305., 617., 355., 588., 96., 86.));
  assert!(is_point_in_rectangle(637., 297., 419., 68., 248., 248.));
  assert!(is_point_in_rectangle(803., 429., 781., 404., 210., 134.));
  assert!(is_point_in_rectangle(135., 88., 104., 55., 162., 305.));
}
