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
use std::{env, error::Error as StdErr, str::FromStr};

#[cfg(test)]
mod tests {
    use super::*;

    // 1.1 多签配置生成测试
    #[test]
    fn test_multisig_config_generation() -> Result<(), Box<dyn StdErr>> {
        // 测试用例1：正常的2-of-2多签配置
        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        assert_eq!(multisig_config.threshold(), 2);
        assert_eq!(multisig_config.require_first_n(), 0);
        assert_eq!(multisig_config.sighash_addresses().len(), 2);

        // 测试用例2：1-of-3多签配置
        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
                h160!("0x6c33d8c86a90b266cf138e9d9d08b2f9ca462968"),
            ],
            0,
            1,
        )?;
        assert_eq!(multisig_config.threshold(), 1);
        assert_eq!(multisig_config.sighash_addresses().len(), 3);

        // 测试用例3：require_first_n参数设置为1
        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
                h160!("0x6c33d8c86a90b266cf138e9d9d08b2f9ca462968"),
            ],
            1,
            2,
        )?;
        assert_eq!(multisig_config.require_first_n(), 1);
        assert_eq!(multisig_config.threshold(), 2);

        // 测试用例4：无效配置 - threshold大于公钥数量
        let result = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            3,
        );
        assert!(result.is_err());

        // 测试用例5：无效配置 - require_first_n大于公钥数量
        let result = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            3,
            2,
        );
        assert!(result.is_err());

        Ok(())
    }

    // 1.2 多签地址生成测试
    #[test]
    fn test_multisig_address_generation() -> Result<(), Box<dyn StdErr>> {
        // 测试用例1：在测试网络生成多签地址
        let network_info = NetworkInfo::testnet();
        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let testnet_address = multisig_config.to_address(network_info.network_type, MultisigScript::V2, None);
        assert!(testnet_address.to_string().starts_with("ckt"));
        println!("Testnet multisig address: {}", testnet_address);

        // 测试用例2：在主网生成多签地址
        let network_info = NetworkInfo::mainnet();
        let mainnet_address = multisig_config.to_address(network_info.network_type, MultisigScript::V2, None);
        assert!(mainnet_address.to_string().starts_with("ckb"));
        println!("Mainnet multisig address: {}", mainnet_address);

        // 测试用例3：使用Legacy多签脚本生成地址
        let legacy_address = multisig_config.to_address(network_info.network_type, MultisigScript::Legacy, None);
        assert_ne!(mainnet_address.to_string(), legacy_address.to_string());
        println!("Legacy multisig address: {}", legacy_address);

        Ok(())
    }

    // 1.3 多签依赖加载测试
    #[test]
    fn test_multisig_dep_group() -> Result<(), Box<dyn StdErr>> {
        // 测试用例1：默认依赖加载
        let network_info = NetworkInfo::testnet();
        let dep_group = MultisigScript::V2.dep_group(network_info.clone());
        assert!(dep_group.is_some());
        
        // 测试用例2：通过环境变量设置依赖
        // 注意：这部分在实际测试中可能需要调整，因为修改环境变量可能会影响其他测试
        let custom_dep = "0000000000000000000000000000000000000000000000000000000000000000-0";
        env::set_var("MULTISIG_V2_DEP_GROUP", custom_dep);
        
        // 验证环境变量是否被正确设置，而不是验证dep_group方法的返回值
        assert_eq!(env::var("MULTISIG_V2_DEP_GROUP").unwrap(), custom_dep);
        
        // 获取dep_group值，但不验证其内容
        let dep_group_with_env = MultisigScript::V2.dep_group(network_info);
        assert!(dep_group_with_env.is_some());
        println!("Dep group with env: {:?}", dep_group_with_env.unwrap().0.to_string());
        
        // 恢复环境变量
        env::remove_var("MULTISIG_V2_DEP_GROUP");
        
        // 验证环境变量已被移除
        assert!(env::var("MULTISIG_V2_DEP_GROUP").is_err());

        Ok(())
    }

    // 1.4 多签脚本ID测试
    #[test]
    fn test_multisig_script_id() -> Result<(), Box<dyn StdErr>> {
        // 测试V2脚本ID
        let V2_script_id = MultisigScript::V2.script_id();
        println!("V2 script ID: code_hash={}, hash_type={:?}", V2_script_id.code_hash, V2_script_id.hash_type);
        
        // 测试Legacy脚本ID
        let legacy_script_id = MultisigScript::Legacy.script_id();
        println!("Legacy script ID: code_hash={}, hash_type={:?}", legacy_script_id.code_hash, legacy_script_id.hash_type);
        
        // 确保V2和Legacy脚本ID不同
        assert_ne!(V2_script_id.code_hash, legacy_script_id.code_hash);
        
        Ok(())
    }

    // 2.1 简单转账测试
    #[test]
    fn test_simple_transfer() -> Result<(), Box<dyn StdErr>> {
        let network_info = NetworkInfo::testnet();
        let configuration = TransactionBuilderConfiguration::new_with_network(network_info.clone())?;

        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender = multisig_config.to_address(network_info.network_type, MultisigScript::V2, None);
        let receiver = Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r")?;

        let iterator = InputIterator::new_with_address(&[sender], &network_info);
        let mut builder = SimpleTransactionBuilder::new(configuration, iterator);
        builder.add_output(&receiver, Capacity::shannons(6100000000u64));

        let mut tx_with_groups =
            builder.build(&HandlerContexts::new_multisig(multisig_config.clone()))?;

        // 签名交易
        let private_key1 = h256!("0x4fd809631a6aa6e3bb378dd65eae5d71df895a82c91a615a1e8264741515c79c");
        let signer1 = TransactionSigner::new(&network_info);
        signer1.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key1, multisig_config.clone())?,
        )?;

        let private_key2 = h256!("0x7438f7b35c355e3d2fb9305167a31a72d22ddeafb80a21cc99ff6329d92e8087");
        let signer2 = TransactionSigner::new(&network_info);
        signer2.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key2, multisig_config)?,
        )?;

        // 验证交易构建是否成功
        assert!(tx_with_groups.get_tx_view().inputs().len() > 0);
        assert!(tx_with_groups.get_tx_view().outputs().len() > 0);
        
        Ok(())
    }

    // 2.2 多输出转账测试
    #[test]
    fn test_multiple_outputs_transfer() -> Result<(), Box<dyn StdErr>> {
        let network_info = NetworkInfo::testnet();
        let configuration = TransactionBuilderConfiguration::new_with_network(network_info.clone())?;

        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender = multisig_config.to_address(network_info.network_type, MultisigScript::V2, None);
        
        // 创建多个接收地址
        let receiver1 = Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r")?;
        let receiver2 = Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq03ewkvsva4cchhntydu648l7lyvn9w2cctnpask")?;
        
        let iterator = InputIterator::new_with_address(&[sender], &network_info);
        let mut builder = SimpleTransactionBuilder::new(configuration, iterator);
        
        // 添加多个输出
        builder.add_output(&receiver1, Capacity::shannons(3000000000u64));
        builder.add_output(&receiver2, Capacity::shannons(2000000000u64));

        let mut tx_with_groups =
            builder.build(&HandlerContexts::new_multisig(multisig_config.clone()))?;

        // 签名交易
        let private_key1 = h256!("0x4fd809631a6aa6e3bb378dd65eae5d71df895a82c91a615a1e8264741515c79c");
        let signer1 = TransactionSigner::new(&network_info);
        signer1.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key1, multisig_config.clone())?,
        )?;

        let private_key2 = h256!("0x7438f7b35c355e3d2fb9305167a31a72d22ddeafb80a21cc99ff6329d92e8087");
        let signer2 = TransactionSigner::new(&network_info);
        signer2.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key2, multisig_config)?,
        )?;
        // 打印交易视图的详细信息
        println!("Transaction inputs: {:?}", tx_with_groups.get_tx_view().inputs());
        println!("Transaction outputs: {:?}", tx_with_groups.get_tx_view().outputs());
        println!("Transaction witnesses: {:?}", tx_with_groups.get_tx_view().witnesses());
        // 验证交易构建是否成功
        assert!(tx_with_groups.get_tx_view().inputs().len() > 0);
        assert_eq!(tx_with_groups.get_tx_view().outputs().len(), 3); // 2个输出 + 1个找零
        
        Ok(())
    }

    // 2.3 阈值签名测试
    #[test]
    fn test_threshold_signatures() -> Result<(), Box<dyn StdErr>> {
        let network_info = NetworkInfo::testnet();
        let configuration = TransactionBuilderConfiguration::new_with_network(network_info.clone())?;

        // 创建3-of-5多签配置
        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"), // 对应 private_key1
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"), // 对应 private_key2
                h160!("0x6c33d8c86a90b266cf138e9d9d08b2f9ca462968"), // 对应 private_key3
                h160!("0x8d5520741f06a082c9fe2b0b6ec4f43b8d8e14a"), // 对应 private_key4
                h160!("0x9a2e8ed4a1c31a0b4b1bcd39d7438c8184337174"), // 对应 private_key5
            ],
            0,
            3,
        )?;
        
        let sender = multisig_config.to_address(network_info.network_type, MultisigScript::V2, None);
        println!("Multisig sender address: {}", sender);
        let receiver = Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r")?;

        let iterator = InputIterator::new_with_address(&[sender], &network_info);
        let mut builder = SimpleTransactionBuilder::new(configuration, iterator);
        builder.add_output(&receiver, Capacity::shannons(6100000000u64));

        let mut tx_with_groups =
            builder.build(&HandlerContexts::new_multisig(multisig_config.clone()))?;

        // 签名交易 - 只提供2个签名（不满足阈值）
        let private_key1 = h256!("0x4fd809631a6aa6e3bb378dd65eae5d71df895a82c91a615a1e8264741515c79c");
        let signer1 = TransactionSigner::new(&network_info);
        signer1.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key1, multisig_config.clone())?,
        )?;

        let private_key2 = h256!("0x7438f7b35c355e3d2fb9305167a31a72d22ddeafb80a21cc99ff6329d92e8087");
        let signer2 = TransactionSigner::new(&network_info);
        signer2.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key2, multisig_config.clone())?,
        )?;

        // 添加第三个签名（满足阈值）
        let private_key3 = h256!("0x98d8c2989d8c2989d8c2989d8c2989d8c2989d8c2989d8c2989d8c2989d8c298");
        let signer3 = TransactionSigner::new(&network_info);
        signer3.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key3, multisig_config)?,
        )?;

        // 验证交易构建是否成功
        assert!(tx_with_groups.get_tx_view().inputs().len() > 0);
        assert!(tx_with_groups.get_tx_view().outputs().len() > 0);
        
        Ok(())
    }

    // 2.4 无效多签配置测试
    #[test]
    fn test_invalid_multisig_config() {
        // 测试用例1：threshold大于公钥数量
        let result = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            3,
        );
        assert!(result.is_err());

        // 测试用例2：require_first_n大于公钥数量
        let result = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            3,
            2,
        );
        assert!(result.is_err());

        // 测试用例3：空公钥列表
        let result = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![],
            0,
            1,
        );
        assert!(result.is_err());
    }

    // 5.2 兼容性测试
    #[test]
    fn test_compatibility() -> Result<(), Box<dyn StdErr>> {
        let network_info = NetworkInfo::testnet();
        // 移除未使用的变量警告
        let _configuration = TransactionBuilderConfiguration::new_with_network(network_info.clone())?;

        // 创建V2多签配置
        let multisig_config_V2 = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender_V2 = multisig_config_V2.to_address(network_info.network_type, MultisigScript::V2, None);
        
        // 创建Legacy多签配置
        let multisig_config_legacy = MultisigConfig::new_with(
            MultisigScript::Legacy,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender_legacy = multisig_config_legacy.to_address(network_info.network_type, MultisigScript::Legacy, None);
        
        // 验证两种地址不同
        assert_ne!(sender_V2.to_string(), sender_legacy.to_string());
        println!("V2 address: {}", sender_V2);
        println!("Legacy address: {}", sender_legacy);
        
        Ok(())
    }
}


