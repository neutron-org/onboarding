"use client";

import { AssetList, Chain } from "@chain-registry/types";
import { GasPrice } from '@cosmjs/stargate';
import { wallets as keplrExtension } from "@cosmos-kit/keplr-extension";
import { ChainProvider } from "@cosmos-kit/react";
import assert from "assert";
import { assets, chains } from "chain-registry";
import React from "react";
import "@interchain-ui/react/styles";

const localnetChain: Chain = (() => {
  const chain = chains.find((chain) => chain.chain_name === "neutrontestnet");
  assert(chain);
  return {
    ...chain,
    chain_id: "test-1",
    chain_name: "neutronlocalnet",
    pretty_name: "Neutron Localnet",
    apis: {
      ...chain.apis,
      rpc: [{ address: "http://localhost:3001/proxy" }],
      rest: [{ address: "http://localhost:3002/proxy" }],
    },
  };
})();

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
