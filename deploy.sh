#!/bin/sh -e

err() {
	>&2 echo $@
	exit 1
}

[ -z "$SPN_ADMIN_ADDR" ] && err "SPN_ADMIN_ADDR unset"
[ -z "$SPN_DILUTION_CONCEPT_AMT" ] && err "SPN_DILUTION_CONCEPT_AMT unset"
[ -z "$SPN_DILUTION_SUBMITTER_AMT" ] && err "SPN_DILUTION_SUBMITTER_AMT unset"
[ -z "$SPN_SHAHMEERSGAME_TOKEN_ADDR" ] && err "SPN_SHAHMEERSGAME_TOKEN_ADDR unset"

if [ -z "$SPN_SHAHMEERSGAME_IMPL" ]; then
	export SPN_SHAHMEERSGAME_IMPL="$(./deploy-stylus.sh shahmeersgame.wasm)"
fi

[ -z "$SPN_SHAHMEERSGAME_IMPL" ] && exit 1

echo "SPN_SHAHMEERSGAME_IMPL=$SPN_SHAHMEERSGAME_IMPL"

forge create \
	--broadcast \
	--rpc-url "$SPN_SUPERPOSITION_URL" \
	--private-key "$SPN_SUPERPOSITION_KEY" \
	foundry-libs/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol:TransparentUpgradeableProxy \
	--constructor-args "$impl" "$SPN_ADMIN_ADDR"  "$SPN_SHAHMEERSGAME_IMPL" "$(cast calldata 'ctor(address,address,uint256,uint256)' "$SPN_SHAHMEERSGAME_TOKEN_ADDR" "$SPN_ADMIN_ADDR" "$SPN_DILUTION_CONCEPT_AMT" "$SPN_DILUTION_SUBMITTER_AMT")" \
		| jq -r .deployedTo
