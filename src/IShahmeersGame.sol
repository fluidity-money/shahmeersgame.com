// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

struct WinnersChosen {
    bytes32 concept;
    uint256 amount;
}

struct AdjustVotes {
    bytes32 concept;
    int256 amount;
}

interface IShahmeersGame {
    function register(bytes32 concept, address benificiary) external;

    function addVotes(bytes32 concept, uint256 stg) external returns (uint256);
    function takeVotes(bytes32 concept, uint256 stg) external returns (uint256);

    function adjustVotes(
        AdjustVotes[] calldata adjustVotes
    ) external returns (uint256[] memory stgDelta);

    function chooseWinners(
        uint256 conceptCount,
        bytes32[] calldata concepts
    ) external returns (WinnersChosen[] memory);

    function pickWinnersThatAccomplished(
        uint64 epoch,
        bytes32[] calldata concepts
    ) external;

    function drawDownWinner(
        uint256 epoch,
        bytes32 concept,
        address winner
    ) external returns (uint256);

    function bumpEpoch() external returns (uint64);

    function getVotes(bytes32 concept) external view returns (uint256);
    function getSTG(bytes32 concept) external view returns (uint256);
    function getUserVotes(bytes32 concept, address user) external view returns (uint256);

    function getUserSTGSpent(address user) external view returns (uint256);

    function areWinnersPicked() external view returns (bool);

    function isConceptCorrect(bytes32 concept) external view returns (bool);

    function isConceptClaimable(bytes32 concept, address user) external view returns (bool);

    function startTime() external view returns (uint64);
}