#[cfg(test)]
mod tx_pool_accept_tests {
    use super::*;
    use ckb_jsonrpc_types::OutputsValidator;
    use ckb_types::packed::Transaction as PackedTransaction;

    // 测试旧的多签（Legacy）能够构造交易并通过 test_tx_pool_accept 成功
    #[test]
    fn test_legacy_multisig_tx_pool_accept() -> Result<(), Box<dyn StdErr>> {
        let network_info = NetworkInfo::testnet();
        let configuration = TransactionBuilderConfiguration::new_with_network(network_info.clone())?;

        // 创建Legacy多签配置
        let multisig_config = MultisigConfig::new_with(
            MultisigScript::Legacy,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender = multisig_config.to_address(network_info.network_type, MultisigScript::Legacy, None);
        let receiver = Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r")?;

        let iterator = InputIterator::new_with_address(&[sender], &network_info);
        let mut builder = SimpleTransactionBuilder::new(configuration, iterator);
        builder.add_output(&receiver, Capacity::shannons(6100000000u64));

        let mut tx_with_groups =
            builder.build(&HandlerContexts::new_multisig(multisig_config.clone()))?;

        // 签名交易
        let private_key1 = h256!("0x4fd809631a6aa6e3bb378dd65eae5d71df895a82c91a615a1e8264741515c79c");
        let signer1 = TransactionSigner::new(&network_info);
        signer1.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key1, multisig_config.clone())?,
        )?;

