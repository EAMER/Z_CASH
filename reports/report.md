# CTS-Gate Test Report

## Summary

- **Total**: 5
- **Passed**: 4 ✅
- **Failed**: 1 ❌

- **Success Rate**: 80%

---

## Test Results

### Test 1: GetBlockRange - Invalid Range - ✅ PASS

**Method**: `GetBlockRange`

**Details**: Error expected and received


#### Expected

```json
{}
```

---

### Test 2: GetBlockRange - Valid Range - ❌ FAIL

**Method**: `GetBlockRange`

**Details**: Unexpected error: status: InvalidArgument, message: "GetBlock: getblock failed, error: error parsing block: error parsing transaction 0: fOverwinter flag must be set", details: [], metadata: MetadataMap { headers: {"server": "nginx/1.18.0 (Ubuntu)", "date": "Tue, 21 Apr 2026 13:11:09 GMT", "content-type": "application/grpc", "content-length": "0", "access-control-max-age": "1728000", "access-control-expose-headers": "grpc-status,grpc-message,grpc-status-details-bin", "strict-transport-security": "max-age=31536000; includeSubDomains", "x-frame-options": "DENY", "x-content-type-options": "nosniff", "x-xss-protection": "1; mode=block", "access-control-allow-origin": "https://testnet.cipherscan.app", "access-control-allow-methods": "POST, GET, OPTIONS", "access-control-allow-headers": "content-type,x-grpc-web,x-user-agent,grpc-timeout"} }


#### Expected

```json
[]
```

---

### Test 3: GetLatestBlock - Basic - ✅ PASS

**Method**: `GetLatestBlock`

**Details**: All validations passed


#### Expected

```json
{
  "hash": "non-empty",
  "height": ">0"
}
```

#### Actual

```json
{
  "hash": "a082dd1796306a8edb95b9b45c4ab919ccee5b27d9554cef5e932ec3aaa75000",
  "height": 3973902
}
```

---

### Test 4: GetLatestBlock - Intentional Fail Demo - ✅ PASS

**Method**: `GetLatestBlock`

**Details**: All validations passed


#### Expected

```json
{
  "height": "<10"
}
```

#### Actual

```json
{
  "hash": "a082dd1796306a8edb95b9b45c4ab919ccee5b27d9554cef5e932ec3aaa75000",
  "height": 3973902
}
```

---

### Test 5: GetLatestTreeState - Basic - ✅ PASS

**Method**: `GetLatestTreeState`

**Details**: All validations passed


#### Expected

```json
{
  "hash": "non-empty",
  "height": ">0"
}
```

#### Actual

```json
{
  "hash": "0050a7aac32e935eef4c55d9275beecc19b94a5cb4b995db8e6a309617dd82a0",
  "height": 3973902,
  "time": 1776777062,
  "tree": "0181dd7d33ea0ed65a82f1f1aa2500c5c5bccedbb5ac43702bca447f0e690abe01001f000001b5670cc387651a337e195d24fe168495ecab2e589197a635925a01aa4709011701a426d83727db65e1e911d6dddf42af210c17e6f3f3c59f017507dce0a9d6602d010c4eb34e3c37a457dacdc2db34af2368739e3e508c63da87767bfacabfb54e45017655aecbf45935be1f2624568b8cee35a246db1c63a5bd6dd67a4f93c9785a5b01984854fa5b5740eb0050ecdb8179a01948d2609a3c9fa588517dd334f576c0540001283784edf06bab946430e3e9f997fc1651db475a50037914b39ba8e055b4fd5000000000000192daea87f9b69435a62ab99f433c0a4146574f28158e729c2cf0ceeab185162c01dd49c2114699f3cdcbf7b573cf9fb0782b4d893bf068cccf83d7b69a63ad271600016c8ccd12a93c240aa208bfcd527270aab8afd0b7e9f160114a10335b8167d34900000000000000000000000000"
}
```

---

