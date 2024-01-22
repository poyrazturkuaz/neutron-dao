use crate::cron_module_param_types::{ParamsCron, ParamsResponseCron};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    from_json, to_json_binary, ContractResult, Empty, OwnedDeps, Querier, QuerierResult,
    QueryRequest, SystemError, SystemResult,
};
use std::marker::PhantomData;

pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_storage = MockStorage::default();
    let custom_querier = WasmMockQuerier::new(MockQuerier::new(&[]));

    OwnedDeps {
        storage: custom_storage,
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<Empty> = match from_json(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return QuerierResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                });
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Stargate { path, data: _ } => match path.as_str() {
                "/neutron.cron.Query/Params" => {
                    let resp = to_json_binary(&ParamsResponseCron {
                        params: ParamsCron {
                            security_address: "neutron_dao_address".to_string(),
                            limit: 10,
                        },
                    });
                    SystemResult::Ok(ContractResult::from(resp))
                }
                _ => todo!(),
            },
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    fn new(base: MockQuerier) -> WasmMockQuerier {
        WasmMockQuerier { base }
    }
}
