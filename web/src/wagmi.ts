import { getDefaultConfig } from "@rainbow-me/rainbowkit";

export const superposition = {
    id: 55244,
    name: "Superposition",
    network: "superposition",
    iconUrl: "https://ipfs.io/ipfs/https://ipfs.io/ipfs/QmeXAvjMNnoVtrMRJZkxz7yVgWXTg7XkgvynPZifQzJuD2/logo.svg",
    iconBackground: "#000000",
    nativeCurrency: {
        decimals: 18,
        name: "ETH",
        symbol: "ETH"
    },
    rpcUrls: {
        default: {
            http: ["https://rpc.superposition.so"]
        },
    },
    blockExplorers: {
        default: { name: "CatScan", url: "https://explorer.superposition.so" }
    }
}

export const config = getDefaultConfig({
    appName: "Shahmeer's Game",
    projectId: "f986c6d0ab630fd316af799a5fc11327",
    chains: [superposition],
    ssr: true,
});
