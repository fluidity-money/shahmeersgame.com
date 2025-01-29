// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

struct WinnersChosen {
    bytes8 concept;
    uint256 amount;
}

interface IShahmeersGame {
    function register(bytes8 concept, address benificiary) external;
    function addVotes(bytes8 concept, uint256 stg) external returns (uint256);
    function takeVotes(bytes8 concept, uint256 stg) external returns (uint256);

    function chooseWinners(
        uint256 conceptCount,
        bytes8[] calldata concepts
    ) external returns (WinnersChosen[] memory);

    function pickWinnersThatAccomplished(
        uint64 epoch,
        bytes8[] calldata concepts
    ) external;

    function drawDownWinner(
        uint256 epoch,
        bytes8 concept,
        address winner
    ) external returns (uint256);

    function bumpEpoch() external returns (uint64);
}
