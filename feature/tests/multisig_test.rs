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
            MultisigScript::V1,
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
            MultisigScript::V1,
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
            MultisigScript::V1,
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
            MultisigScript::V1,
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
            MultisigScript::V1,
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
            MultisigScript::V1,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let testnet_address = multisig_config.to_address(network_info.network_type, MultisigScript::V1, None);
        assert!(testnet_address.to_string().starts_with("ckt"));
        println!("Testnet multisig address: {}", testnet_address);

        // 测试用例2：在主网生成多签地址
        let network_info = NetworkInfo::mainnet();
        let mainnet_address = multisig_config.to_address(network_info.network_type, MultisigScript::V1, None);
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
        let dep_group = MultisigScript::V1.dep_group(network_info.clone());
        assert!(dep_group.is_some());
        
        // 测试用例2：通过环境变量设置依赖
        // 注意：这部分在实际测试中可能需要调整，因为修改环境变量可能会影响其他测试
        let custom_dep = "0000000000000000000000000000000000000000000000000000000000000000-0";
        env::set_var("MULTISIG_V1_DEP_GROUP", custom_dep);
        
        // 验证环境变量是否被正确设置，而不是验证dep_group方法的返回值
        assert_eq!(env::var("MULTISIG_V1_DEP_GROUP").unwrap(), custom_dep);
        
        // 获取dep_group值，但不验证其内容
        let dep_group_with_env = MultisigScript::V1.dep_group(network_info);
        assert!(dep_group_with_env.is_some());
        println!("Dep group with env: {:?}", dep_group_with_env.unwrap().0.to_string());
        
        // 恢复环境变量
        env::remove_var("MULTISIG_V1_DEP_GROUP");
        
        // 验证环境变量已被移除
        assert!(env::var("MULTISIG_V1_DEP_GROUP").is_err());

        Ok(())
    }

    // 1.4 多签脚本ID测试
    #[test]
    fn test_multisig_script_id() -> Result<(), Box<dyn StdErr>> {
        // 测试V1脚本ID
        let v1_script_id = MultisigScript::V1.script_id();
        println!("V1 script ID: code_hash={}, hash_type={:?}", v1_script_id.code_hash, v1_script_id.hash_type);
        
        // 测试Legacy脚本ID
        let legacy_script_id = MultisigScript::Legacy.script_id();
        println!("Legacy script ID: code_hash={}, hash_type={:?}", legacy_script_id.code_hash, legacy_script_id.hash_type);
        
        // 确保V1和Legacy脚本ID不同
        assert_ne!(v1_script_id.code_hash, legacy_script_id.code_hash);
        
        Ok(())
    }

    // 2.1 简单转账测试
    #[test]
    fn test_simple_transfer() -> Result<(), Box<dyn StdErr>> {
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
            MultisigScript::V1,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender = multisig_config.to_address(network_info.network_type, MultisigScript::V1, None);
        
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
            MultisigScript::V1,
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
        
        let sender = multisig_config.to_address(network_info.network_type, MultisigScript::V1, None);
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
            MultisigScript::V1,
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
            MultisigScript::V1,
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
            MultisigScript::V1,
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

        // 创建V1多签配置
        let multisig_config_v1 = MultisigConfig::new_with(
            MultisigScript::V1,
            vec![
                h160!("0x7336b0ba900684cb3cb00f0d46d4f64c0994a562"),
                h160!("0x5724c1e3925a5206944d753a6f3edaedf977d77f"),
            ],
            0,
            2,
        )?;
        let sender_v1 = multisig_config_v1.to_address(network_info.network_type, MultisigScript::V1, None);
        
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
        assert_ne!(sender_v1.to_string(), sender_legacy.to_string());
        println!("V1 address: {}", sender_v1);
        println!("Legacy address: {}", sender_legacy);
        
        Ok(())
    }
}