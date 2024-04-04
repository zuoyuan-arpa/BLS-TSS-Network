#!/bin/bash

# 设置变量
avs_directory_address="0x055733000064333CaDDbC92763c58BF0192fFeBf"
function_signature="calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)(bytes32)"
operator_address="0x7D40a1082e1685FBF66f3202cb5132147Bd42673"
avs_address="0x7D40a1082e1685FBF66f3202cb5132147Bd42673"
salt="0x4d4b520000000000000000000000000000000000000000000000000000000000"
expiry="134234235"
rpc_url="https://holesky.gateway.tenderly.co/4X987cMYL0drsf7LIZXlSr"
private_key="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
wallet_address="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"

# 调用calculateOperatorAVSRegistrationDigestHash函数
echo "Calling calculateOperatorAVSRegistrationDigestHash function..."
digest_hash=$(cast call $avs_directory_address "$function_signature" $operator_address $avs_address $salt $expiry --rpc-url $rpc_url)
echo "Digest hash: $digest_hash"

# 签名digest hash
echo "Signing digest hash..."
signature=$(cast wallet sign {$digest_hash} --private-key $private_key)
echo "Signature: $signature"

# 验证签名
echo "Verifying signature..."
cast wallet verify --address $wallet_address {$digest_hash} $signature