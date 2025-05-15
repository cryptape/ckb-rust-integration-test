use ckb_sdk::{
    constants::MultisigScript,
    transaction::{
        builder::{CkbTransactionBuilder, SimpleTransactionBuilder},
        handler::HandlerContexts,
        input::InputIterator,
        signer::{SignContexts, TransactionSigner},
        TransactionBuilderConfiguration,
    },
    unlock::MultisigConfig,
    Address, CkbRpcClient, NetworkInfo,
};
use ckb_types::{core::Capacity, h160, h256};
use std::{error::Error as StdErr, str::FromStr};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_ckb_multisig() -> Result<(), Box<dyn StdErr>> {
        let network_info = NetworkInfo::testnet();
        let configuration = TransactionBuilderConfiguration::new_with_network(network_info.clone())?;

        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V1,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender = multisig_config.to_address(network_info.network_type, MultisigScript::V1, None);
        println!("sender: {}", sender);
        let receiver = Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r")?;

        let iterator = InputIterator::new_with_address(&[sender], &network_info);
        let mut builder = SimpleTransactionBuilder::new(configuration, iterator);
        builder.add_output(&receiver, Capacity::shannons(6100000000u64));

        let mut tx_with_groups =
            builder.build(&HandlerContexts::new_multisig(multisig_config.clone()))?;
        println!("tx_with_groups:{:?}", &tx_with_groups);

        let json_tx = ckb_jsonrpc_types::TransactionView::from(tx_with_groups.get_tx_view().clone());
        println!("tx: {}", serde_json::to_string_pretty(&json_tx).unwrap());

        let private_key1 = h256!("0x4fd809631a6aa6e3bb378dd65eae5d71df895a82c91a615a1e8264741515c79c");
        let signer1 = TransactionSigner::new(&network_info);
        signer1.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key1, multisig_config.clone())?,
        )?;

        let json_tx = ckb_jsonrpc_types::TransactionView::from(tx_with_groups.get_tx_view().clone());
        println!("tx: {}", serde_json::to_string_pretty(&json_tx).unwrap());

        let signer2 = TransactionSigner::new(&network_info);
        let private_key2 = h256!("0x7438f7b35c355e3d2fb9305167a31a72d22ddeafb80a21cc99ff6329d92e8087");
        signer2.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key2, multisig_config)?,
        )?;

        let json_tx = ckb_jsonrpc_types::TransactionView::from(tx_with_groups.get_tx_view().clone());
        println!("tx: {}", serde_json::to_string_pretty(&json_tx).unwrap());

        // 注释掉实际发送交易的部分，避免在测试中真正发送交易
        // 如果需要在测试中验证交易构建是否正确，可以保留这部分代码
        // let tx_hash = CkbRpcClient::new(network_info.url.as_str())
        //     .send_transaction(json_tx.inner, None)
        //     .expect("send transaction");
        // println!(">>> tx {} sent! <<<", tx_hash);

        // 在测试中，我们可以断言交易构建是否成功
        assert!(tx_with_groups.get_tx_view().inputs().len() > 0);
        assert!(tx_with_groups.get_tx_view().outputs().len() > 0);
        
        Ok(())
    }
}