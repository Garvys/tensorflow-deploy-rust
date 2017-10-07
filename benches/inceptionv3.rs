#[macro_use]
extern crate bencher;
extern crate image;
extern crate itertools;
extern crate flate2;
extern crate ndarray;
extern crate reqwest;
extern crate tar;
extern crate tfdeploy;

#[path = "../examples/inceptionv3.rs"]
mod inceptionv3;

#[cfg(feature = "tensorflow")]
fn dummy(_bencher: &mut bencher::Bencher) {
    inceptionv3::download().unwrap();
    ::tfdeploy::tf::for_path(inceptionv3::INCEPTION_V3).unwrap();
}

#[cfg(feature = "tensorflow")]
fn tf(bencher: &mut bencher::Bencher) {
    inceptionv3::download().unwrap();
    let mut tf = ::tfdeploy::tf::for_path(inceptionv3::INCEPTION_V3).unwrap();
    let input = inceptionv3::load_image(inceptionv3::HOPPER);
    bencher.iter(|| {
        tf.run(
            vec![("input", input.clone())],
            "InceptionV3/Predictions/Reshape_1",
        ).unwrap()
    });
}

fn tfd(bencher: &mut bencher::Bencher) {
    inceptionv3::download().unwrap();
    let tfd = ::tfdeploy::for_path(inceptionv3::INCEPTION_V3).unwrap();
    let input = inceptionv3::load_image(inceptionv3::HOPPER);
    bencher.iter(|| {
        tfd.run(
            vec![("input", input.clone())],
            "InceptionV3/Predictions/Reshape_1",
        ).unwrap();
    });
}

#[cfg(feature = "tensorflow")]
benchmark_group!(benches, dummy, tf, tfd);
#[cfg(not(feature = "tensorflow"))]
benchmark_group!(benches, tfd);
benchmark_main!(benches);
