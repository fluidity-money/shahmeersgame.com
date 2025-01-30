// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "./IShahmeersGame.sol";

contract Lens {
    IShahmeersGame immutable SHAHMEERSGAME;

    constructor(IShahmeersGame shahmeersGame) {
        SHAHMEERSGAME = shahmeersGame;
    }

    function getVotes(
        bytes32[] calldata concepts
    ) external view returns (uint256[] memory votes) {
        votes = new uint256[](concepts.length);
        for (uint i = 0; i < concepts.length; ++i) {
            votes[i] = SHAHMEERSGAME.getVotes(concepts[i]);
        }
    }

    function userVoted(
        address user,
        bytes32[] calldata concepts
    ) external view returns (uint256[] memory voted) {
        voted = new uint256[](concepts.length);
        for (uint i = 0; i < concepts.length; ++i) {
            voted[i] = SHAHMEERSGAME.getUserVotes(concepts[i], user);
        }
    }

    function claimableForUser(
        address user,
        bytes32[] calldata concepts
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
