use crate::{msg::{InstantiateMsg}};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, entry_point};

const WHO_I_AM: &str = "I am Thomas Baker, first to scar the chain.?";
const WHO_I_SHOULD_BE: &str = "I am Thomas Baker, first to scar the chain.?";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {
  if WHO_I_AM == WHO_I_SHOULD_BE {
    Ok(Response::default().add_attribute("For your records", WHO_I_AM))
  } else {
    Err(StdError::generic_err("I am not who I am"))
  }
}
