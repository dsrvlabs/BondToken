# Near Contract Enviroment

해당 환경은 Near Contract 개발을 위한 것으로, `🧑‍💻Visual Studio Code`, `🐋Docker for Mac`을 필요로 합니다.

## 시작 방법

```sh
# near localnet을 background로 실행, 기본적으로 4개의 샤드 및 4개의 Account가 설정됨.
$ nearup localnet --binary-path /nearcore/target/release

# cli의 작동 모드를 local로 변경
$ export NODE_ENV=local

# node0의 full access key를 생성함.
# 참고: https://github.com/near/nearup/issues/64
$ near create-account yoonsung.node0 --masterAccount node0 --initialBalance 100000 --keyPath ~/.near/localnet/node0/validator_key.json
```

다양한 `near-cli` 사용은 [docs.near.org](https://docs.near.org/docs/roles/developer/contracts/cli) 에서 확인하실 수 있습니다.


## Build

```sh
# Contract가 위치한 폴더 내부
$ ./build.sh
```

## Test
```sh
# 전체 테스트
$ cargo test
# `fungible_token`만 테스트 하는 경우
$ cargo test --package fungible_token -- --nocapture
```

## Deploy
```sh
# 배포할 컨트랙트의 Account를 미리 생성
$ near create-account contract_name.yoonsung.node0 --masterAccount yoonsung.node0 --keyPath ~/.near/localnet/node0/validator_key.json
# 폴더 이동
$ cd fungible_token
# 컨트랙트 배포
$ near deploy --accountId contract_name.yoonsung.node0 --wasmFile ./res/fungible_token.wasm --keyPath ~/.near/localnet/node0/validator_key.json
# 이 단계에서 Node 접근에 대한 오류가 발생하면, nearup stop을 수행후 다시 노드들을 실행시켜 사용 가능.
```
