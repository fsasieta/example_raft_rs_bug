#[derive(Debug)]
pub struct PublicStructWithPrvateMembers {
    pub public_int: u64,
    private_int: u64
}

pub mod public_module {

    #[derive(Debug)]
    pub struct Unreachable {
        pub public_int: u64,
        private_int: u64,
    }
}

fn main() {

    // fine because their on the same module
    let sample = PublicStructWithPrvateMembers  { public_int: 10_u64, private_int: 20_u64 };

    println!("{:?}", sample);

    let sample2 = public_module::Unreachable { public_int: 10_64, private_int: 20_64 };

    println!("{:?}", sample2);
}
