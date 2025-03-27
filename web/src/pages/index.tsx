import { useEffect, useState, useMemo } from "react";
import { ConnectButton } from "@rainbow-me/rainbowkit";
import type { NextPage } from "next";
import Head from "next/head";
import styles from "../styles/Home.module.css";
import { useQuery } from '@apollo/client';
import {
  useReadContract,
  useReadContracts,
  useAccount,
  useWriteContract,
} from 'wagmi';
import { formatUnits, parseUnits, padHex } from "viem";
import { ShahmeersGame, SGToken, Lens } from "../contracts/contracts";
import { ideasQuery } from "../gql/query";

function zipThree<T, U, V>(arr1: T[], arr2: U[], arr3: V[]): [T, U, V][] {
  return arr1.map((_, i) => [arr1[i], arr2[i], arr3[i]]);
}

const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";

const Home: NextPage = () => {
  const { data } = useQuery(ideasQuery, {});
  const ideas = data ? data.ideas : [];
  const { address: address_ } = useAccount();
  const address = address_
    ? address_
    : ZERO_ADDRESS;
  const conceptHashes = ideas.map(
    ({ hash }) => `0x${hash}`
  ) as readonly `0x${string}`[];
  const [userAllocatedAmounts, setUserAllocatedAmounts] = useState(new Map<string, number>());

  const { data: timepoint } = useReadContract({
    ...ShahmeersGame,
    functionName: "startTime",
  });
  const { data: contractResData } = useReadContracts({
    contracts: [
      {
        ...Lens,
        functionName: "getVotes",
        args: [conceptHashes],
      },
      {
        ...Lens,
        functionName: "userVoted",
        args: [address, conceptHashes],
      },
      {
        ...SGToken,
        functionName: "balanceOf",
        args: [address],
      },
      {
        ...SGToken,
        functionName: "getVotes",
        args: [address],
      },
      {
        ...SGToken,
        functionName: "getPastVotes",
        args: [address, timepoint ? timepoint : BigInt(0)],
      },
      {
        ...ShahmeersGame,
        functionName: "getUserSTGSpent",
        args: [address],
      },
    ],
  });

  const convertSgtToVotes = (amount: bigint): number => {
    const sgt = Number(formatUnits(amount, 18));
    const votes = Math.round(Math.sqrt(sgt));       
    return votes;
  };

//  const conceptVotes = contractResData
//    ? [...(contractResData[0].result ? contractResData[0].result : [])]
//    : [];
//  const userVotes = contractResData
//    ? [...(contractResData[1].result ? contractResData[1].result : [])]
//    : [];

const conceptVotes = [...(contractResData?.[0]?.result ?? [])];
const userVotes = [...(contractResData?.[1]?.result ?? [])];

  const [sgtBal, setSgtBal] = useState(BigInt(0));

useEffect(() => {
    const newBalance = contractResData?.[2]?.result;
    if (newBalance !== undefined) {
      setSgtBal(newBalance);
    }
  }, [contractResData]);

  const curVotes = contractResData?.[3]?.result ?? BigInt(0);
  const pastVotes = contractResData?.[4]?.result ?? BigInt(0);
  const votesAlreadySpent = contractResData?.[5]?.result ?? BigInt(0);
  const [votesRemaining, setVotesRemaining] = useState(BigInt(0));

useEffect(() => {
    if (pastVotes !== undefined && votesAlreadySpent !== undefined) {
    setVotesRemaining(pastVotes - votesAlreadySpent);
    }
  }, [pastVotes, votesAlreadySpent]);

  const concepts = (() => {
    if (!ideas || !conceptVotes || !userVotes) return [];
    return zipThree(ideas, conceptVotes, userVotes);
  })();

  const sortedConcepts = useMemo(() => {
    return concepts.sort((a, b) => Number(b[0].time) - Number(a[0].time));
  }, [concepts]);

const calculateVotesCost = (currentVotes: number, action: "increase" | "decrease") => {
    const newVote = action === "increase" ? currentVotes + 1 : currentVotes - 1;
    const currentVotesCost = currentVotes ** 2;
    const newVotesCost = newVote ** 2;
  
    return {
      newVote,
      currentVotesCost,
      newVotesCost,
      sgtCostDelta: newVotesCost - currentVotesCost
    };
  };
  


const handleQuadraticVoting = (
    hash: string,
    action: "increase" | "decrease",
    currentVoteCount: number
  ) => {
    const { newVote, currentVotesCost, newVotesCost, sgtCostDelta } = calculateVotesCost(currentVoteCount, action);
  
    const sgtBalNum = Number(formatUnits(sgtBal, 18));
    const votesRemainingNum = Number(formatUnits(votesRemaining, 18));
  
    if (action === "increase" && votesRemainingNum <= 0) {
      alert("You don't have enough remaining votes");
      return;
    }
  
    if (action === "increase" && sgtBalNum < sgtCostDelta) {
      alert("You don't have enough SGT on your balance");
      return;
    }
  
    setUserAllocatedAmounts((prev) => {
      const currentVotes = prev.get(hash) ?? 0;
      const newVotes = action === "increase" ? currentVotes + 1 : currentVotes - 1;
      if (newVotes < 0) return prev;
  
      const newMap = new Map(prev);
      newMap.set(hash, newVotes);
      return newMap;
    });
  
    const newSGT = action === "increase"
      ? sgtBalNum - sgtCostDelta
      : sgtBalNum - sgtCostDelta;
  
    const newRemaining = action === "increase"
      ? votesRemainingNum - 1
      : votesRemainingNum + 1;
  
    setSgtBal(parseUnits(newSGT.toString(), 18));
    setVotesRemaining(parseUnits(newRemaining.toString(), 18));
  };
  
  const { writeContractAsync } = useWriteContract();
  


  const commitVotes = async () => {
    
    if (userAllocatedAmounts.size === 0) {
        alert("You haven't allocated any votes.");
        return;
    }

    const adjustVotesArray = Array.from(userAllocatedAmounts.entries()).map(([hash, amount]) => ({
        //concept: hash as `0x${string}`,
        concept: padHex(hash as `0x${string}`, { size: 32 }),
        //amount: BigInt(amount),
        amount: parseUnits(String(amount), 18)
    }));



    try {
        const tx = await writeContractAsync({
            ...ShahmeersGame,
            functionName: "adjustVotes",
            args: [adjustVotesArray],
        });
        alert("Votes committed successfully!");
        setUserAllocatedAmounts(new Map());
    } catch (err) {
        console.error("Error committing votes:", err);
        alert("Failed to commit votes. Please try again.");
    }
};

  return (
    <div className={styles.container}>
      <Head>
        <title>Shahmeer&#39;s Game</title>
        <meta content="How will you play Shahmeer's Game?" name="description" />
        <link href="/favicon.ico" rel="icon" />
      </Head>

      <main className={styles.main}>
        <ConnectButton />

        <h1 className={styles.title}>Welcome To Shahmeer&#39;s Game</h1>

        <div className={styles.grid}>
          <div className={styles.card}>
            <h1>AIM</h1>
            <h3>
              Increase the percentage of users visiting 9lives minting by 2%.
            </h3>
          </div>

          <div className={styles.card}>
            <h2>How does this work?</h2>
            <p>
              Shahmeer&#39;s game is a implementation of a product prediction
              market: STG tokens are distributed to players when their ideas are
              included in a week's sprint, and they achieve the target in the
              following week. In two months, SGT token will be LPd on Longtail
              with USDC.
            </p>
          </div>

          <div className={styles.card}>
            <p>
              Discussion is held in{" "}
              <b>
                <a
                  href="https://discord.gg/D8Yue858"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  Discord
                </a>
              </b>
              .
            </p>
            <p>
              Keep ideas simple! Let&#39;s make the most advanced prediction
              market: together.
            </p>
          </div>

          <div className={styles.card}>
            <h2>Your SGT (Shahmeer&#39;s Game Token)</h2>
            <h3>{formatUnits(sgtBal, 18)}</h3>
            <h2>Your remaining voting power for this epoch</h2>
            <h3>{formatUnits(votesRemaining, 18)}</h3>
            {/*<h2>Your future voting power</h2>
            <h3>{formatUnits(curVotes, 18)}</h3>*/}
            <button onClick={commitVotes}>Commit votes</button>
          </div>

          {sortedConcepts.length > 0 && sortedConcepts.map(([{ desc, time, hash }, userVotes, cumVotes]) =>
            (() => {

              if (
                hash === undefined ||
                userVotes === undefined ||
                cumVotes === undefined
              )
                return;

                const userVoteCount = convertSgtToVotes(userVotes);
                const cumVoteCount = convertSgtToVotes(cumVotes);
                const amt = userAllocatedAmounts.get(hash) ?? 0;
                
                const totalUserVotes = userVoteCount + amt;
                const totalCumVotes = cumVoteCount + amt;
                
              return (
                <div className={styles.card} key={hash}>
                  <h3>{desc}</h3>
                  <h4>Your votes: 
                    {totalUserVotes}
                    </h4>
                    
                  <h4>
                    Cumulative votes: 
                    {totalCumVotes}
                    </h4>
                  <h1>
                    <button
                    onClick={() => handleQuadraticVoting(hash, "increase", totalUserVotes)}
                    disabled={sgtBal === BigInt(0) || votesRemaining === BigInt(0) || votesRemaining < BigInt(0)}
                    >
                      +
                    </button>{" "}
                    <button
                        onClick={() => handleQuadraticVoting(hash, "decrease", totalUserVotes)}
                        disabled={userVotes + BigInt(amt) === BigInt(0)}
                    >
                      -
                    </button>
                  </h1>
                </div>
              );
            })()
          )}

          <div className={styles.card}>
            <h2>Suggest idea</h2>
          </div>
        </div>
      </main>
    </div>
  );
};

export default Home;
