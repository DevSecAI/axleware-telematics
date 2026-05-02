// AXL-SAST-006: rand::thread_rng() — convenient but not auditable as CSPRNG;
// production should use ring/SystemRandom or rand_core::OsRng directly.
use rand::Rng;

pub fn invite() -> String {
    let mut rng = rand::thread_rng();
    (0..24).map(|_| char::from(rng.gen_range(b'a'..=b'z'))).collect()
}
