// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "./IShahmeersGame.sol";

contract Lens {
    IShahmeersGame immutable SHAHMEERSGAME;

    constructor(IShahmeersGame shahmeersGame) {
        SHAHMEERSGAME = shahmeersGame;
    }

    function claimableForUser(
        address user,
        bytes8[] calldata concepts
    ) external view returns (bool[] memory claimable) {
        claimable = new bool[](concepts.length);
        for (uint i = 0; i < concepts.length; ++i) {
            claimable[i] = SHAHMEERSGAME.isConceptClaimable(
                concepts[i],
                user
            );
        }
    }
}
