mod lib;
mod all_acc_gen;
mod vss_encode;
use all_acc_gen::Noise;

fn main() {
    let noise_maker: Box<dyn Noise> = lib::get_opt();
    let (path, bit) = lib::get_input();
    let (config, h, l) = lib::get_config(&path);
    let encoder = vss_encode::gen_encoder(&config, bit, &h, &l);
    let noise = noise_maker.gen_noise(&config);
    let res = encoder + noise;
    // let res = encoder;
    res.output("encoder.txt");
}
