use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use face_cropper::{
    process_folder_with_images_iter, process_folder_with_images_rayon, process_image,
};
use std::path::PathBuf; // Import PathBuf directly for clarity

pub fn criterion_process_image(c: &mut Criterion) {
    let cascade_path = "./assets/haarcascade_frontalface_default.xml";
    let image_path = "./Lenna.png";
    let output_path = "./";

    println!("{:?}", cascade_path);
    println!("{:?}", image_path);
    println!("{:?}", output_path);

    let cascase_pathbuf = PathBuf::from(cascade_path);
    let output_pathbuf = PathBuf::from(output_path);

    c.bench_function("process signle image", |b| {
        b.iter(|| {
            process_image(&cascase_pathbuf, image_path, output_pathbuf.clone());
        });
    });
}

pub fn criterion_process_image_folder_without_rayon(c: &mut Criterion) {
    let cascade_path = "./assets/haarcascade_frontalface_default.xml";
    let image_folder = "./test_images/";
    let output_path = "./";

    println!("{:?}", cascade_path);
    println!("{:?}", image_folder);
    println!("{:?}", output_path);

    let cascase_pathbuf = PathBuf::from(cascade_path);
    let output_pathbuf = PathBuf::from(output_path);

    c.bench_function("process image folders without rayon", |b| {
        b.iter(|| {
            process_folder_with_images_iter(&cascase_pathbuf, image_folder, output_pathbuf.clone());
        });
    });
}

pub fn criterion_process_image_folder_with_rayon(c: &mut Criterion) {
    let cascade_path = "./assets/haarcascade_frontalface_default.xml";
    let image_folder = "./test_images/";
    let output_path = "./";

    println!("{:?}", cascade_path);
    println!("{:?}", image_folder);
    println!("{:?}", output_path);

    let cascase_pathbuf = PathBuf::from(cascade_path);
    let output_pathbuf = PathBuf::from(output_path);

    let mut group = c.benchmark_group("my_benchmarks");

    group.sample_size(10); // set sample size to 20

    group.bench_function("Wtih Rayon", |b| {
        b.iter(|| {
            process_folder_with_images_rayon(
                &cascase_pathbuf,
                image_folder,
                output_pathbuf.clone(),
            );
        });
    });
}

fn becnch_image_folders(c: &mut Criterion) {
    let cascade_path = "./assets/haarcascade_frontalface_default.xml";
    let image_folder = "./test_images/";
    let output_path = "./";

    println!("{:?}", cascade_path);
    println!("{:?}", image_folder);
    println!("{:?}", output_path);

    let cascase_pathbuf = PathBuf::from(cascade_path);
    let output_pathbuf = PathBuf::from(output_path);

    let mut group = c.benchmark_group("my_benchmarks");

    group.sample_size(10); // set sample size to 20

    group.bench_function("Without Rayon", |b| {
        b.iter(|| {
            process_folder_with_images_iter(&cascase_pathbuf, image_folder, output_pathbuf.clone());
        });
    });

    group.bench_function("with Rayon", |b| {
        b.iter(|| {
            process_folder_with_images_rayon(
                &cascase_pathbuf,
                image_folder,
                output_pathbuf.clone(),
            );
        });
    });
}

criterion_group!(benches, criterion_process_image, becnch_image_folders);
criterion_main!(benches);
