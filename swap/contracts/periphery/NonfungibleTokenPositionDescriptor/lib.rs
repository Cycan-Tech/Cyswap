#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod PositionDescriptor {
    use ink_storage::traits::SpreadAllocate;
    use crabswap::traits::periphery::position_descriptor::*;
    use primitives::Address;
    use primitives::Uint256;
    use ink_prelude::string::String;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct PositionDescriptor {
    }

    impl Descriptor for PositionDescriptor{
        #[ink(message)]
        fn tokenURI(&self,positionManager:Address, tokenId:u128) -> String{
            String::from("")
        }
    }
    
    impl PositionDescriptor {
        #[ink(constructor)]
        // constructor(address _WETH9, bytes32 _nativeCurrencyLabelBytes) {
        pub fn new(_WETH9: AccountId, _nativeCurrencyLabelBytes: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut PositionDescriptor| {
            })
        }
    }
}