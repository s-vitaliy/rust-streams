pub mod shape {

    struct InletImpl {
        name: String
    }

    trait InPort {
        fn id(&self) -> i32;
    }

    trait OutPort {
        fn id(&self) -> i32;
    }


    pub trait Inlet<T>: InPort {

    }

    pub trait Outlet<T>: OutPort {

    }

    pub trait Shape<T> {
        fn inlets(&self) -> Vec<& dyn Inlet<T>>;
        // fn outlets(&self) -> Vec<dyn &Outlet<T>>;
    }

    impl<T> Inlet<T> for InletImpl {
    }

    impl InPort for InletImpl {
        fn id(&self) -> i32 {
            return -1;
        }
    }

}