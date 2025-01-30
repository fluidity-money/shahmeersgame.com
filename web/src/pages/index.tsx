import { useEffect, useState } from "react";
import { ConnectButton } from "@rainbow-me/rainbowkit";
import type { NextPage } from "next";
import Head from "next/head";
import styles from "../styles/Home.module.css";
import { graphql } from "../gql";
import { useQuery } from '@apollo/client';
import { useReadContract, useReadContracts, useAccount } from 'wagmi';

const ideasQuery = graphql(`
  query getIdeas{
    ideas {
      time
      desc
      hash
    }
  }
`);

const explainMutation = graphql(`
  mutation explainIdea($desc: String!, $submitter: String!) {
    explainIdea(desc: $desc, submitter: $submitter)
  }
`);

const ShahmeersGame = {
  address: "0x9f98a61646eBF8C7c13394ad352C192eb90c740F",
  abi: [{"type":"function","name":"addVotes","inputs":[{"name":"concept","type":"bytes32","internalType":"bytes32"},{"name":"stg","type":"uint256","internalType":"uint256"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"nonpayable"},{"type":"function","name":"adjustVotes","inputs":[{"name":"adjustVotes","type":"tuple[]","internalType":"struct AdjustVotes[]","components":[{"name":"concept","type":"bytes32","internalType":"bytes32"},{"name":"amount","type":"int256","internalType":"int256"}]}],"outputs":[{"name":"stgDelta","type":"uint256[]","internalType":"uint256[]"}],"stateMutability":"nonpayable"},{"type":"function","name":"areWinnersPicked","inputs":[],"outputs":[{"name":"","type":"bool","internalType":"bool"}],"stateMutability":"view"},{"type":"function","name":"bumpEpoch","inputs":[],"outputs":[{"name":"","type":"uint64","internalType":"uint64"}],"stateMutability":"nonpayable"},{"type":"function","name":"chooseWinners","inputs":[{"name":"conceptCount","type":"uint256","internalType":"uint256"},{"name":"concepts","type":"bytes32[]","internalType":"bytes32[]"}],"outputs":[{"name":"","type":"tuple[]","internalType":"struct WinnersChosen[]","components":[{"name":"concept","type":"bytes32","internalType":"bytes32"},{"name":"amount","type":"uint256","internalType":"uint256"}]}],"stateMutability":"nonpayable"},{"type":"function","name":"drawDownWinner","inputs":[{"name":"epoch","type":"uint256","internalType":"uint256"},{"name":"concept","type":"bytes32","internalType":"bytes32"},{"name":"winner","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"nonpayable"},{"type":"function","name":"getSTG","inputs":[{"name":"concept","type":"bytes32","internalType":"bytes32"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"getUserSTGSpent","inputs":[{"name":"user","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"getUserVotes","inputs":[{"name":"concept","type":"bytes32","internalType":"bytes32"},{"name":"user","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"getVotes","inputs":[{"name":"concept","type":"bytes32","internalType":"bytes32"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"isConceptClaimable","inputs":[{"name":"concept","type":"bytes32","internalType":"bytes32"},{"name":"user","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"bool","internalType":"bool"}],"stateMutability":"view"},{"type":"function","name":"isConceptCorrect","inputs":[{"name":"concept","type":"bytes32","internalType":"bytes32"}],"outputs":[{"name":"","type":"bool","internalType":"bool"}],"stateMutability":"view"},{"type":"function","name":"pickWinnersThatAccomplished","inputs":[{"name":"epoch","type":"uint64","internalType":"uint64"},{"name":"concepts","type":"bytes32[]","internalType":"bytes32[]"}],"outputs":[],"stateMutability":"nonpayable"},{"type":"function","name":"register","inputs":[{"name":"concept","type":"bytes32","internalType":"bytes32"},{"name":"benificiary","type":"address","internalType":"address"}],"outputs":[],"stateMutability":"nonpayable"},{"type":"function","name":"startTime","inputs":[],"outputs":[{"name":"","type":"uint64","internalType":"uint64"}],"stateMutability":"view"},{"type":"function","name":"takeVotes","inputs":[{"name":"concept","type":"bytes32","internalType":"bytes32"},{"name":"stg","type":"uint256","internalType":"uint256"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"nonpayable"}]
} as const;

const SGToken = {
  address: "0xFDab24861F407765E6E64c282420585ef7cf68fe",
  abi: [{"type":"constructor","inputs":[],"stateMutability":"nonpayable"},{"type":"function","name":"CLOCK_MODE","inputs":[],"outputs":[{"name":"","type":"string","internalType":"string"}],"stateMutability":"pure"},{"type":"function","name":"DOMAIN_SEPARATOR","inputs":[],"outputs":[{"name":"","type":"bytes32","internalType":"bytes32"}],"stateMutability":"view"},{"type":"function","name":"allowance","inputs":[{"name":"owner","type":"address","internalType":"address"},{"name":"spender","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"approve","inputs":[{"name":"spender","type":"address","internalType":"address"},{"name":"value","type":"uint256","internalType":"uint256"}],"outputs":[{"name":"","type":"bool","internalType":"bool"}],"stateMutability":"nonpayable"},{"type":"function","name":"balanceOf","inputs":[{"name":"account","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"checkpoints","inputs":[{"name":"account","type":"address","internalType":"address"},{"name":"pos","type":"uint32","internalType":"uint32"}],"outputs":[{"name":"","type":"tuple","internalType":"struct Checkpoints.Checkpoint208","components":[{"name":"_key","type":"uint48","internalType":"uint48"},{"name":"_value","type":"uint208","internalType":"uint208"}]}],"stateMutability":"view"},{"type":"function","name":"clock","inputs":[],"outputs":[{"name":"","type":"uint48","internalType":"uint48"}],"stateMutability":"view"},{"type":"function","name":"decimals","inputs":[],"outputs":[{"name":"","type":"uint8","internalType":"uint8"}],"stateMutability":"view"},{"type":"function","name":"delegate","inputs":[{"name":"delegatee","type":"address","internalType":"address"}],"outputs":[],"stateMutability":"nonpayable"},{"type":"function","name":"delegateBySig","inputs":[{"name":"delegatee","type":"address","internalType":"address"},{"name":"nonce","type":"uint256","internalType":"uint256"},{"name":"expiry","type":"uint256","internalType":"uint256"},{"name":"v","type":"uint8","internalType":"uint8"},{"name":"r","type":"bytes32","internalType":"bytes32"},{"name":"s","type":"bytes32","internalType":"bytes32"}],"outputs":[],"stateMutability":"nonpayable"},{"type":"function","name":"delegates","inputs":[{"name":"account","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"address","internalType":"address"}],"stateMutability":"view"},{"type":"function","name":"eip712Domain","inputs":[],"outputs":[{"name":"fields","type":"bytes1","internalType":"bytes1"},{"name":"name","type":"string","internalType":"string"},{"name":"version","type":"string","internalType":"string"},{"name":"chainId","type":"uint256","internalType":"uint256"},{"name":"verifyingContract","type":"address","internalType":"address"},{"name":"salt","type":"bytes32","internalType":"bytes32"},{"name":"extensions","type":"uint256[]","internalType":"uint256[]"}],"stateMutability":"view"},{"type":"function","name":"getPastTotalSupply","inputs":[{"name":"timepoint","type":"uint256","internalType":"uint256"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"getPastVotes","inputs":[{"name":"account","type":"address","internalType":"address"},{"name":"timepoint","type":"uint256","internalType":"uint256"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"getVotes","inputs":[{"name":"account","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"name","inputs":[],"outputs":[{"name":"","type":"string","internalType":"string"}],"stateMutability":"view"},{"type":"function","name":"nonces","inputs":[{"name":"owner","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"numCheckpoints","inputs":[{"name":"account","type":"address","internalType":"address"}],"outputs":[{"name":"","type":"uint32","internalType":"uint32"}],"stateMutability":"view"},{"type":"function","name":"permit","inputs":[{"name":"owner","type":"address","internalType":"address"},{"name":"spender","type":"address","internalType":"address"},{"name":"value","type":"uint256","internalType":"uint256"},{"name":"deadline","type":"uint256","internalType":"uint256"},{"name":"v","type":"uint8","internalType":"uint8"},{"name":"r","type":"bytes32","internalType":"bytes32"},{"name":"s","type":"bytes32","internalType":"bytes32"}],"outputs":[],"stateMutability":"nonpayable"},{"type":"function","name":"symbol","inputs":[],"outputs":[{"name":"","type":"string","internalType":"string"}],"stateMutability":"view"},{"type":"function","name":"totalSupply","inputs":[],"outputs":[{"name":"","type":"uint256","internalType":"uint256"}],"stateMutability":"view"},{"type":"function","name":"transfer","inputs":[{"name":"to","type":"address","internalType":"address"},{"name":"value","type":"uint256","internalType":"uint256"}],"outputs":[{"name":"","type":"bool","internalType":"bool"}],"stateMutability":"nonpayable"},{"type":"function","name":"transferFrom","inputs":[{"name":"from","type":"address","internalType":"address"},{"name":"to","type":"address","internalType":"address"},{"name":"value","type":"uint256","internalType":"uint256"}],"outputs":[{"name":"","type":"bool","internalType":"bool"}],"stateMutability":"nonpayable"},{"type":"event","name":"Approval","inputs":[{"name":"owner","type":"address","indexed":true,"internalType":"address"},{"name":"spender","type":"address","indexed":true,"internalType":"address"},{"name":"value","type":"uint256","indexed":false,"internalType":"uint256"}],"anonymous":false},{"type":"event","name":"DelegateChanged","inputs":[{"name":"delegator","type":"address","indexed":true,"internalType":"address"},{"name":"fromDelegate","type":"address","indexed":true,"internalType":"address"},{"name":"toDelegate","type":"address","indexed":true,"internalType":"address"}],"anonymous":false},{"type":"event","name":"DelegateVotesChanged","inputs":[{"name":"delegate","type":"address","indexed":true,"internalType":"address"},{"name":"previousVotes","type":"uint256","indexed":false,"internalType":"uint256"},{"name":"newVotes","type":"uint256","indexed":false,"internalType":"uint256"}],"anonymous":false},{"type":"event","name":"EIP712DomainChanged","inputs":[],"anonymous":false},{"type":"event","name":"Transfer","inputs":[{"name":"from","type":"address","indexed":true,"internalType":"address"},{"name":"to","type":"address","indexed":true,"internalType":"address"},{"name":"value","type":"uint256","indexed":false,"internalType":"uint256"}],"anonymous":false},{"type":"error","name":"CheckpointUnorderedInsertion","inputs":[]},{"type":"error","name":"ECDSAInvalidSignature","inputs":[]},{"type":"error","name":"ECDSAInvalidSignatureLength","inputs":[{"name":"length","type":"uint256","internalType":"uint256"}]},{"type":"error","name":"ECDSAInvalidSignatureS","inputs":[{"name":"s","type":"bytes32","internalType":"bytes32"}]},{"type":"error","name":"ERC20ExceededSafeSupply","inputs":[{"name":"increasedSupply","type":"uint256","internalType":"uint256"},{"name":"cap","type":"uint256","internalType":"uint256"}]},{"type":"error","name":"ERC20InsufficientAllowance","inputs":[{"name":"spender","type":"address","internalType":"address"},{"name":"allowance","type":"uint256","internalType":"uint256"},{"name":"needed","type":"uint256","internalType":"uint256"}]},{"type":"error","name":"ERC20InsufficientBalance","inputs":[{"name":"sender","type":"address","internalType":"address"},{"name":"balance","type":"uint256","internalType":"uint256"},{"name":"needed","type":"uint256","internalType":"uint256"}]},{"type":"error","name":"ERC20InvalidApprover","inputs":[{"name":"approver","type":"address","internalType":"address"}]},{"type":"error","name":"ERC20InvalidReceiver","inputs":[{"name":"receiver","type":"address","internalType":"address"}]},{"type":"error","name":"ERC20InvalidSender","inputs":[{"name":"sender","type":"address","internalType":"address"}]},{"type":"error","name":"ERC20InvalidSpender","inputs":[{"name":"spender","type":"address","internalType":"address"}]},{"type":"error","name":"ERC2612ExpiredSignature","inputs":[{"name":"deadline","type":"uint256","internalType":"uint256"}]},{"type":"error","name":"ERC2612InvalidSigner","inputs":[{"name":"signer","type":"address","internalType":"address"},{"name":"owner","type":"address","internalType":"address"}]},{"type":"error","name":"ERC5805FutureLookup","inputs":[{"name":"timepoint","type":"uint256","internalType":"uint256"},{"name":"clock","type":"uint48","internalType":"uint48"}]},{"type":"error","name":"ERC6372InconsistentClock","inputs":[]},{"type":"error","name":"InvalidAccountNonce","inputs":[{"name":"account","type":"address","internalType":"address"},{"name":"currentNonce","type":"uint256","internalType":"uint256"}]},{"type":"error","name":"InvalidShortString","inputs":[]},{"type":"error","name":"SafeCastOverflowedUintDowncast","inputs":[{"name":"bits","type":"uint8","internalType":"uint8"},{"name":"value","type":"uint256","internalType":"uint256"}]},{"type":"error","name":"StringTooLong","inputs":[{"name":"str","type":"string","internalType":"string"}]},{"type":"error","name":"VotesExpiredSignature","inputs":[{"name":"expiry","type":"uint256","internalType":"uint256"}]}]
} as const;

const Lens = {
  address: "0x5f8049771d5f2fb86Bf64d92C2511Eff10818f73",
  abi: [{"type":"constructor","inputs":[{"name":"shahmeersGame","type":"address","internalType":"contract IShahmeersGame"}],"stateMutability":"nonpayable"},{"type":"function","name":"claimableForUser","inputs":[{"name":"user","type":"address","internalType":"address"},{"name":"concepts","type":"bytes32[]","internalType":"bytes32[]"}],"outputs":[{"name":"claimable","type":"bool[]","internalType":"bool[]"}],"stateMutability":"view"},{"type":"function","name":"getVotes","inputs":[{"name":"concepts","type":"bytes32[]","internalType":"bytes32[]"}],"outputs":[{"name":"votes","type":"uint256[]","internalType":"uint256[]"}],"stateMutability":"view"},{"type":"function","name":"userVoted","inputs":[{"name":"user","type":"address","internalType":"address"},{"name":"concepts","type":"bytes32[]","internalType":"bytes32[]"}],"outputs":[{"name":"voted","type":"uint256[]","internalType":"uint256[]"}],"stateMutability":"view"}]
} as const;

function zipThree<T, U, V>(arr1: T[], arr2: U[], arr3: V[]): [T, U, V][] {
  return arr1.map((_, i) => [arr1[i], arr2[i], arr3[i]]);
}

const Home: NextPage = () => {
  const { data } = useQuery(ideasQuery, {});
  const ideas = data ? data.ideas : [];
  const { address: address_ } = useAccount();
  const address = address_ ? address_ : "0x0000000000000000000000000000000000000000";
  const conceptHashes = (ideas.map(({hash}) => `0x${hash}`)) as readonly `0x${string}`[];
  const [userAllocatedAmounts, setUserAllocatedAmounts] = useState({});
  const { data: timepoint } = useReadContract({
    ...ShahmeersGame,
    functionName: "startTime"
  });
  const { data: contractResData } = useReadContracts({contracts: [
    {
      ...Lens,
      functionName: "getVotes",
      args: [conceptHashes]
    },
    {
      ...Lens,
      functionName: "userVoted",
      args: [address, conceptHashes]
    },
    {
      ...SGToken,
      functionName: "balanceOf",
      args: [address]
    },
    {
      ...SGToken,
      functionName: "getVotes",
      args: [address]
    },
    {
      ...SGToken,
      functionName: "getPastVotes",
      args: [address, timepoint ? timepoint : BigInt(0)]
    }
  ]});
  const conceptVotes = contractResData ? [...contractResData[0].result ? contractResData[0].result : []] : [];
  const userVotes = contractResData ? [...contractResData[1].result ? contractResData[1].result : []] : [];
  const sgtBal = contractResData ? contractResData[2].result : BigInt(0);
  const curVotes = contractResData ? contractResData[3].result : BigInt(0);
  const pastVotes = contractResData ? contractResData[4].result : BigInt(0);
  console.log("concept shit", contractResData);
  const concepts = (() => {
    if (!ideas || !conceptVotes || !userVotes) return [];
    return zipThree(ideas, conceptVotes, userVotes);
  })();
  concepts.sort((a, b) => (b[0] > a[0] ? 1 : -1));
  return (
    <div className={styles.container}>
      <Head>
        <title>Shahmeer&#39;s Game</title>
        <meta
          content="How will you play Shahmeer's Game?"
          name="description"
        />
        <link href="/favicon.ico" rel="icon" />
      </Head>

      <main className={styles.main}>
        <ConnectButton />

        <h1 className={styles.title}>
          Welcome To Shahmeer&#39;s Game
        </h1>

        <div className={styles.grid}>
          <div className={styles.card}>
            <h1>AIM</h1>
            <h3>Increase the percentage of users visiting 9lives minting by 2%.</h3>
          </div>

          <div className={styles.card}>
            <h2>How does this work?</h2>
            <p>
Shahmeer&#39;s game is a implementation of a product prediction market: STG tokens are
distributed to players when their ideas are included in a week's sprint, and they achieve
the target in the following week. In two months, SGT token will be LPd on Longtail with
USDC.
            </p>
          </div>

          <div className={styles.card}>
            <p>
Discussion is held in <b><a href="https://discord.gg/D8Yue858" target="_blank"
 rel="noopener noreferrer">Discord</a></b>.
            </p>
            <p>
Keep ideas simple! Let&#39;s make the most advanced prediction market: together.
            </p>
          </div>

          <div className={styles.card}>
            <h2>Your SGT (Shahmeer&#39;s Game Token)</h2>
            <h3>{sgtBal}</h3>
            <h2>Your remaining voting power for this epoch</h2>
            <h3>{pastVotes}</h3>
            <button>Update voting power</button>
          </div>

          {concepts.map(([{desc, time, hash}, userVotes, cumVotes]) =>
            <div className={styles.card}>
              <h3>{desc}</h3>
              <h4>Your votes: {userVotes}</h4>
              <h4>Cumulative votes: {cumVotes}</h4>
              <h1>+ -</h1>
              <button>Update</button>
            </div>)
          }

          <div className={styles.card}>
            <h2>Suggest idea</h2>
          </div>
        </div>
      </main>
    </div>
  );
};

export default Home;
