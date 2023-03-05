// 조합의 합의 합
use std::io::stdin;

const MOD: u64 = 1000_000_007;

fn main() {
    let mut lines = stdin().lines();
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();

    fn init_or_get_inv(invs: &mut [u64], n: usize) -> u64 {
        if invs[n] == 0 {
            invs[n] =
                ((MOD - MOD / n as u64) * init_or_get_inv(invs, (MOD % n as u64) as usize)) % MOD;
        }
        return invs[n];
    }

    fn combination(facs: &[u64], fac_invs: &[u64], n: usize, k: usize) -> u64 {
        let mut ret = (facs[n] * fac_invs[n - k]) % MOD;
        ret = (ret * fac_invs[k]) % MOD;
        return ret;
    }

    let m2 = m * 2;
    let mut facs = vec![0; m2 + 1];
    let mut fac_invs = vec![0; m2 + 1];
    let mut invs = vec![0; m2 + 1];
    (facs[0], facs[1], fac_invs[0], fac_invs[1], invs[1]) = (1, 1, 1, 1, 1);
    for i in 2..=m2 {
        facs[i] = (facs[i - 1] * i as u64) % MOD;
        fac_invs[i] = (fac_invs[i - 1] * init_or_get_inv(&mut invs, i)) % MOD;
    }

    let mut answer = 0;
    for i in 3..=m {
        answer = (answer + combination(&facs, &fac_invs, i * 2, i)) % MOD;
    }

    println!("{answer}");
}
