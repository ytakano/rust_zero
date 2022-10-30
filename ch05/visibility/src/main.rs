mod a {
    struct TypeA {
        // a1: a_1::TypeA1, // エラー。子のプライベートな要素は見えない
        a2: Box<a_2::TypeA2>, // 子のパブリックな要素は見える
    }

    mod a_1 {
        struct TypeA1 {
            // 親が見えるものは見える
            a: Box<super::TypeA>,
            a2: Box<super::a_2::TypeA2>,
        }
    }

    mod a_2 {
        pub struct TypeA2 {
            // 親が見えるものは見える
            a: Box<super::TypeA>,
            // a1: super::a_1::TypeA1, // エラー。親の見えないものは見えない
        }
    }
}

mod b {
    pub struct TypeB;

    mod b_1 {
        pub struct TypeB1 {
            pub n: usize,
            m: usize,
        }

        impl TypeB1 {
            fn g(&self) {}
            pub fn h(&self) {}
        }

        fn f1(p: &super::b_1::TypeB1) {
            println!("{}", p.n);
            println!("{}", p.m);
            p.g();
            p.h();
        }
    }

    pub mod b_2 {
        pub struct TypeB2;

        fn f2(p: &super::b_1::TypeB1) {
            println!("{}", p.n);
            // println!("{}", p.m); // エラー。mはプライベート
            // p.g(); // エラー。gはプライベート
            p.h();
        }
    }
}

mod c {
    mod c_1_outer {
        pub mod c_1_inner {
            pub(crate) struct TypeC1; // 同じクレート内からのみ見える
            pub(super) struct TypeC2; // 親モジュールからのみ見える
            pub(in crate::c::c_1_outer) struct TypeC3; // 親モジュールからのみ見える
            pub(self) struct TypeC4; // プライベートと同義
        }

        fn f() {
            let p1 = c_1_inner::TypeC1;
            let p2 = c_1_inner::TypeC2;
            let p3 = c_1_inner::TypeC3;
            // let p4 = c_1_inner::TypeC4; // エラー。プライベートなので見えない
        }
    }

    fn g() {
        let p1 = c_1_outer::c_1_inner::TypeC1;
        // let p2 = c_1_outer::c_1_inner::TypeC2; // エラー
        // let p3 = c_1_outer::c_1_inner::TypeC3; // エラー
        // let p4 = c_1_outer::c_1_inner::TypeC4; // エラー
    }
}

mod d {
    pub struct TypeD;
}

mod e {
    pub use crate::d::TypeD;
}

fn main() {
    // let a = a::TypeA; // エラー。子のプライベートな要素は見えない
    let b = b::TypeB; // 子のパブリックな要素は見える

    //let b1 = b::b_1::TypeB1; // 子のプライベートな要素なモジュールb_1は見えない
    let b2 = b::b_2::TypeB2; // パブリックな孫b_2のパブリックな要素TypeB2は見える

    let e = e::TypeD; // 再エクスポートされた型を利用
}
