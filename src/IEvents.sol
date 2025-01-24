// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

interface IEvents {
    event Registered(bytes32 indexed concept, address indexed beneficiary);

    event WinnerChosen(
        bytes32 indexed concept,
        uint256 indexed stgToGain
    );

    event EpochBumped(uint256 indexed prevEpoch);

    event ConceptCameTrue(bytes32 indexed concept);

    event STGClaimed(
        bytes32 indexed concept,
        address indexed winner,
        uint256 indexed amt
     );
}