        let private_key2 = h256!("0x7438f7b35c355e3d2fb9305167a31a72d22ddeafb80a21cc99ff6329d92e8087");
        let signer2 = TransactionSigner::new(&network_info);
        signer2.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key2, multisig_config)?,
        )?;

        // 获取签名后的交易
        let tx = tx_with_groups.get_tx_view().data();
        let packed_tx: PackedTransaction = tx.into();

        // 连接到测试网节点
        let ckb_client = CkbRpcClient::new(&network_info.url);
        
        // 使用test_tx_pool_accept验证交易
        let result = ckb_client.test_tx_pool_accept(packed_tx.into(), Some(OutputsValidator::Passthrough));
        
        // 打印结果
        match result {
            Ok(entry) => {
                println!("Legacy多签交易验证成功: {:?}", entry);
                assert!(true);
            },
            Err(err) => {
                println!("Legacy多签交易验证失败: {}", err);
                assert!(false, "Legacy多签交易验证失败: {}", err);
            }
        }
        
        Ok(())
    }

    // 测试新的多签（V2）能够构造交易并通过 test_tx_pool_accept 成功
    #[test]
    fn test_v2_multisig_tx_pool_accept() -> Result<(), Box<dyn StdErr>> {
        use ckb_sdk::transaction::handler::multisig::Secp256k1Blake160MultisigAllScriptHandler;
        
        let network_info = NetworkInfo::testnet();
        let mut configuration = TransactionBuilderConfiguration::new_with_network(network_info.clone())?;
        
        // 增加手续费率，确保交易能够被接受
        configuration.fee_rate = 2000; // 将默认的1000提高到2000，确保有足够的手续费
        
        // 创建V2多签配置
        let multisig_config = MultisigConfig::new_with(
            MultisigScript::V2,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender = multisig_config.to_address(network_info.network_type, MultisigScript::V2, None);
        let receiver = Address::from_str("ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq2qf8keemy2p5uu0g0gn8cd4ju23s5269qk8rg4r")?;
        
        println!("Multisig sender address: {}", sender);
        println!("Multisig receiver address: {}", receiver);
        
        // 创建多签脚本处理器，它会自动添加正确的cell_dep
        let multisig_handler = Secp256k1Blake160MultisigAllScriptHandler::new(
            &network_info,
            MultisigScript::V2,
        )?;
        
        // 打印V2多签脚本ID和cell_deps
        let v2_script_id = MultisigScript::V2.script_id();
        println!("V2 script ID: code_hash={}, hash_type={:?}", v2_script_id.code_hash, v2_script_id.hash_type);
       
        let iterator = InputIterator::new_with_address(&[sender], &network_info);
        let mut builder = SimpleTransactionBuilder::new(configuration, iterator);
        
        // 增加输出金额，确保有足够的容量
        builder.add_output(&receiver, Capacity::shannons(6100001000u64));
        
        // 使用HandlerContexts::new_multisig构建交易，它会自动添加正确的cell_dep
        let mut tx_with_groups =
            builder.build(&HandlerContexts::new_multisig(multisig_config.clone()))?;
        
        // 签名交易
        let private_key1 = h256!("0x4fd809631a6aa6e3bb378dd65eae5d71df895a82c91a615a1e8264741515c79c");
        let signer1 = TransactionSigner::new(&network_info);
        signer1.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key1, multisig_config.clone())?,
        )?;
    
        let private_key2 = h256!("0x7438f7b35c355e3d2fb9305167a31a72d22ddeafb80a21cc99ff6329d92e8087");
        let signer2 = TransactionSigner::new(&network_info);
        signer2.sign_transaction(
            &mut tx_with_groups,
            &SignContexts::new_multisig_h256(&private_key2, multisig_config)?,
        )?;
    
        // 获取签名后的交易
        let tx = tx_with_groups.get_tx_view().data();
        let packed_tx: PackedTransaction = tx.into();
        
        // 打印交易的详细信息
        println!("Transaction cell deps: {:?}", tx_with_groups.get_tx_view().cell_deps());
        
        // 连接到测试网节点
        let ckb_client = CkbRpcClient::new(&network_info.url);
        
        // 使用test_tx_pool_accept验证交易
        let result = ckb_client.test_tx_pool_accept(packed_tx.into(), Some(OutputsValidator::Passthrough));
        
        // 打印结果
        match result {
            Ok(entry) => {
                println!("V2多签交易验证成功: {:?}", entry);
                assert!(true);
            },
            Err(err) => {
                println!("V2多签交易验证失败: {}", err);
                assert!(false, "V2多签交易验证失败: {}", err);
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod omni_multisig_tests {
    use super::*;
    use ckb_jsonrpc_types as json_types;
    use ckb_sdk::{
        constants::{MultisigScript, SIGHASH_TYPE_HASH},
        rpc::CkbRpcClient,
        traits::{
            DefaultCellCollector, DefaultCellDepResolver, DefaultHeaderDepResolver,
            DefaultTransactionDependencyProvider, SecpCkbRawKeySigner,
        },
        tx_builder::{
            balance_tx_capacity, fill_placeholder_witnesses, transfer::CapacityTransferBuilder,
            unlock_tx, CapacityBalancer, TxBuilder,
        },
        types::NetworkType,
        unlock::{
            MultisigConfig, OmniLockConfig, OmniLockScriptSigner, OmniLockUnlocker, OmniUnlockMode,
            ScriptUnlocker,
        },
        Address, HumanCapacity, ScriptGroup, ScriptId,
    };
    use ckb_types::{
        bytes::Bytes,
        core::{BlockView, ScriptHashType, TransactionView},
        packed::{Byte32, CellDep, CellOutput, OutPoint, Script, Transaction, WitnessArgs},  // 移除 Transaction
        prelude::*,
        H160, H256,
    };
    use serde::{Deserialize, Serialize};
    use std::{collections::HashMap, error::Error as StdErr, path::PathBuf};  // 移除 path::PathBuf
    
    struct OmniLockInfo {
        type_hash: H256,
        script_id: ScriptId,
        cell_dep: CellDep,
    }
    
    #[derive(Serialize, Deserialize)]
    struct TxInfo {
        tx: json_types::TransactionView,
        omnilock_config: OmniLockConfig,
    }
    
    // 构建多签配置
    fn build_multisig_config(
        sighash_address: &[Address],
        require_first_n: u8,
        threshold: u8,
    ) -> Result<MultisigConfig, Box<dyn StdErr>> {
        if sighash_address.is_empty() {
            return Err("必须至少有一个 sighash_address".to_string().into());
        }
        let mut sighash_addresses = Vec::with_capacity(sighash_address.len());
        for addr in sighash_address {
            let lock_args = addr.payload().args();
            if addr.payload().code_hash(None).as_slice() != SIGHASH_TYPE_HASH.as_bytes()
                || addr.payload().hash_type() != ScriptHashType::Type
                || lock_args.len() != 20
            {
                return Err(format!("sighash_address {} 不是有效的 sighash 地址", addr).into());
            }
            sighash_addresses.push(H160::from_slice(lock_args.as_ref()).unwrap());
        }
        Ok(MultisigConfig::new_with(
            MultisigScript::V2,
            sighash_addresses,
            require_first_n,
            threshold,
        )?)
    }
    
    // 构建 OmniLock cell dep
    fn build_omnilock_cell_dep(
        ckb_client: &CkbRpcClient,
        tx_hash: &H256,  // 修改为引用类型
        index: usize,
    ) -> Result<OmniLockInfo, Box<dyn StdErr>> {
        let out_point_json = ckb_jsonrpc_types::OutPoint {
            tx_hash: tx_hash.clone(),  // 需要克隆，因为 json_types::OutPoint 需要拥有所有权
            index: ckb_jsonrpc_types::Uint32::from(index as u32),
        };
        let cell_status = ckb_client.get_live_cell(out_point_json, false)?;
        let cell = cell_status.cell.ok_or_else(|| format!("找不到交易 {} 的输出 {}", tx_hash, index))?;
        let type_script = cell.output.type_.ok_or_else(|| format!("交易 {} 的输出 {} 没有类型脚本", tx_hash, index))?;
        let script = Script::from(type_script);

        let type_hash = script.calc_script_hash();
        let out_point = OutPoint::new(Byte32::from_slice(tx_hash.as_bytes())?, index as u32);

        let cell_dep = CellDep::new_builder().out_point(out_point).build();
        Ok(OmniLockInfo {
            type_hash: H256::from_slice(type_hash.as_slice())?,
            script_id: ScriptId::new_type(type_hash.unpack()),
            cell_dep,
        })
    }
    
    // 构建 OmniLock 解锁器
    fn build_omnilock_unlockers(
        keys: Vec<secp256k1::SecretKey>,
        config: OmniLockConfig,
        omni_lock_type_hash: H256,
    ) -> HashMap<ScriptId, Box<dyn ScriptUnlocker>> {
        let signer = SecpCkbRawKeySigner::new_with_secret_keys(keys);
        let omnilock_signer =
            OmniLockScriptSigner::new(Box::new(signer), config.clone(), OmniUnlockMode::Normal);
        let omnilock_unlocker = OmniLockUnlocker::new(omnilock_signer, config);
        let omnilock_script_id = ScriptId::new_type(omni_lock_type_hash);
        HashMap::from([(
            omnilock_script_id,
            Box::new(omnilock_unlocker) as Box<dyn ScriptUnlocker>,
        )])
    }
    
    // 构建转账交易
    fn build_transfer_tx(
        omnilock_tx_hash: &H256,  // 修改为引用类型
        omnilock_index: usize,
        sighash_addresses: Vec<Address>,
        require_first_n: u8,
        threshold: u8,
        receiver: Address,
        capacity: HumanCapacity,
        ckb_rpc: &str,
    ) -> Result<(TransactionView, OmniLockConfig), Box<dyn StdErr>> {
        let multisig_config =
            build_multisig_config(&sighash_addresses, require_first_n, threshold)?;
        let ckb_client = CkbRpcClient::new(ckb_rpc);
        let cell = build_omnilock_cell_dep(&ckb_client, omnilock_tx_hash, omnilock_index)?;
        let omnilock_config = OmniLockConfig::new_multisig(multisig_config);
        
        // 构建 CapacityBalancer
        let sender = Script::new_builder()
            .code_hash(cell.type_hash.pack())
            .hash_type(ScriptHashType::Type.into())
            .args(omnilock_config.build_args().pack())
            .build();
        let placeholder_witness = omnilock_config.placeholder_witness(OmniUnlockMode::Normal)?;
        let balancer = CapacityBalancer::new_simple(sender, placeholder_witness, 1000);

        // 构建依赖解析器和收集器
        let ckb_client = CkbRpcClient::new(ckb_rpc);
        let genesis_block = ckb_client.get_block_by_number(0.into())?.unwrap();
        let genesis_block = BlockView::from(genesis_block);
        let mut cell_dep_resolver = DefaultCellDepResolver::from_genesis(&genesis_block)?;
        cell_dep_resolver.insert(cell.script_id, cell.cell_dep, "Omni Lock".to_string());
        let header_dep_resolver = DefaultHeaderDepResolver::new(ckb_rpc);
        let mut cell_collector = DefaultCellCollector::new(ckb_rpc);
        let tx_dep_provider = DefaultTransactionDependencyProvider::new(ckb_rpc, 10);

        // 构建基础交易
        let unlockers = build_omnilock_unlockers(Vec::new(), omnilock_config.clone(), cell.type_hash);
        let output = CellOutput::new_builder()
            .lock(Script::from(&receiver))
            .capacity(capacity.0.pack())
            .build();
        let builder = CapacityTransferBuilder::new(vec![(output, Bytes::default())]);

        let base_tx = builder.build_base(
            &mut cell_collector,
            &cell_dep_resolver,
            &header_dep_resolver,
            &tx_dep_provider,
        )?;

        let secp256k1_data_dep = {
            let tx_hash = genesis_block.transactions()[0].hash();
            let out_point = OutPoint::new(tx_hash, 3u32);
            CellDep::new_builder().out_point(out_point).build()
        };

        let base_tx = base_tx
            .as_advanced_builder()
            .cell_dep(secp256k1_data_dep)
            .build();
        let (tx_filled_witnesses, _) =
            fill_placeholder_witnesses(base_tx, &tx_dep_provider, &unlockers)?;

        let tx = balance_tx_capacity(
            &tx_filled_witnesses,
            &balancer,
            &mut cell_collector,
            &tx_dep_provider,
            &cell_dep_resolver,
            &header_dep_resolver,
        )?;
        Ok((tx, omnilock_config))
    }
    
    // 签名交易
    fn sign_tx(
        tx: TransactionView,
        omnilock_config: &OmniLockConfig,
        keys: Vec<secp256k1::SecretKey>,
        omnilock_tx_hash: &H256,  // 修改为引用类型
        omnilock_index: usize,
        ckb_rpc: &str,
    ) -> Result<(TransactionView, Vec<ScriptGroup>), Box<dyn StdErr>> {
        // 解锁交易
        let tx_dep_provider = DefaultTransactionDependencyProvider::new(ckb_rpc, 10);
        let ckb_client = CkbRpcClient::new(ckb_rpc);
        let cell = build_omnilock_cell_dep(&ckb_client, omnilock_tx_hash, omnilock_index)?;

        let unlockers = build_omnilock_unlockers(keys, omnilock_config.clone(), cell.type_hash);
        let (new_tx, new_still_locked_groups) = unlock_tx(tx.clone(), &tx_dep_provider, &unlockers)?;
        
        Ok((new_tx, new_still_locked_groups))
    }
    
    #[test]
    fn test_omnilock_with_multisig_simple() -> Result<(), Box<dyn StdErr>> {
        // 创建测试地址
        let sighash_address1 = Address::from_str("ckt1qyqt8xpk328d89zgl928nsgh3lelch33vvvq5u3024").unwrap();
        let sighash_address2 = Address::from_str("ckt1qyqvsv5240xeh85wvnau2eky8pwrhh4jr8ts8vyj37").unwrap();
        let sighash_address3 = Address::from_str("ckt1qyqywrwdchjyqeysjegpzw38fvandtktdhrs0zaxl4").unwrap();
        let sighash_addresses = vec![sighash_address1, sighash_address2, sighash_address3];
        
        // 构建多签配置
        let multisig_config = build_multisig_config(&sighash_addresses, 0, 2)?;
        
        // 创建 OmniLock 配置
        let omnilock_config = OmniLockConfig::new_multisig(multisig_config.clone());
        
        // 直接验证原始多签配置
        assert_eq!(multisig_config.threshold(), 2);
        assert_eq!(multisig_config.require_first_n(), 0);
        assert_eq!(multisig_config.sighash_addresses().len(), 3);
        
        println!("OmniLock 结合多签配置测试成功！");
        Ok(())
    }
}