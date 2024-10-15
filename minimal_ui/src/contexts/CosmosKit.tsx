"use client";

import { AssetList, Chain } from "@chain-registry/types";
import { GasPrice } from '@cosmjs/stargate';
// You can add more wallets here
import { wallets as keplrExtension } from "@cosmos-kit/keplr-extension";
import { ChainProvider } from "@cosmos-kit/react";
import assert from "assert";
import { assets, chains } from "chain-registry";
import React from "react";
import "@interchain-ui/react/styles";

// This is a Neutron Localnet chain, we need to add it manually because it's not in chain-registry
const localnetChain: Chain = (() => {
  const chain = chains.find((chain) => chain.chain_name === "neutrontestnet");
  assert(chain);
  return {
    ...chain,
    chain_id: "ntrntest",
    chain_name: "neutronlocalnet",
    pretty_name: "Neutron Localnet",
    apis: {
      ...chain.apis,
      rpc: [{ address: "http://localhost:3001/proxy" }],
      rest: [{ address: "http://localhost:3002/proxy" }],
    },
  };
})();

// The same for assets
const localnetAssets: AssetList = (() => {
  const asset = assets.find((asset) => asset.chain_name === "neutrontestnet");
  assert(asset);
  return {
    ...asset,
    chain_name: "neutronlocalnet",
  };
})();

export const CosmosKitProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => (
  <ChainProvider
    chains={[...chains, localnetChain]}
    assetLists={[...assets, localnetAssets]}
    // We need to specify gas price to be able to sign transactions
    // The provided value works just fine for the localnet
    signerOptions={{
      signingCosmwasm: () => ({
        gasPrice: GasPrice.fromString('0.01untrn'),
      }),
      signingStargate: () => ({
        gasPrice: GasPrice.fromString('0.01untrn'),
      }),
    }}
    wallets={[...keplrExtension]}
  >
    {children}
  </ChainProvider>
);
