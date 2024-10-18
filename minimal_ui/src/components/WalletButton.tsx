import { Button } from "@/components/ui/button";
import { useChain } from "@cosmos-kit/react";

const formatAddress = (address: string) => {
  return address.slice(0, 11) + "..." + address.slice(-3);
};

export const WalletButton = () => {
  const { address, connect } = useChain("neutronlocalnet", true);

  return (
    <Button
      variant={address ? "outline" : "default"}
      type="button"
      onClick={connect}
    >
      {address ? formatAddress(address) : "Connect Wallet"}
    </Button>
  );
};
