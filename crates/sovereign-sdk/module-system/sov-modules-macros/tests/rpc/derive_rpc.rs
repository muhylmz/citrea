use jsonrpsee::core::RpcResult;
use sov_modules_api::default_context::ZkDefaultContext;
use sov_modules_api::macros::rpc_gen;
use sov_modules_api::{Context, ModuleInfo, WorkingSet};
use sov_state::ZkStorage;

#[derive(ModuleInfo)]
pub struct TestStruct<C: ::sov_modules_api::Context> {
    #[address]
    pub(crate) address: C::Address,
}

#[rpc_gen(client, server, namespace = "test")]
impl<C: sov_modules_api::Context> TestStruct<C> {
    #[rpc_method(name = "firstMethod")]
    pub fn first_method(&self, _working_set: &mut WorkingSet<C>) -> RpcResult<u32> {
        Ok(11)
    }

    #[rpc_method(name = "secondMethod")]
    pub fn second_method(
        &self,
        result: u32,
        _working_set: &mut WorkingSet<C>,
    ) -> RpcResult<u32> {
        Ok(result)
    }

    #[rpc_method(name = "thirdMethod")]
    pub fn third_method(&self, result: u32) -> RpcResult<u32> {
        Ok(result)
    }

    #[rpc_method(name = "fourthMethod")]
    pub fn fourth_method(
        &self,
        _working_set: &mut WorkingSet<C>,
        result: u32,
    ) -> RpcResult<u32> {
        Ok(result)
    }
}

pub struct TestRuntime<C: Context> {
    test_struct: TestStruct<C>,
}

// This is generated by a macro annotating the state transition runner,
// but we do not have that in scope here so generating the struct manually.
struct RpcStorage<C: Context> {
    pub storage: C::Storage,
}

impl TestStructRpcImpl<ZkDefaultContext> for RpcStorage<ZkDefaultContext> {
    fn get_working_set(
        &self,
    ) -> ::sov_modules_api::WorkingSet<ZkDefaultContext> {
        ::sov_modules_api::WorkingSet::new(self.storage.clone())
    }
}

fn main() {
    let storage = ZkStorage::new();
    let r: RpcStorage<ZkDefaultContext> = RpcStorage {
        storage: storage.clone(),
    };
    {
        let result =
            <RpcStorage<ZkDefaultContext> as TestStructRpcServer<ZkDefaultContext>>::first_method(
                &r,
            )
            .unwrap();
        assert_eq!(result, 11);
    }

    {
        let result =
            <RpcStorage<ZkDefaultContext> as TestStructRpcServer<ZkDefaultContext>>::second_method(
                &r, 22,
            )
            .unwrap();
        assert_eq!(result, 22);
    }

    {
        let result =
            <RpcStorage<ZkDefaultContext> as TestStructRpcServer<ZkDefaultContext>>::third_method(
                &r, 33,
            )
            .unwrap();
        assert_eq!(result, 33);
    }

    {
        let result =
            <RpcStorage<ZkDefaultContext> as TestStructRpcServer<ZkDefaultContext>>::fourth_method(
                &r, 44,
            )
            .unwrap();
        assert_eq!(result, 44);
    }

    {
        let result =
            <RpcStorage<ZkDefaultContext> as TestStructRpcServer<ZkDefaultContext>>::health(&r)
                .unwrap();
        assert_eq!(result, ());
    }

    // {
    //     let result =
    //         <RpcStorage<ZkDefaultContext> as TestStructRpcServer<ZkDefaultContext>>::module_address(&r)
    //             .unwrap();
    //     assert_eq!(
    //         result,
    //         "sov1y34qkrqwffa3hmpdzvj0fqc0ahmlgrjf5ltfan9ugt82v5ej6lkshg9ypu"
    //     );
    // }

    println!("All tests passed!");
}
