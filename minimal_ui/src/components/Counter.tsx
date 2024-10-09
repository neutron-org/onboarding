import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useChain } from "@cosmos-kit/react";
import { useQuery } from "@tanstack/react-query";
import assert from "assert";
import { useCallback, useState } from "react";

const CONTRACT_ADDRESS =
  "neutron1nyuryl5u5z04dx4zsqgvsuw7fe8gl2f77yufynauuhklnnmnjncqcls0tj";

const useCounter = () => {
  const { address, getCosmWasmClient, getSigningCosmWasmClient } = useChain(
    "neutronlocalnet",
    true
  );

  const { data: value, refetch: refetchValue } = useQuery({
    queryKey: ["counter/value"],
    queryFn: async () => {
      const client = await getCosmWasmClient();

      const { current_value } = (await client.queryContractSmart(
        CONTRACT_ADDRESS,
        { current_value: {} }
      )) as { current_value: string };

      return current_value;
    },
  });

  const increaseValue = useCallback(
    async (amount: string) => {
      assert(address, "Address is required");

      const client = await getSigningCosmWasmClient();

      const { transactionHash } = await client.execute(
        address,
        CONTRACT_ADDRESS,
        {
          increase_count: {
            amount,
          },
        },
        "auto"
      );

      void refetchValue();

      return transactionHash;
    },
    [address, getSigningCosmWasmClient, refetchValue]
  );

  return { value, increaseValue };
};

export const Counter = () => {
  const { address } = useChain("neutronlocalnet", true);

  const { value, increaseValue } = useCounter();

  const [amount, setAmount] = useState("");

  const handleAmountChange = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      setAmount(event.currentTarget.value);
    },
    []
  );

  const handleIncreaseClick = useCallback(async () => {
    if (!address || !amount) return;

    const transactionHash = await increaseValue(amount);
    console.log(transactionHash);

    setAmount("");
  }, [address, amount, increaseValue]);

  return (
    <Card className="w-[350px]">
      <CardHeader>
        <CardTitle>Counter contract</CardTitle>
      </CardHeader>

      <CardContent>
        <div className="grid w-full items-center gap-4">
          <div className="flex flex-col space-y-1.5">
            <Label htmlFor="value">Current value</Label>
            <Input id="value" value={value ?? ""} disabled />
          </div>

          <div className="flex flex-col space-y-1.5">
            <Label htmlFor="amount">Amount to increase</Label>
            <Input
              id="amount"
              type="number"
              min="1"
              value={amount}
              onChange={handleAmountChange}
            />
          </div>
        </div>
      </CardContent>

      <CardFooter className="flex justify-between">
        <Button
          disabled={!address || !amount}
          type="button"
          onClick={handleIncreaseClick}
        >
          Increase
        </Button>
      </CardFooter>
    </Card>
  );
};
